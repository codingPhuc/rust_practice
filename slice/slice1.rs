// if you need allowcation used String 
// if you need a different view of a String used &str 

fn main()
{
    let  s = String::from("Jason Todd") ;  
    let _first_name   = get_first_name(&s)  ;  
    println!("{} ",_first_name);
    println!(" ");
}


fn  get_first_name(first : &String) -> &str
{
    let bytes   =  first.bytes()  ;
    let  mut _break_point:u32 = 0  ; 
    for b  in bytes  
    {
        _break_point+=1  ;
        println!("{} ",b.to_string());
        if b == (*b" ")[0]
        {        
            break  ;  
        }
         
    }
    println!("{} ", _break_point);
    &first[0.._break_point as usize]
}