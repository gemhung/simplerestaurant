use crate::errors::AppError;

pub fn validate_table(table: impl Into<Option<i32>>) -> Result<i32, AppError> {
    let table = table.into();
    let table = match table {
        Some(table @ 1..=100) => table,
        Some(_) => {
            return Err(AppError::invalid_filter(
                "Expecting table to be in a range from 1 to 100",
            ));
        }
        None => {
            return Err(AppError::missing_filter("Missing table filter"));
        }
    };

    Ok(table)
}
