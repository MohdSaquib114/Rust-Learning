fn main() {
    let a = 1;
    let b = 10;
    for i in a..b {
         println!("{} ",i)
    }

    let is_even = true;

    if is_even {
        println!("Number is even");
    }else if !is_even {
        println!("Number is odd")
    }

let greetings = String::from("Hello World");

println!("{}",greetings);


let mut is_male = false;
let is_above_18 = true;
if is_above_18 || is_male {
    is_male = true;

}

  
    let  x : u32 = 100;
    let y = -32;
    let z = 100.002;
    println!("x: {}, y: {}, z: {}",x,y,z);
    println!("Hello, world!");
}
//All  variables are immutable by default

//i8
//i16
//i32
//i128

//u8
//u16
//u32
//u128

//f32
//f64