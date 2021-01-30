use std::num::ParseIntError;
use std::time::{SystemTime, UNIX_EPOCH};

/// Get a time to start playing the media, in milliseconds, which will
/// be now + the given number of seconds.
pub fn get_playtime(offset_secs: usize) -> u128 {
    let now = SystemTime::now();
    let now_millis = now
        .duration_since(UNIX_EPOCH)
        .expect("We've gone back in time?")
        .as_millis();
    now_millis + (offset_secs as u128 * 1000)
}

/// Decode hour:min:sec seek string into number of seconds.
pub fn from_seekstr(s: &str) -> Result<usize, ParseIntError> {
    let mut parts: Vec<&str> = s.split(":").collect();
    let hours = if parts.len() >= 3 {
        parts.remove(0).parse()?
    } else {
        0
    };
    let mins = if parts.len() >= 2 {
        parts.remove(0).parse()?
    } else {
        0
    };
    let secs = if parts.len() >= 1 {
        parts.remove(0).parse()?
    } else {
        0
    };
    Ok(hours * 60 * 60 + mins * 60 + secs)
}
