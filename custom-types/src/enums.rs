#![allow(dead_code)]

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 }
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed `{}`", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}", x, y)
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y
        }
    }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

enum Status {
    Rich,
    Poor
}

enum Work {
    Civilian,
    Soldier
}

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff
}

pub fn enums() {
    let pressed = WebEvent:: KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add.run(34, 12);
    let y = Operations::Subtract.run(3, 21);

    println!("Adding 34 and 12 is {}", x);
    println!("Subtracting 3 and 21 is {}", y);

    // Explicitly `use` each name so they are available without manual scoping
    use Status::{Poor, Rich};
    use Work::*;

    // Equivalent to `Status::Poor`
    let status = Poor;

    // Equivalent to `Work::Civilian`
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("the poor have no money...")
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!")
    }

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}