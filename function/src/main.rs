fn main() {
    println!("Hello, world!");
    //let y = (let y  =20 )  ; 
    // let here is a statement it does not return any thing 

    test() ;  
    add_numbers()  ; 
    // anything that return or give you a function call is an expression 
    let number = {
        let x = 3 ; // start by having an expression 
        x +1  // then return the value if you dont put a  ; you will return a value 
    }; 
    println!("{}",number)  ; 
}
// the resond we will used function is beacause we want to reused the code 
fn test()  
{ 
    println!("test have been called") ; 

}
fn add_numbers(x: i32, y : i32) {
    println!("the sum is : {} ",x+y)  ; 

}
// if you want to return a value you need to put the type of the value 
// the expression only happen if you put return in the statement 
fn  add_numbers1(x:i32 ,  y : i32) -> i32{ 
    x +  y   

}





// rust function can return a expression but cannot a statement 
