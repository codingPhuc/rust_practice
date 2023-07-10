// fn main() {
//     let string = String::from("hello world")  ; 
//     let std:&str  = "hello there" ; 
//     println!("{} " , string.len())  ;
//     println!("{} ", std.len())  ;
// }
// reverse the string 

pub fn reverse(input: &str) -> String  {
    
    input.chars().rev().collect()  
}


fn main () 
{ 
    let string:&str    =  "uüu"; 

    println!("{}" , reverse(string)); 
    
}