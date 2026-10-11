#[derive(Default)]
pub struct CpuInfo {
    vendor: String,
    model_name: String,
    family: u32,
    model: u32,
    flags: Vec<String>,
}

