//! src/configuration.rs
use secrecy::{ExposeSecret, Secret};
/*
* To manage configuration with config we must
* represent our application settings as a Rust type
* that implements serde’s Deserialize trait.
* */
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

impl ServerSettings {
    // TERMINOLOGY CLARIFICATION:
    // - TCP_SOCKET_ADDRESS: A string representing where to bind ("127.0.0.1:8000")
    // - TCP Socket: The actual OS resource created when .bind() is called
    // - TCP Connection: An accepted connection on that socket
    pub fn tcp_socket_address(self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn with_random_port(self) -> String {
        let random_port = 0; // (i.e OS scan and takes whatever is available)
        format!("{}:{}", self.host, random_port)
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct DBUser {
    pub name: String,
    pub password: Secret<String>,
}

// DatabaseSettings must also dervice Deserialize
// It makes sense: all fields in a type have to be deserialisable in order for the type as a whole to be deserialisable.
// without it, Settings is not Deserializable anymore.
#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub user: DBUser,
}

impl DatabaseSettings {
    // motivation: PgConnection::connect wants a single connection string as input
    pub fn connection_string(self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user.name,
            self.user.password.expose_secret(),
            self.host,
            self.port,
            self.name
        ))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}
