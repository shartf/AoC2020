use anyhow;
use std::ops::RangeInclusive;

struct PasswordPolicy {
    byte: u8,
    range: RangeInclusive<usize>,
}

impl PasswordPolicy {
    fn is_valid(&self, password: &str) -> bool {
        self.range.contains(
            &password
                .as_bytes()
                .iter()
                .copied()
                .filter(|&b| b == self.byte)
                .count(),
        )
    }
}

pub fn day_2() -> anyhow::Result<()> {
    let input = include_str!("day2.txt")
        .trim()
        .lines()
        .map(parse_line)
        .map(Result::unwrap)
        .filter(|(policy, password)| policy.is_valid(password))
        .count();

    Ok(())
}

fn parse_line(s: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
    todo!()
}
