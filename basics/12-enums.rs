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
  // let home: IpAddrKind = IpAddrKind::V6(String::from("::1"));
  
  //----------------------------------------------------------

  // Enhanced Enum

  // it is like tuple struct

  enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
  }

  let home: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1); // this also works!
  let home: IpAddrKind = IpAddrKind::V6(String::from("::1"));
}

// I wondered how do i get access to the data, which i will update it sooner :)