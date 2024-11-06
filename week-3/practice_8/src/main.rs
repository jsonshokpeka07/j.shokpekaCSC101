fn main() {
    let mut fees = 25_000; // Declare fees as mutable
    println!("fees is {}", fees);

    fees = 35_000; // Now we can change the value

    println!("fees changed is {}", fees);
}