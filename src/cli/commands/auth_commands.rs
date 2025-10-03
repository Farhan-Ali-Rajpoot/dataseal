use super::{
    structs::Database,
};



pub fn change_root_password(db: &mut Database, parts: &[&str]) -> bool {
    let prev_pass = parts[1];
    let new_pass = parts[2];

    db.change_master_password_independent(prev_pass, new_pass);
    true
}