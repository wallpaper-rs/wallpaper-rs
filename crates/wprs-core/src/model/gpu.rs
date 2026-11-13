model! {
    pub struct PreviewState {
        pub error: Option<String>,
        pub console: Vec<String>,
    }

    pub struct GpuAdapter {
        pub name: String,
        pub kind: String,
        pub backend: String,
        pub used: bool,
        #[serde(default)]
        pub bus: String,
        #[serde(default)]
        pub compositor: bool,
    }
}
