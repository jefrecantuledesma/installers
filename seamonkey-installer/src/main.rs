use std::fs;
use std::process::Command;
use tempfile::tempdir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL of the PKGBUILD file
    let url =
        "https://raw.githubusercontent.com/jefrecantuledesma/pkgbuilds/main/seamonkey/PKGBUILD";

    // Create a temporary directory
    let dir = tempdir()?;
    let pkgbuild_path = dir.path().join("PKGBUILD");

    // Download the PKGBUILD file
    let response = reqwest::blocking::get(url)?;
    let pkgbuild_content = response.text()?;
    fs::write(&pkgbuild_path, pkgbuild_content)?;

    // Run makepkg
    Command::new("makepkg")
        .arg("-si")
        .current_dir(&dir)
        .status()?;

    Ok(())
}
