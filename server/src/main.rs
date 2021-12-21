use actix_web::{App, HttpServer, Responder, HttpResponse, get};
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, Debug)]
struct JokeResponse {
    setup: String,
    delivery: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ActivityResponse {
    activity: String,
}

async fn call_api<T>(url: String) -> reqwest::Result<T>
    where T: DeserializeOwned + Serialize
{
    let _response = reqwest::get(url).await;
    let result = match _response {
        Ok(data) => data.json::<T>().await,
        Err(err) => Err(err)
    };
    result
}

async fn call_joke() -> reqwest::Result<JokeResponse> {
    call_api(String::from("https://v2.jokeapi.dev/joke/Any")).await
}

async fn call_activity() -> reqwest::Result<ActivityResponse> {
    call_api(String::from("https://www.boredapi.com/api/activity")).await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/joke")]
async fn joke() -> impl Responder {
    let _response = call_joke().await.unwrap();
    HttpResponse::Ok().json(_response)
}

#[get("/activity")]
async fn activity() -> impl Responder {
    let _response = call_activity().await.unwrap();
    HttpResponse::Ok().json(_response)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(joke)
            .service(activity)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}