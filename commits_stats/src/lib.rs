#[allow(unused_imports)]
use std::fs;
use std::collections::HashMap;
use json::JsonValue;
use chrono::{DateTime, Utc, TimeZone, Datelike, Weekday};

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    if let JsonValue::Array(commits) = data {
        println!("{:?}", data[0]["commit"]["author"]["date"]);
        for d in commits {
            if let Some(date) = d["commit"]["author"]["date"].as_str() {
                let datetime = Utc.datetime_from_str(date, "%Y-%m-%dT%H:%M:%SZ").expect("Failed to parse date");
                let year = datetime.iso_week().year();
                let week = datetime.iso_week().week();
                let formatted_string = format!("{}-W{:02}", year, week);
                let counter = map.entry(formatted_string.to_string()).or_insert(0);
                *counter += 1;
            }
        }
    }
    map
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    if let JsonValue::Array(commits) = data {
        for d in commits {
            if let Some(author_name) = d["author"]["login"].as_str() {
                println!("{:?}", author_name);
                let counter = map.entry(author_name.to_string()).or_insert(0);
                *counter += 1;
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_setup() -> json::JsonValue {
        let contents = fs::read_to_string("commits.json").unwrap();
        json::parse(&contents).unwrap()
    }

    #[test]
    fn test_commits_per_week() {
        let serialized = test_setup();
        let commits_per_week = commits_per_week(&serialized);
        println!("{:#?}", &commits_per_week);
        let date = [
            "2020-W47".to_string(),
            "2020-W43".to_string(),
            "2020-W36".to_string(),
            "2020-W50".to_string(),
            "2020-W40".to_string(),
            "2020-W44".to_string(),
            "2020-W46".to_string(),
            "2020-W31".to_string(),
            "2020-W45".to_string(),
            "2020-W49".to_string(),
        ];

        let mut com_per_week = HashMap::new();
        let commits = [3, 1, 1, 2, 2, 5, 4, 1, 4, 7];

        for i in 0..date.len() {
            com_per_week.insert(date[i].clone(), commits[i].clone());
        }

        assert_eq!(com_per_week, commits_per_week);
    }

    #[test]
    fn test_commits_per_author() {
        let serialized = test_setup();
        let logins = [
            "RPigott",
            "RedSoxFan",
            "Xyene",
            "paul-ri",
            "JayceFayne",
            "mwenzkowski",
            "psnszsn",
            "emersion",
            "tamirzb",
            "ifreund",
            "homembaixinho",
        ];
        let commits = [1, 1, 7, 2, 1, 3, 1, 10, 1, 1, 2];
        let mut expected = HashMap::new();

        for i in 0..logins.len() {
            expected.insert(logins[i].to_owned(), commits[i].to_owned());
        }

        let commits_per_author = commits_per_author(&serialized);
        println!("{:#?}", &commits_per_author);
        assert_eq!(expected, commits_per_author);
    }
}
