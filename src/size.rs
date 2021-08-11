#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Size {
    w: usize,
    h: usize,
}

impl Size {
    pub fn new(w: usize, h: usize) -> Self {
        Size { w: w, h: h }
    }

    pub fn w(&self) -> usize {
        self.w
    }

    pub fn h(&self) -> usize {
        self.h
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_size() {
        assert_eq!(Size { w: 128, h: 128 }, Size::new(128, 128));
    }

    #[test]
    fn test_canvas_width_height() {
        let size = Size::new(8, 16);
        assert_eq!(8, Size::w(&size));
        assert_eq!(16, Size::h(&size));
    }
}
