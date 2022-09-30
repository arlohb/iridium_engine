mod stages {
    pub trait ShaderStage {}

    pub struct VertexShaderStage;
    impl ShaderStage for VertexShaderStage {}

    pub struct FragmentShaderStage;
    impl ShaderStage for FragmentShaderStage {}
}

pub trait ShaderTrait {
    type Stage: stages::ShaderStage;
}

pub struct DefaultVertexShader;

impl ShaderTrait for DefaultVertexShader {
    type Stage = stages::VertexShaderStage;
}

pub struct DefaultFragmentShader;

impl ShaderTrait for DefaultFragmentShader {
    type Stage = stages::FragmentShaderStage;
}

// Cleaning up asset.rs
// Instead of finding the addr of the guard,
// could I just deref the guard, then std::mem::transmute to extend the lifetime.
// Also shouldn't the return type have the 'a lifetime too?
