// fn define_x() -> &str {
//     // Specify the return type of define_x as &'static str
//     let love    =  String::from("hello");
//     love // Return the string "hello"
// }
// fn main() {
//     let x: i32 = 10;
//     {
//         let y: i32 = 5;
//         println!("The value of x is {} and value of y is {}", x, y);
//     }
//     println!("The value of x is {} and value of y is ", x);
//     let x = define_x(); // Assign the value returned by define_x to x
//     println!("{}, world", x);
// }

// STATE MENTS AND EXPRESSION 

// Make it work with two ways
// fn main() {
//     let v = {
//         let mut  x = 1;
//         x += 2;
//         x
//     };
 
//     assert_eq!(v, 3);
    
//     println!("Success!");
//  }


// fn main() {
//     let v = {let x = 3 ;
//     x };
 
//     assert!(v == 3);
 
//     println!("Success!");
//  }




// fn main() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);

//     println!("Success!");
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }
 
 
// Don't modify code in main!
// fn main() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1);

//     println!("{}", s2);
// }

// // Only modify the code below!
// fn take_ownership(s: String) -> String {
//     println!("{}", s);
//     s
// }


fn main() {
    let mut s = String::from("hello, ");
    
    // Modify this line only !
    let  s1 = &mut s;

    s1.push_str("world");
    println!("{} ",s )  ; 
    println!("Success!");
}


