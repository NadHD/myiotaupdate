#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use iota_streams::app_channels::api::tangle::{
    Address,
    Subscriber,
    Transport,
    MessageContent,
};

pub fn subscribe<T: Transport>(subscriber: &mut Subscriber<T>, application_instance: &String, announcement_id: &String,) {
    let announcement_link = Address::from_str(&application_instance,&announcement_id)
        .expect("Failed to create the Announcement Link");
    
        
    subscriber.receive_announcement(&announcement_link)
        .expect("Failed to subscribe to the Channel");

    println!("Subscribed to the Channel");
}

pub fn receive<T: Transport>(subscriber: &mut Subscriber<T>) {
    let messages = subscriber.fetch_next_msgs();
    if messages.is_empty() {
        println!("No notifications");
    } 
    else {
        for message in messages {
            match message.body {
                MessageContent::SignedPacket { pk: _, public_payload, masked_payload: _ } => {
                    println!("Notification: {}", String::from_utf8(public_payload.0).unwrap());
                },
                _ => {}
            }
        }
    }
}
