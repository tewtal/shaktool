use rusqlite::Connection;
use rusqlite;
use std::collections::HashMap;

pub fn query(q: &str, args: &[&rusqlite::types::ToSql]) -> rusqlite::Result<Vec<HashMap<String, String>>>
{
    let conn = Connection::open("bot.db").unwrap();
    let mut stmt = try!(conn.prepare(q));
    let columns: Vec<String> = stmt.column_names().iter().map(|x| x.to_string()).collect();
    let mut rows = try!(stmt.query(args));

    let mut ret: Vec<HashMap<String, String>> = Vec::new();

    while let Some(result_row) = rows.next()
    {
        let row = try!(result_row);
        let mut r = HashMap::new();
        for i in 0..row.column_count() 
        {
            let column_name = columns.get(i as usize).unwrap().to_string();
            let column_value: String = match column_name.as_str()
            {
                "id" => 
                {
                    let iv: i32 = row.get(i);
                    iv.to_string()
                }
                _ => row.get(i)
            };
            r.insert(column_name, column_value);
        }

        ret.push(r);
    }

    Ok(ret)
}

pub fn exec(q: &str, args: &[&rusqlite::types::ToSql]) -> rusqlite::Result<i32>
{
    let conn = Connection::open("bot.db").unwrap();
    let mut stmt = try!(conn.prepare(q));
    let rows = try!(stmt.execute(args));

    Ok(rows)    
}