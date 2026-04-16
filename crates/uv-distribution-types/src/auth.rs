use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum AuthPolicy {
    #[default]
    Auto,
    Always,
    Never,
}

impl PartialOrd for AuthPolicy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AuthPolicy {
    fn cmp(&self, other: &Self) -> Ordering {
        fn rank(p: &AuthPolicy) -> u8 {
            match p {
                AuthPolicy::Auto => 0,
                AuthPolicy::Always => 1,
                AuthPolicy::Never => 2,
            }
        }
        rank(self).cmp(&rank(other))
    }
}

impl Display for AuthPolicy {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Always => write!(f, "always"),
            Self::Never => write!(f, "never"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Credentials {
    Basic {
        username: Option<String>,
        password: Option<String>,
    },
    Bearer {
        token: String,
    },
}

impl Credentials {
    pub fn from_env(_env_var: String) -> Option<Self> {
        None
    }

    pub fn from_url(url: &Url) -> Option<Self> {
        let username = url.username();
        let password = url.password();
        if username.is_empty() && password.is_none() {
            return None;
        }
        Some(Self::Basic {
            username: if username.is_empty() {
                None
            } else {
                Some(username.to_string())
            },
            password: password.map(String::from),
        })
    }
}

#[derive(Debug)]
pub struct RealmRef<'a> {
    pub scheme: &'a str,
    pub host: Option<&'a str>,
    pub port: Option<u16>,
}

impl<'a> From<&'a Url> for RealmRef<'a> {
    fn from(url: &'a Url) -> Self {
        Self {
            scheme: url.scheme(),
            host: url.host_str(),
            port: url.port(),
        }
    }
}

impl PartialEq for RealmRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.scheme == other.scheme && self.host == other.host && self.port == other.port
    }
}

impl Eq for RealmRef<'_> {}
