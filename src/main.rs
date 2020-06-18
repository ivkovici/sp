mod db;

extern crate clap;
extern crate open;
use yansi::Paint;
use clap::{Arg, App, SubCommand};
use std::fs;
use std::borrow::Cow;
use crate::db::Replace;
use pickledb::{PickleDb};

/*

Runing tests:
./target/debug/sp --open=perkele
./target/debug/sp -o=perkele

p:\ ---> /mnt/public/
p:\sanoma.profession\media\layout\e-com\termek-kivalasztasa03.png

t:\ ---> /mnt/temp/
t:\igor\path\sccpre.cat-kim-jung-un-png-913514.png
/mnt/temp/igor/path/sccpre.cat-kim-jung-un-png-913514.png

./target/debug/sp -f='p:\' -r='/mnt/public/'
./target/debug/sp -f='t:\' -r='/mnt/temp/'
./target/debug/sp -o='t:\igor\path\sccpre.cat-kim-jung-un-png-913514.png'

https://github.com/Byron/open-rs

*/


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

        db::set_replace_pair(pair);
    }

    let db = db::get_db();

    if let Some(o) = matches.value_of("open") {
        let path = self::replace_path_name(o, &db);

        if self::path_exists(&path) {
            self::open_file(&path);
        } else {
            println!("{}", Paint::red("Path/file not exists."));
            println!("{}", Paint::red(&path));
        }
    }

    let rpairs = db.get::<Vec<Replace>>("replace_pairs");
    let replace_pairs;

    if rpairs.is_some() {
        replace_pairs = rpairs.unwrap();

        println!("{:?}", replace_pairs);
    }
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

/** Replaces the necessary parts in the path */
fn replace_path_name<'a>(path: &'a str, db: &'a PickleDb) -> Cow<'a, str> {
    let mut tmp = Cow::from(path);

    if let Some(replace_pairs) = db.get::<Vec<Replace>>("replace_pairs") {        
        for pair in &*replace_pairs {
            tmp = tmp.replace(&*pair.find, &*pair.replace).into();
        }
    }

    tmp = tmp.replace("\\", "/").into();
    tmp
}
