// SPDX-License-Identifier: GPL-3.0

enum Color {
    RGB(i32, i32, i32),
    HSV(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum NewMessage {
    Hello { id: i32 },
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

fn main() {
    // note: Matching literals
    println!("Matching Literals");
    {
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    println!();

    // note: Matching named variables
    println!("Matching Named Variables");
    {
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got: 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {x:?}"),
        }
        println!("at the end: x = {x:?}, y = {y}");
        println!("\n");
    }
    println!();

    // note: Multiple pattern
    println!("Multiple Pattern");
    {
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    println!();

    // note: Matching ranges of values with ..=
    println!("Matching Ranges of Values with ..=");
    {
        let x = 5;
        match x {
            1..=5 => println!("one through five"),
            _ => print!("something else"),
        }

        let x = 'c';
        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }
    println!();

    // note: Destructuring structs
    println!("Destructuring Structs");
    {
        let p = Point { x: 0, y: 7 };

        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);
        println!("a: {a}, b: {b}");

        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("on the y axis at {y}"),
            Point { x, y } => println!("on neither axis: ({x}, {y})"),
        }
    }
    println!();

    // note: Destructuring enums
    println!("Destructuring Enums");
    {
        let msg = Message::Move { x: 4, y: 3 };
        match msg {
            Message::Quit => println!("The Quit variant has no data to destructure"),
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}")
            }
            Message::Write(text) => println!("Text message: {text}"),
            _ => (),
        }
    }
    println!();

    // note Destructuring nested structs and enums
    println!("Destructuring Nested Structs and Enums");
    {
        let msg = Message::ChangeColor(Color::HSV(0, 160, 255));
        match msg {
            Message::ChangeColor(Color::RGB(r, g, b)) => {
                println!("Change color to red {r}, green {g} and blue {b}")
            }
            Message::ChangeColor(Color::HSV(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s} and value {v}")
            }
            _ => (),
        }
    }
    println!();

    // note: Destructuring structs and tuples
    println!("Destructuring Structs and Tuples");
    {
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
        println!("({feet}, {inches}), Point(x: {x}, y: {y})");
    }
    println!();

    // note: Ignoring values in a pattern
    println!("Ignoring Values in a Pattern");
    {
        foo(3, 4);
    }
    println!();

    // note: Ignoring parts of a value with nested _
    println!("Ignoring Parts of a Value with Nested _");
    {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);
        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
            _ => setting_value = new_setting_value,
        }
        println!("setting is {setting_value:?}");

        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, _, third, _, fifth) => println!("some numbers: {first}, {third}, {fifth}"),
        }
    }
    println!();

    // note: Ignoring an unused variable by starting its name with _
    println!("Ignoring an Unused Variable by Starting its Name with _");
    {
        let s = Some("Hello!".to_string());
        if let Some(_) = s {
            println!("found a string");
        }
        println!("{s:?}");
    }
    println!();

    // note: Ignoring remaining parts of a value with ..
    println!("Ignoring Remaining Parts of a Value with ..");
    {
        let origin = Point3D { x: 0, y: 0, z: 0 };
        match origin {
            Point3D { x, .. } => println!("x is {x}"),
        }

        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => println!("some numbers: {first}, {last}"),
        }
    }
    println!();

    // note: Extra conditional with match guards
    println!("Extra Conditional with Match Guards");
    {
        let num = Some(4);
        match num {
            Some(x) if 0 == x % 2 => println!("The number {x} is even"),
            Some(x) => println!("The number {x} is odd"),
            None => (),
        }

        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {n}"),
            _ => println!("Default case, x = {x:?}"),
        }

        let x = 4;
        let y = false;
        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }
    println!();

    // note: @ bindings
    println!("@ Bindings");
    {
        let msg = NewMessage::Hello { id: 5 };
        match msg {
            NewMessage::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {id_variable}"),
            NewMessage::Hello { id: 10..=12 } => println!("Found an id in another range"),
            NewMessage::Hello { id } => println!("Found some other id: {id}"),
        }
    }
    println!();
}
