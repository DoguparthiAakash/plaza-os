use plaza_foundation::core::PlazaResult;

pub struct FilesystemSnapshotter;

impl FilesystemSnapshotter {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn create_snapshot(&self, _vol: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
