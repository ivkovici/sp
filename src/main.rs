mod db;
mod path;

extern crate clap;
extern crate open;
use crate::db::Replace;

use yansi::Paint;
use clap::{Arg, App};


/*

Runing tests:
./target/debug/sp --open=perkele
./target/debug/sp -o=perkele

p:\ ---> /mnt/public/
p:\sanoma.profession\media\layout\e-com\termek-kivalasztasa03.png

t:\ ---> /mnt/temp/
t:\igor\path\sccpre.cat-kim-jung-un-png-913514.png
/mnt/temp/igor/path/sccpre.cat-kim-jung-un-png-913514.png

./target/debug/sp -f 'p:\' -r '/mnt/public/'
./target/debug/sp -f 't:\' -r '/mnt/temp/'
./target/debug/sp -o 't:\igor\path\sccpre.cat-kim-jung-un-png-913514.png'
./target/debug/sp -t /mnt/temp/igor/path/sccpre.cat-kim-jung-un-png-913514.png

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
    .arg(Arg::with_name("translate")
        .short("t")
        .long("translate")
        .value_name("TRANSLATE")
        .help("Translates a given path and copies it to the clipboard.")
        .takes_value(true)
    )
    .arg(Arg::with_name("empty")
        .short("e")
        .long("empty")
        .value_name("EMPTY")
        .help("Empties the replace pairs.")
        .takes_value(false)
    )
    .arg(Arg::with_name("list")
        .short("l")
        .long("list")
        .value_name("list")
        .help("Lists the existing path replace pairs.")
        .takes_value(false)
    )
    .get_matches(); 

    let find = matches.value_of("find");
    let replace = matches.value_of("replace");

    // sets a new replace pair
    if  find.is_some() && replace.is_some() {
        db::set_replace_pair(Replace {
            find: String::from(find.unwrap()),
            replace: String::from(replace.unwrap())
        });
    }

    let mut db = db::get_db();

    // opens the file or the path
    if let Some(o) = matches.value_of("open") {
        path::open(o, &db)
    }

    // translates the path and copies it to the clipboard
    if let Some(t) = matches.value_of("translate") {
        path::translate(t, &db);
    }

    // lists the existing path replace pairs
    if matches.is_present("list") {
        path::list(&db);
    }

    // empties the stored replace pairs
    if matches.is_present("empty") {
        db.rem("replace_pairs").unwrap();
        println!("{}", Paint::green("The replace pairs were emptied."));
    }
}


