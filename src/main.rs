use std::env;
use std::str::FromStr;

fn gcd(mut n: i32, mut m: i32) -> i32 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        // Exchange the values of m, n if m<n
        if m < n {
            let temp = m;
            m = n;
            n = temp;
        }
        m %= n
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    // Getting arguments from the command line
    let args = env::args().skip(1);
    for arg in args {
        // convert the arguments into numbers
        let result = i32::from_str(&arg);
        match result {
            Ok(val) => {
                numbers.push(val);
            }
            Err(e) => {
                eprintln!("Parsing failed: {e}");
                std::process::exit(1);
            }
        }
    }

    if numbers.is_empty() {
        eprintln!("Usage gcd num1, num2 ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The GCD is: {:?}", d);
}
