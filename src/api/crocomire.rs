use reqwest;
use serde_json;
use std::io::{Error, ErrorKind};

#[derive(Deserialize, Debug)]
pub struct Strategy
{
    pub area_name: String,
    pub category_name: String,
    pub created_on: String,
    pub description: String,
    pub difficulty: i32,
    pub game_name: String,
    pub id: i32,
    pub link: String,
    pub name: String,
    pub room_name: String,
    pub user_name: String
}

impl Strategy
{
    pub fn find(strat: &String) -> Result<Vec<Strategy>, Box<::std::error::Error>>
    {
        let reqclient = reqwest::Client::new();
        let mut response = reqclient.get(format!("https://crocomi.re/api/strats/{}", strat).as_str()).send()?;

        let body = response.text()?;
        let data: serde_json::Value = serde_json::from_str(&body).unwrap();
        
        if let Some(strats) = data.pointer("/strats")
        {
            if let Ok(strat_list) = serde_json::from_value::<Vec<Strategy>>(strats.clone())
            {
                return Ok(strat_list);
            }
        }

        Err(Box::new(Error::new(ErrorKind::InvalidData, "Could not parse crocomire data")))        
    }
}