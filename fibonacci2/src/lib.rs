// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

//F(n)=F(n−1)+F(n−2)

pub fn fibonacci(n: u32) -> u32 {

    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut tmp: u32;
    let mut pos_one: u32 = 0;
    let mut pos_two: u32 = 1;
    for _ in 2..n+1 {
        tmp = pos_two;
        pos_two = pos_one + pos_two;
        pos_one = tmp;
    }
    pos_two
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = fibonacci(20);
        assert_eq!(result, 6765);
    }
}
