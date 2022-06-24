#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

use chrono::{NaiveDate, NaiveDateTime, ParseResult};
use clap::Parser;
use epochs;
use itertools::Itertools;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;

/// Command line options for epochs.
#[derive(Debug, Parser)]
struct Opt {
    /// Strings to test for epochness.
    candidates: Vec<String>,

    /// Activate debug mode
    #[clap(short, long)]
    debug: bool,

    /// Don't report dates after this.
    #[clap(long, parse(try_from_str = parse_date), default_value = "2100-12-31")]
    max: NaiveDate,

    /// Don't report dates before this.
    #[clap(long, parse(try_from_str = parse_date), default_value = "2000-01-01")]
    min: NaiveDate,

    /// Desired format for output.
    #[clap(short, long, arg_enum, default_value = "text", case_insensitive = true)]
    output_format: OutputFormat,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
}

#[derive(Debug, Clone, clap::ArgEnum)]
enum OutputFormat {
    JSON,
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
    UUIDv1,
}

fn main() {
    let opt = Opt::parse();
    if opt.debug {
        println!("{:?}", opt);
    }

    let mut dates = vec![];

    for c in opt.candidates {
        if opt.debug {
            println!("{:?}", c);
        }

        if let Ok(int) = c.parse::<i64>() {
            if opt.verbose > 1 {
                println!("  As decimal integer: {}", int);
            }
            dates.push(Datelike {
                source: c.to_string(),
                viewed_as: View::Decimal,
                epochs: get_epochs(int, opt.min, opt.max),
            });
        }
        if let Ok(int) = i64::from_str_radix(&c, 16) {
            if opt.verbose > 1 {
                println!("  As hexadecimal integer: {}", int);
            }
            dates.push(Datelike {
                source: c.to_string(),
                viewed_as: View::Hexadecimal,
                epochs: get_epochs(int, opt.min, opt.max),
            });
        }

        if let Ok(float) = c.parse::<f64>() {
            if opt.verbose > 1 {
                println!("  As a float! {:?}", float);
            }

            if let Some(ndt) = epochs::icq(float) {
                dates.push(Datelike {
                    source: c.to_string(),
                    viewed_as: View::Float,
                    epochs: hashmap! {"icq".to_string() => ndt},
                });
            }
        }

        if let Some(int) = get_uuid_v1_int(&c) {
            if opt.verbose > 1 {
                println!("  Looks like a UUIDv1! {:?}", int);
            }
            dates.push(Datelike {
                source: c.to_string(),
                viewed_as: View::UUIDv1,
                epochs: get_epochs(int, opt.min, opt.max),
            });
        }
    }

    if opt.debug {
        println!("{:?}", dates);
    }

    match opt.output_format {
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
        OutputFormat::JSON => {
            let json = serde_json::to_string(&dates).unwrap();
            println!("{}", json);
        }
        OutputFormat::JsonPretty => {
            let json = serde_json::to_string_pretty(&dates).unwrap();
            println!("{}", json);
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
            ndts.insert("UUIDv1".to_string(), ndt);
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

/// See if the given string contains a UUID version 1 string.  If it
/// does, extract the portion that represents a date-time and return it
/// as an integer.
///
fn get_uuid_v1_int(text: &str) -> Option<i64> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            (?P<low>[0-9A-Fa-f]{8})    -?
            (?P<mid>[0-9A-Fa-f]{4})    -?
            1                            # this means version 1
            (?P<high>[0-9A-Fa-f]{3})   -?
            [0-9A-Fa-f]{4}             -?
            [0-9A-Fa-f]{12}
            ",
        )
        .unwrap();
    }
    if let Some(cap) = RE.captures(text) {
        let hex = RE.replace_all(&cap[0], "${high}${mid}${low}");
        if let Ok(int) = i64::from_str_radix(&hex, 16) {
            return Some(int);
        }
    }
    None
}

/// Try to parse the given string as a NaiveDate.
fn parse_date(src: &str) -> ParseResult<NaiveDate> {
    NaiveDate::parse_from_str(src, "%Y-%m-%d")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_uuid_v1_int_run() {
        let result = get_uuid_v1_int("33c41a44-6cea-11e7-907b-a6006ad3dba0");
        assert_eq!(result.unwrap(), 137198066804726340);

        assert_eq!(
            epochs::uuid_v1(result.unwrap()).unwrap().to_string(),
            "2017-07-20 01:24:40.472634",
        );
    }

    #[test]
    fn get_uuid_v1_int_fail() {
        // This is not a version 1 UUID------------v
        let result = get_uuid_v1_int("33c41a44-6cea-21e7-907b-a6006ad3dba0");
        assert_eq!(result, None);
    }
}
