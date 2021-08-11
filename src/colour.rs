#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r: r, g: g, b: b }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_colour() {
        assert_eq!(Colour { r: 255, g: 0, b: 0 }, Colour::new(255, 0, 0));
    }

    #[test]
    fn test_colour_rgb() {
        let c = Colour::new(236, 13, 159);
        assert_eq!(236, Colour::r(&c));
        assert_eq!(13, Colour::g(&c));
        assert_eq!(159, Colour::b(&c));
    }
}
