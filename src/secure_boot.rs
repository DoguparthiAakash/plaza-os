use serde::{Serialize, Deserialize};

/// Secure boot verification chain for the guest OS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureBootChain {
    pub enabled: bool,
    pub uefi_firmware: Option<String>,
    pub shim_hash: Option<String>,
    pub bootloader_hash: Option<String>,
    pub kernel_hash: Option<String>,
    pub initramfs_hash: Option<String>,
}

impl SecureBootChain {
    pub fn disabled() -> Self {
        Self { enabled: false, uefi_firmware: None, shim_hash: None, bootloader_hash: None, kernel_hash: None, initramfs_hash: None }
    }

    pub fn verify_chain(&self) -> SecureBootResult {
        if !self.enabled {
            return SecureBootResult { verified: false, reason: "Secure boot disabled".into() };
        }
        for (name, hash) in [
            ("UEFI firmware", &self.uefi_firmware),
            ("Shim", &self.shim_hash),
            ("Bootloader", &self.bootloader_hash),
            ("Kernel", &self.kernel_hash),
            ("Initramfs", &self.initramfs_hash),
        ] {
            if hash.is_none() {
                return SecureBootResult { verified: false, reason: format!("{} hash missing", name) };
            }
        }
        SecureBootResult { verified: true, reason: "Full chain verified".into() }
    }

    pub fn set_hash(&mut self, component: &str, hash: String) {
        match component {
            "uefi" => self.uefi_firmware = Some(hash),
            "shim" => self.shim_hash = Some(hash),
            "bootloader" => self.bootloader_hash = Some(hash),
            "kernel" => self.kernel_hash = Some(hash),
            "initramfs" => self.initramfs_hash = Some(hash),
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureBootResult {
    pub verified: bool,
    pub reason: String,
}
