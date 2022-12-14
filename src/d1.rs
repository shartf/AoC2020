use itertools::Itertools;

pub fn day_1() -> anyhow::Result<()> {
    let (a, b, c) = include_str!("day1.txt")
        .trim()
        .lines()
        .map(|a| {
            dbg!(a);
            a
        })
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .tuple_combinations()
        .find(|(a, b, c)| a + b + c == 2020)
        .expect("no  pair had a sum of 2020");

    dbg!(a + b + c);
    dbg!(a * b * c);

    Ok(())
}
