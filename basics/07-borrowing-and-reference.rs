// Oh god, i didn't expect rust is so strict on borrowing

// BIG NOTE: println!(); function do take temporary borrows.

// The Problem

// fn main()
// {
//   let mut name : String = String::from("Sudheer"); // we initialised the owner here, which is mutable

//   // [name]

//   let another_name = &mut name; // a mutable borrow for another_name.

//   // [another_name]-->[name]

//   *another_name = String::from("Zang"); // since it is mutable borrow, we dereference to access name's value to change

//   println!("The name: {}", name); // we are printing the owner by taking borrow, and it is an immutable borrow
//   println!("The another name: {}", another_name); // here for another_name there is conflict to 
//   // what name we need to call, is it &mut name by declaration or &name from println!() function
     
     // [another_name]        [name](mutable during declaring)
     //                       [name](immutable from println!)
// }


// SOlution

fn main()
{
  let mut name : String = String::from("Sudheer");
  let another_name = &mut name;

  *another_name = String::from("Zang");

  // Solution 1
  println!("the Another name: {}", another_name); // for this, [another_name]--> [name](mutable one)
  println!("the name: {}", name); // [name]--> value

  // Solution 2
  println!("Name: {}", name); // [name]--> value
}