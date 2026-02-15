Param(
    [Parameter(ValueFromRemainingArguments=$true)]
    $RemainingArgs
)

$local = Join-Path $PSScriptRoot "scripts\install.ps1"
$raw = "https://raw.githubusercontent.com/loganbek/1install/main/scripts/install.ps1"

if (Test-Path $local) {
    Write-Output "Running local $local"
    & $local @RemainingArgs
} else {
    Write-Output "Fetching installer from $raw"
    if (Get-Command iwr -ErrorAction SilentlyContinue) {
        iwr -UseBasicParsing -Uri $raw | iex
    } else {
        try {
            $wc = New-Object System.Net.WebClient
            $script = $wc.DownloadString($raw)
            Invoke-Expression $script
        } catch {
            Write-Error "Failed to download installer: $_"
            exit 1
        }
    }
}
