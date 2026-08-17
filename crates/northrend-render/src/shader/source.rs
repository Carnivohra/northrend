#[derive(Debug, Clone, Copy)]
pub enum ShaderSource<'a> {
    Wgsl(&'a str),
}
