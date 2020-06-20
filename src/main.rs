mod db;
mod path;
mod cli;

extern crate open;
extern crate dirs;

use structopt::StructOpt;

/* Application starts here */
fn main() { 
    let opt = cli::Opt::from_args();
    cli::option_replace_pairs(&opt);
    let db = db::get_db();
    cli::option_open(&opt, &db);
    cli::option_translate(&opt, &db);
    cli::option_list(&opt, &db);
    cli::option_empty(&opt, db);
}


