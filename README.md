🛡️ Dataseal

Dataseal is a secure, lightweight encryption locker designed to keep your files and sensitive information safe.
Built with Rust, it combines strong cryptography with a minimal database engine, giving you both performance and protection.

Dataseal uses AES-256-GCM encryption with PBKDF2-HMAC-SHA256 key derivation, and introduces a unique design where your master password protects only the encryption keys, not the files directly. This makes it possible to change your password instantly without re-encrypting all data.


Installation: 

    curl -L https://github.com/<your-username>/dataseal/releases/latest/download/dataseal-linux-x86_64.tar.gz -o dataseal.tar.gz
    tar -xzf dataseal.tar.gz
    sudo mv dataseal /usr/local/bin/


⚠️ Rights & Usage

No individual, organization, or third party has the right to reproduce, distribute, or commercialize this software or its source code.

The intellectual property of Dataseal belongs solely to Farhan Ali Rajpoot.

Unauthorized usage of this code or branding to generate revenue is strictly prohibited.


👤 Owner

This project is created and owned by Farhan Ali Rajpoot.
It is not an open-source project at this stage. Future public releases may include binaries and user-facing builds, but the source code remains private property.

Contact:

Gmail: midlelnight@gmail.com
website: https://agencytendor.vercel.app



