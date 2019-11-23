use strum::EnumProperty;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::EnumProperty;

#[derive(EnumProperty, EnumIter, Debug, Eq, PartialEq)]
pub enum Element {
    #[strum(props(symbol = "H"))]
    Hydrogen,
    #[strum(props(symbol = "He"))]
    Helium,
    #[strum(props(symbol = "Li"))]
    Lithium,
    #[strum(props(symbol = "Be"))]
    Beryllium,
    #[strum(props(symbol = "B"))]
    Boron,
    #[strum(props(symbol = "C"))]
    Carbon,
    #[strum(props(symbol = "N"))]
    Nitrogen,
    #[strum(props(symbol = "O"))]
    Oxygen,
    #[strum(props(symbol = "F"))]
    Fluorine,
    // TODO: Full list
}

impl Element {
    pub fn from_symbol(symbol: &str) -> Option<Self> {
        for element in Self::iter() {
            let element_symbol = element.get_str("symbol").unwrap();
            if element_symbol == symbol {
                return Some(element);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_from_symbol() {
        assert_eq!(Element::Boron, Element::from_symbol("B").unwrap());
        assert_eq!(None, Element::from_symbol("Bla"));
    }
}
