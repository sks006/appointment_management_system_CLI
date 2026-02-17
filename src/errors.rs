#[derive(Debug)]
pub enum RwaError {
    DatabaseLoadFailure(String),
    WriteFailure(String),
    NotFound(String),
}