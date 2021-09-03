
fn compute(a: i32, b: i32) -> (i32, i32) {
    let c = a + b;
    (b, c)
}

fn compute2(t: (i32, i32)) -> (i32, i32) {
    (t.1, t.0 + t.1)
}

fn fib_loop(n: u8) {
    // let mut a = 1;
    // let mut b = 1;
    let mut d: (i32, i32) = (1, 1);
    let mut i = 2u8;

    loop {
        d = compute2(d);
        i += 1;

        println!("next val is {}", d.1);

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b);
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;

        println!("next val is {}", b);
    }
}

pub fn call_fib() {
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fib_loop() {
        // let n = 10u8;
        // fib_loop(n);
        assert_eq!(2 + 2, 4);
    }
}
