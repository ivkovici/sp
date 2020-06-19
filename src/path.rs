use std::fs;
use std::borrow::Cow;
use pickledb::{PickleDb};
use std::env;
use std::path::Path;
use clipboard_ext::prelude::*;
use clipboard_ext::x11_fork::ClipboardContext;
use yansi::Paint;
use crate::db::Replace;

/** Lists the replace pairs */
pub fn list(db: &PickleDb) {
    if let Some(replace_pairs) = db.get::<Vec<Replace>>("replace_pairs") {        
        for pair in &*replace_pairs {
            println!("{} => {}", Paint::blue(&*pair.find), Paint::blue(&*pair.replace));
        }
    } else {
        println!("{}", Paint::red("There are no replace pairs."));
    }
}

/** Translates the path and copies it to the clipboard */
pub fn translate(path: &str, db: &PickleDb) {
    let path = self::replace_path_name(path, &db, false);
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(String::from(&*path).into()).unwrap();
    println!("The translated path is copied to the clipboard: {}", Paint::green(path));
}

/** Opens up the given path in the default app */
pub fn open(path: &str, db: &PickleDb) {
    let path = self::replace_path_name(path, &db, true);

    if self::path_exists(&path) {
        self::open_file(&path);
    } else {
        println!("{}", Paint::red("Path/file not exists."));
        println!("{}", Paint::red(&path));
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
fn replace_path_name<'a>(path: &'a str, db: &'a PickleDb, to_linux: bool) -> Cow<'a, str> {
    let mut tmp = Cow::from(path);

    if to_linux {
        if let Some(replace_pairs) = db.get::<Vec<Replace>>("replace_pairs") {        
            for pair in &*replace_pairs {
                tmp = tmp.replace(&*pair.find, &*pair.replace).into();
            }
        }
    
        tmp = tmp.replace("\\", "/").into();
    } else {
        if let Some(replace_pairs) = db.get::<Vec<Replace>>("replace_pairs") {        
            for pair in &*replace_pairs {
                tmp = tmp.replace(&*pair.replace, &*pair.find).into();
            }
        }
    
        tmp = tmp.replace("/", "\\").into();
    }

    tmp
}