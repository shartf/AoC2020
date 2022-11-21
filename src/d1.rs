pub fn day_1() -> anyhow::Result<()> {
    let s = std::fs::read_to_string("files/day1.txt")?;
    dbg!(&s[..40]);

    Ok(())
}
