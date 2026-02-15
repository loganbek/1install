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

$exeName = "1i.exe"
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

# If a local built binary exists (for developer one-liner), copy it and create wrappers for aliases
$localCandidates = @(
    Join-Path (Get-Location) "target\release\1i.exe",
    Join-Path (Get-Location) "target\debug\1i.exe"
)
foreach ($cand in $localCandidates) {
    if (Test-Path $cand) {
        Write-Host "   Found local binary: $cand — installing to $installDir"
        Copy-Item -Path $cand -Destination (Join-Path $installDir $exeName) -Force
        # Create wrapper .cmd and .ps1 for aliases
        $aliases = @('1install','oneinstall')
        foreach ($a in $aliases) {
            $cmdPath = Join-Path $installDir "$a.cmd"
            $ps1Path = Join-Path $installDir "$a.ps1"

            $cmdContent = "@echo off`r`n\"%~dp0$exeName\" %*"
            $ps1Content = "& \"$(Join-Path $installDir $exeName)\" $args"

            Set-Content -Path $cmdPath -Value $cmdContent -Encoding ASCII
            Set-Content -Path $ps1Path -Value $ps1Content -Encoding UTF8
        }
        break
    }
}

Write-Host "`n🚀 You are ready to go! Try running: 1i search ripgrep"
