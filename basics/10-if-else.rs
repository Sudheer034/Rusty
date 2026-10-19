// lets begin if-else statements!

// BASICS

// fn main()
// {
//   let age : u8 = 18;

//   if age < 18 // normal syntax for rust, man i really need parenthesis +_+
//   {
//     println!("You are young.");
//   }
//   else if age >= 18 && age <= 40
//   {
//     println!("You are an adult.");
//   }
//   else
//   {
//     println!("You are a senior.");
//   }
// }

//-----------------------------------------------------------------------------------------

// SETTING VALUE TO A VARIABLE THROUGH CONDITION

fn main()
{
  let condition : bool = false;
  let check : i32 = if condition {5} else {-6};

  println!("Check: {check}");
}