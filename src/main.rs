mod am_forth;

use devzat_rs;
use tokio::try_join;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let instance_host = match std::env::var("PLUGIN_HOST") {
        Ok(host) => host,
        Err(_) => "https://devzat.hackclub.com:5556".to_string(),
    };

    let mut forth_state = am_forth::AMForth::init();

    let auth_token = match std::env::var("PLUGIN_TOKEN") {
        Ok(token) => token,
        Err(_) => panic!("Missing PLUGIN_TOKEN"),
    };

    let login_room = match std::env::var("LOGIN_ROOM") {
        Ok(room) => room,
        Err(_) => "#bots".to_string(),
    };

    let dev_nick = match std::env::var("DEV_NICK") {
        Ok(nick) => nick,
        Err(_) => "Arkaeriit".to_string(),
    };

    let last_action_file_str = match std::env::var("LAST_ACTION_FILE") {
        Ok(nick) => nick,
        Err(_) => "/tmp/devzat_forth".to_string(),
    };
    let last_action_file = &last_action_file_str;

    let client = devzat_rs::Client::new(
        instance_host,
        auth_token,
    ).await?;

    login_notify(&client, &get_bot_name(), "Hi! I just logged in.", &login_room, &dev_nick).await;

    let forth_cmd = client.register_cmd("forth", "Execute some forth code", "<code>", |event| async move {
        update_last_action(last_action_file);
        forth_state.parse_string(&event.args);
        forth_state.parse_string(" ");
        md_spaces(&forth_state.get_output())
    });


   let args: Vec<_> = env::args().collect();
   if args.len() > 1 {
       if args[1] == "help" {
           println!("No help for you. LOL");
            return Ok(());
        } else if args[1] == "login-notify" {
            if args.len() > 2 {
                println!("Good, good");
                if is_last_action_up_to_date(last_action_file) {
                   speak_up(&client, &args[2]).await;
                }
            } else {
               println!("Please, give login notification as arguments.");
            }
            return Ok(());
        }
    }


    let _ = try_join!(forth_cmd);
    Ok(())

}

/// Get the name of the bot.
fn get_bot_name() -> String {
    match std::env::var("BOT_NAME") {
        Ok(name) => name,
        Err(_) => "Forth".to_string(),
    }
}

/// Try to tell a message to the room login_msg_room. then, try to
/// send a message to login_msg_target on #main. If this fails, give up.
async fn login_notify(client: &devzat_rs::Client, name: &str, msg: &str, login_msg_room: &str, login_msg_target: &str) {
    match client.send_message( login_msg_room.to_string(), Some(name.to_string()), msg.to_string(), None).await {
        Ok(()) => {},
        Err(_) => {},
    }
    match client.send_message( "#main".to_string(), Some(name.to_string()), msg.to_string(), Some(login_msg_target.to_string())).await {
        Ok(()) => {},
        Err(_) => {},
    }
}

/// Sends a messgae on #main
async fn speak_up(client: &devzat_rs::Client, msg: &str) {
    let _ = client.send_message("#main".to_string(), Some(get_bot_name()), msg.to_string(), None).await;
}

/// Add two lines in front of new lines to ensure that the markdown parser
/// handles them well.
fn md_spaces(s: &str) -> String {
    s.replace("\n", "  \\n")
}

/// Rewrite the last action file
fn update_last_action(filename: &str) {
    let _who_got_time_to_check_error = fs::remove_file(filename);
    let _who_got_time_to_check_error = fs::OpenOptions::new().create(true).write(true).open(filename);
}

/// Tell if the last action file was updated in the last 40 seconds
fn is_last_action_up_to_date(filename: &str) -> bool {
    match fs::metadata(filename) {
        Ok(meta) => {
            match meta.modified() {
                Ok(time) => {
                    match time.elapsed() {
                        Ok(elapsed) => elapsed.as_secs() < 40,
                        Err(_) => false,
                    }
                },
                Err(_) => false,
            }
        },
        Err(_) => false,
    }
}

