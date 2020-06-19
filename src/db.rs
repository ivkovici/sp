use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::{Deserialize, Serialize};
use yansi::Paint;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Replace {
    pub find: String,
    pub replace: String,
}

/** Loads or creates the db */
pub fn get_db() -> PickleDb {
    let home = dirs::home_dir().unwrap();
    let mut path = String::from(home.to_str().unwrap());
    path.push_str("/sp.db");

    let db;
    let load_db = PickleDb::load(
        &path,
        PickleDbDumpPolicy::DumpUponRequest,
        SerializationMethod::Json,
    );

    match load_db {
        Ok(db_loaded) => db = db_loaded,
        Err(_) => {
            db = PickleDb::new(
                path,
                PickleDbDumpPolicy::AutoDump,
                SerializationMethod::Json,
            );
        },
    }

    db
}

/** Expands the replace pairs list with new replace pairs */
pub fn set_replace_pair(pair: Replace) {
    let mut db = self::get_db();
    let mut replace_pairs: Vec<Replace>;

    match db.get::<Vec<Replace>>("replace_pairs") {
        Some(pairs) => {
            replace_pairs = pairs;
            replace_pairs.push(pair.clone());
        },
        None => {
            replace_pairs = vec![pair.clone()];
        }
    }
    
    db.set("replace_pairs", &replace_pairs).unwrap();
    println!("New replace pair is set: {} => {}", Paint::blue(&*pair.find), Paint::blue(&*pair.replace));
}