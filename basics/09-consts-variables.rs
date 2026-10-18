// Lets do some basics

fn main()
{
  // we got const keyword
  // it is similar to macro initilisation in C
  const VALUE : u32 = 5; // btw const keyword, needs to initiliased with its datatype(thats recommended)
  println!("value: {VALUE}");

  // some might think we use let for immutable datatype, and we add mut to make it as mutable
  // so for new beginners might think we can use "const mut var_name" well this is wrong

  let mut mut_var = 23;
  println!("Before modifying : {mut_var}");
  mut_var = 12;
  println!("After modifying : {mut_var}");

  const CHECK : u32 = { // it is also works like returning a function
    let price = 32;
    let product = 12;
    product * price 
  };

  println!("Just check {CHECK}");

  // ig you already know this :)
}