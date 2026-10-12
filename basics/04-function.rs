// Rust allows hoisting!

fn main()
{
  print_hello();
  show_name("Zang");
  show_id("Zang", 19, false);

// ---------------------------------------

// variable for code-block

  let _x : i32 = {
    let a = 12;
    let b = 13;
    a-b // it returns this value, and it implicit to `return a-b;`
    // they kinda removed semi-colon for this :(
  }; 

  println!("Value of X: {}", _x);

  // Calling BMI

  let _weight = 50.0;
  let _height = 1.71;
  let _y = bmi(_weight, _height);

  println!("My BMI: {:.2}", _y); // .2 for truncat to this decimal position
}

fn print_hello()
{
  println!("Hello");
}

fn show_name(name : &str) // for declaring, we need to specify the variables data-type
{
  println!("Name: {}", name);
}

fn show_id(name : &str, age : u32, online : bool)
{
  println!("My name is {}, an {} years old. status: {}", name, age, online);
}

// ----------------------------------------------

// function returns a value


fn bmi(weight_kg : f64, height_m : f64) -> f64 // -> f64 means returns floating point number
{
  weight_kg/(height_m * height_m) // well here also this syntax applies :(
}