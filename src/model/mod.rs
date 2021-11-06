use crate::db;

#[crud_table]
#[derive(Clone, Debug, PartialEq)]
pub struct Envelope {
    pub rid: String,
    pub uid: String,
    pub snatch_time: u64,
    pub status: u8,
    pub value: u64,
}

#[sql(db::RB, "select * from envelope where rid = ?")]
pub async fn select_by_rid(rid: &str) -> Envelope {}

#[sql(db::RB, "UPDATE envelope SET status = 1 WHERE rid = ?")]
pub async fn update_status_by_rid(rid: &str) -> () {}

// #[sql(db::RB, "INSERT INTO envelope (rid,uid,snatch_time,status,value) VALUES (?, ?, ?, ?, ?)")]
// pub async fn add_envelope(rid: &Envelope) -> () {}