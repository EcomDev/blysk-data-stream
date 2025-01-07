use std::fmt::Formatter;
use thiserror::Error;

#[derive(Debug)]
pub enum IdErrorField {
    SingleField(&'static str, String),
    MultipleFields(Vec<&'static str>, Vec<String>),
}

#[derive(Debug, Error)]
pub enum IdError {
    #[error("Not found mapped identifier for {0}")]
    IdNotFound(IdErrorField),
    #[error("Cannot allocate identifier for {0} because {1}")]
    IdNotAllocated(IdErrorField, String),
}

impl std::fmt::Display for IdErrorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SingleField(name, val) => write!(f, r#"`{name}` equal to "{val}""#),
            Self::MultipleFields(fields, values) => {
                write!(f, "(")?;
                let mut fields = fields.iter().peekable();
                while let Some(field) = fields.next() {
                    write!(f, "`{field}`")?;
                    if fields.peek().is_some() {
                        write!(f, ", ")?;
                    }
                }

                write!(f, ") matching (")?;

                let mut values = values.iter().peekable();
                while let Some(field) = values.next() {
                    write!(f, r#""{field}""#)?;
                    if values.peek().is_some() {
                        write!(f, ", ")?;
                    }
                }

                write!(f, ")")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_an_error_for_singular_field_not_found() {
        assert_eq!(
            format!(
                "{}",
                IdError::IdNotFound(IdErrorField::SingleField("sku", "sku1".to_string()))
            ),
            r#"Not found mapped identifier for `sku` equal to "sku1""#
        )
    }

    #[test]
    fn formats_an_error_for_multiple_fields_not_found() {
        assert_eq!(
            format!(
                "{}",
                IdError::IdNotFound(IdErrorField::MultipleFields(vec!["sku", "type_id"], vec![
                    "sku1".to_string(),
                    "simple".to_string()
                ]))
            ),
            r#"Not found mapped identifier for (`sku`, `type_id`) matching ("sku1", "simple")"#
        )
    }
}
