use std::ptr::hash;
use hyper::{Body, Request, Response};
use mysql::prelude::Queryable;
use url::Url;
use crate::structs::{ApiResponse, DatabaseMeme, IndexResultMeme, Meme, ResponseMeme};

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::process::Command;
use crate::HeaderValue;

pub async fn add_meme(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let body = hyper::body::to_bytes(req.into_body()).await?;
    let body = String::from_utf8(Vec::from(body)).unwrap();

    let parsed:crate::structs::UploadData = serde_json::from_str(&body).expect("Could not parse json");

    let title = parsed.title;
    let details = parsed.details;
    let tags = parsed.tags;
    let data_base64 = parsed.data;
    let data = base64::decode(data_base64).unwrap();
    let data_type = parsed.datatype;

    std::fs::create_dir_all("tmp".to_string()).unwrap();
    let mut hasher = DefaultHasher::new();
    hash(&data, &mut hasher);

    let hash = format!("{:x}", hasher.finish());

    let video_path = String::from("tmp/meme_") + &*hash + "." + &*data_type;
    let thumbnail_path = String::from("tmp/meme_") + &*hash + ".png";

    println!("{}", &data.len());

    std::fs::write(&video_path, &data).unwrap();

    let out = Command::new("ffmpeg")
        .args(["-i", &video_path, "-vframes", "1", "-f", "image2", &thumbnail_path])
        .output()
        .expect("failed to execute process");

    let thumbnail_data;

    if out.status.success() {
        thumbnail_data = std::fs::read(&thumbnail_path).unwrap();
        println!("{}", "success");
    } else {
        thumbnail_data = vec![];
        println!("{}", "fail");
    }

    let _ = std::fs::remove_file(&video_path);
    let _ = std::fs::remove_file(&thumbnail_path);

    println!("{:?}", out);

    println!("{:?}", title);

    let mut conn = crate::get_mysql_connection();
    conn.exec_drop("INSERT INTO memes (title, details, tags, thumbnail, data, data_type, data_size) VALUES (?, ?, ?, ?, ?, ?, ?)",
                   (title, details, serde_json::to_string(&tags).expect("wtf"),
                    thumbnail_data,
                    &data,
                    data_type,
                    &data.len())
    ).expect("Could not insert meme");

    let response = serde_json::to_string(&ApiResponse {
        success: true,
        message: "Meme added successfully".to_string(),
        data: "".parse().unwrap()
    }).unwrap();

    //println!("{}", body);
    Ok(Response::new(Body::from(
        response
    )))
}

pub async fn get_memes(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let uri_string = String::from("https://localhost") + &*req.uri().to_string();
    let request_url = Url::parse(&uri_string).unwrap();
    let params = request_url.query_pairs();

    let mut page = 0;
    let mut amount = 25;

    params.for_each(|(key, value)| {
        if key == "page" {
            page = value.parse::<i32>().expect("Could not parse value as i32");

        }
        if key == "pp" {
            amount = value.parse::<i32>().expect("Could not parse value as i32");

        }
        if key == "query" {
            println!("query: {}", value);
        }

    });
    println!("page: {}", page);
    println!("per page: {}", amount);

    let mut conn = crate::get_mysql_connection();

    let memes = conn.query_map("SELECT id, title, details, tags, thumbnail, data_type, data_size FROM memes",
                                   |(id, title, details, tags, thumbnail, data_type, data_size)| {
                                       DatabaseMeme { id, title, details, tags, thumbnail, data_type, data_size }
                                   },
    ).expect("Could not query memes");

    let mut memes_vec:Vec<IndexResultMeme> = Vec::new();

    for x in memes {
        let id = x.id;
        let title = x.title;
        let details = x.details;
        let tags:Vec<String> = serde_json::from_str(&x.tags).unwrap();
        let thumbnail = base64::encode(&x.thumbnail);
        let data_type = x.data_type;
        let data_size = x.data_size;

        memes_vec.push(IndexResultMeme { id, title, details, tags, thumbnail, data_type, data_size });
    }

    let mut response = Response::new(Body::from(
        serde_json::to_string(&memes_vec).unwrap()
    ));

    *response.headers_mut()
        .entry("Content-Type")
        .or_insert(HeaderValue::from_str("application/json").unwrap()) = HeaderValue::from_str("application/json").unwrap();

    Ok(response)
}

pub async fn get_meme(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let uri_string = String::from("https://localhost") + &*req.uri().to_string(); // dumb workaround

    let request_url = Url::parse(&uri_string).unwrap();
    let params = request_url.query_pairs();

    let mut id = 1;

    params.for_each(|(key, value)| {
        if key == "id" {
            id = value.parse::<i32>().expect("Could not parse value as i32");
        }
    });

    let mut conn = crate::get_mysql_connection();
    let memes = conn.query_map(format!("SELECT id, title, details, tags, data, data_type, data_size FROM memes WHERE id = {}", id),
                               |(id, title, details, tags, data, data_type, data_size)| {
                                   Meme { id, title, details, tags, data, data_type, data_size }
                               },
    ).expect("Could not query memes");

    let meme = memes.first().unwrap();
    let id = meme.id;
    let title = String::from(&meme.title);
    let details = String::from(&meme.details);
    let tags:Vec<String> = serde_json::from_str(&meme.tags).unwrap();
    let data = base64::encode(&meme.data);
    let data_type = String::from(&meme.data_type);
    let data_size = meme.data_size;

    let meme = ResponseMeme { id, title, details, tags, data, data_type, data_size };

    let mut response = Response::new(Body::from(
        serde_json::to_string(&meme).unwrap()
    ));

    *response.headers_mut()
        .entry("Content-Type")
        .or_insert(HeaderValue::from_str("application/json").unwrap()) = HeaderValue::from_str("application/json").unwrap();

    Ok(response)
}