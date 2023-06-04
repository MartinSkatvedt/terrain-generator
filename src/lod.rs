#[derive(Clone, Copy)]
pub struct LevelOfDetailInfo {
    pub lod: u32,
    pub distance: f32,
}

impl LevelOfDetailInfo {
    pub fn new(lod: u32, distance: f32) -> Self {
        Self { lod, distance }
    }
}
