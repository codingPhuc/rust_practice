fn main() 
{
    let tup:(i8  , bool  , char) = (1, true  ,'s')  ; 
    let  (x , y , z) = tup ;
    // tup are immutable 
    println!("{} {} {} ",x, y ,z) ; 
    // if you want to change to tup you need the mut types 
    let mut arr = [1,2,3,4,5]  ; 
    // you can only create a new array in rust cannot add 
    arr[4]= 3  ;  
    println!("{}",  arr[4]);  
}

