use crate::ShaderSource;

#[derive(Debug, Clone, Copy)]
pub struct ShaderDescriptor<'a> {
    pub label: Option<&'a str>,
    pub source: ShaderSource<'a>,
}

impl<'a> ShaderDescriptor<'a> {
    pub const fn new(source: ShaderSource<'a>) -> Self {
        Self {
            label: None,
            source,
        }
    }
}
