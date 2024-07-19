use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {

    // add hotfix to the program
    let (x,y) = (1,2);
    println!("{}", format_args!("{x} + {y} = 3"));
    
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
