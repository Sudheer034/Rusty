// INTRODUCTION

// fn main()
// {
//   #[derive(Debug)] // idk what it is but compiler made me to add it here for enum Number to debug ig
//   enum Number
//   {
//     Zero,
//     One,
//     Two,
//   }

//   let two : Number = Number::Two;

//   // here debugging takes place and outputs variable name, i thought it is C like enum +_+

//   println!("{:?}", two); // Two 
// }

// --------------------------------------------------------------------------------------------

// IP ADDRESSES KIND

fn main()
{
  // enum IpAddrKind
  // {
  //   V4, // well this enum just hold their data(their label name, ex: V4 is V4)
  //   V6
  // }
  
  // let home : IpAddrKind = IpAddrKind::V4;
  // let loopback : IpAddrKind = IpAddrKind::V6;

  //----------------------------------------------------------
  
  // Using Struct
  
  // enum IpAddrKind
  // {
    //   V4, // well this enum just hold their data(their label name, ex: V4 is V4)
    //   V6
    // }
    // struct IpAddr
    // {
      //   kind : IpAddrKind,
      //   address : String
      // }
      
      // let home: IpAddrKind = IpAddrKind {
        //   kind : IpAddrKind::V4,
        //   address : String::from("127.0.0.1");
        // }
        
        // let loopback: IpAddrKind = IpAddrKind {
          //   kind : IpAddrKind::V6,
          //   address : String::from("::1");
          // }
          
          // To solve this rust has enums variables as constructors of datatype, like V4 we can set it to String or u8
//----------------------------------------------------------

// Enum Way

// enum IpAddrKind
// {
  //   V4(String), // as constructors
  //   V6(String)
  // }
  
  // let home: IpAddrKind = IpAddrKind::V4(String::from("127.0.0.1")); // syntax looks cleaner
  // let local: IpAddrKind = IpAddrKind::V6(String::from("::1"));
  
  //----------------------------------------------------------

  // Enhanced Enum

  // it is like tuple struct

  enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
  }

  let home: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1); // this also works!
  let local: IpAddrKind = IpAddrKind::V6(String::from("::1"));

  // To access the data inside an enum is by pattern matching in rust +_+

  // match keyword is used to access this variable or enum ig

  // match &home{ // we are borrowing not giving ownership
    
  //   // here were match implements
  //   IpAddrKind::V4(a, b, c, d) => { // ig => means if it exists implement this 
  //     println!("IP(V4) ADDRESS OF HOME: {a}.{b}.{c}.{d}");
  //   }
    
  //   IpAddrKind::V6(s) => { // since this doesn't exists it doesn't implement it
  //     println!("IP(V6) ADDRESS OF LOCAL: {s}");
  //   }
  // }
  
  // match &local{

  //   IpAddrKind::V4(a, b, c, d) => {
  //     println!("IP(V4) ADDRESS OF HOME: {a}.{b}.{c}.{d}");
  //   }

  //   IpAddrKind::V6(s) => {
  //     println!("IP(V6) ADDRESS OF LOCAL: {s}");
  //   }
  // } // to be honest, the syntax is more explicit

  // To get over that, we use "if let" keyword to point to a single variable

  if let IpAddrKind::V4(a, b, c, d) = &home {
    println!("IP ADDRESS OF HOME: {}.{}.{}.{}",a,b,c,d);
  };

  if let IpAddrKind::V6(s) = &local {
    println!("IP ADDRESS OF LOCAL: {}", s);
  };
}
