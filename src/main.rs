mod db;
mod hash;
mod regex;

const DB_PATH: &str = "./db/links.nodb";
const DICT: &str = "./db/dict.txt";

use std::{
    env::args,
    fs::{File, OpenOptions},
    io::Write,
};

use anyhow::Result;
use db::{init_dir, start_db};
use hash::hash;
use regex::is_url;

fn main() -> Result<()> {
    init_dir()?;
    let args = get_args();
    let mut dict = open_dict()?;
    let mut db = start_db()?;
    for url in args {
        if is_url(&url)? {
            let hash = hash(&url)?;
            db.set(&hash, &url)?;
            dict.write_all(format!("{} : {}\n", hash, url).as_bytes())?;
        } else if let Some(url) = db.get::<&str, String>(&url) {
            println!("{}", url);
        } else {
            println!("No such link found");
        }
    }
    Ok(())
}

fn open_dict() -> Result<File> {
    Ok(OpenOptions::new()
        .write(true)
        .truncate(false)
        .create(true)
        .open(DICT)?)
}

fn get_args() -> Vec<String> {
    let args = args().collect::<Vec<String>>();
    let args = &args[1..];
    args.iter().map(|arg| arg.to_string()).collect()
}
