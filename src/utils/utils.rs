use std::fs::File;
use std::io::prelude::*;

pub async fn download_file(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = reqwest::get(url)
    .await?
    .text()
    .await?;

    write_file(content, String::from(path));
    Ok(())
}

fn write_file(context: String, path: String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(context.as_bytes())?;
    Ok(())
}

pub fn strbetween(context: &str, firstStr: &str, lastStr: &str) -> String {
    let start_bytes = context.find(firstStr).unwrap_or(0);
    let end_bytes = context.find(lastStr).unwrap_or(context.len());                                   
    let result = &context[start_bytes..end_bytes].replace(firstStr, "");
    String::from(result)
}