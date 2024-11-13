#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        fn swap_values(val1: &mut u8, val2: u8, swap_with: u8) {
            if *val1 == swap_with {
                *val1 = val2;
            } else if *val1 == val2 {
                *val1 = swap_with;
            }
        }
        
        swap_values(&mut self.r, second, first);
        swap_values(&mut self.g, second, first);
        swap_values(&mut self.b, second, first);
        swap_values(&mut self.a, second, first);
        
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let c = Color {
            r: 255,
            g: 200,
            b: 10,
            a: 30,
        };
        // swap r
        assert_eq!(
            c.swap(c.r, c.b),
            Color {
                r: 10,
                g: 200,
                b: 255,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.r, c.g),
            Color {
                r: 200,
                g: 255,
                b: 10,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.r, c.a),
            Color {
                r: 30,
                g: 200,
                b: 10,
                a: 255
            }
        );

        // swap g
        assert_eq!(
            c.swap(c.g, c.r),
            Color {
                r: 200,
                g: 255,
                b: 10,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.g, c.b),
            Color {
                r: 255,
                g: 10,
                b: 200,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.g, c.a),
            Color {
                r: 255,
                g: 30,
                b: 10,
                a: 200
            }
        );

        // swap b
        assert_eq!(
            c.swap(c.b, c.r),
            Color {
                r: 10,
                g: 200,
                b: 255,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.b, c.g),
            Color {
                r: 255,
                g: 10,
                b: 200,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.b, c.a),
            Color {
                r: 255,
                g: 200,
                b: 30,
                a: 10
            }
        );

        // swap a
        assert_eq!(
            c.swap(c.a, c.r),
            Color {
                r: 30,
                g: 200,
                b: 10,
                a: 255
            }
        );
        assert_eq!(
            c.swap(c.a, c.b),
            Color {
                r: 255,
                g: 200,
                b: 30,
                a: 10
            }
        );
        assert_eq!(
            c.swap(c.a, c.g),
            Color {
                r: 255,
                g: 30,
                b: 10,
                a: 200
            }
        );
    }
}
