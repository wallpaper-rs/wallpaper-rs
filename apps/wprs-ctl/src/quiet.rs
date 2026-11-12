const EGL_LOG: &str = "EGL_LOG_LEVEL";

fn egl_level(current: Option<std::ffi::OsString>) -> Option<&'static str> {
    current.is_none().then_some("fatal")
}

pub(crate) fn libraries() {
    if let Some(level) = egl_level(std::env::var_os(EGL_LOG)) {
        unsafe { std::env::set_var(EGL_LOG, level) };
    }
}

#[cfg(test)]
mod tests {
    use super::egl_level;

    #[test]
    fn the_graphics_library_is_silenced_unless_the_user_chose_a_level() {
        assert_eq!(egl_level(None), Some("fatal"));
        assert_eq!(egl_level(Some("debug".into())), None);
    }
}
