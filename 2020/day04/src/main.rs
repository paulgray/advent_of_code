fn is_valid_password(n : i32) -> bool {
    let a = n % 10;
    let b = (n / 10) % 10;
    let c = (n / 100) % 10;
    let d = (n / 1000) % 10;
    let e = (n / 10000) % 10;
    let f = (n / 100000) % 10;

    // digits never decrease
    if f > e ||
       e > d ||
       d > c || 
       c > b || 
       b > a {
           return false;
    }

    // the two adjacent matching digits are not part of a larger group of matching digits
    let mut exact_doubles = false;
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