// 3 keywords for loops: loop, for, while

// LOOP

// fn main()
// {
  // loop // this is just a loop that loops indefinetely
  // {
  //   println!("Hello");
  // }

  // LOOP with CONDITIONS

//   let mut count =  0;

//   'say_hello : loop /* we can set labels too, thats cool, though it feels weird to add a single 
//   coat at the beginning  */
//   {
//     println!("Hello {count}");

//     if count == 3
//     {
//       // break 'say_hello; // using label we can set constrol flow, it is more like assembly here
//       break count = 5; // btw break has another feature, we can set values here. it is useful
//     }

//     count+=1;
//   }

//   println!("count: {count}");
// }

//-----------------------------------------------------------------------------------

// WHILE

fn main()
{
  let mut count = 0;

  while count <= 4
  {
    println!("{count}");
    count += 1
  } 
}

// ---------------------------------------------------------------------------

// FOR LOOP

// fn main()
// {
//   let fruits = ["apple", "banana", "pineapple", "orange"];

//   for i in fruits // well it does look JS's forEach method, and it really is
//   {
//     println!("{i}");
//   }

//   for j in 1..6 // i really forgot this exists in rust, like it is slice and also range 
//   {
//     println!("{j}");
//   }
// }


// we can set variables to all type of loops