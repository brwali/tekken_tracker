use rusqlite::{Connection, Result, params};
use std::env;

#[derive(Clone, Debug)]
pub struct User {
    id: String,
    name: String,
    playtime: f32,
    hours_owed: f32,
    steam_id: String,
    monthly_hours: f32,
    weekly_hours: f32,
    bet_hours_available: f32,
    polaris_id: String,
    played_yesterday: i32,
}

#[derive(Clone)]
pub struct Time {
    id: i32,
    month: u32,
    week: i32,
    year: i32,
    zero_day_streak: u32,
}

impl Time {
    pub fn new() -> Self {
        Time {
            id: 0,
            month: 0,
            week: 0,
            year: 0,
            zero_day_streak: 0,
        }
    }
    pub fn get_month(&self) -> u32 {
        self.month
    }
    pub fn get_week(&self) -> i32 {
        self.week
    }
    pub fn get_year(&self) -> i32 {
        self.year
    }
    pub fn get_zero_day_streak(&self) -> u32 {
        self.zero_day_streak
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
    pub fn set_zero_day_streak(&mut self, zero_day_streak: u32) {
        self.zero_day_streak = zero_day_streak;
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
        weekly_hours: f32,
        bet_hours_available: f32,
        polaris_id: String,
        played_yesterday: i32,
    ) -> Self {
        User {
            id,
            name,
            playtime,
            hours_owed,
            steam_id,
            monthly_hours,
            weekly_hours,
            bet_hours_available,
            polaris_id,
            played_yesterday,
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
    pub fn get_polar_id(&self) -> &str {
        &self.polaris_id
    }
    pub fn get_played_yesterday(&self) -> i32 {
        self.played_yesterday
    }
    pub fn get_hours_owed(&self) -> f32 {
        self.hours_owed
    }
    pub fn get_monthly_hours(&self) -> f32 {
        self.monthly_hours
    }
    pub fn get_weekly_hours(&self) -> f32 {
        self.weekly_hours
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
    pub fn set_weekly_hours(&mut self, new_val: f32) {
        self.weekly_hours = new_val;
    }
    pub fn set_bet_hours_available(&mut self, new_val: f32) {
        self.bet_hours_available = new_val;
    }
    pub fn set_playtime(&mut self, new_val: f32) {
        self.playtime = new_val;
    }
    pub fn set_played_yesterday(&mut self, new_val: i32) {
        self.played_yesterday = new_val;
    }
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("data.db")?;
    {
        let mut stmt =
            conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='users'")?;
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
                    bet_hours_available FLOAT NOT NULL,
                    polaris_id TEXT NOT NULL
                )",
                [],
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available, polaris_id, played_yesterday) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                (&format!("{}", env::var("JACKSON_ID").unwrap()), "Jackson", 8.33, 20, &format!("{}", env::var("JACKSON_STEAM_ID").unwrap()), 0.0, 0.0, &format!("{}", env::var("JACKSON_POL_ID").unwrap()), 0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available, polaris_id, played_yesterday) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                (&format!("{}", env::var("MASON_ID").unwrap()), "Mason", 14.55, 95, &format!("{}", env::var("MASON_STEAM_ID").unwrap()), 0.0, 0.0, &format!("{}", env::var("MASON_POL_ID").unwrap()), 0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available, polaris_id, played_yesterday) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                (&format!("{}", env::var("JON_ID").unwrap()), "Jonathan", 16.48, 150, &format!("{}", env::var("JON_STEAM_ID").unwrap()), 0.0, 0.0, &format!("{}", env::var("JON_POL_ID").unwrap()), 0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available, polaris_id, played_yesterday) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                (&format!("{}", env::var("LOGAN_ID").unwrap()), "Logan", 35.05, 115, &format!("{}", env::var("LOGAN_STEAM_ID").unwrap()), 0.0, 0.0, &format!("{}", env::var("LOGAN_POL_ID").unwrap()), 0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available, polaris_id, played_yesterday) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                (&format!("{}", env::var("BRANDON_ID").unwrap()), "Brandon", 66.1, 50, &format!("{}", env::var("BRANDON_STEAM_ID").unwrap()), 0.0, 0.0, "", 0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available, polaris_id, played_yesterday) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                (&format!("{}", env::var("WYATT_ID").unwrap()), "Wyatt", 17.1, 15, &format!("{}", env::var("WYATT_STEAM_ID").unwrap()), 0.0, 0.0, "", 0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available, polaris_id, played_yesterday) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                (&format!("{}", env::var("BRYAN_ID").unwrap()), "Bryan", 2, 2, &format!("{}", env::var("BRYAN_STEAM_ID").unwrap()), 0.0, 0.0, "", 0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available, polaris_id, played_yesterday) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                (&format!("{}", env::var("KWANGWON_ID").unwrap()), "Kwangwon", 2, 2, &format!("{}", env::var("KWANGWON_STEAM_ID").unwrap()), 0.0, 0.0, "", 0),
            )?;
            conn.execute(
                "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available, polaris_id, played_yesterday) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                (&format!("{}", env::var("KRIS_ID").unwrap()), "Kris", 2, 2, &format!("{}", env::var("KRIS_STEAM_ID").unwrap()), 0.0, 0.0, "", 0),
            )?;
        }
        // DB migration statements
        let mut stmt = conn.prepare("PRAGMA table_info(users)")?;
        let columns: Vec<String> = stmt
            .query_map([], |row| {
                let name: String = row.get(1)?;
                Ok(name)
            })?
            .collect::<Result<Vec<_>>>()?;

        if !columns.contains(&"polaris_id".to_string()) {
            conn.execute(
                "ALTER TABLE users ADD COLUMN polaris_id TEXT NOT NULL DEFAULT ''",
                [],
            )?;
        }
        if !columns.contains(&"played_yesterday".to_string()) {
            conn.execute(
                "ALTER TABLE users ADD COLUMN played_yesterday INT NOT NULL DEFAULT 0",
                [],
            )?;
        }
        if !columns.contains(&"weekly_hours".to_string()) {
            conn.execute(
                "ALTER TABLE users ADD COLUMN weekly_hours FLOAT NOT NULL DEFAULT 0.0",
                [],
            )?;
        }
        // Every time the db get intitalized it means that we are updating the bot
        // the update may not happen in a single day so its better to be able to control
        // the day counter whenever we choose to launch the bot
        let mut stmt =
            conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='time'")?;
        let table_exists = stmt.exists([])?;
        if table_exists {
            conn.execute("DROP TABLE time", [])?;
        }
        conn.execute(
            "CREATE TABLE IF NOT EXISTS time (
                    id INTEGER PRIMARY KEY,
                    month INTEGER NOT NULL,
                    week INTEGER NOT NULL,
                    year INTEGER NOT NULL,
                    zero_day_streak INTEGER NOT NULL
                )",
            [],
        )?;
        // Make sure to check that this is right before deployment lol
        conn.execute(
            "INSERT INTO time (month, week, year, zero_day_streak) VALUES (?1, ?2, ?3, ?4)",
            (4, 1, 2026, 0),
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
            polaris_id: row.get(7)?,
            played_yesterday: row.get(8)?,
            weekly_hours: row.get(9)?,
        })
    })?;
    let users: Result<Vec<User>> = user_collection.collect();
    users
}

pub fn update_user(conn: &Connection, user: User) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE users SET playtime = ?, hours_owed = ?, monthly_hours = ?, bet_hours_available = ?, played_yesterday = ? WHERE id = ?",
        params![user.playtime, user.hours_owed, user.monthly_hours, user.bet_hours_available, user.played_yesterday, user.id],
    )?;
    Ok(())
}
// This function exists so that I can manually update columns
pub fn update_user_column(
    conn: &Connection,
    polaris_id: &str,
    user_id: &str,
) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE users SET polaris_id = ? WHERE id = ?",
        params![polaris_id, user_id],
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
            polaris_id: row.get(7)?,
            played_yesterday: row.get(8)?,
            weekly_hours: row.get(9)?,
        })
    })?;
    let mut bet_total = amount;
    let mut weekly_change = 0.0;
    for user in user_collection {
        let clone_user = user?.clone();
        bet_total += clone_user.get_hours_owed();
        // negative amount means this is the winner so adjust the weekly_hours
        if amount < 0.0 {
            weekly_change = (amount * -1.0) + clone_user.get_weekly_hours();
        } else {
            weekly_change = clone_user.get_weekly_hours();
        }
    }
    
    conn.execute(
        "UPDATE users SET hours_owed = ?, weekly_hours = ? WHERE id = ?",
        params![bet_total, weekly_change, id],
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
            zero_day_streak: row.get(4)?,
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
            polaris_id: row.get(7)?,
            played_yesterday: row.get(8)?,
            weekly_hours: row.get(9)?,
        };
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

pub fn update_time(conn: &Connection, time: Time) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE time SET zero_day_streak = ?, month = ?, week = ?, year = ? WHERE id = ?",
        params![
            time.zero_day_streak,
            time.month,
            time.week,
            time.year,
            time.id
        ],
    )?;
    Ok(())
}

pub fn add_user(conn: &Connection, new_user: User) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO users (id, name, playtime, hours_owed, steam_id, monthly_hours, bet_hours_available, polaris_id, played_yesterday) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (new_user.get_id(), new_user.get_name(), new_user.get_playtime(), new_user.get_hours_owed(), new_user.get_hours_owed(), 0.0, 10.0, new_user.get_polar_id(), 0),
    )?;
    Ok(())
}

pub fn update_hours_owed(conn: &Connection, id: &str, hours: f32, monthly_hours: f32, weekly_hours: f32) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE users SET hours_owed = ?, monthly_hours = ?, weekly_hours = ? WHERE id = ?",
        params![hours, monthly_hours, weekly_hours, id],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_user_getters_setters() {
        let mut u = User::new(
            "id1".to_string(),
            "Alice".to_string(),
            1.23,
            10.0,
            "steam123".to_string(),
            0.0,
            5.0,
            0.0,
            "polar1".to_string(),
            0,
        );
        assert_eq!(u.get_id(), "id1");
        assert_eq!(u.get_name(), "Alice");
        assert_relative_eq!(u.get_playtime(), 1.23);
        assert_relative_eq!(u.get_hours_owed(), 10.0);
        u.set_playtime(2.5);
        assert_relative_eq!(u.get_playtime(), 2.5);
        u.set_hours_owed(7.75);
        assert_relative_eq!(u.get_hours_owed(), 7.75);
    }

    #[test]
    fn test_time_getters_setters() {
        let mut t = Time::new();
        t.set_month(12);
        t.set_week(3);
        t.set_year(2025);
        assert_eq!(t.get_month(), 12);
        assert_eq!(t.get_week(), 3);
        assert_eq!(t.get_year(), 2025);
        t.set_month(1);
        t.set_week(7);
        t.set_year(2026);
        assert_eq!(t.get_month(), 1);
        assert_eq!(t.get_week(), 7);
        assert_eq!(t.get_year(), 2026);
    }
}

#[cfg(test)]
mod db_mock_tests {
    use super::*;
    use approx::assert_relative_eq;
    use rusqlite::Connection;

    fn get_in_memory_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                playtime FLOAT NOT NULL,
                hours_owed FLOAT NOT NULL,
                steam_id TEXT NOT NULL,
                monthly_hours FLOAT NOT NULL,
                bet_hours_available FLOAT NOT NULL,
                polaris_id TEXT NOT NULL,
                played_yesterday INT NOT NULL DEFAULT 0,
                weekly_hours FLOAT NOT NULL DEFAULT 0.0
            )",
            [],
        ).unwrap();
        conn
    }

    #[test]
    fn test_add_user() {
        let conn = get_in_memory_db();
        let u = User::new(
            "test_id".to_string(),
            "test_name".to_string(),
            5.0,
            10.0,
            "steam_1".to_string(),
            0.0,
            0.0,
            0.0,
            "polar_1".to_string(),
            0,
        );
        add_user(&conn, u).unwrap();
        
        let fetched = get_user(&conn, "test_id").unwrap().unwrap();
        assert_eq!(fetched.get_name(), "test_name");
    }

    #[test]
    fn test_update_user() {
        let conn = get_in_memory_db();
        let u = User::new(
            "test_id".to_string(),
            "test_name".to_string(),
            5.0,
            10.0,
            "steam_1".to_string(),
            0.0,
            0.0,
            0.0,
            "polar_1".to_string(),
            0,
        );
        add_user(&conn, u).unwrap();
        
        let mut u_updated = get_user(&conn, "test_id").unwrap().unwrap();
        u_updated.set_playtime(20.5);
        update_user(&conn, u_updated).unwrap();
        
        let fetched_updated = get_user(&conn, "test_id").unwrap().unwrap();
        assert_relative_eq!(fetched_updated.get_playtime(), 20.5);
    }

    #[test]
    fn test_update_user_column() {
        let conn = get_in_memory_db();
        let u = User::new(
            "test_id".to_string(),
            "test_name".to_string(),
            5.0,
            10.0,
            "steam_1".to_string(),
            0.0,
            0.0,
            0.0,
            "polar_1".to_string(),
            0,
        );
        add_user(&conn, u).unwrap();
        
        update_user_column(&conn, "polar_2", "test_id").unwrap();
        let fetched_updated = get_user(&conn, "test_id").unwrap().unwrap();
        assert_eq!(fetched_updated.get_polar_id(), "polar_2");
    }

    #[test]
    fn test_update_hours_owed() {
        let conn = get_in_memory_db();
        let u = User::new(
            "test_id".to_string(),
            "test_name".to_string(),
            5.0,
            10.0,
            "steam_1".to_string(),
            0.0,
            0.0,
            0.0,
            "polar_1".to_string(),
            0,
        );
        add_user(&conn, u).unwrap();

        update_hours_owed(&conn, "test_id", 33.3, 11.1, 22.2).unwrap();
        let fetched = get_user(&conn, "test_id").unwrap().unwrap();
        assert_relative_eq!(fetched.get_hours_owed(), 33.3);
        assert_relative_eq!(fetched.get_monthly_hours(), 11.1);
        assert_relative_eq!(fetched.get_weekly_hours(), 22.2);
    }

    #[test]
    fn test_bet_result() {
        let conn = get_in_memory_db();
        let u = User::new(
            "test_id".to_string(),
            "test_name".to_string(),
            5.0,
            10.0,
            "steam_1".to_string(),
            0.0,
            0.0,
            0.0,
            "polar_1".to_string(),
            0,
        );
        
        add_user(&conn, u).unwrap();
        bet_result(&conn, -5.0, "test_id").unwrap();
        
        let fetched = get_user(&conn, "test_id").unwrap().unwrap();
        assert_relative_eq!(fetched.get_hours_owed(), 5.0);
        assert_relative_eq!(fetched.get_weekly_hours(), 5.0);
    }
}
