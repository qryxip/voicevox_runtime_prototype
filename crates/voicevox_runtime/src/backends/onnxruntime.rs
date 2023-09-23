use ort::Environment;
use ort::LoggingLevel;

use crate::error::ErrorRepr;

pub(crate) struct Backend {
    env: &'static Environment,
}

impl Backend {
    pub(crate) fn new() -> crate::Result<Self> {
        let env = ENV
            .get_or_try_init(|| {
                Environment::builder()
                    .with_name(env!("CARGO_PKG_NAME"))
                    .with_log_level(LoggingLevel::Info)
                    .build()
            })
            .map_err(ErrorRepr::from)?;
        return Ok(Self { env });

        static ENV: once_cell::sync::OnceCell<Environment> = once_cell::sync::OnceCell::new();
    }
}
