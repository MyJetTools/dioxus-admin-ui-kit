use std::str::FromStr;

use rust_extensions::AsStr;
use serde::{Deserialize, Serialize};

use crate::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeOffset {
    #[serde(rename = "-12:00")]
    UtcMinus12,
    #[serde(rename = "-11:00")]
    UtcMinus11,
    #[serde(rename = "-10:00")]
    UtcMinus10,
    #[serde(rename = "-09:30")]
    UtcMinus930,
    #[serde(rename = "-09:00")]
    UtcMinus9,
    #[serde(rename = "-08:00")]
    UtcMinus8,
    #[serde(rename = "-07:00")]
    UtcMinus7,
    #[serde(rename = "-06:00")]
    UtcMinus6,
    #[serde(rename = "-05:00")]
    UtcMinus5,
    #[serde(rename = "-04:00")]
    UtcMinus4,
    #[serde(rename = "-03:30")]
    UtcMinus330,
    #[serde(rename = "-03:00")]
    UtcMinus3,
    #[serde(rename = "-02:00")]
    UtcMinus2,
    #[serde(rename = "-01:00")]
    UtcMinus1,
    #[serde(rename = "+00:00")]
    Utc,
    #[serde(rename = "+01:00")]
    UtcPlus1,
    #[serde(rename = "+02:00")]
    UtcPlus2,
    #[serde(rename = "+03:00")]
    UtcPlus3,
    #[serde(rename = "+03:30")]
    UtcPlus330,
    #[serde(rename = "+04:00")]
    UtcPlus4,
    #[serde(rename = "+04:30")]
    UtcPlus430,
    #[serde(rename = "+05:00")]
    UtcPlus5,
    #[serde(rename = "+05:30")]
    UtcPlus530,
    #[serde(rename = "+05:45")]
    UtcPlus545,
    #[serde(rename = "+06:00")]
    UtcPlus6,
    #[serde(rename = "+06:30")]
    UtcPlus630,
    #[serde(rename = "+06:45")]
    UtcPlus645,
    #[serde(rename = "+07:00")]
    UtcPlus7,
    #[serde(rename = "+08:00")]
    UtcPlus8,
    #[serde(rename = "+08:45")]
    UtcPlus845,
    #[serde(rename = "+09:00")]
    UtcPlus9,
    #[serde(rename = "+09:30")]
    UtcPlus930,
    #[serde(rename = "+10:00")]
    UtcPlus10,
    #[serde(rename = "+10:30")]
    UtcPlus1030,
    #[serde(rename = "+11:00")]
    UtcPlus11,
    #[serde(rename = "+11:30")]
    UtcPlus1130,
    #[serde(rename = "+12:00")]
    UtcPlus12,
    #[serde(rename = "+13:00")]
    UtcPlus13,
    #[serde(rename = "+14:00")]
    UtcPlus14,
}

impl Default for TimeOffset {
    fn default() -> Self {
        Self::Utc
    }
}

impl TimeOffset {
    pub const ALL: &'static [Self] = &[
        Self::UtcMinus12,
        Self::UtcMinus11,
        Self::UtcMinus10,
        Self::UtcMinus930,
        Self::UtcMinus9,
        Self::UtcMinus8,
        Self::UtcMinus7,
        Self::UtcMinus6,
        Self::UtcMinus5,
        Self::UtcMinus4,
        Self::UtcMinus330,
        Self::UtcMinus3,
        Self::UtcMinus2,
        Self::UtcMinus1,
        Self::Utc,
        Self::UtcPlus1,
        Self::UtcPlus2,
        Self::UtcPlus3,
        Self::UtcPlus330,
        Self::UtcPlus4,
        Self::UtcPlus430,
        Self::UtcPlus5,
        Self::UtcPlus530,
        Self::UtcPlus545,
        Self::UtcPlus6,
        Self::UtcPlus630,
        Self::UtcPlus645,
        Self::UtcPlus7,
        Self::UtcPlus8,
        Self::UtcPlus845,
        Self::UtcPlus9,
        Self::UtcPlus930,
        Self::UtcPlus10,
        Self::UtcPlus1030,
        Self::UtcPlus11,
        Self::UtcPlus1130,
        Self::UtcPlus12,
        Self::UtcPlus13,
        Self::UtcPlus14,
    ];

    pub fn try_from_str(src: &str) -> Option<Self> {
        let normalized = src.trim();
        Self::ALL
            .iter()
            .copied()
            .find(|offset| offset.as_str() == normalized)
    }

    pub fn from_str(src: &str) -> Self {
        let normalized = src.trim();
        Self::ALL
            .iter()
            .copied()
            .find(|offset| offset.as_str() == normalized)
            .unwrap_or_default()
    }

    pub fn get_value_as_minutes(&self) -> i32 {
        let repr = self.as_str();
        let sign = if repr.starts_with('-') { -1 } else { 1 };
        let hours: i32 = repr[1..3]
            .parse()
            .expect("TimeOffset::as_str must return valid hours");
        let minutes: i32 = repr[4..6]
            .parse()
            .expect("TimeOffset::as_str must return valid minutes");

        sign * (hours * 60 + minutes)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TimeOffset::UtcMinus12 => "-12:00",
            TimeOffset::UtcMinus11 => "-11:00",
            TimeOffset::UtcMinus10 => "-10:00",
            TimeOffset::UtcMinus930 => "-09:30",
            TimeOffset::UtcMinus9 => "-09:00",
            TimeOffset::UtcMinus8 => "-08:00",
            TimeOffset::UtcMinus7 => "-07:00",
            TimeOffset::UtcMinus6 => "-06:00",
            TimeOffset::UtcMinus5 => "-05:00",
            TimeOffset::UtcMinus4 => "-04:00",
            TimeOffset::UtcMinus330 => "-03:30",
            TimeOffset::UtcMinus3 => "-03:00",
            TimeOffset::UtcMinus2 => "-02:00",
            TimeOffset::UtcMinus1 => "-01:00",
            TimeOffset::Utc => "+00:00",
            TimeOffset::UtcPlus1 => "+01:00",
            TimeOffset::UtcPlus2 => "+02:00",
            TimeOffset::UtcPlus3 => "+03:00",
            TimeOffset::UtcPlus330 => "+03:30",
            TimeOffset::UtcPlus4 => "+04:00",
            TimeOffset::UtcPlus430 => "+04:30",
            TimeOffset::UtcPlus5 => "+05:00",
            TimeOffset::UtcPlus530 => "+05:30",
            TimeOffset::UtcPlus545 => "+05:45",
            TimeOffset::UtcPlus6 => "+06:00",
            TimeOffset::UtcPlus630 => "+06:30",
            TimeOffset::UtcPlus645 => "+06:45",
            TimeOffset::UtcPlus7 => "+07:00",
            TimeOffset::UtcPlus8 => "+08:00",
            TimeOffset::UtcPlus845 => "+08:45",
            TimeOffset::UtcPlus9 => "+09:00",
            TimeOffset::UtcPlus930 => "+09:30",
            TimeOffset::UtcPlus10 => "+10:00",
            TimeOffset::UtcPlus1030 => "+10:30",
            TimeOffset::UtcPlus11 => "+11:00",
            TimeOffset::UtcPlus1130 => "+11:30",
            TimeOffset::UtcPlus12 => "+12:00",
            TimeOffset::UtcPlus13 => "+13:00",
            TimeOffset::UtcPlus14 => "+14:00",
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl EnumIterator for TimeOffset {
    type TItem = Self;

    fn get_value(&self) -> Self
    where
        Self: Sized,
    {
        *self
    }

    fn get_all() -> &'static [Self::TItem]
    where
        Self: Sized,
    {
        Self::ALL
    }
}

impl AsStr for TimeOffset {
    fn as_str(&self) -> &'static str {
        self.as_str()
    }
}

impl FromStr for TimeOffset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = Self::from_str(s);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_half_hour_negative_offset() {
        let offset = TimeOffset::try_from_str("-03:30").expect("offset should parse");
        assert_eq!(offset.get_value_as_minutes(), -210);
    }

    #[test]
    fn parses_quarter_hour_positive_offset() {
        let offset = TimeOffset::try_from_str("+05:45").expect("offset should parse");
        assert_eq!(offset.get_value_as_minutes(), 345);
    }

    #[test]
    fn parse_utc_3() {
        let offset = TimeOffset::try_from_str("+03:00").expect("offset should parse");
        assert_eq!(offset.get_value_as_minutes(), 180);
    }
}
