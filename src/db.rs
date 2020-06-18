use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Replace {
    pub find: String,
    pub replace: String,
}

/** Loads or creates the db */
pub fn get_db() -> PickleDb {
    let db;
    let load_db = PickleDb::load(
        "sp.db",
        PickleDbDumpPolicy::DumpUponRequest,
        SerializationMethod::Json,
    );

    match load_db {
        Ok(db_loaded) => db = db_loaded,
        Err(_) => {
            db = PickleDb::new(
                "sp.db",
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
            replace_pairs.push(pair);
        },
        None => {
            replace_pairs = vec![pair];
        }
    }
    
    db.set("replace_pairs", &replace_pairs).unwrap();
}