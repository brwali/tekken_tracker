use rusqlite::{Connection, Result, params};
use std::env;

#[derive(Clone)]
pub struct User {
    id: String,
    name: String,
    playtime: f32,
    hours_owed: f32,
    steam_id: String,
    monthly_hours: f32,
    bet_hours_available: f32,
}

#[derive(Clone)]
pub struct Time {
    id: i32,
    month: u32,
    week: i32,
    year: i32,
}

impl Time {
    pub fn get_month(&self) -> u32 {
        self.month
    }
    pub fn get_week(&self) -> i32 {
        self.week
    }
    pub fn get_year(&self) -> i32 {
        self.year
    }
    pub fn set_month(&mut self, new_month: u32) {
        self.month = new_month;
    }
    pub fn set_week(&mut self, new_week: i32) {
        self.week = new_week;
    }
    pub fn set_year(&mut self, new_year: i32) {
        self.year = new_year;
    }
}

impl User {
    pub fn new(
        id: String,
        name: String,
        playtime: f32,
        hours_owed: f32,
        steam_id: String,
        monthly_hours: f32,
        bet_hours_available: f32,
    ) -> Self {
        User {
            id,
            name,
            playtime,
            hours_owed,
            steam_id,
            monthly_hours,
            bet_hours_available,
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_steamid(&self) -> &str {
        &self.steam_id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_hours_owed(&self) -> f32 {
        self.hours_owed
    }
    pub fn get_monthly_hours(&self) -> f32 {
        self.monthly_hours
    }
    pub fn get_playtime(&self) -> f32 {
        self.playtime
    }
    pub fn set_hours_owed(&mut self, new_val: f32) {
        self.hours_owed = new_val;
    }
    pub fn set_monthly_hours(&mut self, new_val: f32) {
        self.monthly_hours = new_val;
    }
    pub fn set_bet_hours_available(&mut self, new_val: f32) {
        self.bet_hours_available = new_val;
    }
    pub fn set_playtime(&mut self, new_val: f32) {
        self.playtime = new_val;
    }
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("data.db")?;
    {
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='users'")?;
        let table_exists = stmt.exists([])?;
        if !table_exists {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS users (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    playtime FLOAT NOT NULL,
                    hours_owed FLOAT NOT NULL,
                    steam_id TEXT NOT NULL,
                    monthly_hours FLOAT NOT NULL,
                    bet_hours_available FLOAT NOT NULL
                )",
                [],
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (&format!("{}", env::var("JACKSON_ID").unwrap()), "Jackson", 8.33, 20, &format!("{}", env::var("JACKSON_STEAM_ID").unwrap()), 0.0, 0.0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (&format!("{}", env::var("MASON_ID").unwrap()), "Mason", 14.55, 95, &format!("{}", env::var("MASON_STEAM_ID").unwrap()), 0.0, 0.0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (&format!("{}", env::var("JON_ID").unwrap()), "Jonathan", 16.48, 150, &format!("{}", env::var("JON_STEAM_ID").unwrap()), 0.0, 0.0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (&format!("{}", env::var("LOGAN_ID").unwrap()), "Logan", 35.05, 115, &format!("{}", env::var("LOGAN_STEAM_ID").unwrap()), 0.0, 0.0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (&format!("{}", env::var("BRANDON_ID").unwrap()), "Brandon", 66.1, 50, &format!("{}", env::var("BRANDON_STEAM_ID").unwrap()), 0.0, 0.0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (&format!("{}", env::var("WYATT_ID").unwrap()), "Wyatt", 17.1, 15, &format!("{}", env::var("WYATT_STEAM_ID").unwrap()), 0.0, 0.0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (&format!("{}", env::var("BRYAN_ID").unwrap()), "Bryan", 2, 2, &format!("{}", env::var("BRYAN_STEAM_ID").unwrap()), 0.0, 0.0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (&format!("{}", env::var("KWANGWON_ID").unwrap()), "Kwangwon", 2, 2, &format!("{}", env::var("KWANGWON_STEAM_ID").unwrap()), 0.0, 0.0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (&format!("{}", env::var("KRIS_ID").unwrap()), "Kris", 2, 2, &format!("{}", env::var("KRIS_STEAM_ID").unwrap()), 0.0, 0.0),
            )?;
        }
        // Every time the db get intitalized it means that we are updating the bot
        // the update may not happen in a single day so its better to be able to control
        // the day counter whenever we choose to launch the bot
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='time'")?;
        let table_exists = stmt.exists([])?;
        if table_exists {
            conn.execute(
                "DROP TABLE time",
                [],
            )?;
        }
        conn.execute(
                "CREATE TABLE IF NOT EXISTS time (
                    id INTEGER PRIMARY KEY,
                    month INTEGER NOT NULL,
                    week INTEGER NOT NULL,
                    year INTEGER NOT NULL
                )",
                [],
            )?;
            // Make sure to check that this is right before deployment lol
            conn.execute(
                "INSERT INTO time (month, week, year) VALUES (?1, ?2, ?3)",
                (12, 1, 2025),
            )?;
    }
    Ok(conn)
}

pub fn get_users(conn: &Connection) -> Result<Vec<User>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM users")?;
    let user_collection = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            playtime: row.get(2)?,
            hours_owed: row.get(3)?,
            steam_id: row.get(4)?,
            monthly_hours: row.get(5)?,
            bet_hours_available: row.get(6)?,
        })
    })?;
    let users: Result<Vec<User>> = user_collection.collect();
    users
}

pub fn update_user(conn: &Connection, user: User) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE users SET playtime = ?, hours_owed = ?, monthly_hours = ?, bet_hours_available = ? WHERE id = ?",
        params![user.playtime, user.hours_owed, user.monthly_hours, user.bet_hours_available, user.id],
    )?;
    Ok(())
}

pub fn bet_result(conn: &Connection, amount: f32, id: &str) -> rusqlite::Result<()> {
    let query = "SELECT * FROM users WHERE id = ? ";
    let mut statement = conn.prepare(query)?;
    let user_collection = statement.query_map([id], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            playtime: row.get(2)?,
            hours_owed: row.get(3)?,
            steam_id: row.get(4)?,
            monthly_hours: row.get(5)?,
            bet_hours_available: row.get(6)?,
        })
    })?;
    let mut bet_total = amount;
    for user in user_collection {
        bet_total += user.unwrap().get_hours_owed();
    }
    conn.execute(
        "UPDATE users SET hours_owed = ? WHERE id = ?",
        params![bet_total, id],
    )?;
    Ok(())
}

pub fn get_time(conn: &Connection) -> Result<Vec<Time>> {
    let mut stmt = conn.prepare("SELECT * FROM time")?;
    let time_collection = stmt.query_map([], |row| {
        Ok(Time {
            id: row.get(0)?,
            month: row.get(1)?,
            week: row.get(2)?,
            year: row.get(3)?,
        })
    })?;
    let time_wizard: Result<Vec<Time>> = time_collection.collect();
    time_wizard
}

pub fn get_user(conn: &Connection, id: &str) -> Result<Option<User>> {
    let mut stmt = conn.prepare("SELECT * FROM users WHERE id = ?1")?;
    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        let user = User {
            id: row.get(0)?,
            name: row.get(1)?,
            playtime: row.get(2)?,
            hours_owed: row.get(3)?,
            steam_id: row.get(4)?,
            monthly_hours: row.get(5)?,
            bet_hours_available: row.get(6)?,
        };
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

pub fn update_time(conn: &Connection, time: Time) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE time SET month = ?, week = ?, year = ? WHERE id = ?",
        params![time.month, time.week, time.year, time.id],
    )?;
    Ok(())
}

pub fn add_user(conn: &Connection, new_user: User) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (new_user.get_id(), new_user.get_name(), new_user.get_playtime(), new_user.get_hours_owed(), new_user.get_hours_owed(), 0.0, 10.0),
    )?;
    Ok(())
}

pub fn update_hours_owed(conn: &Connection, id: &str, hours: f32) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE users SET hours_owed = ? WHERE id = ?",
        params![hours, id],
    )?;
    Ok(())
}
