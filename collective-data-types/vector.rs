fn main()
{
  // Vector Declaration
  // let mut _v : Vec<i32> = Vec::new(); // "new" method is to create a dynamic array 
  // _v.push(1); // we can push elements here
  // _v.push(2);
  // _v.push(3);
  // _v.push(4);

  // println!("{:?}", _v);

//-----------------------------------------------------------------------

  // Vector declaration with some data
  let mut _v : Vec<i32> = vec![1,2,3]; // "vec!" is a macro and ig println! is also macro by name pattern
  _v.push(4);

  println!("{:?}", _v);

  let mut _v : Vec<String> = Vec::new();
  _v.push("Crassia".to_string()); // or you can use String::from("Crassia"), if you want to edit
  _v.push("Laevia".to_string());
  _v.push("Dressia".to_string());
  _v.push("Placko".to_string());

  _v.pop(); // to pop the last element

  println!("{:#?}", _v);

  // Getting an element from an index

  let _i : &String = &_v[2]; // by borrowing
  println!("{}", _i);

  // By get method

  let _i : Option<&String> = _v.get(5); // by get method, well it uses enum, borrowing datatype

  // to read the data, we use match
  match &_i{
    Option::Some(val) => { // i realised we don't need to use "Option::" to access constructors, 
    // though for only builtin enum ig
      println!("{}", val);
    }

    Option::None => {
      println!("Not Found");
    }
  }

  // Match tool looks hefty work, but it is good for no compilation error for the least +_+
}