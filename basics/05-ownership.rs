// OwnerShip-1: Introduction

// fn main()
// {
//   let owner : String = String::from("Rust"); // i created an owner variable, it has access to this variable only

//   let trans_owner : String = owner; // i could transfer ownership of value to another variable, but you couldn't get access to original owner

//   // println!("Original Owner: {}", owner); // well since the ownership has transferred, og owner is owner anymore 

//   println!("Tranferred Owner: {}", trans_owner);

//  // well this ownership term works for all datatypes
    // UPDATE: well, i realised it isn't for all datatypes, like primitive datatypes i meant
    // they just copy the bytes
// }

// ------------------------------------------------------------------------------------------

// // OwnerShip-2 : Borrowing ownership as reference

// fn main()
// {
//   let owner : String = String::from("Rusty");
//   let trans_owner : &String = &owner; // we can borrow ownership as reference, but still the original owner is owner variable

//   println!("Original Owner: {}", owner);
//   println!("Transferred Owner: {}", trans_owner); // borrowed owner is depended on owner's ownership

//   // Note: owner lives inside the main function, you can pass reference from main function.
//   //       there is no ownership after main function's
// }

// ------------------------------------------------------------------------------------------

// OwnerShip-3 : rust recommended method for borrowing

// fn main()
// {
//   let owner : String = String::from("Rusty");
//   let trans_owner : String = owner.clone(); // idk, what it outputs, it might be reference

//   println!("Orignal Owner: {}", owner);
//   println!("Transferred Owner: {}", trans_owner);
// }

// ---------------------------------------------------------------------------------

// // OwnerShip-4 : with functions :)

// fn main()
// {
//   let owner : String = String::from("Rusty");

//   let len : usize = calc_length(owner);

//   println!("Orignal Owner: {}", owner);  
// }

// fn calc_length(s : String) -> usize // sadly, this function variable is owner now +_+
// {
//   s.len()
// }

// i just realised all pass by value in rust is just transferring the ownership of the variable +_+

// -------------------------------------------------------------------------------------------

// OwnerShip-5 : with functions ><

fn main()
{
  let owner = String::from("Rusty");

  let _length : usize = calc_length(&owner); // we pass reference to solve it!

  println!("Orignal Owner: {}", owner); // it works!
}

fn calc_length(s : &String) -> usize
{
  s.len()
} // after this function-call this owner is freed from ownership, so no worry about forgotten bytes