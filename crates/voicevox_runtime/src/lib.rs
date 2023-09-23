mod backends;
mod capi;
mod error;

pub use self::error::Error;

pub type Result<T> = std::result::Result<T, self::error::Error>;

pub struct ModelRuntime {
    backend: backends::onnxruntime::Backend,
}

impl ModelRuntime {
    pub fn new() -> crate::Result<Self> {
        let backend = backends::onnxruntime::Backend::new()?;
        Ok(Self { backend })
    }
}
