use api::deertier;
use api::src;
use std::io::{Error, ErrorKind};
use std::fmt;
use std::collections::HashMap;
use time;
use num;
use db;

// This implements the records as stored internally
#[derive(Copy, Clone, FromPrimitive, PartialEq)]
pub enum Category
{
    AnyPercent,
    AnyPercentGlitched,
    AnyPercentGTCode,
    OneHundredPercent,
    LowPercentIce,
    LowPercentSpeed,
    LowPercentXIce,
    LowPercentSpeedBoots,
    LowPercentIceBoots,
    LowPercentIceBooster,
    LowPercentAllBosses,
    LowPercentGlitched,
    RBO,
    GTClassic,
    CeresEscape,
    MapCompletion,
    SporeSpawnRTA,
    BotwoonRTA,
    CrocomireRTA,
    Unknown
}

impl Category
{
    pub fn from_name(category: &str) -> Category
    {
        match category.to_lowercase().as_str()
        {
            "any%" => Category::AnyPercent,
            "100%" => Category::OneHundredPercent,
            "100% map" => Category::MapCompletion,
            "map completion" => Category::MapCompletion,
            "low% ice" => Category::LowPercentIce,
            "14% ice" => Category::LowPercentIce,
            "low% speed" => Category::LowPercentSpeed,
            "14% speed" => Category::LowPercentSpeed,
            "rbo" => Category::RBO,
            "ceres" => Category::CeresEscape,
            "ssrta" => Category::SporeSpawnRTA,
            "any% glitched" => Category::AnyPercentGlitched,
            "any% gt" => Category::AnyPercentGTCode,
            "any% gt code" => Category::AnyPercentGTCode,
            "gt classic" => Category::GTClassic,
            "low% glitched" => Category::LowPercentGlitched,
            "3%" => Category::LowPercentGlitched,
            "0%" => Category::LowPercentGlitched,
            "low% all bosses" => Category::LowPercentAllBosses,
            "12%" => Category::LowPercentAllBosses,
            "any% pal" => Category::AnyPercent,
            "low% iceboots" => Category::LowPercentIceBoots,
            "14% iceboots" => Category::LowPercentIceBoots,
            "low% speedboots" => Category::LowPercentSpeedBoots,
            "14% speedboots" => Category::LowPercentSpeedBoots,
            "low% icebooster" => Category::LowPercentIceBooster,
            "14% icebooster" => Category::LowPercentIceBooster,
            "low% xice" => Category::LowPercentXIce,
            "low% x-ice" => Category::LowPercentXIce,
            "14% xice" => Category::LowPercentXIce,
            "14% x-ice" => Category::LowPercentXIce,
            "crocomire rta" => Category::CrocomireRTA,
            _ => Category::Unknown       
        }
    }

    pub fn from_dt_category(category: &str) -> Category
    {
        match category
        {
            "AnyPercentRealTime" => Category::AnyPercent,
            "OneHundredPercent" => Category::OneHundredPercent,
            "OneHundredPercentMap" => Category::MapCompletion,
            "LowPercentIce" => Category::LowPercentIce,
            "LowPercentSpeed" => Category::LowPercentSpeed,
            "RBO" => Category::RBO,
            "Ceres" => Category::CeresEscape,
            "SporeSpawnRTA" => Category::SporeSpawnRTA,
            "AnyPercentGlitched" => Category::AnyPercentGlitched,
            "AnyPercentGTCode" => Category::AnyPercentGTCode,
            "GTClassic" => Category::GTClassic,
            "LowPercentGlitched" => Category::LowPercentGlitched,
            "LowPercentAllBosses" => Category::LowPercentAllBosses,
            "PALAnyPercentRealTime" => Category::AnyPercent,
            "PALLowPercentIceboots" => Category::LowPercentIceBoots,
            "PALLowPercentSpeedBoots" => Category::LowPercentSpeedBoots,
            "LowPercentIceBooster" => Category::LowPercentIceBooster,
            "LowPercentXIce" => Category::LowPercentXIce,
            "BotwoonRTA" => Category::BotwoonRTA,
            "CrocomireRTA" => Category::CrocomireRTA,
            _ => Category::Unknown
        }
    }
    
    pub fn to_db(&self) -> String
    {
        let value = *self as i32;
        value.to_string()
    }
}

impl fmt::Display for Category
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self
        {
            Category::AnyPercent => write!(f, "Any%"),
            Category::AnyPercentGlitched => write!(f, "Any% Glitched"),
            Category::AnyPercentGTCode => write!(f, "Any% GT Code"),
            Category::BotwoonRTA => write!(f, "Botwoon RTA"),
            Category::CeresEscape => write!(f, "Ceres Escape"),
            Category::CrocomireRTA => write!(f, "Crocomire RTA"),
            Category::GTClassic => write!(f, "GT Classic"),
            Category::LowPercentAllBosses => write!(f, "Low% Glitched All Bosses"),
            Category::LowPercentGlitched => write!(f, "Low% Glitched"),
            Category::LowPercentIce => write!(f, "14% Ice"),
            Category::LowPercentIceBooster => write!(f, "14% IceBooster"),
            Category::LowPercentIceBoots => write!(f, "14% IceBoots"),
            Category::LowPercentSpeed => write!(f, "14% Speed"),
            Category::LowPercentSpeedBoots => write!(f, "14% SpeedBoots"),
            Category::LowPercentXIce => write!(f, "14% X-Ice"),
            Category::MapCompletion => write!(f, "Map Completion"),
            Category::OneHundredPercent => write!(f, "100%"),
            Category::RBO => write!(f, "RBO"),
            Category::SporeSpawnRTA => write!(f, "Spore Spawn RTA"),
            Category::Unknown => write!(f, "Unknown"),
            _ => write!(f, "Invalid category")
        }
    }
}

#[derive(Debug, Copy, Clone, FromPrimitive, PartialEq)]
pub enum Region
{
    NTSC,
    PAL
}

impl Region
{
    pub fn from_dt_category(category: &str) -> Region
    {
        match category
        {
            "PALAnyPercentRealTime" => Region::PAL,
            "PALLowPercentIceboots" => Region::PAL,
            "PALLowPercentSpeedBoots" => Region::PAL,
            _ => Region::NTSC
        }
    }

    pub fn to_db(&self) -> String
    {
        let value = *self as i32;
        value.to_string()
    }
}

pub struct Runner
{
    pub id: i32,
    pub name: String,
    pub dt_id: String,
    pub src_id: String,
    pub sync: i32
}

impl Runner
{
    pub fn new(name: String, dt_id: String, src_id: String, sync: i32) -> Runner
    {
        Runner
        {
            id: 0,
            name: name,
            dt_id: dt_id,
            src_id: src_id,
            sync: sync
        }
    }

    pub fn from_db(record: &HashMap<String, String>) -> Runner
    {
        Runner
        {
            id: record["id"].parse().unwrap(),
            name: record["name"].clone(),
            dt_id: record["dt_id"].clone(),
            src_id: record["src_id"].clone(),
            sync: record["sync"].parse().unwrap()
        }
    }

    pub fn from_id(id: i32) -> Result<Runner, Box<::std::error::Error>>
    {
        let records = db::query("SELECT * FROM runners WHERE id=?", &[&id.to_string()])?;
        if let Some(record) = records.first()
        {
            Ok(Runner::from_db(record))
        } else {
            Err(Box::new(Error::new(ErrorKind::NotFound, "Could not find the runner in the database")))
        }
    }

    pub fn from_dt_id(dt_id: &String) -> Result<Runner, Box<::std::error::Error>>
    {
        let records = db::query("SELECT * FROM runners WHERE dt_id=?", &[&dt_id.as_str()])?;
        if let Some(record) = records.first()
        {
            Ok(Runner::from_db(record))
        } else {
            Err(Box::new(Error::new(ErrorKind::NotFound, "Could not find the runner in the database")))
        }
    }

    pub fn from_src_id(src_id: &String) -> Result<Runner, Box<::std::error::Error>>
    {
        let records = db::query("SELECT * FROM runners WHERE src_id=?", &[&src_id.as_str()])?;
        if let Some(record) = records.first()
        {
            Ok(Runner::from_db(record))
        } else {
            Err(Box::new(Error::new(ErrorKind::NotFound, "Could not find the runner in the database")))
        }
    }

    pub fn from_name(name: &String) -> Result<Runner, Box<::std::error::Error>>
    {
        let records = db::query("SELECT * FROM runners WHERE name=? COLLATE NOCASE", &[&name.as_str()])?;
        if let Some(record) = records.first()
        {
            Ok(Runner::from_db(record))
        } else {
            Err(Box::new(Error::new(ErrorKind::NotFound, "Could not find the runner in the database")))
        }
    }

    pub fn save(&mut self)
    {
        if self.id != 0
        {
            let result = db::exec("UPDATE runners SET name=?,dt_id=?,src_id=?,sync=? WHERE id=?", &[&self.name, &self.dt_id, &self.src_id, &self.sync.to_string(), &self.id.to_string()]);
        } else {
            let result = db::exec("INSERT INTO runners (name, dt_id, src_id, sync) VALUES (?, ?, ?, ?)", &[&self.name, &self.dt_id, &self.src_id, &self.sync.to_string()]);
            if let Ok(ids) = db::query("SELECT seq AS id FROM sqlite_sequence WHERE name='runners'", &[])
            {
                if let Some(id) = ids.first()
                {
                    self.id = id["id"].parse().unwrap();
                }
            }
        }
    }
}

pub struct Record
{
    pub id: i32,
    pub dt_id: i32,
    pub src_id: String,
    pub runner_id: i32,
    pub category: Category,
    pub region: Region,
    pub realtime: i32,
    pub gametime: i32,
    pub comment: String,
    pub video: String,
    pub active: i32,
}

impl Record
{
    pub fn new(dt_id: i32, src_id: String, runner: &Runner, category: Category, region: Region, realtime: i32, gametime: i32, comment: String, video: String, active: i32) -> Record
    {
        Record
        {
            id: 0,
            dt_id: dt_id,
            src_id: src_id,
            runner_id: runner.id,
            category: category,
            region: region,
            realtime: realtime,
            gametime: gametime,
            comment: comment,
            video: video,
            active: active
        }
    }

    pub fn from_db(record: &HashMap<String, String>) -> Record
    {
        Record
        {
            id: record["id"].parse().unwrap(),
            dt_id: record["dt_id"].parse().unwrap(),
            src_id: record["src_id"].clone(),
            runner_id: record["runner_id"].parse().unwrap(),
            category: num::FromPrimitive::from_i32(record["category"].parse::<i32>().unwrap()).unwrap(),
            region: num::FromPrimitive::from_i32(record["region"].parse::<i32>().unwrap()).unwrap(),
            realtime: record["realtime"].parse().unwrap(),
            gametime: record["gametime"].parse().unwrap(),
            comment: record["comment"].clone(),
            video: record["video"].clone(),
            active: record["active"].parse().unwrap()
        }
    }
    
    pub fn from_dt_id(dt_id: i32) -> Result<Record, Box<::std::error::Error>>
    {
        let records = db::query("SELECT * FROM records WHERE dt_id=?", &[&dt_id.to_string()])?;
        if let Some(record) = records.first()
        {
            Ok(Record::from_db(record))
        } else {
            Err(Box::new(Error::new(ErrorKind::NotFound, "Could not find the record in the database")))
        }
    }

    pub fn from_src_id(src_id: &String) -> Result<Record, Box<::std::error::Error>>
    {
        let records = db::query("SELECT * FROM records WHERE src_id=?", &[&src_id.to_string()])?;
        if let Some(record) = records.first()
        {
            Ok(Record::from_db(record))
        } else {
            Err(Box::new(Error::new(ErrorKind::NotFound, "Could not find the record in the database")))
        }
    }

    pub fn from_deertier_record(record: &deertier::DeerTierRecord) -> Record
    {
        /* Check if this record already exists */
        if let Ok(r) = Record::from_dt_id(record.id)
        {
            return r;
        }

        /* No existing records found, create a new one */        
        let runner = 
            match Runner::from_dt_id(&record.username)
            {
                Ok(r) => r,
                _ => 
                {
                    match Runner::from_name(&record.username)
                    {
                        Ok(mut r) =>
                        {
                            r.dt_id = record.username.clone();
                            r.save();
                            r
                        },
                        _ => 
                        {
                            let mut new_runner = Runner::new(record.username.clone(), record.username.clone(), String::from(""), 1);
                            new_runner.save();
                            new_runner
                        }
                    }
                }
            };

        Record
        {
            id: 0,
            dt_id: record.id,
            src_id: String::from(""),
            runner_id: runner.id,
            category: Category::from_dt_category(record.category.as_ref()),
            region: Region::from_dt_category(record.category.as_ref()),
            realtime: Record::convert_dt_time(&record.real_time),
            gametime: Record::convert_dt_time(&record.game_time) * 60,
            comment: record.comment.as_ref().cloned().unwrap_or(String::from("")),
            video: record.video_url.as_ref().cloned().unwrap_or(String::from("")),
            active: 1        
        }
    }

    pub fn save(&mut self)
    {
        if self.id != 0
        {
            let result = db::exec("UPDATE records SET dt_id=?, src_id=?, runner_id=?, category=?, region=?, realtime=?, gametime=?, comment=?, video=?, active=? WHERE id=?", &[&self.dt_id.to_string(), &self.src_id.to_string(), &self.runner_id.to_string(), &self.category.to_db(), &self.region.to_db(), &self.realtime.to_string(), &self.gametime.to_string(), &self.comment, &self.video, &self.active.to_string(), &self.id.to_string()]);
        } else {

            /* Before inserting, try to find this record in the database */
            let records = db::query("SELECT * FROM records WHERE runner_id=? AND category=? AND region=? AND realtime=?", &[&self.runner_id.to_string(), &self.category.to_db(), &self.region.to_db(), &self.realtime.to_string()]);

            if let Ok(rs) = records
            {
                if let Some(r) = rs.first()
                {
                    let record = Record::from_db(r);
                    self.id = record.id;
                    self.active = record.active;
                    if self.dt_id == 0 { self.dt_id = record.dt_id; };
                    if self.src_id == "" { self.src_id = record.src_id; };
                    let _ = db::exec("UPDATE records SET dt_id=?, src_id=?, runner_id=?, category=?, region=?, realtime=?, gametime=?, comment=?, video=?, active=? WHERE id=?", &[&self.dt_id.to_string(), &self.src_id.to_string(), &self.runner_id.to_string(), &self.category.to_db(), &self.region.to_db(), &self.realtime.to_string(), &self.gametime.to_string(), &record.comment, &record.video, &self.active.to_string(), &self.id.to_string()]);
                    return;
                }
            }             

            /* Try again, matching on video instead of runner */
            if self.video != ""
            {
                let records = db::query("SELECT * FROM records WHERE video=? AND category=? AND region=? AND realtime=?", &[&self.video.to_string(), &self.category.to_db(), &self.region.to_db(), &self.realtime.to_string()]);
                if let Ok(rs) = records
                {
                    if let Some(r) = rs.first()
                    {
                        let record = Record::from_db(r);

                        /* First update the runner with the new missing data */
                        let old_runner = Runner::from_id(self.runner_id).unwrap();
                        let mut new_runner = Runner::from_id(record.runner_id).unwrap();
                        if new_runner.dt_id == "" { new_runner.dt_id = old_runner.dt_id };
                        if new_runner.src_id == "" { new_runner.src_id = old_runner.src_id };
                        new_runner.save();

                        self.id = record.id;
                        self.runner_id = new_runner.id;
                        self.active = record.active;
                        if self.dt_id == 0 { self.dt_id = record.dt_id; };
                        if self.src_id == "" { self.src_id = record.src_id; };
                        let _ = db::exec("UPDATE records SET dt_id=?, src_id=?, runner_id=?, category=?, region=?, realtime=?, gametime=?, comment=?, video=?, active=? WHERE id=?", &[&self.dt_id.to_string(), &self.src_id.to_string(), &self.runner_id.to_string(), &self.category.to_db(), &self.region.to_db(), &self.realtime.to_string(), &self.gametime.to_string(), &record.comment, &record.video, &self.active.to_string(), &self.id.to_string()]);
                        
                        return;
                    }
                }
            } 

            /* Try again, matching on comment instead of runner */
            if self.comment != ""
            {
                let records = db::query("SELECT * FROM records WHERE comment=? AND category=? AND region=? AND realtime=?", &[&self.comment.to_string(), &self.category.to_db(), &self.region.to_db(), &self.realtime.to_string()]);
                if let Ok(rs) = records
                {
                    if let Some(r) = rs.first()
                    {
                        let record = Record::from_db(r);

                        /* First update the runner with the new missing data */
                        let old_runner = Runner::from_id(self.runner_id).unwrap();
                        let mut new_runner = Runner::from_id(record.runner_id).unwrap();
                        if new_runner.dt_id == "" { new_runner.dt_id = old_runner.dt_id };
                        if new_runner.src_id == "" { new_runner.src_id = old_runner.src_id };
                        new_runner.save();

                        self.id = record.id;
                        self.runner_id = new_runner.id;
                        self.active = record.active;
                        if self.dt_id == 0 { self.dt_id = record.dt_id; };
                        if self.src_id == "" { self.src_id = record.src_id; };
                        let _ = db::exec("UPDATE records SET dt_id=?, src_id=?, runner_id=?, category=?, region=?, realtime=?, gametime=?, comment=?, video=?, active=? WHERE id=?", &[&self.dt_id.to_string(), &self.src_id.to_string(), &self.runner_id.to_string(), &self.category.to_db(), &self.region.to_db(), &self.realtime.to_string(), &self.gametime.to_string(), &record.comment, &record.video, &self.active.to_string(), &self.id.to_string()]);
                        
                        return;
                    }
                }
            }           

            /* Is this the best run for this player and category? */
            let records = db::query("SELECT * FROM records WHERE runner_id=? AND category=? AND region=? AND CAST(realtime as INTEGER)<CAST(? as INTEGER)", &[&self.runner_id.to_string(), &self.category.to_db(), &self.region.to_db(), &self.realtime.to_string()]);
            if let Ok(rs) = records
            {
                if let Some(_r) = rs.first()
                {
                    self.active = 0;
                }
            }
            
            if self.active == 1
            {
                let _ = db::exec("UPDATE records SET active=0 WHERE runner_id=? AND category=? AND region=? AND CAST(realtime as INTEGER)>CAST(? as INTEGER)", &[&self.runner_id.to_string(), &self.category.to_db(), &self.region.to_db(), &self.realtime.to_string()]);
            }

            let _ = db::exec("INSERT INTO records (dt_id, src_id, runner_id, category, region, realtime, gametime, comment, video, active, status, sync_status) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, '0', '0')", &[&self.dt_id.to_string(), &self.src_id.to_string(), &self.runner_id.to_string(), &self.category.to_db(), &self.region.to_db(), &self.realtime.to_string(), &self.gametime.to_string(), &self.comment, &self.video, &self.active.to_string()]);
            if let Ok(ids) = db::query("SELECT seq AS id FROM sqlite_sequence WHERE name='records'", &[])
            {
                if let Some(id) = ids.first()
                {
                    self.id = id["id"].parse().unwrap();
                }
            }

        }      
    }

    pub fn get_rank(&self) -> i32
    {
        let mut rank: i32 = 999;
        if let Ok(records) = db::query("SELECT COUNT(*) as id FROM records WHERE category=? AND region=? AND active=1 AND realtime!=0 AND CAST(realtime AS integer)<=CAST(? AS integer) ORDER BY CAST(realtime AS integer) ASC", &[&self.category.to_db(), &self.region.to_db(), &self.realtime])
        {
            if let Some(record) = records.first()
            {
                rank = record["id"].parse().unwrap();
            }
        }
        rank
    }

    pub fn get_top(category: Category) -> Result<Vec<Record>, Box<::std::error::Error>>
    {
        let mut top: Vec<Record> = Vec::new();
        let records = db::query("SELECT * FROM records WHERE category=? AND active=1 AND realtime!=0 ORDER BY CAST(realtime AS integer) ASC LIMIT 10", &[&category.to_db()])?;
        for record in records
        {
            let r = Record::from_db(&record);
            top.push(r);
        }
        Ok(top)
    }

    pub fn get_records(runner_id: i32) -> Result<Vec<Record>, Box<::std::error::Error>>
    {
        let mut top: Vec<Record> = Vec::new();
        let records = db::query("SELECT * FROM records WHERE runner_id=? AND active=1 AND realtime!=0 ORDER BY CAST(category AS integer) ASC", &[&runner_id.to_string()])?;
        for record in records
        {
            let r = Record::from_db(&record);
            top.push(r);
        }
        Ok(top)      
    }

    pub fn get_pb(runner_id: i32, category: Category) -> Result<Record, Box<::std::error::Error>>
    {
        let record = db::query("SELECT * FROM records WHERE runner_id=? AND category=? AND active=1", &[&runner_id.to_string(), &category.to_db()])?;
        if let Some(rec) = record.first()
        {
            Ok(Record::from_db(&rec))
        } else {
            Err(Box::new(Error::new(ErrorKind::NotFound, "Could not find the record in the database")))
        }
    }

    pub fn get_wr(category: Category) -> Result<Record, Box<::std::error::Error>>
    {
        let record = db::query("SELECT * FROM records WHERE category=? AND active=1 AND realtime!=0 ORDER BY CAST(realtime AS integer) ASC LIMIT 1", &[&category.to_db()])?;
        if let Some(rec) = record.first()
        {
            Ok(Record::from_db(&rec))
        } else {
            Err(Box::new(Error::new(ErrorKind::NotFound, "Could not find the record in the database")))
        }
    }

    pub fn realtime_str(&self) -> String
    {
        Record::from_seconds(self.realtime as i64)
    }

    pub fn gametime_str(&self) -> String
    {
        Record::from_seconds(self.gametime as i64)
    }

    fn from_seconds(seconds: i64) -> String
    {
        if seconds >= 3600
        {
            return time::strftime("%k:%M:%S", &time::at_utc(time::Timespec::new(seconds, 0))).unwrap_or(String::new()).trim().to_string();    
        }
        else 
        {
            return time::strftime("%M:%S", &time::at_utc(time::Timespec::new(seconds, 0))).unwrap_or(String::new());
        }
    }

    fn convert_dt_time(dt_time: &Option<String>) -> i32
    {
        if let Some(timestr) = dt_time.as_ref()
        {
            let times: Vec<i32> = timestr.split(":").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            if times.len() == 2
            {
                return times[1] + (times[0] * 60);
            } else {
                return times[2] + (times[1] * 60) + (times[0] * 3600);
            }        
        }
        0
    }
}