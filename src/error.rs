use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum SqlBuilderError {
    #[error("No table name")]
    NoTableName,
    #[error("No values")]
    NoValues,
    #[error("No set fields")]
    NoSetFields,
    #[error("WHERE condition is empty")]
    NoWhereCond,
    #[error("WHERE field not defined")]
    NoWhereField,
    #[error("WHERE value for field \"{0}\" not defined")]
    NoWhereValue(String),
    #[error("WHERE list for field \"{0}\" not defined")]
    NoWhereList(String),
    #[error("WHERE query for field \"{0}\" not defined")]
    NoWhereQuery(String),
}
