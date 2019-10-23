extern crate reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://feeds.feedburner.com/Metafilter")?
        .text()?;
    println!("{:#?}", resp);
    Ok(())
}
