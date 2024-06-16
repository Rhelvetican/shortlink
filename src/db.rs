use anyhow::Result;
use nodb::{DumpPolicy, NoDb, SerializationMethod};
use std::{fs::create_dir_all, path::Path};

use crate::DB_PATH;

pub fn init_dir() -> Result<()> {
    let path = Path::new(DB_PATH).parent().unwrap();

    if !path.exists() {
        create_dir_all(path)?;
    }

    Ok(())
}

fn is_db_init() -> bool {
    Path::new(DB_PATH).exists()
}

pub fn start_db() -> Result<NoDb> {
    let db = if is_db_init() {
        NoDb::load(DB_PATH, DumpPolicy::Auto, SerializationMethod::Json)?
    } else {
        NoDb::new(DB_PATH, DumpPolicy::Auto, SerializationMethod::Json)
    };
    Ok(db)
}
