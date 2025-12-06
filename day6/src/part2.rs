pub fn solve(s: &str) -> u64 {
    let cols = s.lines().next().expect("a line should exist").len();
    let rows = s.lines().count();
    let data = s.as_bytes();
    let mut total = 0;
    let mut n = 0;
    let mut subtotal_sum = 0;
    let mut subtotal_mul = 1;
    for col in (0..cols).rev() {
        for row in 0..rows {
            let symbol = data[col + row * (cols + 1)] as char;
            match symbol {
                '0'..='9' => {
                    n *= 10;
                    n += symbol as u64 - '0' as u64;
                }
                '+' => {
                    subtotal_sum += n;
                    total += subtotal_sum;
                    subtotal_sum = 0;
                    subtotal_mul = 1;
                    n = 0;
                }
                '*' => {
                    if n != 0 {
                        subtotal_mul *= n;
                    }
                    total += subtotal_mul;
                    subtotal_sum = 0;
                    subtotal_mul = 1;
                    n = 0;
                }
                ' ' => {
                    if n != 0 {
                        dbg!(n);
                        subtotal_sum += n;
                        subtotal_mul *= n;
                        n = 0;
                    }
                }
                _ => {}
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(solve(data), 3263827);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 11494432585168);
    }
}
