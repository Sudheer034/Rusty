// // Basic Struct

// struct Blast // naming skills are bad
// {
//   damage : u32, // members syntax
//   isBlasted : bool,
//   Type : String
// }

// impl Blast // impl stands implementation or just method's section for struct
// {
//   fn announce(&self)
//   {
//     println!("The missle {}. Launching in 10s", self.Type);
//   }
// }

// fn main()
// {
//   let missile : Blast = Blast { // initialising object, man i don't know like syntax
//     damage : 25523,
//     Type : String::from("A102"),
//     isBlasted : true
//   };

//   // ig C has better syntax for this 
//   // struct Data data = {.data = 12, .num = 23};

//   missile.announce();
// }

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