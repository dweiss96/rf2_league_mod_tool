use serde_json::Result;

mod models { automod::dir!(pub "src/models"); }

use crate::models::person::Person;
use crate::models::company::Company;


slint::include_modules!();
fn main() {
    HelloWorld::new().unwrap().run().unwrap();
    _ = typed_example_a();
    _ = typed_example_b();
}

fn typed_example_a() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}

fn typed_example_b() -> Result<()> {
  // Some JSON input data as a &str. Maybe this comes from the user.
  let data = r#"
      {
          "name": "eXa",
          "workercount": 43,
          "phones": [
              "+44 1234567",
              "+44 2345678"
          ]
      }"#;

  // Parse the string of data into a Person object. This is exactly the
  // same function as the one that produced serde_json::Value above, but
  // now we are asking it for a Person as output.
  let p: Company = serde_json::from_str(data)?;

  // Do things just like with any other Rust data structure.
  println!("Please call {} at the number {}", p.name, p.phones[0]);

  Ok(())
}