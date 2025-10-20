mod domain;
mod infrastructure;
mod services;
mod web;

#[tokio::main]
async fn main(){
    dotenv::dotenv().ok();
    web::run().await;
}
