# 🛡️ Dataseal

Dataseal is a secure, lightweight encryption locker designed to keep your files and sensitive information safe.
Built with Rust, it combines strong cryptography with a minimal database engine, giving you both performance and protection.

Dataseal uses AES-256-GCM encryption with PBKDF2-HMAC-SHA256 key derivation, and introduces a unique design where your master password protects only the encryption keys, not the files directly. This makes it possible to change your password instantly without re-encrypting all data.

---

## ⚡ Features
- Strong cryptography (AES-256-GCM, PBKDF2-HMAC-SHA256)
- Lightweight and fast
- Minimal database engine for secure key storage
- Master password protects keys, not files, for instant password changes

---

💻 Installation

### Linux (bash) : 
```bash
# Download the latest release
curl -L https://github.com/FARHAN-ALI-RAJPOOT/dataseal/releases/latest/download/dataseal-linux-x86_64-latest.tar.gz -o dataseal.tar.gz
# Extract the binary
tar -xzf dataseal.tar.gz
# Move to a folder in PATH
sudo mv dataseal /usr/local/bin/
# Verify installation
dataseal --help

```
### Windows (power_shell)  :
```powershell
# Create bin folder in user profile
mkdir $env:USERPROFILE\bin
# Download the latest release
curl -L https://github.com/FARHAN-ALI-RAJPOOT/dataseal/releases/latest/download/dataseal-windows-x86_64-latest.zip -o dataseal.zip
# Extract to bin folder
Expand-Archive dataseal.zip -DestinationPath $env:USERPROFILE\bin
# Add to PATH for current user
[Environment]::SetEnvironmentVariable("PATH", $env:PATH + ";$env:USERPROFILE\bin", [EnvironmentVariableTarget]::User)
# Verify installation
dataseal --help

```
