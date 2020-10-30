// Testcase: List is a data structure where the elements must be handled sequentially.
// Each `write!` generates a `fmt::Result`. Proper handling of this requires dealing
// with all the results. Rust provides the `?` operation for exactly this purpose.

use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Try `write!` to see if it erros. If it errors, return
        // the error. Otherwise continue.
        write!(f, "[")?;

        let vec = &self.0;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

pub fn list_print() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}