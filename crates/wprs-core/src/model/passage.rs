model! {
    #[derive(Default)]
    pub struct TransitionState {
        pub choice: String,
        pub own: Option<String>,
        pub plays: String,
        pub why: String,
    }
}
