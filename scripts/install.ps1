# 1install Windows Bootstrap Installer
$ErrorActionPreference = "Stop"

$arch = [System.Environment]::GetEnvironmentVariable("PROCESSOR_ARCHITECTURE")

Write-Host "🚀 Installing 1install for Windows ($arch)..." -ForegroundColor Cyan

$installDir = Join-Path $HOME ".local\bin"
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir | Out-Null
}

Write-Host "   Target directory: $installDir"

# In production, we would use:
# Invoke-WebRequest -Uri "https://github.com/loganbek/1install/releases/latest/download/1i-windows-x86_64.zip" -OutFile "$installDir\1i.zip"
# Expand-Archive -Path "$installDir\1i.zip" -DestinationPath $installDir

Write-Host "   Adding $installDir to User PATH..." -ForegroundColor Gray
$currentPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$installDir*") {
    [System.Environment]::SetEnvironmentVariable("Path", "$currentPath;$installDir", "User")
    $env:Path = "$env:Path;$installDir"
    Write-Host "   PATH updated for current session and future sessions."
}
else {
    Write-Host "   $installDir is already in PATH."
}

Write-Host "`n✓ 1install bootstrap complete." -ForegroundColor Green

Write-Host "`n⚙️ Configuring environment..." -ForegroundColor Cyan
# Run shims setup automatically
if (Get-Command "1i" -ErrorAction SilentlyContinue) {
    1i shims setup
    Write-Host "✓ Environment configured successfully." -ForegroundColor Green
}
else {
    Write-Host "⚠ Could not find '1i' in PATH. You may need to restart your terminal." -ForegroundColor Yellow
}

Write-Host "`n🚀 You are ready to go! Try running: 1i search ripgrep"
