use std::path::Path;

use crate::{client_error, error::TgError, io_error};

/// Credential information.
///
/// since 0.5.0
#[derive(Clone)]
pub enum Credential {
    Null,
    UserPassword {
        user: String,
        password: Option<String>,
    },
    AuthToken(String),
    File {
        encrypted: String,
        comments: Vec<String>,
    },
}

impl std::fmt::Debug for Credential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Credential::Null => write!(f, "Credential::Null"),
            Credential::UserPassword { user, .. } => {
                write!(f, "Credential::UserPassword({})", user)
            }
            Credential::AuthToken(token) => write!(
                f,
                "Credential::AuthToken({}****)",
                token.chars().take(16).collect::<String>()
            ),
            Credential::File {
                encrypted,
                comments,
            } => write!(
                f,
                "Credential::File{{ encrypted: {}****, comments: {:?} }}",
                encrypted.chars().take(16).collect::<String>(),
                comments
            ),
        }
    }
}

impl Credential {
    /// Returns a null credential.
    pub fn null() -> Credential {
        Credential::Null
    }

    /// Creates a new user/password credential.
    pub fn from_user_password(
        user: impl Into<String>,
        password: Option<impl Into<String>>,
    ) -> Credential {
        Credential::UserPassword {
            user: user.into(),
            password: password.map(Into::into),
        }
    }

    /// Creates a new authentication token credential.
    pub fn from_auth_token(token: impl Into<String>) -> Credential {
        Credential::AuthToken(token.into())
    }

    /// Load credential from file.
    pub fn load(path: impl AsRef<Path>) -> Result<Credential, TgError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| io_error!("failed to read credential file", e))?;
        let mut lines = content.lines();

        let encrypted = lines
            .next()
            .ok_or_else(|| io_error!("credential file is empty"))?
            .to_string();
        let comments: Vec<String> = lines.map(|s| s.to_string()).collect();

        Ok(Credential::File {
            encrypted,
            comments,
        })
    }
}

impl Default for Credential {
    fn default() -> Self {
        Self::null()
    }
}

/// The format version field name in credential file.
const KEY_FORMAT_VERSION: &str = "format_version";

/// The encrypted user name field name in credential file.
const KEY_USER: &str = "user";

/// The encrypted password field name in credential file.
const KEY_PASSWORD: &str = "password";

/// The expiration date field name in credential file.
const KEY_EXPIRATION_DATE: &str = "expiration_date";

/// The current format version.
const FORMAT_VERSION: u32 = 1;

impl Credential {
    pub(crate) fn to_json_text(
        &self,
        expiration_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<String, TgError> {
        self.to_json(expiration_date)
            .map_err(|e| client_error!("json convert error", e))
    }

    fn to_json(
        &self,
        expiration_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<String, std::fmt::Error> {
        use std::fmt::Write;
        match self {
            Credential::UserPassword { user, password } => {
                let mut json = String::with_capacity(256);
                json.push('{');

                write!(json, r#""{}":{}"#, KEY_FORMAT_VERSION, FORMAT_VERSION)?;
                write!(json, r#","{}":"{}""#, KEY_USER, user)?;
                write!(
                    json,
                    r#","{}":"{}""#,
                    KEY_PASSWORD,
                    password.as_ref().unwrap_or(&"".to_string())
                )?;
                if let Some(date_time) = expiration_date {
                    write!(
                        json,
                        r#","{}":"{}""#,
                        KEY_EXPIRATION_DATE,
                        date_time.to_rfc3339_opts(chrono::SecondsFormat::Micros, true)
                    )?;
                }

                json.push('}');
                Ok(json)
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Utc};

    use super::*;

    #[test]
    fn test_credential_to_json() {
        let credential = Credential::from_user_password("user", Some("password"));
        let date_time = DateTime::parse_from_rfc3339("2025-08-28T23:59:59.123456789+00:00")
            .unwrap()
            .with_timezone(&Utc);
        let json = credential.to_json_text(Some(date_time)).unwrap();
        assert_eq!(
            json,
            r#"{"format_version":1,"user":"user","password":"password","expiration_date":"2025-08-28T23:59:59.123456Z"}"#
        );
    }
}
