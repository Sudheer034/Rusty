// Rust Generics needs more info like how it returns.

fn smallest<T: std::cmp::PartialOrd>(a: T, b: T) -> T { // from standard library we calling cmp(compare) to check which type of comparion is it.

  /* you can see, i used '>' for both a and b, it comes under PartialOrd*/  
  if a > b { b }
  else { a }
}

/*
There are four comparision how rust returns

ParialEq (==, != ) -> it does most of it, but some values cannot equal to itself(NAN[not a number])

Eq (==, != ) -> it has full flexibility to see

PartialOrd (>=, <=, >, <) -> it uses comparision <operators

Ord (>=, <=, >, <, .cmp(), ...) -> it has more functions that comes under compares
*/

fn main()
{
  // accessing value explicitly
  let c = smallest::<i32>(2, 1); // its called turbofish operator/ 
  println!("The value of C: {c}");
}