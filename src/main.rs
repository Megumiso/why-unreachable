
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let client = reqwest::Client::new();
    let hoge  = client.get("http://example.com") //Unused???
        .send()
        .await
        .expect({ // Unreachable??
            std::process::exit(1);
        })
        .text()
        .await
        .expect("convert to text is failed somehow!");
    println!("Reached!: Here's the output: {}", hoge);
    std::process::exit(0);
}
