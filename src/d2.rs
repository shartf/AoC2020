use anyhow;
use std::ops::RangeInclusive;

#[derive(PartialEq, Debug)]
pub struct PasswordPolicy {
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
    let _input = include_str!("day2.txt")
        .trim()
        .lines()
        .map(parse_line)
        .map(Result::unwrap)
        .filter(|(policy, password)| policy.is_valid(password))
        .count();

    Ok(())
}

fn parse_line(s: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
    peg::parser! {
      grammar parser() for str {
        rule number() -> usize
          = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule range() -> RangeInclusive<usize>
          = min:number() "-" max:number() { min..=max }

        rule byte() -> u8
          = letter:$(['a'..='z']) { letter.as_bytes()[0] }

        rule password() -> &'input str
          = letters:$([_]*) { letters }

        pub(crate) rule line() -> (PasswordPolicy, &'input str)
          = range:range() " " byte:byte() ": " password:password() {
              (PasswordPolicy { range, byte }, password)
          }
      }
    }

    Ok(parser::line(s)?)
}

#[cfg(test)]
mod tests {
    use super::PasswordPolicy;

    #[test]
    fn test_is_valid() {
        let pp = PasswordPolicy {
            range: 1..=3,
            byte: b'a',
        };
        assert_eq!(pp.is_valid("zeus"), false, "no 'a's");
        assert_eq!(pp.is_valid("hades"), true, "single 'a'");
        assert_eq!(pp.is_valid("banana"), true, "three 'a's");
        assert_eq!(pp.is_valid("aaaah"), false, "too many 'a's");
    }
}
