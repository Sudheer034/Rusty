mod math; // the subsequent file name(crate name prob)
mod util; // the subsequent folder that contains crates

fn main()
{
  let c = math::sub(12, 6);
  println!("The value of c: {c}");

  let d = util::math::add(12, 6);
  println!("The value of d: {d}");
}