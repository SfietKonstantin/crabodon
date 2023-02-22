pub use pretend::{Json, Result};

use pretend::pretend;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a user of Mastodon and their associated profile
///
/// Cf https://docs.joinmastodon.org/entities/Account/
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Account {
    /// The account id
    pub id: String,
    /// The username of the account, not including domain
    pub username: String,
    /// The Webfinger account URI
    ///
    /// Equal to `username` for local users, or `username@domain` for remote users.
    pub acct: String,
    /// The location of the user’s profile page
    pub url: String,
    /// The profile’s display name
    pub display_name: String,
    /// The profile’s bio or description
    pub note: String,
    /// An image icon that is shown next to statuses and in the profile
    pub avatar: String,
    /// A static version of the avatar
    ///
    /// Equal to `avatar` if its value is a static image; different if `avatar` is an animated GIF.
    pub avatar_static: String,
    /// An image banner that is shown above the profile and in profile cards
    pub header: String,
    /// A static version of the header
    ///
    /// Equal to `header` if its value is a static image; different if `header` is an animated GIF
    pub header_static: String,
    /// Whether the account manually approves follow requests
    pub locked: bool,
    /// Additional metadata attached to a profile as name-value pairs
    pub fields: Vec<AccountField>,
    // emojis
    /// Indicates that the account may perform automated actions,
    /// may not be monitored, or identifies as a robot
    pub bot: bool,
    /// Indicates that the account represents a Group actor
    pub group: bool,
    /// Whether the account has opted into discovery features such as the profile directory
    pub discoverable: Option<bool>,
    /// Whether the local user has opted out of being indexed by search engines
    pub noindex: Option< bool>,
    /// Indicates that the profile is currently inactive and that its user has moved to a new
    /// account
    pub moved: Option<Box<Account>>,
    /// An extra attribute returned only when an account is suspended
    pub suspended: Option<bool>,
    /// An extra attribute returned only when an account is silenced
    ///
    /// If true, indicates that the account should be hidden behind a warning screen.
    pub limited: Option<bool>,
    /// When the account was created
    pub created_at: DateTime<Utc>,
    /// When the most recent status was posted
    pub last_status_at: Option<DateTime<Utc>>,
    /// How many statuses are attached to this account
    pub statuses_count: i32,
    /// The reported followers of this profile
    pub followers_count: i32,
    /// The reported follows of this profile
    pub following_count: i32,
}

/// Field for an account
///
/// Cf https://docs.joinmastodon.org/entities/Account/#Field
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct AccountField {
    /// The key of a given field’s key-value pair
    pub name: String,
    /// The value associated with the `name` key
    pub value: String,
    /// Timestamp of when the server verified a URL value for a rel=“me” link
    pub verified_at: Option<DateTime<Utc>>,
}

/// Represents a status posted by an account
///
/// Cf https://docs.joinmastodon.org/entities/Status/
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Status {
    /// ID of the status in the database
    pub id: String,
    /// URI of the status used for federation
    pub uri: String,
    /// The date when this status was created
    pub created_at: DateTime<Utc>,
    /// The account that authored this status
    pub account: Account,
    /// HTML-encoded status content
    pub content: String,
    // visibility
    /// Is this status marked as sensitive content?
    pub sensitive: bool,
    /// Subject or summary line, below which status content is collapsed until expanded
    pub spoiler_text: String,
    // media_attachments
    /// The application used to post this status
    pub application: Option<Application>,
    /// Mentions of users within the status content
    pub mentions: Vec<Mention>,
    /// Hashtags used within the status content
    pub tags: Vec<Tag>,
    // emojis
    /// How many boosts this status has received
    pub reblogs_count: i32,
    /// How many favourites this status has received
    pub favourites_count: i32,
    /// How many replies this status has received
    pub replies_count: u32,
    /// A link to the status’s HTML representation
    pub url: Option<String>,
    /// ID of the status being replied to
    pub in_reply_to_id: Option<String>,
    /// ID of the account that authored the status being replied to
    pub in_reply_to_account_id: Option<String>,
    /// The status being reblogged
    pub reblog: Option<Box<Status>>,
    // poll
    // card
    /// Primary language of this status
    pub language: Option<String>,
    /// Plain-text source of a status.
    ///
    /// Returned instead of content when status is deleted, so the user may redraft from the
    /// source text without the client having to reverse-engineer the original text from the
    /// HTML content
    pub text: Option<String>,
    /// Timestamp of when the status was last edited
    pub edited_at: Option<DateTime<Utc>>,
    /// If the current token has an authorized user: Have you favourited this status?
    pub favourited: Option<bool>,
    /// If the current token has an authorized user: Have you boosted this status?
    pub reblogged: Option<bool>,
    /// If the current token has an authorized user: Have you muted notifications for this status’s conversation?
    pub muted: Option<bool>,
    /// If the current token has an authorized user: Have you bookmarked this status?
    pub bookmarked: Option<bool>,
    /// If the current token has an authorized user: Have you pinned this status? Only appears if the status is pinnable.
    pub pinned: Option<bool>,
    // filtered
}

/// A simplified application for a status
///
/// Cf https://docs.joinmastodon.org/entities/Application/
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Application {
    /// The name of your application
    pub name: String,
    /// The website associated with your application
    pub website: Option<String>,
}

/// Mention for a status
///
/// Cf https://docs.joinmastodon.org/entities/Status/#Mention
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Mention {
    /// The account ID of the mentioned user
    pub id: String,
    /// The username of the mentioned user
    pub username: String,
    /// The location of the mentioned user’s profile
    pub url: String,
    /// The Webfinger account URI
    ///
    /// Equal to `username` for local users, or `username@domain` for remote users.
    pub acct: String,
}

/// Tag for a status
///
/// Cf https://docs.joinmastodon.org/entities/Status/#Tag
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Tag {
    /// The value of the hashtag after the # sign
    pub name: String,
    /// A link to the hashtag on the instance
    pub url: String,
}

/// Represents an application that interfaces with the REST API to access accounts or post statuses
///
/// Cf https://docs.joinmastodon.org/entities/Application/
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct AuthApplication {
    /// The name of your application
    pub name: String,
    /// The website associated with your application
    pub website: Option<String>,
    /// Used for Push Streaming API
    ///
    /// Returned with POST /api/v1/apps. Equivalent to WebPushSubscription#server_key
    pub vapid_key: String,
    /// Client ID key, to be used for obtaining OAuth tokens
    pub client_id: String,
    /// Client secret key, to be used for obtaining OAuth tokens
    pub client_secret: String,
}

/// Represents an OAuth token used for authenticating with the API and performing actions.
///
/// Cf https://docs.joinmastodon.org/entities/Token/
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Token {
    /// An OAuth token to be used for authorization
    pub access_token: String,
    /// The OAuth token type
    ///
    /// Mastodon uses `Bearer` tokens.
    pub token_type: Option<String>,
    /// The OAuth scopes granted by this token, space-separated
    pub scope: String,
    /// When the token was generated
    pub created_at: i64,
}




#[derive(Debug, Serialize)]
pub struct ApplicationBody {
    client_name: String,
    redirect_uris: String,
    scopes: Option<String>,
    website: Option<String>,
}

impl ApplicationBody {
    /// Constructor
    pub fn new(
        client_name: String,
        redirect_uris: String,
        scopes: Option<String>,
        website: Option<String>,
    ) -> Self {
        ApplicationBody {
            client_name,
            redirect_uris,
            scopes,
            website,
        }
    }

    pub fn for_client(client_name: String, redirect_uris: String) -> Self {
        Self::new(client_name, redirect_uris, None, None)
    }

    pub fn with_scopes(mut self, scopes: String) -> Self {
        self.scopes = Some(scopes);
        self
    }

    pub fn with_website(mut self, website: String) -> Self {
        self.website = Some(website);
        self
    }
}

#[derive(Debug, Serialize)]
pub struct TokenBody {
    grant_type: String,
    code: Option<String>,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    scope: Option<String>,
}

impl TokenBody {
    /// Constructor
    pub fn new(
        grant_type: String,
        code: Option<String>,
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        scope: Option<String>,
    ) -> Self {
        TokenBody {
            grant_type,
            code,
            client_id,
            client_secret,
            redirect_uri,
            scope,
        }
    }

    pub fn with_code(
        code: String,
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    ) -> Self {
        Self::new(
            "authorization_code".to_string(),
            Some(code),
            client_id,
            client_secret,
            redirect_uri,
            None,
        )
    }

    pub fn with_scope(mut self, scope: String) -> Self {
        self.scope = Some(scope);
        self
    }
}

/// Mastodon authentication REST API
///
/// This pretend-extended trait implements (some) Mastodon
/// application and OAuth endpoints.
#[pretend]
pub trait MastodonAuthApi {
    /// Create an application
    ///
    /// Cf https://docs.joinmastodon.org/methods/apps/#create
    #[request(method = "POST", path = "/api/v1/apps")]
    async fn post_application(&self, json: ApplicationBody) -> Result<Json<AuthApplication>>;

    /// Obtain a token
    ///
    /// Cf https://docs.joinmastodon.org/methods/oauth/#token
    #[request(method = "POST", path = "/api/v1/oauth/token")]
    async fn post_token(&self, json: TokenBody) -> Result<Json<Token>>;
}

/// Mastodon REST API
///
/// This pretend-extended trait implements (some) Mastodon
/// REST endpoints.
#[pretend]
pub trait MastodonApi {
    /// View public timeline
    ///
    /// Cf https://docs.joinmastodon.org/methods/timelines/#public
    #[request(method = "GET", path = "/api/v1/timelines/public")]
    async fn get_public_timeline(&self) -> Result<Json<Vec<Status>>>;

    /// View a single status
    ///
    /// Cf https://docs.joinmastodon.org/methods/statuses/#get
    #[request(method = "GET", path = "/api/v1/statuses/{id}")]
    async fn get_status(&self, id: String) -> Result<Json<Status>>;
}
