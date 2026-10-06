use schema_core::{DatabaseConfig, DomainError, ws_connection_address};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

pub async fn connect(config: &DatabaseConfig) -> Result<Surreal<Client>, DomainError> {
    let address = ws_connection_address(&config.endpoint);
    let db = Surreal::new::<Ws>(address.to_owned())
        .await
        .map_err(|error| DomainError::DatabaseError(error.to_string()))?;

    db.signin(Root { username: config.username.clone(), password: config.password.clone() })
        .await
        .map_err(|error| DomainError::DatabaseError(error.to_string()))?;

    db.use_ns(&config.namespace)
        .use_db(&config.database)
        .await
        .map_err(|error| DomainError::DatabaseError(error.to_string()))?;

    Ok(db)
}
