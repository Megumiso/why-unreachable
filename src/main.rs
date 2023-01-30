
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let client = reqwest::Client::new();
    let _hoge = client.post("http://example.com")
        .send()
        .await
        .expect({
            std::process::exit(1);
        });
    std::process::exit(0);
}
