use lib::run;

mod lib;

#[tokio::main]
async fn main() {
    run().await.unwrap();
}
