use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// OS-level configuration for guest instances.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsConfig {
    pub hostname: String,
    pub dns_servers: Vec<String>,
    pub ntp_servers: Vec<String>,
    pub sysctl: HashMap<String, String>,
    pub environment: HashMap<String, String>,
    pub swap_mb: u64,
    pub console: ConsoleConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleConfig {
    pub serial: bool,
    pub vga: bool,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
}

impl Default for OsConfig {
    fn default() -> Self {
        Self {
            hostname: "plaza-vm".into(),
            dns_servers: vec!["1.1.1.1".into(), "8.8.8.8".into()],
            ntp_servers: vec!["pool.ntp.org".into()],
            sysctl: HashMap::from([
                ("net.ipv4.ip_forward".into(), "0".into()),
                ("kernel.panic".into(), "10".into()),
                ("vm.swappiness".into(), "10".into()),
            ]),
            environment: HashMap::new(),
            swap_mb: 0,
            console: ConsoleConfig {
                serial: true, vga: false,
                framebuffer_width: 1024, framebuffer_height: 768,
            },
        }
    }
}

impl OsConfig {
    /// Generate /etc/resolv.conf content.
    pub fn resolv_conf(&self) -> String {
        self.dns_servers.iter().map(|s| format!("nameserver {}", s)).collect::<Vec<_>>().join("\n")
    }

    /// Generate sysctl.conf content.
    pub fn sysctl_conf(&self) -> String {
        let mut sorted: Vec<_> = self.sysctl.iter().collect();
        sorted.sort_by_key(|(k, _)| k.clone());
        sorted.iter().map(|(k, v)| format!("{} = {}", k, v)).collect::<Vec<_>>().join("\n")
    }

    /// Generate kernel boot parameters.
    pub fn cmdline(&self) -> String {
        let mut params = vec![];
        if self.console.serial {
            params.push("console=ttyS0,115200");
        }
        if self.console.vga {
            params.push("console=tty0");
        }
        params.push("panic=10");
        params.push("quiet");
        params.join(" ")
    }
}
