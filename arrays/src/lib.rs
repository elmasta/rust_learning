pub fn sum(a: &[i32]) -> i32 {
	//type of argument missing in the signature here
    let mut to_return: i32 = 0;
    for nmb in a {
        to_return += nmb;
    }
    return to_return;
}

pub fn thirtytwo_tens() -> [i32; 32] {
    let to_return: [i32; 32] = [10; 32];
    return to_return;
}

#[cfg(test)]
mod tests {
    use super::*;
}
