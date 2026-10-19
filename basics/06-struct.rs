// // Basic Struct

struct Blast // naming skills are bad
{
  damage : u32, // members syntax
  isBlasted : bool,
  Type : String
}

impl Blast // impl stands implementation or just method's section for struct
{
  fn announce(&self)
  {
    println!("The missle {}. Launching in 10s", self.Type);
    println!("DAMAGE: {}", self.damage);
  }

  fn constructor(&self, damage : u32, Type : String) -> Blast // we can have constructor like this 
  {
    Blast {
      damage,
      Type,
      isBlasted : true
    }
  }
}

// or Like this
  fn constructor(damage : u32, Type : String) -> Blast 
  {
    Blast {
      damage,
      Type,
      isBlasted : true
    }
  }

fn main()
{
  let missile : Blast = Blast { // initialising object, man i don't know like syntax
  // and you cannot initialise the object again using mutable keyword, but you can use shadowing
    damage : 25523,
    Type : String::from("A102"),
    isBlasted : true
  };

  // ig C has better syntax for this 
  // struct Data data = {.data = 12, .num = 23};

  missile.announce();
  
  let missile : Blast = missile.constructor(2332, "T101".to_string()); // shadowing here, objects are only initialised first ig

  let t_1 : Blast = Blast {
    // pasting same object data of missile to here
    Type : String::from("C-77"),
    ..missile // Note: if ..missile only exists here, then t_1 takes the ownership of missile
  };

  let t_2 : Blast = constructor(10101, "L-101".to_string());

  missile.announce();
  t_1.announce();
  t_2.announce();
}

// ----------------------------------------------------------------------------

// Tuple Structs

// struct Color(u8, u8, u8);

// fn main()
// {
//   let white : Color = Color(255,255,255);

//   println!("Red in white color: {}", white.0);
//   println!("Green in white color: {}", white.1);
//   println!("Blue in white color: {}", white.2);
// }

// ----------------------------------------------------------------------------

// Unit-structs i think, thats what they called