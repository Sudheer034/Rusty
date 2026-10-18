// in Rust, theren isn't clash on borrowing in methods of structs functions, they are all independent to each other

struct Bank
{
  name : String,
  amount : u32,
}

impl Bank
{
  fn withdraw(&mut self, amount : u32) // this withdraw function exists only at this scope only
  {
    println!("{} is withdrawed from {}", amount, self.name);
    self.amount -= amount;
  }

  fn balance(&self) // this is in different scope, so it doesn't clash with withdraw function
  {
    println!("The current balance from {} is {}", self.name, self.amount);
  }
}

fn main()
{
  let mut user1 = Bank {
    name : "Alice".to_string(),
    amount : 2000,
  };

  user1.balance();
  user1.withdraw(500);
  user1.balance();
}