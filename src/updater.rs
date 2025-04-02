use std::fs;
use std::path::Path;
use reqwest;
use tokio;

const GITHUB_RELEASE_URL: &str = "https://api.github.com/bsly86/mhur_packer/releases/latest/download/MyHeroPak.exe";
const HERO_PAK_EXE: &str = "MyHeroPak.exe";
const BACKUP_EXE: &str = "MyHeroPak.exe.bak";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("MyHeroPak Updater");

    if !Path::new(HERO_PAK_EXE).exists() {
        return Err("MyHeroPak.exe not found".into());
    }

    println!("Creating backup...");
    fs::copy(HERO_PAK_EXE, BACKUP_EXE)?;

    println!("Downloading new package...");
    let response = reqwest::get(GITHUB_RELEASE_URL).await?;
    let content = response.bytes().await?;

    let temp_exe = "MyHeroPak.exe.tmp";
    fs::write(temp_exe, content)?;

    println!("Replacing old package...");
    fs::remove_file(HERO_PAK_EXE)?;
    fs::rename(temp_exe, HERO_PAK_EXE)?;

    fs::remove_file(BACKUP_EXE)?;

    println!("Update complete!");
    Ok(())

}