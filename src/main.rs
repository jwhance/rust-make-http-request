use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;

// https://www.becomebetterprogrammer.com/rust-how-to-make-an-http-request/

#[tokio::main]
async fn main() {
    let fact = get_cat_fact().await;

    //let cat_fact = fact.unwrap().fact;
    println!("fact = {:#?}", fact);
}

async fn get_cat_fact() -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        // .get("https://catfact.ninja/fact")
        .get("http://192.168.100.62:8080/simple-service-webapp/webapi/myresource")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;

    Ok(body)
}

#[derive(Deserialize, Debug)]
struct CatFact {
    fact: String,
    length: i32,
}

/*
    {
        "firstName": "John",
        "htLabel": 10,
        "lastName": "Doe"
    }
*/
#[derive(Deserialize, Debug)]
struct ApiResponse {
    firstName: String,
    lastName: String,
    htLabel: i32,
}
