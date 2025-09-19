use crate::db::{Database};



pub fn show_database_info(db: &mut Database) -> bool {
    db.show_database_info();
    true
}