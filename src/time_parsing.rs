use serde::{
    de::{
        Deserialize,
        Deserializer,
        Error as DeError,
    },
};

use super::Error;

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
    extract_segment(&s[..s.len() - 1], &['.', 'M', 'H', 'T'], 1000)
}

fn extract_seconds<'a>(s: &'a str) -> Result<(i64, &'a str), Error> {
    extract_segment(s, &['M', 'H', 'T'], 1000)

}

fn extract_minutes<'a>(s: &'a str) -> Result<(i64, &'a str), Error> {
    extract_segment(s, &['H', 'T'], 60 * 1000)
}

fn extract_hours<'a>(s: &'a str) -> Result<(i64, &'a str), Error> {
    extract_segment(s, &['T'], 12 * 60 * 1000)
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
        let val = s[idx+1..].parse::<i64>()?;
        Ok((val * multiplier, &s[idx..]))
    } else {
        Ok((0, s))
    }
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
            println!("val: {}", val);
        }
    }
}