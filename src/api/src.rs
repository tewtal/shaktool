use reqwest;
use serde_json;
use std::io::{Error, ErrorKind};
use chrono::{DateTime, Utc};
use models::records;
use models::records::{Category, Region};

#[derive(Deserialize, Serialize, Debug)]
pub struct SrcLeaderboard
{
    pub weblink: String,
    pub game: String,
    pub category: String,
    pub runs: Vec<SrcLeaderboardRuns>,
    pub players: SrcLeaderboardPlayers
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SrcLeaderboardPlayers
{
    pub data: Vec<SrcUser>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SrcLeaderboardRuns
{
    pub place: i32,
    pub run: SrcRun
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SrcRun
{
    pub id: String,
    pub weblink: Option<String>,
    pub category: Option<String>,
    pub videos: Option<SrcVideo>,
    pub comment: Option<String>,
    pub status: SrcStatus,
    pub players: Vec<SrcPlayer>,
    pub date: Option<String>,
    pub times: SrcTimes,
    pub system: Option<SrcSystem>,
    pub splits: Option<SrcSplits>,
    pub values: Option<SrcValue>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SrcValue
{
    #[serde(rename="onv6jzw8")]
    pub low_percent: Option<String>,
    #[serde(rename="kn02d083")]
    pub gt_code: Option<String>,
    #[serde(rename="wl360wwl")]
    pub miniboss: Option<String>,
    #[serde(rename="wle6dpr8")]
    pub region: Option<String>
}

impl SrcRun
{
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SrcSplits
{
    pub rel: String,
    pub uri: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SrcSystem
{
    pub platform: Option<String>,
    pub emulated: bool,
    pub region: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SrcTimes
{
    pub primary: Option<String>,
    pub primary_t: Option<f64>,
    pub realtime: Option<String>,
    pub realtime_t: Option<f64>,
    pub realtime_noloads: Option<String>,
    pub ingame: Option<String>,
    pub ingame_t: Option<f64>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SrcPlayer
{
    pub rel: String,
    pub id: Option<String>,
    pub name: Option<String>,
    pub uri: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SrcUser
{
    pub rel: String,
    pub id: Option<String>,
    pub name: Option<String>,
    pub names: Option<SrcNames>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SrcNames
{
    pub international: String,
    pub japanese: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SrcStatus
{
    pub status: String,
    pub examiner: Option<String>,
    #[serde(rename="verify-date")]
    pub verify_date: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SrcVideo
{
    pub text: Option<String>,
    pub links: Option<Vec<SrcLink>>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SrcLink
{
    pub uri: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SrcCategory
{
    pub id: String,
    pub name: String,
    pub weblink: String,
    #[serde(rename="type")]
    pub category_type: String,
    pub rules: String,
}

pub struct Src
{
    api_key: String
}

impl Src
{
    pub fn new(api_key: String) -> Src
    {
        Src
        {
            api_key: api_key
        }
    }

    pub fn get_run(&self) -> Result<SrcRun, Box<::std::error::Error>>
    {
        let reqclient = reqwest::Client::new();
        let mut response = reqclient.get("https://www.speedrun.com/api/v1/runs/yo4dnx1m").send()?;
        let body = response.text()?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        if let Some(run) = data.pointer("/data")
        {
            let src_run: SrcRun = serde_json::from_value(run.clone()).unwrap();
            Ok(src_run)
        } 
        else
        {
            Err(Box::new(Error::new(ErrorKind::InvalidData, "Could not parse src run data")))
        }
    }

    pub fn get_categories(&self, game_id: String) -> Result<Vec<SrcCategory>, Box<::std::error::Error>>
    {
        let reqclient = reqwest::Client::new();
        let url = format!("https://www.speedrun.com/api/v1/games/{}/categories", game_id);
        let mut response = reqclient.get(url.as_str()).send()?;
        let body = response.text()?;
        let data: serde_json::Value = serde_json::from_str(&body).unwrap();
        
        if let Some(categories) = data.pointer("/data")
        {
            if let Ok(src_categories) = serde_json::from_value::<Vec<SrcCategory>>(categories.clone())
            {
                return Ok(src_categories);
            }
        }
        Err(Box::new(Error::new(ErrorKind::InvalidData, "Could not parse src category data")))
    }

    pub fn get_leaderboard(&self, game_id: String) -> Result<Vec<SrcLeaderboard>, Box<::std::error::Error>>
    {
        let reqclient = reqwest::Client::new();
        let mut leaderboard: Vec<SrcLeaderboard> = Vec::new();
        let categories = self.get_category_urls();

        for c in categories
        {
            let url = format!("https://www.speedrun.com/api/v1/leaderboards/{}/category/{}", &game_id, &c);
            let mut response = reqclient.get(url.as_str()).send()?;
            let body = response.text()?;
            let data: serde_json::Value = serde_json::from_str(&body).unwrap();
    
            if let Some(runs) = data.pointer("/data")
            {
                if let Ok(src_board) = serde_json::from_value::<SrcLeaderboard>(runs.clone())
                {
                    leaderboard.push(src_board);
                }
                else
                {
                    return Err(Box::new(Error::new(ErrorKind::InvalidData, "Could not parse src leaderboard data")));    
                }
            } 
            else 
            {
                return Err(Box::new(Error::new(ErrorKind::InvalidData, "Could not parse src leaderboard data")));
            }
        }

        Ok(leaderboard)
    }

    pub fn get_all_records(&self, game_id: String) -> Result<Vec<records::Record>, Box<::std::error::Error>>
    {
        let leaderboards = self.get_leaderboard(game_id.clone())?;
        let mut records: Vec<records::Record> = Vec::new();
        
        for lb in leaderboards
        {
            for lr in lb.runs
            {
                let run = lr.run;

                /* Check if this record already exists */
                if let Ok(r) = records::Record::from_src_id(&run.id)
                {
                    records.push(r);
                    continue;
                }

                if let Some(player) = run.players.first()
                {
                    if let Some(user) =
                        match player.rel.as_str()
                        {
                            "user" => lb.players.data.iter().find(|&x| x.id.is_some() && x.id.as_ref().unwrap() == player.id.as_ref().unwrap_or(&String::from(""))),
                            _ => lb.players.data.iter().find(|&x| x.name.is_some() && x.name.as_ref().unwrap() == player.name.as_ref().unwrap_or(&String::from("")))
                        }
                    {
                        let runner = match user.rel.as_str()
                        {
                            "user" => match records::Runner::from_src_id(user.id.as_ref().unwrap())
                            {
                                Ok(r) => r,
                                _ => match records::Runner::from_name(&user.names.as_ref().unwrap().international)
                                {
                                    Ok(mut rr) => {                                
                                        rr.src_id = user.id.as_ref().unwrap().clone();
                                        rr.save();
                                        rr
                                    },
                                    _ =>
                                    {
                                        let mut new_runner = records::Runner::new(user.names.as_ref().unwrap().international.clone(), String::from(""), user.id.as_ref().unwrap().clone(), 1);
                                        new_runner.save();
                                        new_runner
                                    }
                                }
                            }
                            _ => 
                            {
                                match records::Runner::from_name(&user.name.as_ref().unwrap())
                                {
                                    Ok(r) => r,
                                    _ =>
                                    {
                                        let mut new_runner = records::Runner::new(user.name.as_ref().unwrap().clone(), String::from(""), String::from(""), 1);
                                        new_runner.save();
                                        new_runner                              
                                    }
                                }
                            }
                        };

                        let record = records::Record
                        {
                            id: 0,
                            dt_id: 0,
                            src_id: run.id,
                            runner_id: runner.id,
                            category: if run.category.is_some() && run.values.is_some() { self.map_category(&run.category.as_ref().unwrap(), &run.values.as_ref().unwrap()) } else { Category::Unknown },
                            region: if run.category.is_some() && run.values.is_some() { self.map_region(&run.category.as_ref().unwrap(), &run.values.as_ref().unwrap()) } else { Region::NTSC },
                            realtime: run.times.realtime_t.unwrap_or(0.0) as i32,
                            gametime: run.times.ingame_t.unwrap_or(0.0) as i32,
                            comment: run.comment.unwrap_or(String::from("")),
                            video: match run.videos
                            {
                                Some(v) => if let Some(links) = v.links { if let Some(video) = links.first() { video.uri.clone().unwrap_or(String::from("")) } else { String::from("") }} else { String::from("") },
                                _ => String::from("")
                            },
                            active: 1            
                        };

                        records.push(record);
                    }
                }
            }
        }

        Ok(records)
    }

    fn map_region(&self, category_id: &String, values: &SrcValue) -> Region
    {
        match self.map_category(category_id, values)
        {
            Category::LowPercentIceBoots => Region::PAL,
            Category::LowPercentSpeedBoots => Region::PAL,
            _ => match values.region.as_ref().unwrap().as_str()
                    {
                        "21gezkxl" => Region::NTSC,
                        "jqzvzo4l" => Region::PAL,
                        _ => Region::NTSC
                    }
        }
    }

    fn get_category_urls(&self) -> Vec<&str>
    {
        let categories = vec![
            "9d8v96lk?var-wle6dpr8=21gezkxl&embed=players",
            "9d8v96lk?var-wle6dpr8=jqzvzo4l&embed=players",
            "xd1mpewd?embed=players",
            "ndx8qmvk?embed=players",
            "w20zowod?embed=players",
            "xd1mplwd?embed=players",
            "n2y1y182?embed=players",
            "w2018jok?embed=players",
            "wdmqjw32?var-kn02d083=81w422oq&embed=players",
            "wdmqjw32?var-kn02d083=5lmo33j1&embed=players",
            "rklgyq8d?var-onv6jzw8=4lx07prl&embed=players",
            "rklgyq8d?var-onv6jzw8=814k0yjl&embed=players",
            "rklgyq8d?var-onv6jzw8=z195omkq&embed=players",
            "rklgyq8d?var-onv6jzw8=p12oydkl&embed=players",
            "rklgyq8d?var-onv6jzw8=xqk9yjy1&embed=players",
            "rklgyq8d?var-onv6jzw8=gq79mjyl&embed=players",
            "7kjrnrx2?var-wl360wwl=21gezknl&embed=players",
            "7kjrnrx2?var-wl360wwl=jqzvzogl&embed=players",
            "7kjrnrx2?var-wl360wwl=klrjo8jq&embed=players",
        ];
        categories
    }

    fn map_category(&self, category_id: &String, values: &SrcValue) -> Category
    {
        match category_id.as_str()
        {
            "9d8v96lk" => Category::AnyPercent,
            "xd1mpewd" => Category::OneHundredPercent,
            "ndx8qmvk" => Category::RBO,
            "w20zowod" => Category::AnyPercentGlitched,
            "xd1mplwd" => Category::MapCompletion,
            "n2y1y182" => Category::CeresEscape,
            "w2018jok" => Category::LowPercentGlitched,
            "wdmqjw32" => 
            {
                match values.gt_code.as_ref().unwrap().as_str()
                {
                    "81w422oq" => Category::GTClassic,
                    "5lmo33j1" => Category::AnyPercentGTCode,
                    _ => Category::Unknown
                }
            },
            "rklgyq8d" => 
            {
                match values.low_percent.as_ref().unwrap().as_str()
                {
                    "4lx07prl" => Category::LowPercentIce,
                    "814k0yjl" => Category::LowPercentSpeed,
                    "z195omkq" => Category::LowPercentIceBooster,
                    "p12oydkl" => Category::LowPercentXIce,
                    "xqk9yjy1" => Category::LowPercentIceBoots,
                    "gq79mjyl" => Category::LowPercentSpeedBoots,
                    _ => Category::Unknown
                }
            },
            "7kjrnrx2" =>
            {
                match values.miniboss.as_ref().unwrap().as_str()
                {
                    "21gezknl" => Category::SporeSpawnRTA,
                    "jqzvzogl" => Category::CrocomireRTA,
                    "klrjo8jq" => Category::BotwoonRTA,
                    _ => Category::Unknown
                }
            }
            _ => Category::Unknown
        }
    }

    pub fn get_json(&self, run: &SrcRun) -> Result<String, Box<::std::error::Error>>
    {
        let json = serde_json::to_string(run)?;
        Ok(json)
    }
}