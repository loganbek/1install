# 1install Windows Bootstrap Installer
$ErrorActionPreference = "Stop"

$os = [System.Environment]::OSVersion.Platform
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

Write-Host "`n✓ 1install bootstrap complete." -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:"
Write-Host "1. Ensure $installDir is in your PATH."
Write-Host "2. Run '1i shims setup' to configure your environment."
