use crate::told::{told, tr};

const TAGS: &[(&str, &str)] = &[
    ("abstract", told("Abstract")),
    ("animals", told("Animals")),
    ("anime", told("Anime")),
    ("architecture", told("Architecture")),
    ("art", told("Art")),
    ("autumn", told("Autumn")),
    ("beach", told("Beach")),
    ("black", told("Black")),
    ("blue", told("Blue")),
    ("car", told("Car")),
    ("cars", told("Cars")),
    ("cat", told("Cat")),
    ("city", told("City")),
    ("clouds", told("Clouds")),
    ("colorful", told("Colorful")),
    ("cyberpunk", told("Cyberpunk")),
    ("dark", told("Dark")),
    ("desert", told("Desert")),
    ("dog", told("Dog")),
    ("dragon", told("Dragon")),
    ("dream", told("Dream")),
    ("fantasy", told("Fantasy")),
    ("fire", told("Fire")),
    ("flowers", told("Flowers")),
    ("forest", told("Forest")),
    ("galaxy", told("Galaxy")),
    ("game", told("Game")),
    ("games", told("Games")),
    ("geometric", told("Geometric")),
    ("gradient", told("Gradient")),
    ("green", told("Green")),
    ("lake", told("Lake")),
    ("landscape", told("Landscape")),
    ("light", told("Light")),
    ("lofi", told("Lo-fi")),
    ("minimal", told("Minimal")),
    ("minimalism", told("Minimalism")),
    ("minimalist", told("Minimalist")),
    ("moon", told("Moon")),
    ("mountain", told("Mountain")),
    ("mountains", told("Mountains")),
    ("music", told("Music")),
    ("nature", told("Nature")),
    ("neon", told("Neon")),
    ("night", told("Night")),
    ("ocean", told("Ocean")),
    ("orange", told("Orange")),
    ("pastel", told("Pastel")),
    ("pink", told("Pink")),
    ("pixel art", told("Pixel art")),
    ("portrait", told("Portrait")),
    ("purple", told("Purple")),
    ("rain", told("Rain")),
    ("red", told("Red")),
    ("retro", told("Retro")),
    ("river", told("River")),
    ("robot", told("Robot")),
    ("sci-fi", told("Sci-fi")),
    ("scifi", told("Sci-fi")),
    ("sea", told("Sea")),
    ("sky", told("Sky")),
    ("snow", told("Snow")),
    ("space", told("Space")),
    ("stars", told("Stars")),
    ("summer", told("Summer")),
    ("sunrise", told("Sunrise")),
    ("sunset", told("Sunset")),
    ("technology", told("Technology")),
    ("texture", told("Texture")),
    ("train", told("Train")),
    ("trees", told("Trees")),
    ("water", told("Water")),
    ("waves", told("Waves")),
    ("white", told("White")),
    ("winter", told("Winter")),
    ("yellow", told("Yellow")),
];

pub fn tag(name: &str) -> String {
    tag_with(name, tr)
}

fn tag_with(name: &str, say: impl Fn(&str) -> String) -> String {
    let lower = name.to_lowercase();
    TAGS.iter().find(|(key, _)| *key == lower).map_or_else(|| name.to_owned(), |(_, text)| say(text))
}

#[cfg(test)]
mod tests {
    use super::tag_with;

    #[test]
    fn a_stored_tag_prints_in_the_language_of_the_catalog() {
        let spanish = crate::told::speaking("es");
        assert_eq!(tag_with("Beach", &spanish), "Playa");
        assert_eq!(tag_with("sci-fi", &spanish), "Ciencia ficción");
        assert_eq!(tag_with("my own tag", &spanish), "my own tag");
    }
}
