use serde::{Serialize, Deserialize};
use plaza_foundation::core::{PlazaResult, PlazaError};

/// OTA update state machine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateState {
    Idle,
    Checking,
    Downloading { progress_percent: u8 },
    Applying,
    PendingReboot,
    RollingBack,
    Failed(String),
}

/// An available OS update.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsUpdate {
    pub version: String,
    pub release_notes: String,
    pub size_bytes: u64,
    pub sha256: String,
    pub mandatory: bool,
}

/// Over-the-air updater for guest OS instances.
pub struct OsUpdater {
    current_version: String,
    state: UpdateState,
    pending_update: Option<OsUpdate>,
}

impl OsUpdater {
    pub fn new(current_version: &str) -> Self {
        Self {
            current_version: current_version.to_string(),
            state: UpdateState::Idle,
            pending_update: None,
        }
    }

    pub fn state(&self) -> &UpdateState { &self.state }

    pub fn check_for_update(&mut self) -> PlazaResult<Option<OsUpdate>> {
        self.state = UpdateState::Checking;
        // Simulated: no update available
        self.state = UpdateState::Idle;
        Ok(None)
    }

    pub fn stage_update(&mut self, update: OsUpdate) -> PlazaResult<()> {
        if self.state != UpdateState::Idle {
            return Err(PlazaError::Internal("Update already in progress".into()));
        }
        self.pending_update = Some(update);
        self.state = UpdateState::Downloading { progress_percent: 0 };
        Ok(())
    }

    pub fn simulate_download_progress(&mut self, percent: u8) {
        if percent >= 100 {
            self.state = UpdateState::PendingReboot;
        } else {
            self.state = UpdateState::Downloading { progress_percent: percent };
        }
    }

    pub fn apply(&mut self) -> PlazaResult<()> {
        if self.state != UpdateState::PendingReboot {
            return Err(PlazaError::Internal("No update pending reboot".into()));
        }
        self.state = UpdateState::Applying;
        // In production this would reboot into the new rootfs
        if let Some(update) = &self.pending_update {
            self.current_version = update.version.clone();
        }
        self.state = UpdateState::Idle;
        self.pending_update = None;
        Ok(())
    }

    pub fn rollback(&mut self) -> PlazaResult<()> {
        self.state = UpdateState::RollingBack;
        self.pending_update = None;
        self.state = UpdateState::Idle;
        Ok(())
    }
}
