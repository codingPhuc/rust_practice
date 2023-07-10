// example program using Structs


fn area(width: u32, height: u32) -> u32 {
    width * height
}
// even thought our code is much cleaner it is unclear what element in tup is witdh and what element is height 
fn area1(dimension : (u32 , u32)) ->u32  
{
    dimension.0 * dimension.1  
}
// we will add struct to add the meaning to our program 
#[derive(Debug)]  
struct Rectangle 
{
     width  : u32  ,
     height  : u32  ,  
} 

fn area2(rectangle : &Rectangle) -> u32 
{
    rectangle.width * rectangle.height   
}

 
fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    let rect1 = (30 , 50)  ; 
    println!("the area of the rectangle is {} square pixels", area1(rect1)) ;
    let rect2  = Rectangle { 
         width :   30 , 
         height : 50   ,
    };
    println!("the area of the rectangle is {} square pixels", area2(&rect2)) ;
    // let say we want to print the value of the instance of the  rectangle 
    // we will used print  {:?}
    // but this will not work so we need to used the debug function 
    println!("the value of the instance of the rectangle is {:#?} ", rect2) ; 
    // now as the progame run we will not see the error code 
    //we can also used the dbg! 
    // we used a referent to the instand beacuase we do not want it to take ownership 
    dbg!(&rect2) ;  


}
