use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::types::Decimal;
use sqlx::{Column, Error, FromRow, Row, ValueRef};

pub struct PortableRow {
    values: Vec<PortableValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PortableValue {
    Decimal(Decimal),
    UnsignedInteger(u64),
    Integer(i64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
    Null,
}

impl FromRow<'_, MySqlRow> for PortableRow {
    fn from_row(row: &MySqlRow) -> Result<Self, Error> {
        let mut value = PortableRow {
            values: Vec::with_capacity(row.len()),
        };

        for column in row.columns() {
            let raw_value = row.try_get_raw(column.ordinal())?;
            if raw_value.is_null() {
                value.values.push(PortableValue::Null);
                continue;
            }
        }

        Ok(value)
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn exports_simple_string_data_from_test_database() {}
}
