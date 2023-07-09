fn main() {
    let mut s = String::from("hello, ");
    
    // Modify this line only !
    let  s1 =  &mut s;

    s1.push_str("world");

    println!("Success! {}",s);
}



