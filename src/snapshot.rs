use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use plaza_foundation::core::{PlazaResult, PlazaError};

/// A VM snapshot capturing memory + disk state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub description: Option<String>,
    pub size_bytes: u64,
    pub created_at: String,
    pub parent_id: Option<String>,
    pub tags: HashMap<String, String>,
}

/// Snapshot manager with tree-based lineage tracking.
pub struct SnapshotManager {
    snapshots: RwLock<HashMap<String, Snapshot>>,
}

impl SnapshotManager {
    pub fn new() -> Self {
        Self { snapshots: RwLock::new(HashMap::new()) }
    }

    pub async fn create_snapshot(&self, workspace_id: &str, name: &str, parent_id: Option<String>) -> PlazaResult<Snapshot> {
        let snap = Snapshot {
            id: uuid::Uuid::new_v4().to_string(),
            workspace_id: workspace_id.to_string(),
            name: name.to_string(),
            description: None,
            size_bytes: 0,
            created_at: chrono::Utc::now().to_rfc3339(),
            parent_id,
            tags: HashMap::new(),
        };
        let mut store = self.snapshots.write().await;
        store.insert(snap.id.clone(), snap.clone());
        Ok(snap)
    }

    pub async fn get_snapshot(&self, id: &str) -> PlazaResult<Snapshot> {
        self.snapshots.read().await.get(id).cloned()
            .ok_or_else(|| PlazaError::NotFound(format!("Snapshot {}", id)))
    }

    pub async fn delete_snapshot(&self, id: &str) -> PlazaResult<()> {
        let mut store = self.snapshots.write().await;
        // Don't delete if children exist
        if store.values().any(|s| s.parent_id.as_deref() == Some(id)) {
            return Err(PlazaError::Internal("Cannot delete snapshot with children".into()));
        }
        store.remove(id).ok_or_else(|| PlazaError::NotFound(format!("Snapshot {}", id)))?;
        Ok(())
    }

    pub async fn list_by_workspace(&self, workspace_id: &str) -> Vec<Snapshot> {
        self.snapshots.read().await.values()
            .filter(|s| s.workspace_id == workspace_id)
            .cloned()
            .collect()
    }

    /// Get the snapshot lineage chain (child -> parent -> grandparent).
    pub async fn lineage(&self, id: &str) -> Vec<Snapshot> {
        let store = self.snapshots.read().await;
        let mut chain = Vec::new();
        let mut current = id.to_string();
        while let Some(snap) = store.get(&current) {
            chain.push(snap.clone());
            match &snap.parent_id {
                Some(pid) => current = pid.clone(),
                None => break,
            }
        }
        chain
    }
}
