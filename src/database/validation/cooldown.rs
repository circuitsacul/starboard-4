use std::sync::LazyLock;

use regex::Regex;

pub fn parse_cooldown(inp: &str) -> Result<(i16, i16), String> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"(?P<count>\d+).+?(?P<secs>\d+)"#).unwrap());

    let found = match RE.captures(inp) {
        None => {
            return Err(concat!(
                "I couldn't parse the cooldown you passed. The ",
                "correct format is `capacity/period` (e.x. `5/6`)."
            )
            .to_string());
        }
        Some(found) => found,
    };

    let capacity = found.name("count").unwrap().as_str();
    let capacity: i16 = match capacity.parse() {
        Err(_) => return Err(format!("{capacity} is not a valid number.")),
        Ok(capacity) => capacity,
    };
    let period = found.name("secs").unwrap().as_str();
    let period: i16 = match period.parse() {
        Err(_) => return Err(format!("{period} is not a valid number.")),
        Ok(period) => period,
    };

    super::starboard_settings::validate_cooldown(capacity, period)?;
    Ok((capacity, period))
}
