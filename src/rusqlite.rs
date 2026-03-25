use super::CarettaId;
use rusqlite::{
    ToSql,
    types::{FromSql, ToSqlOutput, Value},
};

impl FromSql for CarettaId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let int = i64::column_result(value)?;
        Ok(Self::from_u64_lossy(int as u64))
    }
}

impl ToSql for CarettaId {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Integer(*self.as_u64() as i64)))
    }
}
