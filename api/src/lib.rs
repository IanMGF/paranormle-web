pub mod history;

pub enum Environment {
    DEBUG,
    RELEASE,
}

impl From<Environment> for &str {
    fn from(value: Environment) -> Self {
        match value {
            Environment::DEBUG => "debug",
            Environment::RELEASE => "release",
        }
    }
}
pub const ENVIRONMENT: Environment = 'env: {
    #[cfg(debug_assertions)]
    break 'env Environment::DEBUG;
    #[cfg(not(debug_assertions))]
    break 'env Environment::RELEASE;
};
