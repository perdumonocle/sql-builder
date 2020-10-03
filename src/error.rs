use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqlBuilderError {
    #[error("No table name")]
    NoTableName,
    #[error("No values")]
    NoValues,
    #[error("No set fields")]
    NoSetFields,
}
