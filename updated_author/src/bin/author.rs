use iota_streams::{
    app::transport::tangle::{
        PAYLOAD_BYTES,
        client::Client,
    },
    app_channels::api::tangle::Author
};

#[tokio::main]
async fn main() {
    let client = Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com");

    let encoding = "utf-8";
    let multi_branching = true;
    let mut author = Author::new("TEST1", encoding, PAYLOAD_BYTES, multi_branching, client);

    let (app_instance, announcement_id) = updated_author::announce(&mut author);
    let notification = "HELLO".to_string();
    updated_author::send(&mut author, &app_instance, &announcement_id, &notification);
    println!("Now use the Subscriber to subscribe to the Channel and receive the notification, by running:");
    println!("cargo run --bin subscriber {} {} {}", "TEST1", app_instance, announcement_id);
}
