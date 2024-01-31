use std::env;
use uuid::Uuid;
use walkdir::WalkDir;
extern crate img_hash;
use image;
use img_hash::HasherConfig;
use std::fs;
// use std::fs::File;
// use std::io::{Read, Write};
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

pub mod db;
pub mod envvars;
pub mod types;

pub fn ext_from(fname: String) -> String {
    let ext_split = fname.split(".").collect::<Vec<&str>>();
    let ext = ext_split.last().unwrap().to_string();

    ext
}

pub fn test_img_open(apath: String) -> bool {
    let image_results = image::open(apath.clone());
    if image_results.is_ok() {
        true
    } else {
        false
    }
}

pub fn calc_hash(apath: String) -> types::ImgHashStruct {
    let image_results = image::open(apath.clone()).expect(apath.clone().as_str());
    let hasher_config = HasherConfig::new().to_hasher();
    let hashed = hasher_config.hash_image(&image_results);
    let imghash = types::ImgHashStruct { hash: hashed };

    imghash
}

pub fn insert_into_db(apath: String) {
    if test_img_open(apath.clone()) {
        let imgid = Uuid::new_v4().to_string();
        let imghash = calc_hash(apath.clone());
        let ext = crate::ext_from(apath.clone());
        let img_meta = types::Meta {
            imgid: imgid.clone(),
            imghash: imghash.hash.to_base64(),
            imgpath: apath.clone(),
        };
        if ext == "jpg".to_string() {
            db::insert_jpg(img_meta).expect("Dup Hash unable to insert");
        } else if ext == "png".to_string() {
            db::insert_png(img_meta).expect("Dup Hash unable to insert");
        } else if ext == "bmp".to_string() {
            db::insert_bmp(img_meta).expect("Dup Hash unable to insert");
        } else {
            println!("Unknown file type: {:?}", apath);
            return;
        };
    } else {
        fs::remove_file(apath).expect("Unable to remove file");
        
    }
}

fn main() {
    envvars::set_env_vars();
    let _tables = db::create_tables();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <apath>", args[0]);
        return;
    }
    let apath = &args[1];
    let mut fn_vec = Vec::new();

    let unable_to_open_dir = env::var("UNABLE_TO_OPEN").expect("UNABLE_TO_OPEN not set");
    fs::create_dir_all(unable_to_open_dir).expect("Unable to create directory");

    for e in WalkDir::new(apath.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            fn_vec.push(fname);
        }
    }

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    for jpg in fn_vec {
        println!("jpg {}", jpg);
        let tx = tx.clone();
        pool.execute(move || {
            let insertdb = crate::insert_into_db(jpg.clone());
            tx.send(insertdb).unwrap();
        });
    }
    drop(tx);

    for t in rx.iter() {
        let info = t;
        println!("Info: {:?}", info.clone());
    }
}
