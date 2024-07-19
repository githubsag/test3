use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    let client = Client::new();

    let user_name = "tesuser".to_string();
    let password: Option<String> = None;

    let response = client
    .get("https://github.com/githubsag/test2/")
    .basic_auth(user_name, password)
    .send();

    println!("{:?}", response);

    Ok(())
}
