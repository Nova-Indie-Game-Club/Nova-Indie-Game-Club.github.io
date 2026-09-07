fn main() -> anyhow::Result<()> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let summary = nova_website::generate(root)?;
    println!(
        "Generated {} works into {}",
        summary.work_count,
        summary.output.display()
    );
    println!("Preview: miniserve dist/exported --index index.html -i 127.0.0.1 -p 8080");
    Ok(())
}
