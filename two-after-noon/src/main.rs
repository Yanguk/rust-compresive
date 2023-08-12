mod luhn;
mod prefix;

use luhn::*;

fn main() {
    println!("Hello, world!");

    let cc_number = "1234 5678 1234 5670";

    println!(
        "Is {cc_number} a valid credit card number? {}",
        if luhn(cc_number) { "yes" } else { "no" }
    );
}
