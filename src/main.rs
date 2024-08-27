use anyhow::Result;
use diesel::prelude::*;
use service::database::{conn::establish_connection, models::User};

mod schema;
mod service;

fn main() -> Result<()> {
    use self::schema::users::dsl::*;
    let connection = &mut establish_connection()?;
    let result = users.load::<User>(connection)?;
    println!("Displaying {} users", result.len());
    for user in result {
        println!("{}", user.username);
    }
    Ok(())
}
