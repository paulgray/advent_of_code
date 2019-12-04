fn is_valid_password(n : i32) -> bool {
    // at least 2 adjacent same digits
    let mut same = false;
    let mut temp : i32 = n;
    while temp > 0 {
        if (temp % 10) == ((temp / 10) % 10) {
            same = true;
            break;
        }
        temp = temp / 10;
    }
    if !same {
        return false;
    }

    // digits never decrease
    let mut increase = false;
    temp = n / 10;
    let mut last = n % 10;
    let mut current : i32;
    while temp > 0 {
        current = temp % 10;
        if current > last {
            increase = true;
            break;
        }

        last = current;
        temp = temp / 10;
    }
    if increase {
        return false;
    }

    // the two adjacent matching digits are not part of a larger group of matching digits
    let mut exact_doubles = false;
    let a = n % 10;
    let b = (n / 10) % 10;
    let c = (n / 100) % 10;
    let d = (n / 1000) % 10;
    let e = (n / 10000) % 10;
    let f = (n / 100000) % 10;
    if (a == b && a != c) ||
       (a != b && b == c && b != d) ||
       (b != c && c == d && d != e) || 
       (c != d && d == e && e != f) ||
       (d != e && e == f) {
           exact_doubles = true;
    }
    if !exact_doubles {
        return false;
    }

    println!("{}", n);
    return true;
}

fn main() {
    let mut count = 0;
    for i in 178416..=676461 {
        if is_valid_password(i) {
            count += 1;
        }
    }

    println!("Valid passwords {}", count);
}

#[test]
fn test() {
    assert!(is_valid_password(112233));
    assert!(!is_valid_password(123444));
    assert!(is_valid_password(111122));
}