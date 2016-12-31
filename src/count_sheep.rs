pub fn count_sheep(x: &[bool]) -> u8 {
    let mut number: u8 = 0;
    for value in x.iter() {
        match value {
            &true => number += 1,
            _ => number += 0,
        };
    }
    return number;
}
