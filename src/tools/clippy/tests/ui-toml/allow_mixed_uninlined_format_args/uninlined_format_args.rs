#![warn(clippy::uninlined_format_args)]
#![allow(clippy::unnecessary_literal_unwrap)]

fn main() {
    let local_i32 = 1;
    let local_f64 = 2.0;
    let local_opt: Option<i32> = Some(3);

    println!("val='{}'", local_i32);
    //~^ uninlined_format_args
    println!("Hello {} is {:.*}", "x", local_i32, local_f64);
    //~^ uninlined_format_args
    //~| print_literal
    println!("Hello {} is {:.*}", local_i32, 5, local_f64);
    //~^ uninlined_format_args
    println!("Hello {} is {2:.*}", local_i32, 5, local_f64);
    //~^ uninlined_format_args
    println!("{}, {}", local_i32, local_opt.unwrap());
    //~^ uninlined_format_args
}
