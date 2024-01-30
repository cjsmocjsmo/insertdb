use std::env;
use uuid::Uuid;
use walkdir::WalkDir;
extern crate img_hash;
use image;
use img_hash::HasherConfig;

pub mod db;
pub mod envvars;
pub mod types;

fn ext_from(fname: String) -> String {
    let ext_split = fname.split(".").collect::<Vec<&str>>();
    let ext = ext_split.last().unwrap().to_string();

    ext
}

pub fn calc_hash(apath: String) -> types::ImgHashStruct {
    let image_results = image::open(apath.clone()).expect(apath.clone().as_str());
    let hasher_config = HasherConfig::new().to_hasher();
    let hashed = hasher_config.hash_image(&image_results);
    let imghash = types::ImgHashStruct { hash: hashed };

    imghash
}

fn main() {
    envvars::set_env_vars();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <apath>", args[0]);
        return;
    }
    let apath = &args[1];

    for e in WalkDir::new(apath.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            let imgid = Uuid::new_v4().to_string();
            let imghash = calc_hash(fname.clone());
            let ext = ext_from(fname.clone());
            let img_meta = types::Meta {
                imgid: imgid.clone(),
                imghash: imghash.hash.to_base64(),
                imgpath: fname.clone(),
            };
            if ext == "jpg".to_string() {
                db::insert_jpg(img_meta).unwrap();
            } else if ext == "png".to_string() {
                db::insert_png(img_meta).unwrap();
            } else if ext == "bmp".to_string() {
                db::insert_bmp(img_meta).unwrap();
            } else {
                println!("Unknown file type: {:?}", fname);
                continue;
            }
        }
    }
}
