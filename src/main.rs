extern crate discord;

use discord::model::Event;
use discord::Discord;

use std::env;

pub mod quirk;
pub mod tests;

fn main() {
	let discord = Discord::from_user_token(&env::var("DISCORD_TOKEN").expect("Expected token"))
		.expect("login failed");

	// Establish and use a websocket connection
	let connection = discord.connect();
    let (mut connection, _) = match connection {
        Ok(res) => res,
        Err(err) => {
            panic!("Error: {}", err);
        },
    };
    
    let bot_id = discord.get_current_user().unwrap().id;
	println!("Ready. {}", bot_id);
	loop {
		match connection.recv_event() {
			Ok(Event::MessageCreate(message)) => {
                if message.author.id == bot_id {
                    println!("{}", message.content);
					let cs = quirk::Characters::from_string(&message.content);
					let quirked_message = &cs.quirked();
					if quirked_message != &message.content {
						println!("Quirkable message");
						println!("{}", quirked_message);
						discord.edit_message(message.channel_id, message.id, quirked_message).unwrap();
					}
                }
			}
			Ok(_) => {}
			Err(discord::Error::Closed(code, body)) => {
				println!("Gateway closed on us with code {:?}: {}", code, body);
				break;
			}
			Err(_) => {},
		}
	}
}
