 use std::ops::Neg;

//main entry
fn main() {
  print_something("my string");
  var_fun();
}

/*
* Trying stuff with strings and format.
*/
fn print_something(string: &str) {
  println!("Your string: {}", string);
  println!("String to integer {}", "10".parse::<u8>().unwrap());
  println!("Integer to string {}", 10.to_string());
}

/*
* Playing with variables
*/
fn var_fun() {
  //define mutable signed integer 8 negate value
  let mut num: i8 = 5_i8.neg();

  num += 10;

  println!("Number is: {}", num)
}