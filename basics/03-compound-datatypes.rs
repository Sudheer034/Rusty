fn main()
{
  let array: [i8; 3] = [12,23,34];
//             |    
//             +-----> tbh im shocked, to declare array comletely like this, you can declare
//
//  Note: i said completly, that doesn't imply that i need to declare like this 
//  everytime, and it is bit useful ig, it says it is an array with the brackets, and sizeof 3, 
//  separating the data with semicolon :)

// well you can declare array like this too, let array = [1,2,3];
// it is C array :)
println!("Array: {:?}", array);
//                  |
//                  +---> ig its role, not only the format specifier, but also has additional quirks
//                        idk what it does, but maybe it decodes whether it is an array, string, tuples and some other

// let string: &str = "string";
//              |
//              +-----> ig compiler act as pointer to parse it :)

// println!("{}", name);

let flower: [&str; 3] = ["rose", "lily", "sunflower"];

  println!("Flowers: {:?}", flower);
  
  // Accessing elements is normal
  
  println!("{}, {}, {}", flower[0], flower[1], flower[2]);

// -----------------------------------------------------------------------
  
  // TUPLES

  let human: (&str, i8, bool) = ("Zang", 19, false); // this is a tuple
  // a tuple can have multiple datatypes

  println!("Human: {:?}", human);

  // Accessing from TUPLE

  println!("Name: {}", human.0); // the heck with rust, and what if i used a loop, how could i access it!
  // i know we don't use it in loops due to it can take multiple datatypes, still what if lol
  println!("Age: {}", human.1);
  println!("Dead: {}", human.2);

// -----------------------------------------------------------------------
  
  // SLICES

  // a quick idea for slices, the slices is like reference having limits, like you need to mention
  // how much the size is. it is read-only

  // btw &str is also slice :)

  // syntax: let var_name : &datatype = value;

  let slice_tuple : &(&str, i8, bool) = &("Crassia", 22, true);
//                  |
//                  +-----> looks cool
  println!("sliced tuple: {:?}", slice_tuple);

  // Note: i just realised, it is reference to a tuple, not a slice of tuple, slices exists for array-datatype

// -----------------------------------------------------------------------
  
  //  String
  // this is a string as name says, but it can grow, and it is a struct or class, ig

  let mut racoon : String = String::from("Purrs "); // rust variables are immutable, so we use "mut"
  racoon.push_str("and Purrs~"); // so we can push or concate a string here
  println!("Raccoon {:?}", racoon);

// -----------------------------------------------------------------------
  
  // String vs &str
  // actually &str is string slice already :)

  // ig every string we declare is a string slice, unless we explicitly change it using .to_string()

  let hello_world : String = String::from("Hello, World.");

  let string_slice : &str = &hello_world[0..5]; // you might think using &String, compiler throws an error 
  // well &str is already string slice and idk why it didn't accept it.

  println!("String Slice: {:?}", string_slice);
}
