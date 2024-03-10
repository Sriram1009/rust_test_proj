// use std::io

fn main() {
    println!("Hello, world!");

    let mut data = 110;

    println!("{}", test_function(&mut data))

}

fn test_function(data: &mut i32 ) -> i32{
    *data = *data +1;
    *data + 10
}