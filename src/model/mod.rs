use crate::db;

#[crud_table]
#[derive(Debug, PartialEq)]
pub struct Envelope {
    pub envelope_id: String,
    pub user_id: String,
    pub snatch_time: u64,
    pub opened: bool,
    pub value: u64,
}


// #[sql(db::RB, "select * from envelope where envelope_id = ?")]
// pub async fn select_by_rid(envelope_id: &str) -> Envelope {}

#[sql(db::RB, "UPDATE envelope SET opened = true WHERE envelope_id = ?")]
pub async fn update_status_by_rid(envelope_id: &str) -> () {}