fn main() -> anyhow::Result<()> {
    let works = website_data::prepare_works("data/works", "static")?;
    println!(
        "Validated {} local works and prepared their static assets.",
        works.len()
    );
    Ok(())
}
