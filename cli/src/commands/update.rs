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
    
    // We run the installer in a separate detached process so it can overwrite
    // the currently running smart.exe and smartd.exe binaries.
    let status = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            "Start-Process powershell -ArgumentList '-NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -Command \"irm https://raw.githubusercontent.com/bbhcoder/smart_term2/main/install.ps1?v=4 | iex\"'"
        ])
        .status();

    match status {
        Ok(_) => println!("Update triggered in the background! Please wait a moment and restart your terminal."),
        Err(e) => eprintln!("Failed to trigger the update script: {}", e),
    }
}
