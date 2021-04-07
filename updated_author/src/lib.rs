#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use iota_streams::app_channels::api::tangle::{
    Author,
    Address,
    Bytes,
    Transport,
};

pub fn announce<T: Transport>(author: &mut Author<T>) -> (String, String){
    let announcement_link = author.send_announce().expect("Failed to announce the Channel");

    println!("Announced the channel");

    (announcement_link.appinst.to_string(), announcement_link.msgid.to_string())
}

pub fn send<T: Transport>(author: &mut Author<T>, app_inst: &String, announcement_id: &String, notification: &String){
    let announcement_link = Address::from_str(app_inst, announcement_id).expect("Failed to create the announcement link");

    let public_payload = Bytes(notification.as_bytes().to_vec());
    let masked_payload = Bytes("".as_bytes().to_vec());

    author.send_signed_packet(&announcement_link, &public_payload, &masked_payload)
        .expect("Failed to send notification to the Channel");

    println!("Sent the notification to the Channel");
}
