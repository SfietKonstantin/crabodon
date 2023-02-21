//! Parse content using a visitor
//!
//! This module offers a visitor-based system that maps closely to Mastodon's
//! HTML representation.
//!
//! Implement [`Visit`] to be notified about elements in the content. A visitor
//! is used with [`visit_content`].

use super::{Hashtag, Link, LinkKind, Mention};
use kuchiki::traits::TendrilSink;
use kuchiki::{parse_html, ElementData, NodeData, NodeRef};
use std::collections::HashSet;
use std::mem;
use url::Url;

/// A trait to visit a Mastodon content
///
/// Implement this trait to parse some content tag by tag. Visitor-like methods
/// notifies about the elements of the content and `finalize` allows building an
/// output type.
///
/// This trait should be used with [`visit_content`]
pub trait Visit {
    /// Output type
    type Output;

    /// Text
    ///
    /// Implement this method to be notified about text
    /// (inside elements). Can be called several times
    /// (once per element)
    fn text(&mut self, _text: String) {}

    /// New line
    ///
    /// Implement this method to be notified about a new line.
    ///
    /// This usually happens inside a paragraph element.
    fn new_line(&mut self) {}

    /// Start of a paragraph
    ///
    /// Implement this method to be notified about the
    /// start of a paragraph element.
    ///
    /// This element can be followed by any other element,
    /// usually text, links, mentions or hashtags.
    ///
    /// It should not be followed by other paragraph
    /// elements.
    fn begin_paragraph(&mut self) {}

    /// End of a paragraph
    ///
    /// Implement this method to be notified about the
    /// end of a paragraph element.
    fn end_paragraph(&mut self) {}

    /// Start of a link
    ///
    /// Implement this method to be notified about the
    /// start of a link. The [`LinkKind`] contain the
    /// type of link being visited, either a link, a
    /// mention or a hashtag.
    ///
    /// This element usually contains text elements.
    ///
    /// In case of a mention, the text will start with an @
    /// and in case of a hashtag, it will start with a #.
    ///
    ///
    fn begin_link(&mut self, _link: &LinkKind) {}

    /// End of a link
    ///
    /// Implement this method to be notified about the
    /// end of a link.
    fn end_link(&mut self, _link: &LinkKind) {}

    /// The end of the content has been reached
    ///
    /// Output must be produced at that step.
    fn finalize(self) -> Self::Output;
}

/// Visit content
///
/// This function uses an implementation of [`Visit`]
/// to extract information from a Mastodon content.
pub fn visit_content<V>(content: &str, visitor: V) -> V::Output
where
    V: Visit,
{
    let node = parse_html().one(content);
    Parser::new(visitor).parse(node)
}

enum VisitKind {
    Nothing,
    Children,
    Text(String),
    NewLine,
    Ellipsis,
    Paragraph,
    Link(LinkKind),
}

struct Parser<V> {
    visitor: V,
    current_text: String,
}

impl<V> Parser<V>
where
    V: Visit,
{
    fn new(visitor: V) -> Self {
        Parser {
            visitor,
            current_text: String::new(),
        }
    }

    fn parse(mut self, node: NodeRef) -> V::Output {
        self.visit(node);
        self.commit_string();
        self.visitor.finalize()
    }

    fn visit(&mut self, node: NodeRef) {
        let kind = self.find_visit_kind(&node);
        match kind {
            VisitKind::Nothing => {}
            VisitKind::Children => self.visit_children(node),
            VisitKind::Text(text) => self.current_text.push_str(&text),
            VisitKind::NewLine => {
                self.commit_string();
                self.visitor.new_line()
            }
            VisitKind::Ellipsis => {
                self.visit_children(node);
                self.current_text.push('…');
            }
            VisitKind::Paragraph => {
                self.commit_string();
                self.visitor.begin_paragraph();
                self.visit_children(node);
                self.commit_string();
                self.visitor.end_paragraph();
            }
            VisitKind::Link(link) => {
                self.commit_string();
                self.visitor.begin_link(&link);
                self.visit_children(node);
                self.commit_string();
                self.visitor.end_link(&link);
            }
        }
    }

    fn find_visit_kind(&self, node: &NodeRef) -> VisitKind {
        match node.data() {
            NodeData::Element(element) => Self::find_visit_kind_element(element),
            NodeData::Text(text) => {
                let text = text.borrow();
                VisitKind::Text(text.clone())
            }
            _ => VisitKind::Children,
        }
    }

    fn find_visit_kind_element(element: &ElementData) -> VisitKind {
        match &*element.name.local {
            "p" => VisitKind::Paragraph,
            "a" => {
                let attributes = element.attributes.borrow();
                let class = attributes.get("class").unwrap_or("");
                let classes = class.split(' ').collect::<HashSet<_>>();
                let href = attributes.get("href").unwrap_or("");
                VisitKind::Link(Self::parse_href(href.to_string(), classes))
            }
            "span" => {
                let attributes = element.attributes.borrow();
                let class = attributes.get("class").unwrap_or("");
                match class {
                    "invisible" => VisitKind::Nothing,
                    "ellipsis" => VisitKind::Ellipsis,
                    _ => VisitKind::Children,
                }
            }
            "br" => VisitKind::NewLine,
            _ => VisitKind::Children,
        }
    }

    fn parse_href(href: String, classes: HashSet<&str>) -> LinkKind {
        if let Some(element) = Self::parse_special_href(&href, classes) {
            element
        } else {
            LinkKind::Link(Link { href })
        }
    }

    fn parse_special_href(href: &str, classes: HashSet<&str>) -> Option<LinkKind> {
        let url = Url::parse(href).ok()?;
        if classes.contains("hashtag") {
            let segments = url.path_segments()?;
            let tag = segments.last()?;
            Some(LinkKind::Hashtag(Hashtag {
                href: href.to_string(),
                tag: tag.to_string(),
            }))
        } else if classes.contains("mention") {
            let host = url.host_str()?;
            let segments = url.path_segments()?;
            let user = segments.last()?;
            Some(LinkKind::Mention(Mention {
                href: href.to_string(),
                host: host.to_string(),
                user: user.to_string(),
            }))
        } else {
            None
        }
    }

    fn visit_children(&mut self, node: NodeRef) {
        for child in node.children() {
            self.visit(child)
        }
    }

    fn commit_string(&mut self) {
        let text = mem::take(&mut self.current_text);
        if !text.is_empty() {
            self.visitor.text(text);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Eq, PartialEq)]
    enum Node {
        Text(String),
        NewLine,
        BeginParagraph,
        EndParagraph,
        BeginLink(LinkKind),
        EndLink(LinkKind),
    }

    impl Visit for Vec<Node> {
        type Output = Self;

        fn text(&mut self, text: String) {
            self.push(Node::Text(text))
        }

        fn new_line(&mut self) {
            self.push(Node::NewLine)
        }

        fn begin_paragraph(&mut self) {
            self.push(Node::BeginParagraph)
        }

        fn end_paragraph(&mut self) {
            self.push(Node::EndParagraph)
        }

        fn begin_link(&mut self, link: &LinkKind) {
            self.push(Node::BeginLink(link.clone()))
        }

        fn end_link(&mut self, link: &LinkKind) {
            self.push(Node::EndLink(link.clone()))
        }

        fn finalize(self) -> Self::Output {
            self
        }
    }

    #[test]
    fn test_real_with_hashtags() {
        // 109805244883278164 on mastodon.social
        let content = include_str!("tests/real_with_hashtags.html");
        let expected = vec![
            Node::BeginParagraph,
            Node::Text(
                "I have a feeling this will appeal to multiple people for multiple reasons."
                    .to_string(),
            ),
            Node::EndParagraph,
            Node::BeginParagraph,
            Node::Text("[original source: ".to_string()),
            Node::BeginLink(LinkKind::Link(Link {
                href: "https://www.reddit.com/r/comics/comments/10rukp8/oc_magic_coding/"
                    .to_string(),
            })),
            Node::Text("reddit.com/r/comics/comments/1…".to_string()),
            Node::EndLink(LinkKind::Link(Link {
                href: "https://www.reddit.com/r/comics/comments/10rukp8/oc_magic_coding/"
                    .to_string(),
            })),
            Node::Text("]".to_string()),
            Node::EndParagraph,
            Node::BeginParagraph,
            Node::BeginLink(LinkKind::Hashtag(Hashtag {
                href: "https://dice.camp/tags/ttrpg".to_string(),
                tag: "ttrpg".to_string(),
            })),
            Node::Text("#ttrpg".to_string()),
            Node::EndLink(LinkKind::Hashtag(Hashtag {
                href: "https://dice.camp/tags/ttrpg".to_string(),
                tag: "ttrpg".to_string(),
            })),
            Node::Text(" ".to_string()),
            Node::BeginLink(LinkKind::Hashtag(Hashtag {
                href: "https://dice.camp/tags/magic".to_string(),
                tag: "magic".to_string(),
            })),
            Node::Text("#magic".to_string()),
            Node::EndLink(LinkKind::Hashtag(Hashtag {
                href: "https://dice.camp/tags/magic".to_string(),
                tag: "magic".to_string(),
            })),
            Node::Text(" ".to_string()),
            Node::BeginLink(LinkKind::Hashtag(Hashtag {
                href: "https://dice.camp/tags/coding".to_string(),
                tag: "coding".to_string(),
            })),
            Node::Text("#coding".to_string()),
            Node::EndLink(LinkKind::Hashtag(Hashtag {
                href: "https://dice.camp/tags/coding".to_string(),
                tag: "coding".to_string(),
            })),
            Node::EndParagraph,
        ];
        assert_eq!(visit_content(content, Vec::new()), expected);
    }

    #[test]
    fn test_real_with_mentions() {
        // 109818097593839444 on mastodon.social
        let content = include_str!("tests/real_with_mentions.html");
        let expected = vec![
            Node::BeginParagraph,
            Node::BeginLink(LinkKind::Mention(Mention {
                href: "https://mastodon.org.uk/@cybette".to_string(),
                host: "mastodon.org.uk".to_string(),
                user: "@cybette".to_string(),
            })),
            Node::Text("@cybette".to_string()),
            Node::EndLink(LinkKind::Mention(Mention {
                href: "https://mastodon.org.uk/@cybette".to_string(),
                host: "mastodon.org.uk".to_string(),
                user: "@cybette".to_string(),
            })),
            Node::Text(" nice ! That's way better :)".to_string()),
            Node::EndParagraph,
            Node::BeginParagraph,
            Node::Text(
                "So basically, you had to take 2 sets of stickers. One for FOSDEM and one for "
                    .to_string(),
            ),
            Node::BeginLink(LinkKind::Mention(Mention {
                href: "https://fosstodon.org/@cfgmgmtcamp".to_string(),
                host: "fosstodon.org".to_string(),
                user: "@cfgmgmtcamp".to_string(),
            })),
            Node::Text("@cfgmgmtcamp".to_string()),
            Node::EndLink(LinkKind::Mention(Mention {
                href: "https://fosstodon.org/@cfgmgmtcamp".to_string(),
                host: "fosstodon.org".to_string(),
                user: "@cfgmgmtcamp".to_string(),
            })),
            Node::Text("  ?".to_string()),
            Node::EndParagraph,
        ];
        assert_eq!(visit_content(content, Vec::new()), expected);
    }

    #[test]
    fn test_real_with_newlines() {
        // 109882001535463183 on mastodon.social
        let content = include_str!("tests/real_with_newlines.html");
        let expected = vec![
            Node::BeginParagraph,
            Node::Text("Test 1 please ignore".to_string()),
            Node::NewLine,
            Node::Text("Test 1 (cont)".to_string()),
            Node::EndParagraph,
        ];
        assert_eq!(visit_content(content, Vec::new()), expected);
    }
}
