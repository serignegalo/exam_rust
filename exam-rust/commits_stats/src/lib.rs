use chrono::{Datelike, NaiveDate};
use json::JsonValue;
use std::collections::HashMap;

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut commits_per_week: HashMap<String, u32> = HashMap::new();

    for commit in data.members() {
        if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
            let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%SZ").unwrap();
            let week_str = format!("{}-W{}", date.year(), date.iso_week().week());

            *commits_per_week.entry(week_str).or_insert(0) += 1;
        }
    }

    commits_per_week
}

pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut commits_per_author: HashMap<String, u32> = HashMap::new();

    for commit in data.members() {
        if let Some(author) = commit["author"]["login"].as_str() {
            *commits_per_author.entry(author.to_string()).or_insert(0) += 1;
        }
    }

    commits_per_author
}
