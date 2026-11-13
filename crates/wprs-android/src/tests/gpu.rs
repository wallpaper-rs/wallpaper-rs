pub(super) fn renderer() -> Option<&'static wprs_render::Renderer> {
    match wprs_render::Renderer::shared() {
        Ok(r) => Some(r),
        Err(e) if std::env::var_os("WPRS_REQUIRE_GPU").is_none() => {
            eprintln!("skipping the scene render: {e}");
            None
        }
        Err(e) => panic!("GPU required but unavailable: {e}"),
    }
}
