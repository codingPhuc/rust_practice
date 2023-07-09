use rand::Rng;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     let nice_slice = &a[1..4];

//     assert_eq!([2, 3, 4], nice_slice);
//     array_printer(&a);
//     array_printer(&nice_slice);

//     let x = String::from("this is it")  ;
//     let mut a =  10  ;
//     let dom = x   ;
//     let b =  &mut a ;
//     *b += 10 ;

//     println!("this is the dom element {} {}  ",dom,b)  ;

// }
// // this is a statement
// fn array_printer(a : &[i32])
// {
//     for ele  in a
//     {
//         println!("{}",ele);
//     }
// }

use extern rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}
// fn  gameRockPaper(s : &String) ->bool
// {

//     if (s.eq("scissor")  )
//     {
//         return true ;
//     }
//     if (s.eq("scissor")  )
//     {
//         return false ;
//     }
// }
