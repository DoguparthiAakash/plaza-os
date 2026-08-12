use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use plaza_foundation::core::{PlazaResult, PlazaError};

/// Guest OS variant.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OsVariant {
    Alpine,
    Ubuntu,
    Fedora,
    Custom(String),
}

/// A package to install in the rootfs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootfsPackage {
    pub name: String,
    pub version: Option<String>,
}

/// Rootfs build configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootfsBuildConfig {
    pub variant: OsVariant,
    pub packages: Vec<RootfsPackage>,
    pub hostname: String,
    pub timezone: String,
    pub locale: String,
    pub users: Vec<UserConfig>,
    pub services: Vec<String>,
    pub extra_files: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub username: String,
    pub uid: u32,
    pub shell: String,
    pub groups: Vec<String>,
    pub ssh_authorized_keys: Vec<String>,
}

/// Guest OS rootfs builder.
pub struct OsBuilder {
    config: RootfsBuildConfig,
    build_steps: Vec<BuildStep>,
}

#[derive(Debug, Clone)]
struct BuildStep {
    name: String,
    completed: bool,
}

impl OsBuilder {
    pub fn new(config: RootfsBuildConfig) -> Self {
        let steps = vec![
            BuildStep { name: "create_root_hierarchy".into(), completed: false },
            BuildStep { name: "install_base_packages".into(), completed: false },
            BuildStep { name: "configure_hostname".into(), completed: false },
            BuildStep { name: "configure_timezone".into(), completed: false },
            BuildStep { name: "create_users".into(), completed: false },
            BuildStep { name: "enable_services".into(), completed: false },
            BuildStep { name: "write_extra_files".into(), completed: false },
            BuildStep { name: "finalize".into(), completed: false },
        ];
        Self { config, build_steps: steps }
    }

    /// Create a PlazaVM Alpine-based minimal rootfs config.
    pub fn plaza_alpine() -> Self {
        Self::new(RootfsBuildConfig {
            variant: OsVariant::Alpine,
            packages: vec![
                RootfsPackage { name: "busybox".into(), version: None },
                RootfsPackage { name: "musl".into(), version: None },
                RootfsPackage { name: "openrc".into(), version: None },
                RootfsPackage { name: "openssh-server".into(), version: None },
            ],
            hostname: "plaza-vm".into(),
            timezone: "UTC".into(),
            locale: "en_US.UTF-8".into(),
            users: vec![UserConfig {
                username: "plaza".into(), uid: 1000, shell: "/bin/sh".into(),
                groups: vec!["wheel".into()], ssh_authorized_keys: vec![],
            }],
            services: vec!["networking".into(), "sshd".into()],
            extra_files: HashMap::new(),
        })
    }

    pub fn step_count(&self) -> usize { self.build_steps.len() }
    pub fn completed_steps(&self) -> usize { self.build_steps.iter().filter(|s| s.completed).count() }

    /// Simulate executing all build steps.
    pub fn execute_all(&mut self) -> PlazaResult<()> {
        for step in &mut self.build_steps {
            step.completed = true;
        }
        Ok(())
    }

    pub fn config(&self) -> &RootfsBuildConfig { &self.config }
}
