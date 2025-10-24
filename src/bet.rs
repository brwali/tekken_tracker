use crate::db;
use crate::daily_task;
use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use std::collections::{HashSet, HashMap};

#[derive(Clone)]
pub struct Bet {
    user1: String,
    user2: String,
    bet: f32,
}
pub struct BetOverlord {
    betters: HashSet<String>,
    trusted_users: HashSet<String>,
    hours_available: HashMap<String, f32>,
    bet_house: Arc<Mutex<HashMap<i32, Bet>>>,
    name_relation: HashMap<String, String>,
    hour_change: HashMap<String, f32>,
    counter: i32,
}

impl Bet {
    pub fn new(u1: &str, u2: &str, b: f32) -> Self {
        Bet {
            user1: u1.to_string(),
            user2: u2.to_string(),
            bet: b,
        }
    }
    pub fn get_amount(&self) -> f32 {
        self.bet
    }
    pub fn get_user1(&self) -> &str {
        &self.user1
    }
    pub fn get_user2(&self) -> &str {
        &self.user2
    }
}

impl BetOverlord {
    pub fn new() -> Self {
        BetOverlord {
            betters: HashSet::<String>::new(),
            trusted_users: HashSet::<String>::new(),
            hours_available: HashMap::<String, f32>::new(),
            bet_house: Arc::new(Mutex::new(HashMap::<i32, Bet>::new())),
            name_relation: HashMap::<String, String>::new(),
            hour_change: HashMap::<String, f32>::new(),
            counter: 0,
        }
    }
    pub fn can_bet(&self, id: &str) -> bool {
        self.betters.contains(id)
    }
    pub fn is_trusted(&self, id: &str) -> bool {
        self.trusted_users.contains(id)
    }
    pub fn add_better(&mut self, id: String) {
        self.betters.insert(id);
    }
    pub fn update_hour_change(&mut self, id: String, amount: f32) {
        if amount == 0.0 {
            self.hour_change.insert(id, amount);
        }
        else {
            let prev = self.hour_change.get(&id);
            let new_value = match prev {
                Some(&val) => daily_task::round_after_math(val + amount),
                None => amount,
            };
            self.hour_change.insert(id, new_value);
        }
    }
    pub fn get_hours_change(&self, id: &str) -> f32 {
        *self.hour_change.get(id).unwrap()
    }
    pub fn add_relation(&mut self, id: String, name: String) {
        self.name_relation.insert(id, name);
    }
    pub fn add_trusted(&mut self, id: String) {
        self.trusted_users.insert(id);
    }
    pub fn remove_trusted(&mut self, id: &str) {
        self.trusted_users.remove(id);
    }
    pub fn update_bet_hours(&mut self, id: String, hours: f32) {
        self.hours_available.insert(id, hours);
    }
    pub fn list_bets(&self) -> String {
        let mut message = String::from("Tekken bets in progress:\n");
        let binding = self.bet_house.lock().unwrap();
        for (ticket_no, bet) in binding.iter() {
            let name1 = self.name_relation.get(bet.get_user1());
            let name2 = self.name_relation.get(bet.get_user2());
            message.push_str(
                &format!("Bet number: {} - debtor one: {} debtor two: {} - bet amount: {}\n",
                    ticket_no, name1.map_or("mystery man", |v| v), name2.map_or("mystery man", |v| v), bet.get_amount()
                )
            );
        }
        message
    }
    pub fn hour_check(&self, id1: &str, id2: &str, bet: f32) -> bool {
        match (self.hours_available.get(id1), self.hours_available.get(id2)) {
            (Some(p1), Some(p2)) => *p1 - bet >= 0.0 && *p2 - bet >= 0.0,
            _ => false,
        }
    }
    pub fn get_bet_hours(&self, id: &str) -> f32 {
        match self.hours_available.get(id) {
            Some(&val) => val,
            None => -1.0,
        }
    }
    pub fn handle_bet_creation(&mut self, id1: String, id2: String, bet: f32) -> i32 {
        let ticket_no: i32 = self.counter;
        let bet_ticket: Bet = Bet::new(&id1, &id2, bet);
        self.counter += 1;
        self.bet_house.lock().unwrap().insert(ticket_no, bet_ticket);
        let p1_hours = daily_task::round_after_math(self.hours_available.get(&id1).unwrap() - bet);
        let p2_hours = daily_task::round_after_math(self.hours_available.get(&id2).unwrap() - bet);
        let _ = self.update_bet_hours(id1, p1_hours);
        let _ = self.update_bet_hours(id2, p2_hours);
        ticket_no
    }

    pub fn handle_bet_resolution(&mut self, db: Arc<Mutex<Connection>>, ticket_no: i32, winner: String) -> (String, String, f32) {
        let ticket_after;
        {
            let mut binding = self.bet_house.lock().unwrap();
            let ticket = binding.get(&ticket_no).unwrap();
            ticket_after = ticket.clone();
            let db_connection = db.lock().unwrap();
            // Check that the user who was passed in was actually on the ticket
            if winner == ticket.get_user1() {
                let _ = db::bet_result(&db_connection, ticket.get_amount() * -1.0, &winner);
                let user2 = ticket.get_user2();
                let _ = db::bet_result(&db_connection, ticket.get_amount(), &user2);
            } else if winner == ticket.get_user2() {
                let _ = db::bet_result(&db_connection, ticket.get_amount() * -1.0, &winner);
                let user1 = ticket.get_user1();
                let _ = db::bet_result(&db_connection, ticket.get_amount(), &user1);   
            } else {
                return ("Fake".to_string(), "Fake".to_string(), -1.0);
            }
            let _ = binding.remove(&ticket_no);
        }
        self.update_hour_change(winner.clone(), ticket_after.get_amount() * -1.0);
        let loser;
        if winner == ticket_after.get_user1() {
            loser = ticket_after.get_user2();
            self.update_hour_change(ticket_after.get_user2().to_string(), ticket_after.get_amount());
        }else {
            loser = ticket_after.get_user1();
            self.update_hour_change(ticket_after.get_user1().to_string(), ticket_after.get_amount());
        }
        return (winner.to_string(), loser.to_string(), ticket_after.get_amount());
    }
}
