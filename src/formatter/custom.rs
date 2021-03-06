use crate::formatter::keys::Key;

pub struct CustomStyle<'a> {
    key: String,
    colors: Vec<Key<'a>>,
}

impl<'a> CustomStyle<'a> {
    pub fn new(key: &str, colors: Vec<&'a str>) -> Self {
        Self {
            key: format!("<{}>", key),
            colors: colors.iter().map(|s| Key::new(s)).collect(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn expand(&self) -> String {
        let mut colors: Vec<String> = Vec::with_capacity(2);

        // Turn it into the ansi values it should be
        for color in self.colors.iter() {
            colors.push(color.to_ansi());
        }

        colors.join("")
    }
}

#[cfg(test)]
mod tests {
    use crate::formatter::custom::CustomStyle;
    use crate::formatter::keys::Key;

    #[test]
    fn ansi_expansion() {
        let style = CustomStyle::new("lol", vec!["blue"]);
        let color = Key::new("blue");

        assert_eq!(style.expand(), color.to_ansi());
    }

    #[test]
    fn ansi_expansion_multiple() {
        let style = CustomStyle::new("lol", vec!["blue", "bold", "on-green"]);
        let colors = vec![Key::new("blue"), Key::new("bold"), Key::new("on-green")];

        let generated: String = colors.iter().map(|k| k.to_ansi()).collect();

        assert_eq!(style.expand(), generated);
    }
}
