//! # plaza-os
//!
//! Guest OS builder and management for PlazaVM.
//!
//! Provides:
//! - **Builder**: Rootfs construction with package/user/service configuration
//! - **Config**: OS-level settings (DNS, sysctl, console, kernel cmdline)
//! - **Snapshot**: Tree-based VM snapshot manager with lineage
//! - **Secure Boot**: Boot chain verification with component hashes
//! - **Updater**: OTA guest OS update lifecycle

pub mod builder;
pub mod config;
pub mod snapshot;
pub mod secure_boot;
pub mod updater;

pub use builder::{OsBuilder, RootfsBuildConfig, OsVariant};
pub use config::OsConfig;
pub use snapshot::{SnapshotManager, Snapshot};
pub use secure_boot::{SecureBootChain, SecureBootResult};
pub use updater::{OsUpdater, OsUpdate, UpdateState};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_builder_alpine() {
        let mut builder = OsBuilder::plaza_alpine();
        assert_eq!(builder.config().variant, OsVariant::Alpine);
        assert!(builder.step_count() > 5);
        builder.execute_all().unwrap();
        assert_eq!(builder.completed_steps(), builder.step_count());
    }

    #[test]
    fn test_os_config_generation() {
        let cfg = OsConfig::default();
        assert!(cfg.resolv_conf().contains("1.1.1.1"));
        assert!(cfg.sysctl_conf().contains("vm.swappiness"));
        assert!(cfg.cmdline().contains("console=ttyS0"));
    }

    #[tokio::test]
    async fn test_snapshot_lineage() {
        let mgr = SnapshotManager::new();
        let s1 = mgr.create_snapshot("ws-1", "base", None).await.unwrap();
        let s2 = mgr.create_snapshot("ws-1", "dev", Some(s1.id.clone())).await.unwrap();
        let s3 = mgr.create_snapshot("ws-1", "feature", Some(s2.id.clone())).await.unwrap();

        let chain = mgr.lineage(&s3.id).await;
        assert_eq!(chain.len(), 3);

        // Can't delete s2 because s3 depends on it
        assert!(mgr.delete_snapshot(&s2.id).await.is_err());
        mgr.delete_snapshot(&s3.id).await.unwrap();
        mgr.delete_snapshot(&s2.id).await.unwrap();
    }

    #[test]
    fn test_secure_boot_chain() {
        let mut chain = SecureBootChain::disabled();
        assert!(!chain.verify_chain().verified);

        chain.enabled = true;
        chain.set_hash("uefi", "abc123".into());
        chain.set_hash("shim", "def456".into());
        chain.set_hash("bootloader", "ghi789".into());
        chain.set_hash("kernel", "jkl012".into());
        // Missing initramfs
        assert!(!chain.verify_chain().verified);
        chain.set_hash("initramfs", "mno345".into());
        assert!(chain.verify_chain().verified);
    }
}
