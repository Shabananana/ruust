// Formatted Print

fn main() {
    println!("{} days", 31);

    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over"
    );

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("{number:>width$}", number=1, width=6);

    println!("{number:>0width$}", number=1, width=6);

    println!("My name is {0}, {1}", "Bond", "James");

    let pi = format!("{:.*}", 3, 3.141592);

    println!("Pi is roughly {pi}", pi=pi);
}
