use std::fmt;

mod debug_mode;
use debug_mode::*;

mod display_mode;
use display_mode::*;

fn main() {
    println!("Test for formatted print.");

    println!("{} days", 31);

    println!("{0}, this is {1}; {1}, this is {0}", "Bob", "Alice");

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    println!("{number:>width$}", number=1, width=6);

    println!("My name is {0}, {1} {0}", "Bond", "-");

    #[allow(dead_code)]
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "hhh: {}", self.0)
        }
    }

    println!("This struct `{}` won't print...", Structure(3));

    debug();
    pretty_debug();

    // Display
    display_demo();
}