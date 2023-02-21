//! Parse content to a higher level representation
//!
//! This module offers [`parse_content`], that parse a Mastodon content
//! to a high level representation.
//!
//! This representation is the following:
//!
//! - A content is a list of paragraphs
//! - A paragraph is a list of [`ParagraphNode`]
//! - A paragraph node can be a link, a text or a new line
//! - A link (link, mention or hashtag) contain a list of [`LinkNode`]
//! - A link node can be a text or a new line

use super::visit;
use super::LinkKind;
use std::mem;

/// A paragraph node
///
/// This node represent every element that
/// can appear inside a paragraph
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ParagraphNode {
    /// A link
    Link(LinkKind, Vec<LinkNode>),
    /// A text
    Text(String),
    /// A new line
    NewLine,
}

/// A link node
///
/// This node represent every element that
/// can appear inside a link
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum LinkNode {
    /// A text
    Text(String),
    /// A new line
    NewLine,
}

/// Parse content
///
/// This function extracts information from Mastodon content
/// as list of paragraphs. Each paragraph is a list of paragraph nodes.
pub fn parse_content(content: &str) -> Vec<Vec<ParagraphNode>> {
    visit::visit_content(content, ParseVisitor::default())
}

#[derive(Default)]
struct ParseVisitor {
    paragraphs: Vec<Vec<ParagraphNode>>,
    paragraph_count: usize,
    link_count: usize,
    current_paragraph: Vec<ParagraphNode>,
    current_link: Vec<LinkNode>,
}

impl visit::Visit for ParseVisitor {
    type Output = Vec<Vec<ParagraphNode>>;

    fn text(&mut self, text: String) {
        if self.paragraph_count > 0 {
            if self.link_count > 0 {
                self.current_link.push(LinkNode::Text(text))
            } else {
                self.current_paragraph.push(ParagraphNode::Text(text))
            }
        }
    }

    fn new_line(&mut self) {
        if self.paragraph_count > 0 {
            if self.link_count > 0 {
                self.current_link.push(LinkNode::NewLine)
            } else {
                self.current_paragraph.push(ParagraphNode::NewLine)
            }
        }
    }

    fn begin_paragraph(&mut self) {
        self.paragraph_count += 1;
    }

    fn end_paragraph(&mut self) {
        self.paragraph_count = self.paragraph_count.saturating_sub(1);
        if self.paragraph_count == 0 {
            let paragraph = mem::take(&mut self.current_paragraph);
            self.paragraphs.push(paragraph);
        }
    }

    fn begin_link(&mut self, _link: &LinkKind) {
        self.link_count += 1;
    }

    fn end_link(&mut self, link: &LinkKind) {
        self.link_count = self.link_count.saturating_sub(1);
        if self.link_count == 0 {
            let children = mem::take(&mut self.current_link);
            self.current_paragraph
                .push(ParagraphNode::Link(link.clone(), children));
        }
    }

    fn finalize(self) -> Self::Output {
        self.paragraphs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{Hashtag, Link, Mention};

    #[test]
    fn test_real_with_hashtags() {
        // 109805244883278164 on mastodon.social
        let content = include_str!("tests/real_with_hashtags.html");
        let expected = vec![
            vec![ParagraphNode::Text(
                "I have a feeling this will appeal to multiple people for multiple reasons."
                    .to_string(),
            )],
            vec![
                ParagraphNode::Text("[original source: ".to_string()),
                ParagraphNode::Link(
                    LinkKind::Link(Link {
                        href: "https://www.reddit.com/r/comics/comments/10rukp8/oc_magic_coding/"
                            .to_string(),
                    }),
                    vec![LinkNode::Text(
                        "reddit.com/r/comics/comments/1…".to_string(),
                    )],
                ),
                ParagraphNode::Text("]".to_string()),
            ],
            vec![
                ParagraphNode::Link(
                    LinkKind::Hashtag(Hashtag {
                        href: "https://dice.camp/tags/ttrpg".to_string(),
                        tag: "ttrpg".to_string(),
                    }),
                    vec![LinkNode::Text("#ttrpg".to_string())],
                ),
                ParagraphNode::Text(" ".to_string()),
                ParagraphNode::Link(
                    LinkKind::Hashtag(Hashtag {
                        href: "https://dice.camp/tags/magic".to_string(),
                        tag: "magic".to_string(),
                    }),
                    vec![LinkNode::Text("#magic".to_string())],
                ),
                ParagraphNode::Text(" ".to_string()),
                ParagraphNode::Link(
                    LinkKind::Hashtag(Hashtag {
                        href: "https://dice.camp/tags/coding".to_string(),
                        tag: "coding".to_string(),
                    }),
                    vec![LinkNode::Text("#coding".to_string())],
                ),
            ],
        ];
        assert_eq!(parse_content(content), expected);
    }

    #[test]
    fn test_real_with_mentions() {
        // 109818097593839444 on mastodon.social
        let content = include_str!("tests/real_with_mentions.html");
        let expected = vec![
            vec![
                ParagraphNode::Link(
                    LinkKind::Mention(Mention {
                        href: "https://mastodon.org.uk/@cybette".to_string(),
                        host: "mastodon.org.uk".to_string(),
                        user: "@cybette".to_string(),
                    }),
                    vec![LinkNode::Text("@cybette".to_string())],
                ),
                ParagraphNode::Text(" nice ! That's way better :)".to_string()),
            ],
            vec![
                ParagraphNode::Text(
                    "So basically, you had to take 2 sets of stickers. One for FOSDEM and one for "
                        .to_string(),
                ),
                ParagraphNode::Link(
                    LinkKind::Mention(Mention {
                        href: "https://fosstodon.org/@cfgmgmtcamp".to_string(),
                        host: "fosstodon.org".to_string(),
                        user: "@cfgmgmtcamp".to_string(),
                    }),
                    vec![LinkNode::Text("@cfgmgmtcamp".to_string())],
                ),
                ParagraphNode::Text("  ?".to_string()),
            ],
        ];
        assert_eq!(parse_content(content), expected);
    }

    #[test]
    fn test_real_with_newlines() {
        // 109882001535463183 on mastodon.social
        let content = include_str!("tests/real_with_newlines.html");
        let expected = vec![vec![
            ParagraphNode::Text("Test 1 please ignore".to_string()),
            ParagraphNode::NewLine,
            ParagraphNode::Text("Test 1 (cont)".to_string()),
        ]];
        assert_eq!(parse_content(content), expected);
    }
}
