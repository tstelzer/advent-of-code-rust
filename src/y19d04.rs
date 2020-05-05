// - It is a six-digit number.
// - The value is within the range given in your puzzle input.
fn is_password(value: &usize) -> bool {
    let n = value.clone();
    let mut r = to_digits(n);
    r.sort();
    if n != join_digits(&r) {
        // digits aren't ascending
        false
    } else {
        r.dedup();
        if r.len() == 6 {
            // no duplicate digits
            false
        } else {
            true
        }
    }
}

fn to_digits(n: usize) -> Vec<usize> {
    vec![
        (n / 100000) % 10,
        (n / 10000) % 10,
        (n / 1000) % 10,
        (n / 100) % 10,
        (n / 10) % 10,
        (n / 1) % 10,
    ]
}

fn join_digits(ns: &Vec<usize>) -> usize {
    let mut n2 = 0;
    for digit in ns {
        if n2 == 0 {
            n2 += digit;
        } else {
            n2 *= 10;
            n2 += digit;
        }
    }

    n2
}

pub fn part1() {
    let result = (246515usize..739105usize).filter(is_password).count();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_inputs() {
        assert_eq!(is_password(&111111usize), true);
        assert_eq!(is_password(&223450usize), false);
        assert_eq!(is_password(&123789usize), false);
    }
}
