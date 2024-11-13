pub fn sum(a: u8, b: u8) -> u8 {
    println!("sum: {}",   a);
    println!("{}",   b);
    a + b
}

pub fn diff(a: i16, b: i16) -> i16 {
    println!("diff: {}",   a);
    println!("{}",   b);
    a - b
}

pub fn pro(a: i8, b: i8) -> i8 {
    println!("pro: {}",   a);
    println!("{}",   b);
    a * b
}

pub fn quo(a: f32, b: f32) -> f32 {
    println!("quo: {}",   a);
    println!("{}",   b);
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    println!("rem: {}",   a);
    println!("{}",   b);
    a % b
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = diff(-2, -4);
//         assert_eq!(result, 0);
//     }
// }
