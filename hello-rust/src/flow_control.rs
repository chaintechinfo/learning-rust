pub fn loop_demo1(multiplier: u16) {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * multiplier;
        }
    };

    println!("The result is {}", result);
}