use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(Debug, Copy, Clone, PartialEq, Default, RandGen)]
pub enum Element {
    #[default]
    Air,
    Cosmic,
    Earth,
    Fire,
    Force,
    Frost,
    Water,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Air => write!(f, "Air"),
            Element::Cosmic => write!(f, "Cosmic"),
            Element::Earth => write!(f, "Earth"),
            Element::Fire => write!(f, "Fire"),
            Element::Force => write!(f, "Force"),
            Element::Frost => write!(f, "Frost"),
            Element::Water => write!(f, "Water"),
        }
    }
}

#[cfg(test)]
mod element_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Element::Air, "Air")]
    #[case(Element::Cosmic, "Cosmic")]
    #[case(Element::Earth, "Earth")]
    #[case(Element::Fire, "Fire")]
    #[case(Element::Force, "Force")]
    #[case(Element::Frost, "Frost")]
    #[case(Element::Water, "Water")]
    fn display(#[case] element: Element, #[case] expected_display: &str) {
        // Then
        assert_eq!(expected_display, element.to_string())
    }
}
