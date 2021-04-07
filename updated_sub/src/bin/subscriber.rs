use iota_streams::{
    app::transport::tangle::{
        PAYLOAD_BYTES,
        client::Client,
    },
    app_channels::api::tangle::Subscriber
};

#[tokio::main]
async fn main() {
    let args : Vec<String> = std::env::args().collect();
    let seed = args[1].as_str();
    let client = Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com");

    let encoding = "utf-8";  
    let application_instance = &args[2];
    let announcement_id = &args[3];
    let mut subscriber = Subscriber::new(seed, encoding, PAYLOAD_BYTES, client);

    updated_sub::subscribe(&mut subscriber, application_instance, announcement_id);
    updated_sub::receive(&mut subscriber);

}
