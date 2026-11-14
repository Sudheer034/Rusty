// i think using this is straightforward
struct Point<T>{
  x: T,
  y: T,
  z: T
}

fn main()
{
  let xyz = Point{x: 1, y: 1, z: 1};

  println!("{}, {}, {}", xyz.x, xyz.y, xyz.z);

  let abc = Point{x: 'a', y: 'b', z: 'c'};

  println!("{}{}{}", abc.x, abc.y, abc.z);
}