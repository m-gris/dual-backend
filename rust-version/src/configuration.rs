//! src/configuration.rs
// use config::Environment;
use secrecy::{ExposeSecret, Secret};
use std::net::Ipv4Addr;
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
    pub host: Ipv4Addr,
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
    let base_path = std::env::current_dir().expect("Failed to determine current dir.");
    let config_dir = base_path.join("configuration");
    let env: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    let env_config_file = format!("{}.yaml", env.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(config_dir.join("base.yaml")))
        .add_source(config::File::from(config_dir.join(env_config_file)))
        .build()?;
    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported env.\nUse either `local` or `production`",
                other
            )),
        }
    }
}
