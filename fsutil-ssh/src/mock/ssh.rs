//! ## Ssh mock
//!
//! Contains mock for SSH protocol

use std::io::Write;

use tempfile::NamedTempFile;

use crate::SshKeyStorage;

/// Mock ssh key storage
pub struct MockSshKeyStorage {
    key: NamedTempFile,
}

impl Default for MockSshKeyStorage {
    fn default() -> Self {
        let mut key = NamedTempFile::new().expect("Failed to create tempfile");
        assert!(writeln!(
            key,
            r"-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
QyNTUxOQAAACCunS8ipomVo8wt/1lxbiIamNxYXLflEx8kcNK+1j71ZwAAAJCRJq2UkSat
lAAAAAtzc2gtZWQyNTUxOQAAACCunS8ipomVo8wt/1lxbiIamNxYXLflEx8kcNK+1j71Zw
AAAEAhehJqeUHYRphKXAB9KjU9lh6lVAq00RxtqCVdsC6+Zq6dLyKmiZWjzC3/WXFuIhqY
3Fhct+UTHyRw0r7WPvVnAAAACHRlc3Qta2V5AQIDBAU=
-----END OPENSSH PRIVATE KEY-----"
        )
        .is_ok());
        Self { key }
    }
}

impl SshKeyStorage for MockSshKeyStorage {
    fn resolve(&self, host: &str, username: &str) -> Option<std::path::PathBuf> {
        match (host, username) {
            ("sftp", "sftp") => Some(self.key.path().to_path_buf()),
            ("scp", "sftp") => Some(self.key.path().to_path_buf()),
            _ => None,
        }
    }
}

// -- config file

/// Create ssh config file
pub fn create_ssh_config(port: u16) -> NamedTempFile {
    let mut temp = NamedTempFile::new().expect("Failed to create tempfile");
    let config = format!(
        r##"
# ssh config
Compression yes
ConnectionAttempts  3
ConnectTimeout      60
Ciphers             aes128-ctr,aes192-ctr,aes256-ctr
KexAlgorithms       diffie-hellman-group-exchange-sha256
MACs                hmac-sha2-512,hmac-sha2-256,hmac-ripemd160
# Hosts
Host sftp
    HostName    127.0.0.1
    Port        {port}
    User        sftp
Host scp
    HostName    127.0.0.1
    Port        {port}
    User        sftp
"##
    );
    temp.write_all(config.as_bytes()).unwrap();
    temp
}
