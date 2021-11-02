use super::config::ApplicationConfig;

pub struct Context {
    pub config: ApplicationConfig,
}

impl Context {
    pub fn new(config: &ApplicationConfig) -> Self {
        Self {
            config: config.clone()
        }
    }
}
