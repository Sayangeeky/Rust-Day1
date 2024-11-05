// fn main () {
//     let x: u8 = 5;
//     let y: f32 = 10.5;
//     let z: i8 = -15;

//     print!("X: {}, Y: {}, Z: {}", x, y, z);
// } 


// fn main() {
//     let mut x: i8 = 5;

//     for i in 0..100 {
//         x += 100;
//     }

//     println!("x: {}", x)
// }


// fn main() {
//     let is_male: bool = true;
//     let is_above_18: bool = true;
    
//     if is_male {
//         print!("You are a male");
//     }
//     else {
//         print!("You are not a male");
//     }

//     if is_male && is_above_18 {
//         print!("You are an adult male");
//     }
// }

// fn main () {
//     let greetings = String::from("Hello World!");
//     println!("{}", greetings);


//     // let greetings: &str = "Hello World!";
//     // println!("{}", greetings);

//     // print!("{}", greetings.chars().nth(1000))

//    match greetings.chars().nth(1000) {
//     Some(c) => println!("Character at index 1000: {}", c),
//     None => print!("No character found at index 1000")
//    }
// }

// pub fn main() {
//     let is_even: bool = true;

//     if is_even {
//         println!("The number is even");
//     }   else if !is_even {
//         println!("The number is odd");
//     }
// }

// pub fn main () {
//     // for i in 0..10 {
//     //     println!("{}", i)
//     // }

//     let sentence: String = String::from("my name is sayan");
//     let first_word = get_first_word(sentence);
//     println!("First word is: {}", first_word);

// }

// fn get_first_word(sentence: String) -> String {
//     let mut ans: String = String::new();
//     for char in sentence.chars() {
//         ans.push_str(char.to_string().as_str());
//         if char == ' ' {
//             break;
//         }
       
//     }
//     ans
// }

// fn main() {
//     let a: i32 = 10;
//     let b: i32 = 20;
//     let sum: i32 = do_sum(a,b);
//     print!("Sum: {}", sum);

// }

// fn do_sum(a: i32, b: i32) -> i32 {
//     return a + b;
// }