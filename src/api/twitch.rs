use reqwest;
use serde_json;
use std::io::{Error, ErrorKind};

header! { (ClientId, "Client-ID") => [String] }
header! { (Authorization, "Authorization") => [String] }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitchStream
{
    pub community_ids: Vec<String>,
    pub game_id: String,
    pub id: String,
    pub language: String,
    pub started_at: String,
    pub thumbnail_url: String,
    pub title: String,
    #[serde(rename="type")]
    pub stream_type: String,
    pub user_id: String,
    pub viewer_count: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitchUser
{
    pub id: String,
    pub login: String,
    pub display_name: String,
    #[serde(rename="type")]
    pub user_type: String,
    pub broadcaster_type: String,
    pub description: String,
    pub profile_image_url: String,
    pub offline_image_url: String,
    pub view_count: i32,
    pub email: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct TwitchAuth
{
    access_token: String,
    refresh_token: String,
    expires_in: i32,
    scope: Vec<String>    
}

pub struct Twitch
{
    client_id: String,
    client_secret: String,
    client_token: String
}

impl Twitch
{
    pub fn new(client_id: &String, client_secret: &String, client_token: &String) -> Option<Twitch>
    {
        let auth = 
            match client_token.as_str()
            {
                "" => Twitch::get_token(client_id, client_secret),
                _ => Twitch::refresh_token(client_id, client_secret, client_token)
            };

        if let Ok(a) = auth
        {
            Some(Twitch
            {
                client_secret: client_secret.clone(),
                client_id: client_id.clone(),
                client_token: a.access_token
            })
        } else 
        {
            None
        }

        /*
        Some(Twitch
        {
            client_secret: client_secret,
            client_id: client_id,
            client_token: client_token
        })
        */
    }
    
    fn get_token(client_id: &String, client_secret: &String) -> Result<TwitchAuth, reqwest::Error>
    {
        let reqclient = reqwest::Client::new();
        let url = format!("https://api.twitch.tv/kraken/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials&scope=", client_id, client_secret);
        let mut response = reqclient.post(url.as_str()).send()?;
        let body = response.text()?;

        let auth: TwitchAuth = serde_json::from_str(&body).unwrap();
        Ok(auth)
    }

    fn refresh_token(client_id: &String, client_secret: &String, client_token: &String) -> Result<TwitchAuth, reqwest::Error>
    {
        let reqclient = reqwest::Client::new();
        let url = format!("https://api.twitch.tv/kraken/oauth2/token?grant_type=refresh_token&refresh_token={}&client_id={}&client_secret={}", client_token, client_id, client_secret);
        let mut response = reqclient.post(url.as_str()).send()?;
        let body = response.text()?;

        let auth: TwitchAuth = serde_json::from_str(&body).unwrap();
        Ok(auth)
    }

    pub fn check_auth(&self) -> bool
    {
        let reqclient = reqwest::Client::new();
        match reqclient.get(format!("https://api.twitch.tv/helix/").as_str())
            .header(Authorization(format!("Bearer {}", self.client_token.clone())))
            .send()
            {
                Ok(_r) => true,
                _ => 
                {
                    if let Ok(_refresh) = Twitch::refresh_token(&self.client_id, &self.client_secret, &self.client_token)
                    {
                        true
                    }
                    else
                    {
                        false
                    }
                }
            }
    }

    pub fn get_streams(&self, game_id: i32) -> Result<Vec<TwitchStream>, Box<::std::error::Error>>
    {
        let reqclient = reqwest::Client::new();
        let mut response = reqclient.get(format!("https://api.twitch.tv/helix/streams?game_id={}", game_id).as_str())
            .header(Authorization(format!("Bearer {}", self.client_token.clone())))
            .send()?;

        let body = response.text()?;
        let data: serde_json::Value = serde_json::from_str(&body).unwrap();
        
        if let Some(streams) = data.pointer("/data")
        {
            if let Ok(twitch_streams) = serde_json::from_value::<Vec<TwitchStream>>(streams.clone())
            {
                return Ok(twitch_streams);
            }
        }
        Err(Box::new(Error::new(ErrorKind::InvalidData, "Could not parse twitch stream data")))
    }

    pub fn get_user(&self, user_id: &String) -> Result<TwitchUser, Box<::std::error::Error>>
    {
        let reqclient = reqwest::Client::new();
        let mut response = reqclient.get(format!("https://api.twitch.tv/helix/users?id={}", user_id).as_str())
            .header(Authorization(format!("Bearer {}", self.client_token.clone())))
            .send()?;

        let body = response.text()?;
        let data: serde_json::Value = serde_json::from_str(&body).unwrap();
        
        if let Some(streams) = data.pointer("/data")
        {
            if let Ok(twitch_user) = serde_json::from_value::<Vec<TwitchUser>>(streams.clone())
            {
                if let Some(user) = twitch_user.first()
                {
                    return Ok(user.clone());
                }
            }
        }

        Err(Box::new(Error::new(ErrorKind::InvalidData, "Could not parse twitch user data")))     
    }

    pub fn get_user_by_name(&self, user_name: &String) -> Result<TwitchUser, Box<::std::error::Error>>
    {
        let reqclient = reqwest::Client::new();
        let mut response = reqclient.get(format!("https://api.twitch.tv/helix/users?login={}", user_name).as_str())
            .header(Authorization(format!("Bearer {}", self.client_token.clone())))
            .send()?;

        let body = response.text()?;
        let data: serde_json::Value = serde_json::from_str(&body).unwrap();
        
        if let Some(streams) = data.pointer("/data")
        {
            if let Ok(twitch_user) = serde_json::from_value::<Vec<TwitchUser>>(streams.clone())
            {
                if let Some(user) = twitch_user.first()
                {
                    return Ok(user.clone());
                }
            }
        }

        Err(Box::new(Error::new(ErrorKind::InvalidData, "Could not parse twitch user data")))     
    }
}

