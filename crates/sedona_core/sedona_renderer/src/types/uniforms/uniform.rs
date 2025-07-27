pub trait Uniform {
    const SIZE: u64;

    fn name() -> &'static str;

    fn field_info() -> &'static [UniformFieldInfo];

    fn get_field_offset(name: &str) -> Option<usize> {
        Self::field_info()
            .iter()
            .find(|f| f.name == name)
            .map(|f| f.offset)
    }

    fn get_field_size(name: &str) -> Option<usize> {
        Self::field_info()
            .iter()
            .find(|f| f.name == name)
            .map(|f| f.size)
    }
}

pub struct UniformFieldInfo {
    pub name: &'static str,
    pub offset: usize,
    pub size: usize,
}
