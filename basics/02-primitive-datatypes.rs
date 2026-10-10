fn main()
{
  // DECLARING A VARIABLE
  // syntax: let var_name: datatype = value;

  // printing an variable
  // it is closely similar to C
  // println!("{}", variable)
  // {} is like format specifier

  // they are primitive datatypes: signed int, usigned int, char, bool, and float

  // for INT
  // sint's datatype notation is i8, i16. i32, i64, i128
  // uint's datatype notation is u8, u16. u32, u64, u128

  let int_var: i8 = 15;
  let u_int_var: u8 = 22;

  println!("Signed int: {}", int_var);
  println!("Unsigned int: {}", u_int_var);

  // Float
  // float dataype in rust, covers both, double and floating point number
  // with this notation: f32, f64

  let float_var: f32 = 10.01001;

  println!("the floating point number: {}", float_var);

  // booleans
  let bool_var = true;

  println!("This is you were {}", bool_var);

  // chars

  let letter: char = 'Z';

  println!("The char is!\n{}", letter);

  // A Quick Note: we can declare variables as JS does, like let x = 15.(its an int)

  // NOTE: all variables declared as constants by default, you need to change it by using "mut" keyword

  let mut mutable_x: i8 = 12;
  println!("Before modifying: {}", mutable_x);
  mutable_x = 16;
  println!("After modifying: {}", mutable_x);
}