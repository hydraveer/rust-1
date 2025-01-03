//********************************************************Struct************************************ */
// struct User{
//     length: u16,
//     width: u16,
// }
// impl  User {
//     fn greet(&self)->u16{
//         return self.length * self.width;
//     }
// }
// struct User; unit struct that help to implement only the function that can attech with other thing,

use std::rc::Weak;

// *************************************Enum************************************************* 
// enum Direction {
//     North,
//     South,
//     East,
//     West,
// }
enum Shape {
    Circle(f32),
    Square(f32),
    Rectangle(f32, f32),
}
fn calculate_are(shape:Shape)->f32{
    match shape{
        Shape::Circle(radius)=>3.14*radius*radius,
        Shape::Rectangle(width, length)=> width*length,
        Shape::Square(side)=>side*side,
    }
}
fn main() {
    let circle = Shape::Circle(3.2);
    print!( "{}", calculate_are(circle));
    //*********************************basic types*************************************************************
    // let mut x:i32 = -10;
    // let y:u32=1000;
    // let z:f32=1.233;
    //***********************************string******************************************************************
    // print!("x: {}, y: {}, z: {}",x,y,z);
    // let x :&str = "Hello world"
    //Type: String slice (&str)
    // let x :String =  String::from("hello world");
    //Type: String
    // print!("{}",x);
    //*****************************condtional statement*****************************************************
    // let even: bool = false;
    // if even {
    //     print!("Even");
    // } 
    // else if !even {
    //     print!("Odd");
    // }
    // let st: String = String::from("Veer Patel");
    // let char = st.chars().nth(0);
    // match char {
    //     Some(c) => print!("{}",c),
    //     None =>print!("No character at index 1")
    // }
    // for i in 0..9{
    //     print!("{}", i)
    // }
    // *****************************MEMORY MANAGEMENT*******************************************
    //
    // below program will help us to know that how much memory will allocate in the stack that point to heap
    // let mut st : String = String::from("Hello World");
    // print!("Capacity: {}, Length: {}, Pointer: {:p}", st.capacity(), st.len(), st.as_ptr());
    // st.push_str("This just for practice adfkljadfjakdsjfkjdfkjasdk;lfjklasjfadks;fajsddkfljalskdf");
    // print!("Capacity: {}, Length: {}, Pointer: {:p}", st.capacity(), st.len(), st.as_ptr());

    // ************************************************Ownership***************************************
    // let s1 = String::from("hello");
    // let _s2 = s1.clone();
    // print!("{}",s1)
    // let mut st = String::from("Hellow");
    // st = somefunction(st);
    // print!("{}",st);
    // *************************************************Reference****************************************** 
    // let st = String::from("Hello");
    // let st1 = &st;
    // let mut st = String::from("Hello");
    // update(&mut st);
    // print!("{}",st);

    // *****************************************Struct**********************************************
    // struct User{
    //     active : bool,
    //     username:String,
    //     email: String,
    //     sign_in_count:u64
    // }
    // let user1 = User{
    //     active:true,
    //     username:String::from("HydraVeer"),
    //     email:String::from("veer@12"),
    //     sign_in_count:30
    // };
    // print!("{:?}",user1.email)
    // let user = User{
    //     length: 23,
    //     width: 23
    // };
    // print!("{}",user.greet());
}

// fn somefunction(st:String)->String{
//     print!("{}",st);
//     return st;
// }
//


// fn update(s: &mut String){
//     s.push_str("World");
// }
