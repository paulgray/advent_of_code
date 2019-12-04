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

    println!("{}", n);
    return true;
}

fn main() {
    let mut count = 0;
    for i in 178416..676462 {
        if is_valid_password(i) {
            count += 1;
        }
    }

    println!("Valid passwords {}", count);
}

#[test]
fn test() {
    assert!(is_valid_password(111111));
    assert!(!is_valid_password(223450));
    assert!(!is_valid_password(123789));
}