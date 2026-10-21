// Lets learn match in rust

/* i didn't expecet we can use match to set values too, i think it is pretty handy rather than
  using multiple if-else statement to set a value, we can use match and also it is like LUT
*/

#[derive(Debug)]
enum Coin { // we initialised an enum :)
    Penny,
    Nickel,
    Dime,
    Quarter,
}


// To get value of specific variable
fn value_in_cents(coin: Coin) -> u8 { // here Coin is initialised, Note Coin::Penny or Coin::Nickel are also Coin datatype

    match coin { // match function checks the pattern of initialised Coin vairable by default
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,// if the pattern is this, and this funtion returns 10.
        Coin::Quarter => 25, // it sounds bit confusing yeah, lets think match is a function and it is just returns code block or values
    }

    // Rust implicity understand what to return

    /*
    let result = match coin{
      ..
      ..  
        }
      result
     */
}


fn main()
{
  let _penny : u8 = value_in_cents(Coin::Penny);
  let _nickel : u8 = value_in_cents(Coin::Nickel);

  println!("{}", _penny);
  println!("{}", _nickel);
}