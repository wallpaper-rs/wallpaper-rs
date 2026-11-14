model! {
    #[derive(Default)]
    pub struct TransitionState {
        pub choice: String,
        pub own: Option<String>,
        pub plays: String,
        pub why: String,
    }
}

model! {
    pub struct Tones {
        pub noche: String,
        pub noche_alta: String,
        pub noche_linea: String,
        pub bruma: String,
        pub bruma_baja: String,
        pub luna: String,
        pub luna_honda: String,
        pub velo: f64,
    }
}
