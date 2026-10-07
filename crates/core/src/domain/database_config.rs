use super::schema::Datasource;
use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseConfig {
    pub endpoint: String,
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl DatabaseConfig {
    pub fn from_datasource(datasource: &Datasource) -> Result<Self, DomainError> {
        if datasource.provider != "surrealdb" {
            return Err(DomainError::UnsupportedProvider(datasource.provider.clone()));
        }

        let endpoint = resolve_datasource_value(datasource.url.as_deref(), "datasource url")?;

        Ok(Self {
            endpoint: normalize_ws_endpoint(&endpoint),
            namespace: datasource.namespace.clone().unwrap_or_else(|| "test".into()),
            database: datasource.database.clone().unwrap_or_else(|| "test".into()),
            username: datasource.extra.get("username").cloned().unwrap_or_else(|| "root".into()),
            password: datasource.extra.get("password").cloned().unwrap_or_else(|| "root".into()),
        })
    }
}

fn resolve_datasource_value(value: Option<&str>, label: &str) -> Result<String, DomainError> {
    let Some(raw) = value else {
        return Err(DomainError::DatabaseError(format!("{label} is required")));
    };

    raw.strip_prefix("env(\"").and_then(|s| s.strip_suffix("\")")).map_or_else(
        || Ok(raw.to_owned()),
        |stripped| {
            std::env::var(stripped).map_err(|_| {
                DomainError::DatabaseError(format!("environment variable `{stripped}` is not set"))
            })
        },
    )
}

fn normalize_ws_endpoint(url: &str) -> String {
    if url.starts_with("ws://") || url.starts_with("wss://") {
        url.to_owned()
    } else if let Some(rest) = url.strip_prefix("http://") {
        format!("ws://{rest}")
    } else {
        format!("ws://{url}")
    }
}

/// Host/port address for `Surreal::new::<Ws>` (no `ws://` prefix).
#[must_use]
pub fn ws_connection_address(endpoint: &str) -> &str {
    endpoint.strip_prefix("ws://").or_else(|| endpoint.strip_prefix("wss://")).unwrap_or(endpoint)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::domain::schema::Datasource;

    #[test]
    fn resolves_env_reference() {
        temp_env::with_var("TEST_SURREAL_URL", Some("127.0.0.1:8000"), || {
            let datasource = Datasource {
                provider: "surrealdb".to_owned(),
                url: Some("env(\"TEST_SURREAL_URL\")".to_owned()),
                namespace: Some("app".to_owned()),
                database: Some("main".to_owned()),
                extra: BTreeMap::new(),
            };

            let config = DatabaseConfig::from_datasource(&datasource).expect("config");
            assert_eq!(config.endpoint, "ws://127.0.0.1:8000");
        });
    }

    #[test]
    fn builds_config_from_literal_url() {
        let datasource = Datasource {
            provider: "surrealdb".to_owned(),
            url: Some("127.0.0.1:8000".to_owned()),
            namespace: Some("app".to_owned()),
            database: Some("main".to_owned()),
            extra: BTreeMap::new(),
        };

        let config = DatabaseConfig::from_datasource(&datasource).expect("config");
        assert_eq!(config.endpoint, "ws://127.0.0.1:8000");
    }

    #[test]
    fn ws_connection_address_strips_scheme() {
        assert_eq!(ws_connection_address("ws://127.0.0.1:8000"), "127.0.0.1:8000");
        assert_eq!(ws_connection_address("wss://db.example:443"), "db.example:443");
        assert_eq!(ws_connection_address("127.0.0.1:8000"), "127.0.0.1:8000");
    }

    #[test]
    fn preserves_wss_endpoint() {
        let datasource = Datasource {
            provider: "surrealdb".to_owned(),
            url: Some("wss://db.example:443".to_owned()),
            namespace: Some("app".to_owned()),
            database: Some("main".to_owned()),
            extra: BTreeMap::new(),
        };
        let config = DatabaseConfig::from_datasource(&datasource).expect("config");
        assert_eq!(config.endpoint, "wss://db.example:443");
    }

    #[test]
    fn rejects_unsupported_provider() {
        let datasource = Datasource {
            provider: "postgres".to_owned(),
            url: Some("postgres://localhost".to_owned()),
            namespace: None,
            database: None,
            extra: BTreeMap::new(),
        };

        let error = DatabaseConfig::from_datasource(&datasource).expect_err("unsupported");
        assert!(matches!(error, DomainError::UnsupportedProvider(_)));
    }

    #[test]
    fn rejects_missing_url() {
        let datasource = Datasource {
            provider: "surrealdb".to_owned(),
            url: None,
            namespace: None,
            database: None,
            extra: BTreeMap::new(),
        };

        let error = DatabaseConfig::from_datasource(&datasource).expect_err("missing url");
        assert!(matches!(error, DomainError::DatabaseError(_)));
    }

    #[test]
    fn rejects_missing_env_var() {
        temp_env::with_var("MISSING_SURREAL_URL", None::<&str>, || {
            let datasource = Datasource {
                provider: "surrealdb".to_owned(),
                url: Some("env(\"MISSING_SURREAL_URL\")".to_owned()),
                namespace: None,
                database: None,
                extra: BTreeMap::new(),
            };

            let error = DatabaseConfig::from_datasource(&datasource).expect_err("missing env");
            assert!(matches!(error, DomainError::DatabaseError(_)));
        });
    }

    #[test]
    fn normalizes_http_url() {
        let datasource = Datasource {
            provider: "surrealdb".to_owned(),
            url: Some("http://127.0.0.1:8000".to_owned()),
            namespace: None,
            database: None,
            extra: BTreeMap::new(),
        };

        let config = DatabaseConfig::from_datasource(&datasource).expect("config");
        assert_eq!(config.endpoint, "ws://127.0.0.1:8000");
    }
}
