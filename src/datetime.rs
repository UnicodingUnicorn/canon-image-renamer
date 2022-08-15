use std::cmp::{ PartialEq, PartialOrd, Eq, Ord };
use std::hash::Hash;

use chrono::NaiveDateTime;
use exif::{ Exif, Tag, In };
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DateTimeError {
    #[error("Cannot parse time fields: {0}")]
    ParseDateTimeError(#[from] chrono::format::ParseError),
    #[error("Cannot parse millis field: {0}")]
    ParseMillisError(#[from] std::num::ParseIntError),
    #[error("EXIF has no date-time field")]
    NoDateTime,
    #[error("EXIF has no milliseconds field")]
    NoMilliseconds,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct PhotoDate {
    dt: NaiveDateTime,
    millis: u16,
}
impl PhotoDate {
    pub fn new(exif:&Exif) -> Result<Self, DateTimeError> {
        let datetime = exif.get_field(Tag::DateTime, In::PRIMARY)
            .ok_or(DateTimeError::NoDateTime)?;
        let subsec_time = exif.get_field(Tag::SubSecTime, In::PRIMARY)
            .ok_or(DateTimeError::NoMilliseconds)?;

        let dt = NaiveDateTime::parse_from_str(&datetime.display_value().to_string(), "%Y-%m-%d %H:%M:%S")?;
        let millis = subsec_time.display_value().to_string();
        let millis = u16::from_str_radix(millis.trim_start_matches("\"").trim_end_matches("\""), 10)?;

        Ok(Self {
            dt,
            millis,
        })
    }

    pub fn folder_name(&self) -> String {
        self.dt.format("%Y_%m_%d").to_string()
    }
}
