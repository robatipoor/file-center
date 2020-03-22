fn get(url :String ) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?.text()?;
    println!("{:#?}", resp);
    Ok(())
}
