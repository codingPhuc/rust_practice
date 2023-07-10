#![allow(dead_code, unused_variables)]
struct User {
    active : bool  ,  
    username : String , 
    email : String  ,  
     sign_in_count : u64 ,  
}
// Unit like Struct Without any fields 
Struct  AlwayEqual    ; 



// TUPLE Struct   
struct Color(i32 , u8  , i8)  ; 

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername 123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail");
    let  user2 = build_user("JasonTodd@2233".to_string(), "todd".to_string());
    println!("{}  {  } { } ",  user2.active   , user1.email , user1.sign_in_count); 

    // NOTE THE VALUE IN BOTH THE USER IS COPY AND MOVE NOT CLONE 
    let user3 =  User{ 
        email : String::from("thisisanother@user") , 
        ..user1   
    } ;
    println!(" this is user3 :{} {} {} ", user3.email  , user3.username , user3.sign_in_count)   ; 
    let subject = AlwaysEqual  ; 
    

}


fn build_user(email: String, username: String) -> User {
     User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }

}
