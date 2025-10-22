use crate::db;
use std::sync::{Arc, Mutex};
use std::collections::{HashSet, HashMap};

#[derive(Clone)]
pub struct Bet {
    user1: String,
    user2: String,
    bet: f32,
    ticket_no: i32,
}
pub struct BetOverlord {
    betters: HashSet<String>,
    trusted_users: HashSet<String>,
    hours_available: HashMap<String, f32>,
    bet_house: Arc<Mutex<HashMap<i32, Bet>>>,
    counter: i32,
}

impl Bet {
    pub fn new(u1: &str, u2: &str, b: f32, tno: i32) -> Self {
        Bet {
            user1: u1.to_string(),
            user2: u2.to_string(),
            bet: b,
            ticket_no: tno,
        }
    }
    fn get_amount(&self) -> f32 {
        self.bet
    }
}

impl BetOverlord {
    pub fn new() -> Self {
        BetOverlord {
            betters: HashSet::<String>::new(),
            trusted_users: HashSet::<String>::new(),
            hours_available: HashMap::<String, f32>::new(),
            bet_house: Arc::new(Mutex::new(HashMap::<i32, Bet>::new())),
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
    pub fn add_trusted(&mut self, id: String) {
        self.trusted_users.insert(id);
    }
    pub fn update_bet_hours(&mut self, id: String, hours: f32) {
        self.hours_available.insert(id, hours);
    }
    pub fn hour_check(&self, id1: &str, id2: &str, bet: f32) -> bool {
        let p1_hours = self.hours_available.get(id1);
        let p2_hours = self.hours_available.get(id2);
        *p1_hours.unwrap() + bet <= 10.0 && *p2_hours.unwrap() + bet <= 10.0
    }
    pub fn get_bet_hours(&self, id: &str) -> f32 {
        match self.hours_available.get(id) {
            Some(&val) => val,
            None => -1.0,
        }
    }
    pub fn handle_bet_creation(&mut self, id1: String, id2: String, bet: f32) -> i32 {
        let ticket_no: i32 = self.counter;
        let bet_ticket: Bet = Bet::new(&id1, &id2, bet, ticket_no);
        self.counter += 1;
        self.bet_house.lock().unwrap().insert(ticket_no, bet_ticket);
        let p1_hours = self.hours_available.get(&id1).unwrap() + bet;
        let p2_hours = self.hours_available.get(&id2).unwrap() + bet;
        let _ = self.update_bet_hours(id1.to_string(), p1_hours);
        let _ = self.update_bet_hours(id2.to_string(), p2_hours);
        ticket_no
    }

    fn bet_payout(&self, _user: String, _winner: bool) {

    }

    pub fn handle_bet_resolution(&mut self, ticket_no: i32, winner: String) {
        let ticket = self.bet_house.get(ticket_no);
        // Check that the user who was passed in was actually on the ticket
        if winner == ticket.user1 {
            self.bet_payout(winner, true);
            self.bet_payout(ticket.user2, false);
        } else if winner == ticket.user2 {
            self.bet_payout(winner, true);
            self.bet_payout(ticket.user1, false);
        } else {

        }
    }
}
