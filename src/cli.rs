use structopt::StructOpt;
use crate::db::Replace;
use crate::db;
use yansi::Paint;
use pickledb::{PickleDb};
use crate::path;

/// Translates windows paths for linux.
#[derive(StructOpt, Debug)]
#[structopt(name = "sp")]
pub struct Opt {
    /// Empties the replace pairs.
    #[structopt(short, long)]
    pub empty: bool,

    /// Lists the existing path replace pairs.
    #[structopt(short, long)]
    pub list: bool,

    /// Opens a file from a Windows or a Linux path.
    #[structopt(short, long)]
    pub open: Option<String>,

    /// We replace this value in the paths with the related REPLACE value.
    #[structopt(short, long)]
    pub find: Option<String>,

    /// We are replacing the related FIND value in the path with this.
    #[structopt(short, long)]
    pub replace: Option<String>,

    /// Translates a given path and copies it to the clipboard.
    #[structopt(short, long)]
    pub translate: Option<String>,
}

/* Sets a new replace pair */
pub fn option_replace_pairs(opt: &Opt) {
    if  let (Some(find), Some(replace)) = (&opt.find, &opt.replace) {
        db::set_replace_pair(Replace {
            find: String::from(find),
            replace: String::from(replace)
        });
    }
}

/** Opens the file or the path */
pub fn option_open(opt: &Opt, db: &PickleDb) {
    if let Some(open) = &opt.open {
        path::open(&open, &db)
    }
}

/** Translates the path and copies it to the clipboard */
pub fn option_translate(opt: &Opt, db: &PickleDb) {
    if let Some(translate) = &opt.translate {
        path::translate(&translate, &db);
    }
}

/** Lists the existing path replace pairs */
pub fn option_list(opt: &Opt, db: &PickleDb) {
    if opt.list {
        path::list(&db);
    }
}

/** Empties the stored replace pairs */
pub fn option_empty(opt: &Opt, mut db: PickleDb) {
    if opt.empty {
        db.rem("replace_pairs").unwrap();
        println!("{}", Paint::green("The replace pairs were emptied."));
    }
}