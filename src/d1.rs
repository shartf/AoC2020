use itertools::Itertools;

pub fn day_1() -> anyhow::Result<()> {
    let (a, b) = include_str!("day1.txt")
        .split('\n')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .tuple_combinations()
        .find(|(a, b)| a + b == 2020)
        .expect("no  pair had a sum of 2020");

    dbg!(a + b);
    dbg!(a * b);

    Ok(())
}
