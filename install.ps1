# SmartTerm Windows Installer Script
$ErrorActionPreference = "Stop"

$Repo = "bbhcoder/smart_term2"
$Target = "x86_64-pc-windows-msvc"
$Url = "https://github.com/$Repo/releases/latest/download/smart_term-$Target.zip"

$InstallDir = "$env:LOCALAPPDATA\SmartTerm"
$TempZip = "$env:TEMP\smart_term.zip"

Write-Host "Downloading SmartTerm ($Target)..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $Url -OutFile $TempZip

Write-Host "Extracting binaries..." -ForegroundColor Yellow
if (!(Test-Path $InstallDir)) { New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null }
Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
Remove-Item $TempZip

Write-Host "Adding to PATH..." -ForegroundColor Yellow
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notmatch [regex]::Escape($InstallDir)) {
    [Environment]::SetEnvironmentVariable("PATH", "$UserPath;$InstallDir", "User")
}

$ProfilePath = $PROFILE
if (!(Test-Path $ProfilePath)) {
    New-Item -ItemType File -Path $ProfilePath -Force | Out-Null
}

$HookCommand = "smart init powershell | Invoke-Expression"
$ProfileContent = Get-Content $ProfilePath -Raw
if ($ProfileContent -notmatch "smart init powershell") {
    Add-Content -Path $ProfilePath -Value "`n$HookCommand"
    Write-Host "Hook added to PowerShell profile: $ProfilePath" -ForegroundColor Green
}

Write-Host "SmartTerm installed successfully! Please restart your terminal." -ForegroundColor Green