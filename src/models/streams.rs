use db;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct Stream
{
    pub id: i32,
    pub user_id: String,
    pub user_name: String,
    pub game_id: String,
    pub game_name: String,
    pub title: String,
    pub viewers: i32
}

impl Stream
{
    pub fn new(user_id: String, user_name: String, game_id: String, game_name: String, title: String, viewers: i32) -> Stream
    {
        Stream
        {
            id: 0,
            user_id: user_id,
            user_name: user_name,
            game_id: game_id,
            game_name: game_name,
            title: title,
            viewers: viewers
        }
    }

    pub fn from_db(record: &HashMap<String, String>) -> Stream
    {
        Stream
        {
            id: record["id"].parse().unwrap(),
            user_id: record["user_id"].clone(),
            user_name: record["user_name"].clone(),
            game_id: record["game_id"].clone(),
            game_name: record["game_name"].clone(),
            title: record["title"].clone(),
            viewers: record["viewers"].parse().unwrap()
        }
    }

    pub fn from_id(id: i32) -> Result<Stream, Box<::std::error::Error>>
    {
        let records = db::query("SELECT * FROM streams WHERE id=?", &[&id.to_string()])?;
        if let Some(record) = records.first()
        {
            Ok(Stream::from_db(record))
        } else {
            Err(Box::new(Error::new(ErrorKind::NotFound, "Could not find the runner in the database")))
        }
    }

    pub fn from_user_id(user_id: &String) -> Result<Stream, Box<::std::error::Error>>
    {
        let records = db::query("SELECT * FROM streams WHERE user_id=?", &[&user_id.as_str()])?;
        if let Some(record) = records.first()
        {
            Ok(Stream::from_db(record))
        } else {
            Err(Box::new(Error::new(ErrorKind::NotFound, "Could not find the runner in the database")))
        }
    }

    pub fn clear_modified()
    {
        let _ = db::exec("UPDATE streams SET modified=0", &[]);
    }

    pub fn delete_unmodified()
    {
        let _ = db::exec("DELETE FROM streams WHERE modified=0", &[]);
    }

    pub fn save(&mut self)
    {
        if self.id != 0
        {
            let _result = db::exec("UPDATE streams SET user_id=?, user_name=?, game_id=?, game_name=?, title=?, viewers=?, modified=1 WHERE id=?", &[&self.user_id, &self.user_name, &self.game_id, &self.game_name, &self.title, &self.viewers.to_string(), &self.id.to_string()]);
        } else {
            let _result = db::exec("INSERT INTO streams (user_id, user_name, game_id, game_name, title, viewers, modified) VALUES (?, ?, ?, ?, ?, ?, 1)", &[&self.user_id, &self.user_name, &self.game_id, &self.game_name, &self.title, &self.viewers.to_string()]);
            if let Ok(ids) = db::query("SELECT seq AS id FROM sqlite_sequence WHERE name='streams'", &[])
            {
                if let Some(id) = ids.first()
                {
                    self.id = id["id"].parse().unwrap();
                }
            }
        }
    } 
}