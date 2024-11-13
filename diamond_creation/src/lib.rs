pub fn get_diamond(c: char) -> Vec<String> {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let size = alphabet.find(c).unwrap();
    let width = size * 2 + 1;
    
    let mut diamond: Vec<String> = Vec::new();
    
    for i in 0..=size {
        let mut line = vec![' '; width];
        
        let ch = alphabet.chars().nth(i).unwrap();
        line[size - i] = ch;
        line[size + i] = ch;
        
        diamond.push(line.iter().collect());
    }
    
    for i in (0..size).rev() {
        diamond.push(diamond[i].clone());
    }
    
    diamond
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(get_diamond('A'), vec!["A"]);
    }

    #[test]
    fn test_b() {
        assert_eq!(get_diamond('B'), vec![" A ", "B B", " A "]);
    }

    #[test]
    fn test_c() {
        assert_eq!(
            get_diamond('C'),
            vec!["  A  ", " B B ", "C   C", " B B ", "  A  "]
        );
    }

    #[test]
    fn test_d() {
        assert_eq!(
            get_diamond('D'),
            vec!["   A   ", "  B B  ", " C   C ", "D     D", " C   C ", "  B B  ", "   A   ",]
        );
    }

    #[test]
    fn test_z() {
        assert_eq!(
            get_diamond('Z'),
            vec![
                "                         A                         ",
                "                        B B                        ",
                "                       C   C                       ",
                "                      D     D                      ",
                "                     E       E                     ",
                "                    F         F                    ",
                "                   G           G                   ",
                "                  H             H                  ",
                "                 I               I                 ",
                "                J                 J                ",
                "               K                   K               ",
                "              L                     L              ",
                "             M                       M             ",
                "            N                         N            ",
                "           O                           O           ",
                "          P                             P          ",
                "         Q                               Q         ",
                "        R                                 R        ",
                "       S                                   S       ",
                "      T                                     T      ",
                "     U                                       U     ",
                "    V                                         V    ",
                "   W                                           W   ",
                "  X                                             X  ",
                " Y                                               Y ",
                "Z                                                 Z",
                " Y                                               Y ",
                "  X                                             X  ",
                "   W                                           W   ",
                "    V                                         V    ",
                "     U                                       U     ",
                "      T                                     T      ",
                "       S                                   S       ",
                "        R                                 R        ",
                "         Q                               Q         ",
                "          P                             P          ",
                "           O                           O           ",
                "            N                         N            ",
                "             M                       M             ",
                "              L                     L              ",
                "               K                   K               ",
                "                J                 J                ",
                "                 I               I                 ",
                "                  H             H                  ",
                "                   G           G                   ",
                "                    F         F                    ",
                "                     E       E                     ",
                "                      D     D                      ",
                "                       C   C                       ",
                "                        B B                        ",
                "                         A                         ",
            ]
        );
    }
}
