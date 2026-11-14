// formatting the display in the terminal
fn print<T: std::fmt::Display>(param: T)
{
  println!("{}", param);
}

fn main()
{
  print(2);
  print("Zang");
}