#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MaterialLayout {
    Pbr,
    Unlit,
    Water,
    Terrain,
    Skinned,
}
