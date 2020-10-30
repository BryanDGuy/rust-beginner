fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    println!("{subject} {verb} {object}", 
        object="the lazy dog.",
        subject="The quick brown fox",
        verb="jumps over"
    );

    // Special formatting can be specified after a `:`
    println!("{} of {:b} people know binary, the other half doesn't.", 1, 2);

    // Right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // Pad numbers with extra zeros.
    println!("{number:>0width$}", number=1, width=6);
}
