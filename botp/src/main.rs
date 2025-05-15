

use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;


struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// lazy_static! {
//     static ref PLAYER: Mutex<Player> = {
//         let player = Player::new("Michael".to_string(), "knife".to_string());
//         Mutex::new(player.unwrap())
//     };
// }




#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>,) -> Result<(), Error> {
    ctx.say(format!("Welcome to Morat County. You are a great rookie monster hunter seeking fame and glory. Type /play to start playing.")).await?;

    Ok(())
}
#[poise::command(slash_command)]
async fn play(ctx: Context<'_>, #[description = "What hunter comes here:"] name:String, #[description = "Choose your weapon: a sword, a hammer, or a horse"] weapon: String,) -> Result<(), Error> {
   let current_name = name;

   let response = match weapon.as_str() {
    "sword" =>  format!("Good choice {}! You are a noble knight. As you admire your new sword, but you notice a dark shadow glint in the reflection of the blade. Type /ghost to continue your story.", &current_name),
    "hammer" => format!("Very practical, {}. Your father must have been a peasant. The narrator appears, you already hate him. Type /kill to continue your story.", &current_name),
    "horse" => format! ("The horse is not as effective as you thought {}, you lose control of your horse and run into a river, under a nearby bridge is a goblin. Type /goblin to continue your story.", &current_name),
    _ => format! ("{}, You ask the king for a weapon he does not have, and he declares you a traitor. You ask for forgiveness. Type /play again to try again.", &current_name),
   };
    ctx.say(response).await?;

    Ok(())
}
#[poise::command(slash_command)]
async fn ghost(ctx: Context<'_>,) -> Result<(), Error> {
    ctx.say(format!("You turn around, inspecting the area around your tent for any specters. A tall, vengeful spirit appears from behind a nearby tree. Type /fight to challenge the ghost and /run to flee.")).await?;

    Ok(())
}
#[poise::command(slash_command)]
async fn run(ctx: Context<'_>,) -> Result<(), Error> {
    ctx.say(format!("The ghost catches up to you fairly easily and takes you to the underworld. To try again, type /play again.")).await?;

    Ok(())
}
#[poise::command(slash_command)]
async fn fight(ctx: Context<'_>, #[description = "Choose a snack before your fight: Monster Energy(tm), Umami Bowl, Sour Patch Kids"] weapon: String) -> Result<(), Error> {
   let response = match weapon.as_str() {
    "monster energy" =>  format!("You quickly chug the can of Monster. You feel a burst of energy and a bit of heartburn- nonetheless you scare off the ghost with ease. Type /sleep to get some rest. "),
    "umami bowl" => format!("You go for the most nutricious option, but it takes longer for you to eat. The ghost strategizes while you eat and goes for a sneak attack. Type /defense to take a defensive approach or /offense to take an offensive approach. "),
    "sour patch kids" => format! ("You eat an entire bag of sour patch kids within 30 seconds. You feel sick so you seek shelter in your cabin. The ghost corners you. Type /play to try again."),
    _ => format! ("You fend for your life on an empty stomach, but you're running low on energy. The ghost notices your sloppy attacks and goes for a strong, sweeping attack. You are knocked to your feet. Type /play to try again."),
   };
    ctx.say(response).await?;


    Ok(())  
}

#[poise::command(slash_command)]
async fn kill(ctx: Context<'_>,) -> Result<(), Error> {
    ctx.say(format!("The narrator laughs at your attempts to take them down. Without a narrator, there is no story. The narrator speaks you out of existance. To try again, type /play.")).await?;

    Ok(())
}
#[poise::command(slash_command)]
async fn goblin(ctx: Context<'_>,) -> Result<(), Error> {
    ctx.say(format!("The goblin admires your horse, Spirit and asks to trade three rocks for your horse. Type /accept to accept the offer. Type /decline to deny the offer.")).await?;

    Ok(())
}
#[poise::command(slash_command)]
async fn accept(ctx: Context<'_>,) -> Result<(), Error> {
    ctx.say(format!("The goblin takes your horse and rides into the sunset. Your left stranded with three rocks. Type /play to try again.")).await?;

    Ok(())
}
#[poise::command(slash_command)]
async fn decline(ctx: Context<'_>,) -> Result<(), Error> {
    ctx.say(format!("The goblin wants to take your horse by force. Type /flee to ride away on your horse or /challenge to fight the goblin ")).await?;

    Ok(())
}
#[poise::command(slash_command)]
async fn flee(ctx: Context<'_>,) -> Result<(), Error> {
    ctx.say(format!("You never got better at controlling your horse Spirit. Spirit runs into a nearby forest and you are totally lost. Type /play to try again.")).await?;

    Ok(())
}
#[poise::command(slash_command)]
async fn sleep(ctx: Context<'_>,) -> Result<(), Error> {
    ctx.say(format!("The caffeine ends up working against you. You don't fall asleep until 4 AM. Type /sleep_in to sleep in or /hunt to go monster hunting. ")).await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn defense(ctx: Context<'_>,) -> Result<(), Error> {
    ctx.say(format!("You strategically take a defensive stance. The ghost is caught off guard by your composure. You defeat him with ease. Type /peace to live peacefully.")).await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn offense(ctx: Context<'_>,) -> Result<(), Error> {
    ctx.say(format!("You take an offensive stance. Little did you know, you were playing right into the ghost's hands. He dodges and goes for his signature attack. You are left incapacitated. Type /play to try again. ")).await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn peace(ctx: Context<'_>,) -> Result<(), Error> {
    ctx.say(format!("After your close encounter with the ghost. You decide to give up monster hunting and start your new career as a farmer. Type /play to play again.")).await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn hunt(ctx: Context<'_>,) -> Result<(), Error> {
    ctx.say(format!("Your wake up and find that your horse returns to you! You and Spirit ride off in search of monsters. You encounter a goblin. Type /goblin to continue.")).await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn sleep_in(ctx: Context<'_>,) -> Result<(), Error> {
    ctx.say(format!("Vengeful monsters are at your door step. You are totally overwhelmed and give up. Type /play to try again.")).await?;

    Ok(())
}

//offense
#[poise::command(slash_command)]
async fn challenge(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!("Spirit decides to stay out of this one. You and the goblin end up chucking rocks at each other for 20 minutes. You're both winded and decide to call it truce. He's actually a nice guy and you adopt his lifestyle. Type /new_life to choose your new occupation")).await?;
    Ok(())
}

//offense
#[poise::command(slash_command)]
async fn new_life(ctx: Context<'_>,#[description = "Choose your occupation: "] job: String,) -> Result<(), Error> {
    ctx.say(format!("The goblin takes you to the village and you acclimate yourself to their lifestyle and live happily as a {}", job)).await?;
    Ok(())
}
#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(), play(),ghost(), run(), fight(), kill(), goblin(), accept(), decline(), flee(), sleep(), offense(), defense(), peace(), challenge(), hunt(), sleep_in(), new_life()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data{})
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
