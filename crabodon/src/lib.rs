//! `crabodon` Mastodon client
//!
//! The goal of `crabodon` is to assist in building Mastodon clients.
//!
//! # Status content
//!
//! Mastodon status content content is written in HTML, and HTML is notoriously hard to parse
//! correctly. `crabodon` ships the [`content`] module to help dealing with them.

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod content;
