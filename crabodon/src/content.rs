//! Parse content
//!
//! Mastodon content is written with a subset of HTML, where links, mentions and hashtags
//! are all modelled with the `<a>` tag. Parsing HTML is a hard task in general.
//!
//! This module uses a real HTML parser to deal with statuses in a safe and user-friendly manner.
//! There are 2 main facilities to parse status content. The lower level module [`visit`] offers
//! a visitor-like trait that maps closely to the HTML representation of a status, while the
//! higher level [`parse`], that provides a processed, tree-like representation of Mastodon content.
//!
//! This module only contain shared structures for those modules. See the documentation for each
//! of them for more information.

pub mod parse;
pub mod visit;

/// A link
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct Link {
    /// href
    ///
    /// ie link
    pub href: String,
}

/// A mention
///
/// A mention is a specific kind of link.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct Mention {
    /// href
    ///
    /// ie link for this mention
    href: String,
    /// Host of the user
    ///
    /// Hostname of the Mastodon instance this user is in,
    /// without the @ prefix.
    host: String,
    /// User
    ///
    /// Always have the @ prefix
    user: String,
}

/// A hashtag
///
/// A hashtag is a specific kind of link.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct Hashtag {
    /// href
    ///
    /// ie link for this hashtag
    href: String,
    /// Hashtag
    ///
    /// Without the # prefix
    tag: String,
}

/// A list specifying types of Mastodon links
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum LinkKind {
    /// A link
    Link(Link),
    /// A mention
    Mention(Mention),
    /// A hashtag
    Hashtag(Hashtag),
}
