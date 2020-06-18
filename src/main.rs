extern crate clap;
extern crate open;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::{Deserialize, Serialize};
use yansi::Paint;
use clap::{Arg, App, SubCommand};
use std::fs;

/*

Runing tests:
./target/debug/sp --open=perkele
./target/debug/sp -o=perkele

p:\ ---> /mnt/public/
p:\sanoma.profession\media\layout\e-com\termek-kivalasztasa03.png

t:\ ---> /mnt/temp/
t:\igor\path\sccpre.cat-kim-jung-un-png-913514.png
/mnt/temp/igor/path/sccpre.cat-kim-jung-un-png-913514.png

https://github.com/Byron/open-rs

*/

#[derive(Serialize, Deserialize, Debug)]
pub struct Replace {
    pub find: String,
    pub replace: String,
}


fn main() { 
    let matches = App::new("sp")
    .version("1.0")
    .author("Igor Ivkovic")
    .about("Translates windows paths for linux.")
    .arg(Arg::with_name("open")
        .short("o")
        .long("open")
        .value_name("FILE")
        .help("Opens a file.")
        .takes_value(true)
    )
    .arg(Arg::with_name("find")
        .short("f")
        .long("find")
        .value_name("FIND")
        .help("We replace this value in the paths with the related REPLACE value.")
        .takes_value(true)
    )
    .arg(Arg::with_name("replace")
        .short("r")
        .long("replace")
        .value_name("REPLACE")
        .help("We are replacing the related FIND value in the path with this.")
        .takes_value(true)
    )
    .get_matches(); 

    let find = matches.value_of("find");
    let replace = matches.value_of("replace");

    if  find.is_some() && replace.is_some() {
        let f = find.unwrap();
        let t = replace.unwrap();

        println!("{}", f);
        println!("{}", t);

        let pair = Replace {
            find: String::from(f),
            replace: String::from(t)
        };

        println!("pair {:?}", pair);

        self::set_replace_pair(pair);
    }

    let db = self::get_db();

    let rpairs = db.get::<Vec<Replace>>("replace_pairs");
    let mut replace_pairs;

    if rpairs.is_some() {
        replace_pairs = rpairs.unwrap();
        //println!("{:?}", replace_pairs.unwrap());
    } else {
        replace_pairs = vec![];
    }

    if let Some(o) = matches.value_of("open") {
        let mut path = o;
        
        for pair in replace_pairs {
            path = str::replace(&path, &pair.find, &pair.replace);
        }

        path = str::replace(&path, "\\", "/");

        if self::path_exists(&path) {
            self::open_file(&path);
        } else {
            println!("{}", Paint::red("Path/file not exists."));
        }

        println!("Value for config: {}", o);
        //println!("Value for config: {}", path);
    }
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


/** Checks if the path or the file is an existing one */
 pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

/** Opens the path or file. If unable to open it then prints out the error message */
pub fn open_file(path: &str) {
    let op_file = open::that(path);

    match op_file {
        Ok(_) => println!("{}", Paint::green("The path/file was opened.")),
        Err(e) => println!("{}", Paint::red(e)),
    }
}
