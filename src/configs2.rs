use config::ConfigError;
use dotenvy::var;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Postgres {
    pub dsn: String,
    pub max_conns: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub web: WebConfig,
    pub postgres: Postgres,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DevConfig {
    pub web: WebConfig,
    pub postgres: Postgres,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProdConfig {
    pub web: WebConfig,
    pub postgres: Postgres,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DevEnv {
    pub app: DevConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProdEnv {
    pub app: ProdConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        match var("ENV").as_deref() {
            Ok("prod") => {
                let config = config::Config::builder()
                    .add_source(config::Environment::default().separator("__"))
                    .build()?
                    .try_deserialize::<ProdEnv>()?;
                Ok(AppConfig {
                    web: config.app.web,
                    postgres: config.app.postgres,
                })
            }
            _ => {
                let config = config::Config::builder()
                    .add_source(config::Environment::default().separator("__"))
                    .build()?
                    .try_deserialize::<DevEnv>()?;
                Ok(AppConfig {
                    web: config.app.web,
                    postgres: config.app.postgres,
                })
            }
        }
    }
}
