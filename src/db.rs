use crate::types;
use rusqlite::{Connection, Result};
use std::env;

pub fn create_tables() {
    let _at = create_jpg_table();
    let _aut = create_png_table();
    let _cmt = create_bmp_table();
}

pub fn create_jpg_table() -> Result<()> {
    let db_path = env::var("DUPS_DB").expect("DUPS_DB not set");
    let conn = Connection::open(db_path.clone())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS jpgs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            imgid TEXT NOT NULL,
            imghash TEXT NOT NULL,
            imgpath TEXT NOT NULL
         )",
        (),
    )?;
    println!("Created jpgs table: {:?}", &db_path);

    Ok(())
}

pub fn create_png_table() -> Result<()> {
    let db_path = env::var("DUPS_DB").expect("DUPS_DB not set");
    let conn = Connection::open(db_path.clone())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS pngs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            imgid TEXT NOT NULL,
            imghash TEXT NOT NULL,
            imgpath TEXT NOT NULL
         )",
        (),
    )?;
    println!("Created pngs table: {:?}", &db_path);

    Ok(())
}

pub fn create_bmp_table() -> Result<()> {
    let db_path = env::var("DUPS_DB").expect("DUPS_DB not set");
    let conn = Connection::open(db_path.clone())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS bmps (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            imgid TEXT NOT NULL,
            imghash TEXT NOT NULL,
            imgpath TEXT NOT NULL
         )",
        (),
    )?;
    println!("Created bmps table: {:?}", &db_path);

    Ok(())
}

pub fn insert_jpg(img_meta: types::Meta) -> Result<()> {
    let db_path = env::var("DUPS_DB").expect("DUPS_DB not set");
    let conn = Connection::open(db_path.clone())?;

    conn.execute(
        "INSERT INTO jpgs (imgid, imghash, imgpath) VALUES (?1, ?2, ?3)",
        &[&img_meta.imgid, &img_meta.imghash, &img_meta.imgpath],
    )?;
    println!("Inserted jpg: {:?}", &img_meta.imgpath);

    Ok(())
}

pub fn insert_png(img_meta: types::Meta) -> Result<()> {
    let db_path = env::var("DUPS_DB").expect("DUPS_DB not set");
    let conn = Connection::open(db_path.clone())?;

    conn.execute(
        "INSERT INTO pngs (imgid, imghash, imgpath) VALUES (?1, ?2, ?3)",
        &[&img_meta.imgid, &img_meta.imghash, &img_meta.imgpath],
    )?;
    println!("Inserted png: {:?}", &img_meta.imgpath);

    Ok(())
}

pub fn insert_bmp(img_meta: types::Meta) -> Result<()> {
    let db_path = env::var("DUPS_DB").expect("DUPS_DB not set");
    let conn = Connection::open(db_path.clone())?;

    conn.execute(
        "INSERT INTO bmps (imgid, imghash, imgpath) VALUES (?1, ?2, ?3)",
        &[&img_meta.imgid, &img_meta.imghash, &img_meta.imgpath],
    )?;
    println!("Inserted bmp: {:?}", &img_meta.imgpath);

    Ok(())
}
