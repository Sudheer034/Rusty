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
//                  +
//                  |---> ig its role, not only the format specifier, but also has additional quirks
//                        idk what it does, but maybe it decodes whether it is an array, string, tuples and some other

// let string: &str = "string";
//              |
//              +-----> ig compiler act as pointer to parse it :)

// println!("{}", name);

  // let flower: [&str; 3] = ["rose", "lily", "sunflower"];

  // println!("Flowers: {:?}", flower);
}