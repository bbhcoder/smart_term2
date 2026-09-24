use reqwest::blocking::Client;
use serde::Deserialize;
use std::process::Command;
use std::env;

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
}

pub fn execute() {
    let current_version = env!("CARGO_PKG_VERSION");
    println!("Current SmartTerm Version: v{}", current_version);
    println!("Checking for latest updates from GitHub...");

    let client = Client::builder()
        .user_agent("smart-term-cli")
        .build();

    let client = match client {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to create HTTP client: {}", e);
            return;
        }
    };

    let url = "https://api.github.com/repos/bbhcoder/smart_term2/releases/latest";
    let res = client.get(url).send();

    let release: GithubRelease = match res {
        Ok(r) => match r.json() {
            Ok(rel) => rel,
            Err(e) => {
                eprintln!("Failed to parse release information: {}", e);
                return;
            }
        },
        Err(e) => {
            eprintln!("Failed to connect to GitHub API: {}", e);
            return;
        }
    };

    let latest_version = release.tag_name.trim_start_matches('v');
    println!("Latest Release on GitHub: v{}", latest_version);

    if current_version == latest_version {
        println!("You are already using the latest version of SmartTerm! No update needed.");
        return;
    }

    println!("A new version (v{}) is available! Starting upgrade process...", latest_version);

    #[cfg(unix)]
    {
        let status = Command::new("sh")
            .arg("-c")
            .arg("curl -sSL https://raw.githubusercontent.com/bbhcoder/smart_term2/main/install.sh | bash")
            .status();

        match status {
            Ok(s) if s.success() => println!("Update completed successfully! Please restart your terminal."),
            _ => eprintln!("Failed to run the update script."),
        }
    }

    #[cfg(windows)]
    {
        let status = Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                "Start-Process powershell -ArgumentList '-NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -Command \"irm https://raw.githubusercontent.com/bbhcoder/smart_term2/main/install.ps1 | iex\"'"
            ])
            .status();

        match status {
            Ok(_) => println!("Update triggered in the background! Please restart your terminal shortly."),
            Err(e) => eprintln!("Failed to trigger the update script: {}", e),
        }
    }
}