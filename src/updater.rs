use std::fs;
use std::path::Path;
use reqwest;
use tokio;

#[cfg(target_os = "windows")]
const GITHUB_RELEASE_URL: &str = "https://github.com/bsly86/mhur_packer/releases/latest/download/MyHeroPak.exe";
#[cfg(target_os = "linux")]
const GITHUB_RELEASE_URL: &str = "https://github.com/bsly86/mhur_packer/releases/latest/download/MyHeroPak_Unix";

#[cfg(target_os = "windows")]
const HERO_PAK: &str = "MyHeroPak.exe";
#[cfg(not(target_os = "windows"))]
const HERO_PAK: &str = "MyHeroPak";

#[cfg(target_os = "windows")]
const BACKUP_PAK: &str = "MyHeroPak.exe.bak";
#[cfg(not(target_os = "windows"))]
const BACKUP_PAK: &str = "MyHeroPak.bak";

#[cfg(target_os = "windows")]
const TEMP_PAK: &str = "MyHeroPak.exe.tmp";
#[cfg(not(target_os = "windows"))]
const TEMP_PAK: &str = "MyHeroPak.tmp";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("MyHeroPak Updater");

    if !Path::new(HERO_PAK).exists() {
        return Err(format!("{} not found", HERO_PAK).into());
    }

    println!("Creating backup...");
    fs::copy(HERO_PAK, BACKUP_PAK)?;

    println!("Downloading new package...");
    let response = reqwest::get(GITHUB_RELEASE_URL).await?;
    let content = response.bytes().await?;

    fs::write(TEMP_PAK, content)?;

    println!("Replacing old package...");
    fs::remove_file(HERO_PAK)?;
    fs::rename(TEMP_PAK, HERO_PAK)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(HERO_PAK)?.permissions();
        perms.set_mode(0o755); // rwxr-xr-x
        fs::set_permissions(HERO_PAK, perms)?;
    }

    fs::remove_file(BACKUP_PAK)?;

    println!("Update complete!");
    Ok(())

}
