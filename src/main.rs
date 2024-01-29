use serenity::{
    all::ReactionType, async_trait, model::{channel::Message, gateway::Ready}, prelude::*
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Check if the message starts with "Fiona"
        if msg.content.starts_with("Fiona") {
            let command_parts: Vec<&str> = msg.content.split_whitespace().collect();

            // Display help message if only "Fiona" is mentioned
            if command_parts.len() == 1 {
                let help_message = "\
                    Hi, I'm Fiona, I live in your walls! Here are some things I can do:\n\
                    - `!poll <question>? <option1> <option2> ...` : Create a poll.\n\
                    - `!remind quiet hours` : Send a reminder about quiet hours to the house.\n\
                    - `!remind event <event_name> <date> <time>` : Send a reminder for an event.\n\
                    - `!spooky : Get spooky in here!`
                    \n\
                    If you need more help, just type `Fiona`.";
                
                if let Err(why) = msg.channel_id.say(&ctx.http, help_message).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            // Process the poll command
            else if command_parts.len() > 1 && command_parts[1] == "!poll" {
                // Combine the parts back into a string, then split by quotes to extract arguments
                let args: Vec<&str> = command_parts[2..].join(" ").split('"')
                    .filter(|x| !x.trim().is_empty())
                    .collect::<Vec<&str>>();

                if args.len() >= 2 {
                    // First argument is the question, the rest are options
                    let question = args[0];
                    let options = &args[1..];

                    // Create and send the poll message
                    let mut poll_message = format!("**Poll:** {}\n", question);
                    for (i, option) in options.iter().enumerate() {
                        poll_message.push_str(&format!("**Option {}: {}**\n", i + 1, option));
                    }

                    let poll_msg = match msg.channel_id.say(&ctx.http, &poll_message).await {
                        Ok(msg) => msg,
                        Err(why) => {
                            println!("Error sending message: {:?}", why);
                            return;
                        }
                    };

                    // React with numbers for each option
                    for i in 1..=options.len() {
                        let emoji = match i {
                            1 => "1️⃣", 
                            2 => "2️⃣",
                            3 => "3️⃣",
                            4 => "4️⃣",
                            5 => "5️⃣",
                            6 => "6️⃣",
                            7 => "7️⃣",
                            8 => "8️⃣",
                            9 => "9️⃣",
                            _ => break,
                        };

                        if let Err(why) = poll_msg.react(&ctx.http, ReactionType::Unicode(emoji.to_string())).await {
                            println!("Error sending reaction: {:?}", why);
                        }
                    }
                } else {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Usage: Fiona !poll \"Question?\" \"Option 1\" \"Option 2\" ...").await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
            // If an unknown command is given after "Fiona", you can also provide feedback
            else {
                if let Err(why) = msg.channel_id.say(&ctx.http, "I didn't recognize that command. Type `Fiona` for help.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
        else if msg.content.starts_with("CatBoyAbe") {
            let command_parts: Vec<&str> = msg.content.split_whitespace().collect();

            if command_parts.len() == 1 {
                let help_message = "\
                    Hi, I'm CatBoyAbe, I live in your walls! Here are some things I can do:\n\
                    - `!liar` : Reminds people to tell the truth!.\n\
                    \n\
                    If you need more help, just type `Fiona`.";
                
                if let Err(why) = msg.channel_id.say(&ctx.http, help_message).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


#[tokio::main]
async fn main() {
    let token: String = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client: Client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}