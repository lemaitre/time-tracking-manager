use chrono::{DateTime, Months, TimeDelta, Utc};

use crate::errors::SplitError;

pub fn split___(s: &str) -> (&str, &str) {
    s.split_once("___").unwrap_or((s, ""))
}

pub fn split_eq(s: &str) -> Result<(&str, &str), SplitError> {
    s.split_once('=').ok_or_else(|| SplitError {
        field: s.to_string(),
        reason: String::from("field should contain one `=` (key=value)"),
    })
}

pub fn end_of_month(date: &DateTime<Utc>) -> DateTime<Utc> {
    date.checked_add_months(Months::new(1))
        .unwrap()
        .checked_sub_signed(TimeDelta::milliseconds(1))
        .unwrap()
}
