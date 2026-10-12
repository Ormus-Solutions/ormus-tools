//! Shared core for Ormus tools: config, auth, and service clients.

pub mod config {
    use std::path::PathBuf;

    /// Runtime configuration shared by every tool.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Config {
        pub profile: String,
        pub state_dir: PathBuf,
    }

    impl Config {
        /// Default state directory, matching the TypeScript harness (~/.oah).
        pub fn default_for_home(home: PathBuf) -> Self {
            Self {
                profile: "default".into(),
                state_dir: home.join(".oah"),
            }
        }
    }
}

pub mod auth {
    /// Source of a credential. Not yet implemented.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Credential {
        Env(String),
        None,
    }

    pub fn from_env(var: &str) -> Credential {
        match std::env::var(var) {
            Ok(v) if !v.is_empty() => Credential::Env(v),
            _ => Credential::None,
        }
    }
}

pub mod linear {
    /// Linear API client stub.
    #[derive(Debug, Default)]
    pub struct LinearClient;
}

pub mod github {
    /// GitHub API client stub.
    #[derive(Debug, Default)]
    pub struct GitHubClient;
}

/// Error returned by commands that have not been ported yet.
pub fn not_yet_ported(name: &str) -> String {
    format!("ormus {name}: not yet ported")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state_dir() {
        let c = config::Config::default_for_home("/home/x".into());
        assert_eq!(c.state_dir, std::path::PathBuf::from("/home/x/.oah"));
    }

    #[test]
    fn not_ported_message() {
        assert_eq!(not_yet_ported("init"), "ormus init: not yet ported");
    }
}
