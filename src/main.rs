#[macro_use]
extern crate maplit;

use chrono::{DateTime, NaiveDate, NaiveDateTime, ParseResult};
use clap::{Parser, ValueEnum};
use itertools::Itertools;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Command line options for epochs.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Strings to test for epochness.
    candidates: Vec<String>,

    /// Activate debug mode
    #[arg(short, long)]
    debug: bool,

    /// Don't report dates after this.
    #[arg(long, value_parser = parse_date, default_value = "2100-12-31")]
    max: NaiveDate,

    /// Don't report dates before this.
    #[arg(long, value_parser = parse_date, default_value = "2000-01-01")]
    min: NaiveDate,

    /// Desired format for output.
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
    output_format: OutputFormat,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    Json,
    JsonPretty,
    Text,
}

#[derive(Debug, Serialize)]
struct Datelike {
    source: String,
    viewed_as: View,
    epochs: HashMap<String, NaiveDateTime>,
}

#[derive(Debug, Serialize)]
enum View {
    Decimal,
    Float,
    Hexadecimal,
    Ulid,
    Uuid,
}

#[derive(Debug, Serialize)]
enum Uuid {
    V1(NaiveDateTime),
    V6(NaiveDateTime),
    V7(NaiveDateTime),
}

fn main() {
    let args = Args::parse();
    if args.debug {
        println!("{args:?}");
    }

    let mut dates = vec![];

    for c in args.candidates {
        if args.debug {
            println!("{c:?}");
        }

        if let Ok(int) = c.parse::<i64>() {
            if args.verbose > 1 {
                println!("  As decimal integer: {int}");
            }
            dates.push(Datelike {
                source: c.to_string(),
                viewed_as: View::Decimal,
                epochs: get_epochs(int, args.min, args.max),
            });
        }
        if let Ok(int) = i64::from_str_radix(&c, 16) {
            if args.verbose > 1 {
                println!("  As hexadecimal integer: {int}");
            }
            dates.push(Datelike {
                source: c.to_string(),
                viewed_as: View::Hexadecimal,
                epochs: get_epochs(int, args.min, args.max),
            });
        }

        if let Ok(float) = c.parse::<f64>() {
            if args.verbose > 1 {
                println!("  As a float! {float:?}");
            }

            if let Some(ndt) = epochs::icq(float) {
                dates.push(Datelike {
                    source: c.to_string(),
                    viewed_as: View::Float,
                    epochs: hashmap! {"icq".to_string() => ndt},
                });
            }
        }

        if let Some(ndt) = get_ulid(&c) {
            dates.push(Datelike {
                source: c.to_string(),
                viewed_as: View::Ulid,
                epochs: hashmap! {"ulid".to_string() => ndt},
            });
        }

        if let Some(uuid) = get_uuid(&c) {
            let epochs = match uuid {
                Uuid::V1(ndt) => hashmap! {"uuid_v1".to_string() => ndt},
                Uuid::V6(ndt) => hashmap! {"uuid_v6".to_string() => ndt},
                Uuid::V7(ndt) => hashmap! {"uuid_v7".to_string() => ndt},
            };
            dates.push(Datelike {
                source: c.to_string(),
                viewed_as: View::Uuid,
                epochs,
            });
        }
    }

    if args.debug {
        println!("{dates:?}");
    }

    match args.output_format {
        OutputFormat::Text => {
            for date in &dates {
                if !date.epochs.is_empty() {
                    println!("\n{} {:?}", date.source, date.viewed_as);
                }
                for epoch in date.epochs.iter().sorted() {
                    println!("  {} => {:?}", epoch.0, epoch.1);
                }
            }
        }
        OutputFormat::Json => {
            let json = serde_json::to_string(&dates).unwrap();
            println!("{json}");
        }
        OutputFormat::JsonPretty => {
            let json = serde_json::to_string_pretty(&dates).unwrap();
            println!("{json}");
        }
    }
}

fn get_epochs(int: i64, min: NaiveDate, max: NaiveDate) -> HashMap<String, NaiveDateTime> {
    let mut ndts = HashMap::new();

    if let Some(ndt) = epochs::apfs(int) {
        let d = ndt.date();
        if d >= min && d <= max {
            ndts.insert("apfs".to_string(), ndt);
        }
    }

    if let Some(ndt) = epochs::chrome(int) {
        let d = ndt.date();
        if d >= min && d <= max {
            ndts.insert("chrome".to_string(), ndt);
        }
    }

    if let Some(ndt) = epochs::cocoa(int) {
        let d = ndt.date();
        if d >= min && d <= max {
            ndts.insert("cocoa".to_string(), ndt);
        }
    }

    if let Some(ndt) = epochs::google_calendar(int) {
        let d = ndt.date();
        if d >= min && d <= max {
            ndts.insert("google calendar".to_string(), ndt);
        }
    }

    if let Some(ndt) = epochs::java(int) {
        let d = ndt.date();
        if d >= min && d <= max {
            ndts.insert("java".to_string(), ndt);
        }
    }

    if let Some(ndt) = epochs::mozilla(int) {
        let d = ndt.date();
        if d >= min && d <= max {
            ndts.insert("mozilla".to_string(), ndt);
        }
    }

    if let Some(ndt) = epochs::symbian(int) {
        let d = ndt.date();
        if d >= min && d <= max {
            ndts.insert("symbian".to_string(), ndt);
        }
    }

    if let Some(ndt) = epochs::unix(int) {
        let d = ndt.date();
        if d >= min && d <= max {
            ndts.insert("unix".to_string(), ndt);
        }
    }

    if let Some(ndt) = epochs::uuid_v1(int) {
        let d = ndt.date();
        if d >= min && d <= max {
            ndts.insert("uuid v1".to_string(), ndt);
        }
    }

    if let Some(ndt) = epochs::windows_date(int) {
        let d = ndt.date();
        if d >= min && d <= max {
            ndts.insert("windows date".to_string(), ndt);
        }
    }

    if let Some(ndt) = epochs::windows_file(int) {
        let d = ndt.date();
        if d >= min && d <= max {
            ndts.insert("windows file".to_string(), ndt);
        }
    }

    ndts
}

/// See if the given string contains a ULID. If it does, extract the
/// timestamp. <https://github.com/ulid/spec>
///
fn get_ulid(text: &str) -> Option<NaiveDateTime> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^[0123456789ABCDEFGHJKMNPQRSTVWXYZ]{26}$").unwrap());

    if RE.is_match(text) {
        // The first ten characters of the ULID contain the timestamp.
        let ts_ms = crockford::decode(&text[..10]).unwrap();
        let secs = (ts_ms / 1000) as i64;
        let nsecs = (ts_ms % 1000) as u32 * 1_000_000;
        DateTime::from_timestamp(secs, nsecs).map(|dt| dt.naive_utc())
    } else {
        None
    }
}

/// See if the given string contains a UUID string. If it does,
/// extract the version and the date-time if it has it.
///
fn get_uuid(text: &str) -> Option<Uuid> {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"(?x)
            ([0-9A-Fa-f]{8})  -?
            ([0-9A-Fa-f]{4})  -?
            ([0-9]{1})
            ([0-9A-Fa-f]{3})  -?
            [0-9A-Fa-f]{4}    -?
            [0-9A-Fa-f]{12}
            ",
        )
        .unwrap()
    });

    if let Some(cap) = RE.captures(text) {
        let version = cap.get(3).unwrap().as_str();
        match version {
            "1" => {
                let hex = format!("{}{}{}", &cap[4], &cap[2], &cap[1]);
                if let Ok(int) = i64::from_str_radix(&hex, 16) {
                    if let Some(ndt) = epochs::uuid_v1(int) {
                        return Some(Uuid::V1(ndt));
                    }
                }
            }
            "6" => {
                let hex = format!("{}{}{}", &cap[1], &cap[2], &cap[4]);
                if let Ok(int) = i64::from_str_radix(&hex, 16) {
                    if let Some(ndt) = epochs::uuid_v1(int) {
                        return Some(Uuid::V6(ndt));
                    }
                }
            }
            "7" => {
                let hex = format!("{}{}", &cap[1], &cap[2]);
                if let Ok(int) = i64::from_str_radix(&hex, 16) {
                    if let Some(ndt) = epochs::java(int) {
                        return Some(Uuid::V7(ndt));
                    }
                }
            }
            _ => (),
        }
    }
    None
}

/// Try to parse the given string as a `NaiveDate`.
fn parse_date(src: &str) -> ParseResult<NaiveDate> {
    NaiveDate::parse_from_str(src, "%Y-%m-%d")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_ulid_tests() {
        let ulid = get_ulid("01FWHE4YDGFK1SHH6W1G60EECF");
        if let Some(ndt) = ulid {
            assert_eq!(ndt.to_string(), "2022-02-22 19:22:22");
        } else {
            panic!();
        }
    }

    #[test]
    fn get_uuid_tests() {
        let uuid = get_uuid("33c41a44-6cea-11e7-907b-a6006ad3dba0");
        if let Some(Uuid::V1(ndt)) = uuid {
            assert_eq!(ndt.to_string(), "2017-07-20 01:24:40.472634");
        } else {
            panic!();
        }

        let uuid = get_uuid("33c41a44-6cea-21e7-907b-a6006ad3dba0");
        assert!(uuid.is_none());

        let uuid = get_uuid("1EC9414C-232A-6B00-B3C8-9E6BDECED846");
        if let Some(Uuid::V6(ndt)) = uuid {
            assert_eq!(ndt.to_string(), "2022-02-22 19:22:22");
        } else {
            panic!();
        }

        let uuid = get_uuid("017F22E2-79B0-7CC3-98C4-DC0C0C07398F");
        if let Some(Uuid::V7(ndt)) = uuid {
            assert_eq!(ndt.to_string(), "2022-02-22 19:22:22");
        } else {
            panic!();
        }
    }
}
