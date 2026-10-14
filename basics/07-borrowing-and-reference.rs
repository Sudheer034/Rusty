// Oh god, i didn't expect rust is so strict on borrowing

// BIG NOTE: println!(); function do take temporary borrows.

// The Problem

// fn main()
// {
//   let mut name : String = String::from("Sudheer"); // we initialised the owner here, which is mutable

//   // [name]

//   let another_name = &mut name; // a mutable borrow for another_name.

//   // [another_name]-->[name], and name is restricted to "&mut name" only until another_name frees out

//   *another_name = String::from("Zang"); // since it is mutable borrow, we dereference to access name's value to change

//   println!("The name: {}", name); // the datatype changed from &mut name to &name, and still another_name is locked it to &mut name, so compilation error
//   println!("The another name: {}", another_name);
// }


// SOlution

fn main()
{
  let mut name : String = String::from("Sudheer");
  let another_name = &mut name;

  *another_name = String::from("Zang");

  // Solution 1
  println!("the Another name: {}", another_name); // for this, [another_name]--> [name](mutable one)
  println!("the name: {}", name); // [name]--> value, // the lock is gone, we can access immutable borrow with no worries

  // Solution 2
  println!("Name: {}", name); // [name]--> value
}

// BIGGEST NOTE: name is locked from `&mut name` until the LAST USE of another_name