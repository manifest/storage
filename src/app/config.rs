use config;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) id: crate::authn::AccountId,
    pub(crate) authn: crate::authn::ConfigMap,
    pub(crate) authz: crate::authz::ConfigMap,
    pub(crate) cors: crate::app::Cors,
}

pub(crate) fn load() -> Result<Config, config::ConfigError> {
    let mut parser = config::Config::default();
    parser.merge(config::File::with_name("App"))?;
    parser.merge(config::Environment::with_prefix("APP").separator("__"))?;
    parser.try_into::<Config>()
}