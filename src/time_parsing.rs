use serde::{
    de::{
        Deserialize,
        Deserializer,
        Error as DeError,
    },
    ser::{
        Serializer,
    }
};

use super::Error;

const ONE_HOUR: i64 = 60 * 60 * 1000;
const ONE_MINUTE: i64 = 60 * 1000;
const ONE_SECOND: i64 = 1000;

pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
where D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_moment_duration(&s).map_err(DeError::custom)
}

fn parse_moment_duration(s: &str) -> Result<i64, Error> {
    if s == "P0D" {
        return Ok(0)
    }
    if !s.starts_with("P") {
        return Err(Error::Other(format!("durations must start with P, {}", s)));
    }
    let s = &s[1..];
    let (ms, s) = extract_ms(s)?;
    let (sec, s) = extract_seconds(s)?;
    let (min, s) = extract_minutes(s)?;
    let (hrs, _) = extract_hours(s)?;
    Ok(hrs + min + sec + ms)
}

fn extract_ms<'a>(s: &'a str) -> Result<(i64, &'a str), Error> {
    if !s.ends_with('S') {
        return Err(Error::Other(format!("duration string does not end with S: {}", s)));
    }
    extract_segment(&s[..s.len() - 1], &['.', 'M', 'H', 'T'], 1)
}

fn extract_seconds<'a>(s: &'a str) -> Result<(i64, &'a str), Error> {
    extract_segment(s, &['M', 'H', 'T'], ONE_SECOND)

}

fn extract_minutes<'a>(s: &'a str) -> Result<(i64, &'a str), Error> {
    extract_segment(s, &['H', 'T'], ONE_MINUTE)
}

fn extract_hours<'a>(s: &'a str) -> Result<(i64, &'a str), Error> {
    extract_segment(s, &['T'], ONE_HOUR)
}

fn extract_segment<'a>(s: &'a str, targets: &[char], multiplier: i64) -> Result<(i64, &'a str), Error> {
    if let Some(idx) = s.rfind(|c| {
        for ch in targets {
           if &c == ch {
               return true
           }
        }
        return false
    }) {
        let section = &s[idx+1..];
        debug!(target: "analytics:debug", "extracting section: {}", section);
        let val = section.parse::<i64>()?;
        Ok((val * multiplier, &s[..idx]))
    } else {
        Ok((0, s))
    }
}

pub fn serialize<S>(ms: &i64, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer
{
    serializer.serialize_str(&serialize_duration(*ms))
}

fn serialize_duration(ms: i64) -> String {
    debug!(target: "analytics:debug","serializing timestamp: {}", ms);
    if ms == 0 {
        return format!("P0D");
    }
    let mut remaining = ms;
    let mut hours = 0;
    let mut minutes = 0;
    let mut seconds = 0;
    let mut dur_str = String::from("PT");
    while remaining >= ONE_HOUR {
            hours += 1;
            remaining -= ONE_HOUR;
    }
    dur_str.push_str(&format!("{}H", hours));
    while remaining >= ONE_MINUTE {
        minutes += 1;
        remaining -= ONE_MINUTE
    }
    dur_str.push_str(&format!("{}M", minutes));
    while remaining >= ONE_SECOND {
        seconds += 1;
        remaining -= ONE_SECOND
    };
    dur_str.push_str(&format!("{}.{}S", seconds, remaining));
    dur_str
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_random_durations() {
        let durations = include_str!("../durations.txt");
        for (i, duration) in durations.lines().enumerate() {
            println!("{}. testing: {}", i, duration);
            let val = parse_moment_duration(duration).unwrap();
            let revert = serialize_duration(val);
            assert_eq!((i, duration), (i, revert.as_str()));
            debug!(target: "analytics:debug", "val: {}", val);
        }
    }
}