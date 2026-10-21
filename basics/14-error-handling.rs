// Error Handling

// we got two types of error handling: Option and Result

// we use enums for error handling

// fn main()
// {
  // We use templates for enums here, T is success here

  // OPTION

  // enum Option<T>{
  //   Some(T), // it results success
  //   None // it results no success
  // }

  // /* We use match to print the info */

  // // Lets create a function division

  // fn division(nominator: f32, denominator: f32) -> Option<f32> // we are returning the Option enum
  // {
  //   if denominator == 0.0
  //   {
  //     Option::None // we gotta access it through enum for error, well i noticed why enums are more used, they are namespaces for us to access it
  //   }
  //   else
  //   {
  //     Option::Some(nominator / denominator)
  //   }
  // }


  // let result: Option<f32> = division(10.0, 2.0);

  // match &result{
  //   Option::Some(val) => println!("Success: {}", val),
  //   Option::None => println!("it Cannot Divide by Zero")
  // }
// }

// -----------------------------------------------------------------------------------------------

// RESULT

fn main()
{
  // Result takes two templates parameters: Success Type, Error Type
  enum Result<T, E>{
    Ok(T),
    Err(E)
  }

  fn division(nominator: f32, denominator: f32) -> Result<f32, String>
  {
    if denominator == 0.0
    {
      Result::Err(String::from("It Cannot Divide by Zero."))
    }
    else
    {
      Result::Ok(nominator / denominator)
    }
  }

  let result: Result<f32, String> = division(10.0, 0.0);

  match &result{
    Result::Err(err) => println!("{}", err),
    Result::Ok(val) => println!("{}", val)
  }
}