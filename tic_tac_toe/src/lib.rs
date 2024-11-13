pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if diagonals("X", &table) || horizontal("X", &table) || vertical("X", &table) {
        return "player X won".to_string();
    } else if diagonals("O", &table) || horizontal("O", &table) || vertical("O", &table) {
        return "player O won".to_string();
    } else {
        return "tie".to_string();
    }
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    println!("Contenu du vecteur : {}", player);
    println!("Contenu du vecteur : {:?}", table);
    let mut found: bool = false;
    let mut nmb: u8 = 0;
    for i in 0..table.len() { //left to right
        if table[i][i] == player {
            nmb += 1;
        }
        if usize::from(nmb) == table.len() {
            found = true;
        }
    }
    println!("Contenu du vecteur : {}", found);
    if found {
        return found;
    }
    let mut x: usize = table.len()-1;
    nmb = 0;
    for y in 0..table.len() { //right to left    
        println!("Contenu du vecteur : {}, {}", y, x);
        println!("Contenu du vecteur : {}", table[y][x]);
        if table[y][x] == player {
            nmb += 1;
        }
        if usize::from(nmb) == table.len() {
            found = true;
        }
        if x != 0 {
            x -= 1;
        }
    }
    println!("Contenu du vecteur : {}", found);
    return found;
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    println!("Contenu du vecteur : {:?}", table);
    let mut found: bool = false;
    for line in table {
        let mut nmb: u8 = 0;
        for square in line {
            if player == *square {
                println!("Contenu du vecteur : {} {}", player, square);
                nmb += 1;
            }
            if usize::from(nmb) == line.len() {
                found = true;
            }
        }
    }
    return found;
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    println!("Contenu du vecteur : {:?}", table);
    let mut found: bool = false;
    for x in 0..table.len() {
        let mut nmb: u8 = 0;
        for y in 0..table.len() {
            if table[y][x] == player {
                nmb += 1;
            }
            if usize::from(nmb) == table.len() {
                found = true;
            }
        }
    }
    return found;
}

