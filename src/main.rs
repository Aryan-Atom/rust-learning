// use std::io;

// fn main() {
//     println!("Whats your name");
//     let mut name = String::new();
//     io::stdin().read_line(&mut name).expect("Failed to read");
//     println!("Welcome, {}!", name.trim());
// }

// Marks Calculator

// use std::io;

// fn main() {
//     println!("Enter the Marks");

//     let mut marks = String::new();

//     io::stdin().read_line(&mut marks).expect("Failed to read");

//     let marks: u32 = marks.trim().parse().expect("Please Enter a Number");

//     if marks >= 90 {
//         println!("A");
//     } else if marks >= 70 {
//         println!("B");
//     } else {
//         println!("Please Improve");
//     }
// }

// fn main() {
//     let mut tries = 0;
//     loop {
//         tries += 1;
//         if tries == 3 {
//             break;
//         }
//     }

//     println!("{}", tries);
// }

// fn main() {
//     let mut countdown = 5;
//     while countdown > 0 {
//         println!("{countdown}...");
//         countdown -= 1;
//     }
// println!("Liftoff");

// }

fn main() {
    // for i in 1..=5 {
    //     println!("Step{i}");
    // }

    let fruits = ["apple", "banana", "mango"];
    for fruit in fruits {
        println!("I like {fruit}");
    }
}
