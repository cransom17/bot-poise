use std::sync::{Mutex, Arc};

use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use lazy_static::lazy_static;
mod player;
use crate::player::Player;
struct Data {player: Mutex<Player>} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// lazy_static! {
//     static ref PLAYER: Mutex<Player> = {
//         let player = Player::new("Michael".to_string(), "knife".to_string());
//         Mutex::new(player.unwrap())
//     };
// }
struct User{
    user: Player,
}
impl User{

pub fn new()-> Self{
 let character = Player::new("K".to_string(), "".to_string()).unwrap();
 Self{
    user: character,
 }
}
pub fn change_name(&mut self)->String{
    self.user.name="Me".to_string();
    return "Me".to_string();
}
}
/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn name(ctx: Context<'_>, #[description = "change your name to:"] new_name:String) -> Result<(), Error> {
    let us: String = {
        let mut all = ctx.data().player.lock().unwrap();
        let mut us: String = all.name.clone();
        us = new_name.clone();
        us.to_string()};
    // let name = &PLAYER.lock().unwrap().name;
    //us.name = new_name;
    
    let response = format!("you are {}", us);
    let new_named = Player::new(new_name, "whatever".to_string());
    ctx.say(response).await?;
    *ctx.data().player =  Mutex::new(new_named.unwrap());
    Ok(())
}

#[poise::command(slash_command)]
async fn equip(ctx: Context<'_>, #[description = "change your equipped weapon to:"] new_weapon:String) -> Result<(), Error> {
   // let mut us = ctx.data().player.;
    // let name = &PLAYER.lock().unwrap().name;

    //TODO: check is IN INVENTORY
   // us.weapon = new_weapon;
    //let response = format!("you have equipped {}", us.weapon);
    let response = "placeholder".to_string();
    ctx.say(response).await?;
    
    Ok(())
}

#[poise::command(slash_command)]
async fn check_hp(ctx: Context<'_>) -> Result<(), Error> {
    let mut us = {
        let all = ctx.data().player.lock().unwrap();
        let us = all.hp;
        us};
    let mut nm = {
        let name_all = ctx.data().player.lock().unwrap();
        let nm = name_all.name.clone();
        nm

    };
    // let name = &PLAYER.lock().unwrap().name;

    //TODO: check is IN INVENTORY
   
    let response = format!("{} you have {} hp", nm, us);
    
    ctx.say(response).await?;
    
    Ok(())
}


#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let mut user = User::new();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(), check_hp(), name(), equip()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let player = Mutex::new(Player::new("Context_player".to_string(), "weapon".to_string()).unwrap());
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {player:player})
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
