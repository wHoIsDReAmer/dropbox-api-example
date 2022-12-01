use reqwest::header;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // input your access token for dropbox
    let access_token = "".to_string();

    // Upload to dropbox
    let file = std::fs::File::open("test.txt")?;
    let file_path = "/test/test.txt";
    let mut client = reqwest::blocking::Client::new();
    let result = client
        .post("https://content.dropboxapi.com/2/files/upload")
        .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header("Dropbox-API-Arg", format!("{{\"path\":\"{}\", \"mode\": {{\".tag\": \"overwrite\"}}}}", file_path))
        .body(file)
        .send()?;
    println!("{}", result.text()?);
    Ok(())
}
