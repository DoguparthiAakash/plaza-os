use plaza_foundation::core::PlazaResult;

pub struct OsBuilder;

impl OsBuilder {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn build_image(&self, _config: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
