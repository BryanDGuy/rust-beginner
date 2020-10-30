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

    // *** Activities *** //

    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    #[allow(dead_code)]
    struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.

    println!("Pi is roughly {pi:.prec$}", pi = 3.141592, prec = 3);
    /*
        Add a println! macro that prints: Pi is roughly 3.142 by
        controlling the number of decimal places shown. For the purposes
        of this exercise, use let pi = 3.141592 as an estimate for pi.
        (Hint: you may need to check the std::fmt documentation for
        setting the number of decimals to display)
    */
}
