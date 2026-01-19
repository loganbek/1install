# Subsystem V: Integrity Verification

1install enforces **"Trust but Verify"** for all package installations.

## The Supply Chain Threat

Package managers are prime targets for supply chain attacks:

- Compromised package uploads
- Dependency confusion attacks
- Typosquatting
- Man-in-the-middle attacks on downloads

## Verification Stages

### Pre-Install Verification

Before executing any installation:

```rust
struct IntegrityChecker {
    trusted_sources: Vec<TrustedSource>,
}

impl IntegrityChecker {
    async fn verify_pre_install(&self, package: &Package) -> Result<(), IntegrityError> {
        // 1. Fetch expected hash from upstream
        let expected_hash = self.fetch_upstream_hash(package).await?;

        // 2. Download package to temp location
        let temp_path = self.download_to_temp(package).await?;

        // 3. Calculate actual hash
        let actual_hash = self.calculate_sha256(&temp_path)?;

        // 4. Compare
        if expected_hash != actual_hash {
            return Err(IntegrityError::HashMismatch {
                expected: expected_hash,
                actual: actual_hash,
            });
        }

        Ok(())
    }
}
```

### Hash Sources by Backend

| Backend | Hash Source                                           |
| ------- | ----------------------------------------------------- |
| PyPI    | `https://pypi.org/pypi/<pkg>/json` → `digests.sha256` |
| NPM     | `https://registry.npmjs.org/<pkg>` → `dist.shasum`    |
| APT     | `Packages.gz` → `SHA256` field                        |
| Brew    | Formula → `sha256`                                    |
| Winget  | Manifest → `InstallerSha256`                          |

### Example: PyPI Hash Verification

```rust
async fn fetch_pypi_hash(package: &str, version: &str) -> Result<String, Error> {
    let url = format!("https://pypi.org/pypi/{}/{}/json", package, version);
    let response: PyPIResponse = reqwest::get(&url).await?.json().await?;

    // Get the sha256 for the appropriate wheel/tarball
    response.urls
        .iter()
        .find(|u| u.packagetype == "bdist_wheel")
        .or_else(|| response.urls.first())
        .map(|u| u.digests.sha256.clone())
        .ok_or(Error::NoHashAvailable)
}
```

## Post-Install Verification

After installation completes:

```rust
async fn verify_post_install(&self, package: &Package) -> Result<(), IntegrityError> {
    match self.get_platform() {
        Platform::Linux => self.verify_gpg_signature(package)?,
        Platform::Windows => self.verify_authenticode(package)?,
        Platform::MacOS => self.verify_codesign(package)?,
    }
    Ok(())
}
```

### Linux: GPG Signature Verification

```bash
# APT packages are GPG signed
apt-key verify <package>.gpg
```

### Windows: Authenticode Verification

```powershell
# Check digital signature
Get-AuthenticodeSignature -FilePath "C:\Path\To\Binary.exe"
```

### macOS: Codesign Verification

```bash
# Verify Apple notarization
codesign --verify --deep --strict /path/to/binary
```

## Alerting and Abort Behavior

```rust
enum IntegrityAction {
    Abort,      // Hard stop, delete downloaded files
    Warn,       // Warn user, proceed if confirmed
    Allow,      // Allow for specific trusted sources
}

fn handle_integrity_failure(error: IntegrityError) -> IntegrityAction {
    match error {
        IntegrityError::HashMismatch { .. } => {
            eprintln!("⚠️  SECURITY ALERT: Hash mismatch detected!");
            eprintln!("   This could indicate a compromised package.");
            eprintln!("   Installation aborted for your safety.");
            IntegrityAction::Abort
        }
        IntegrityError::NoSignature => {
            eprintln!("⚠️  Warning: Package has no signature.");
            IntegrityAction::Warn
        }
        _ => IntegrityAction::Abort
    }
}
```

## User Output

```
$ 1i install suspicious-package

[1/4] Fetching package metadata...     ✓
[2/4] Verifying integrity...           ✗

⚠️  SECURITY ALERT: Hash mismatch detected!

   Expected: a94d32c8e7b...
   Received: f7e219a3b4c...

   This could indicate:
   • A compromised package
   • A man-in-the-middle attack
   • A corrupted download

   Installation aborted for your safety.

   If you believe this is a false positive, please report it:
   https://github.com/loganbek/1install/issues
```
