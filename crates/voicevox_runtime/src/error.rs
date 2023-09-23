use ort::OrtError;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] ErrorRepr);

#[derive(Debug, thiserror::Error)]
pub(crate) enum ErrorRepr {
    #[error(transparent)]
    Ort(#[from] OrtError),
}
