#[macro_use]
macro_rules! var {
    ( $var:tt ) => {
        &std::env::var(stringify!($var)).unwrap_or(default_var!($var).to_owned());
    };
}

macro_rules! default_var {
    ( ROMF_SERVE_PATH ) => {
        "./uploads"
    };

    ( ROMF_BASE_URL ) => {
        "http://localhost:80"
    };

    ( ROMF_ADDRESS ) => {
        "0.0.0.0:80"
    };
}

mod prelude {
    pub use serde::{Deserialize, Serialize};
    pub use tide::prelude::*;
    pub use tide::{Request, Response, StatusCode};
}

use crate::prelude::*;
use async_std::fs::OpenOptions;
use async_std::io::prelude::*;
use multipart::server::Multipart;
use rand::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use tide::http::headers::HeaderValue;
use tide::http::Mime;
use tide::http::Url;
use tide::security::{CorsMiddleware, Origin};
use tide::Body;

pub const UPLOAD_LOG: &str = "./upload_log";
pub const ACCESS_LOG: &str = "./access_log";
pub const BLACKLIST_PATH: &str = "./blacklist";
const BLOCKED_EXTS: &[&str] = &["html", "htm", "php"];

mod blacklist;

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadRecord {
    id: String,
    file_name: String,
    source: String,
    address: String,
    timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessRecord {
    id: String,
    exists: bool,
    address: String,
    timestamp: String,
}

pub trait RequestExt: Sized {
    fn url_param<T: core::str::FromStr>(&self, key: &str) -> Result<T, tide::Error>;
    fn query_param<T: core::str::FromStr>(&self, key: &str) -> Result<T, tide::Error>;
}

macro_rules! invalid_type_template {
    () => {
        "{{'msg': 'Query parameter has wrong type: {}'}}"
    };
}
macro_rules! missing_param_template {
    () => {
        "{{'msg': 'Missing query parameter: {}'}}"
    };
}

impl<State> RequestExt for Request<State> {
    fn url_param<T: core::str::FromStr>(&self, key: &str) -> Result<T, tide::Error> {
        let val: String = self.param(key)?;
        let val = urlencoding::decode(&val);
        if let Ok(val) = val {
            if let Ok(val) = val.parse::<T>() {
                return Ok(val);
            }
            return Err(tide::Error::new(
                StatusCode::BadRequest,
                RequestError(format!(invalid_type_template!(), val)),
            ));
        }
        return Err(tide::Error::new(
            StatusCode::BadRequest,
            RequestError(format!(missing_param_template!(), key)),
        ));
    }

    fn query_param<T: core::str::FromStr>(&self, key: &str) -> Result<T, tide::Error> {
        let url = urlencoding::decode(self.url().as_str()).unwrap();
        let url: Url = url.parse()?;
        let map = url
            .query_pairs()
            .into_owned()
            .collect::<HashMap<String, String>>();
        if let Some(val) = map.get(key) {
            if let Ok(val) = val.parse::<T>() {
                return Ok(val);
            }
            return Err(tide::Error::new(
                StatusCode::BadRequest,
                RequestError(format!(invalid_type_template!(), val)),
            ));
        }

        return Err(tide::Error::new(
            StatusCode::BadRequest,
            RequestError(format!(missing_param_template!(), key)),
        ));
    }
}

#[derive(Debug)]
pub struct RequestError(String);

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for RequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    std::fs::create_dir_all(var!(ROMF_SERVE_PATH))?;
    let mut app = tide::new();
    app.middleware(blacklist::BlacklistMiddleware::new());

    app.middleware(
        CorsMiddleware::new()
            .allow_methods(
                "GET, POST, OPTIONS, DELETE, PATCH, PUT"
                    .parse::<HeaderValue>()
                    .unwrap(),
            )
            .allow_origin(Origin::from("*"))
            .allow_credentials(false),
    );

    app.at("/").get(serve_frontpage);
    app.at("/*").serve_dir("public").unwrap();

    app.at("/upload").post(upload);
    app.at("/files/:id").get(serve_files);

    app.listen(var!(ROMF_ADDRESS)).await.unwrap();
    Ok(())
}

pub async fn serve_frontpage(_req: Request<()>) -> tide::Result<Response> {
    let mut res = tide::Response::new(StatusCode::Ok);
    res.set_body(Body::from_file("./public/index.html").await.unwrap());
    res.set_content_type(Mime::from_str("text/html; charset=utf-8").unwrap());
    Ok(res)
}

pub async fn serve_files(req: Request<()>) -> tide::Result<Response> {
    let id: String = req.param("id")?;
    let path: PathBuf = format!("{}/{}", var!(ROMF_SERVE_PATH), id).into();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(ACCESS_LOG)
        .await?;

    if path.exists() {
        let record = AccessRecord {
            id,
            exists: true,
            address: req.remote().unwrap_or("unknown").to_string(),
            timestamp: chrono::Local::now().to_rfc3339(),
        };
        let mut record = serde_json::to_string(&record).unwrap();
        record.push('\n');
        file.write_all(record.as_bytes()).await?;

        let mut res = tide::Response::new(StatusCode::Ok);
        res.set_body(Body::from_file(path).await.unwrap());
        return Ok(res);
    }

    let record = AccessRecord {
        id,
        exists: false,
        address: req.remote().unwrap_or("unknown").to_string(),
        timestamp: chrono::Local::now().to_rfc3339(),
    };
    let mut record = serde_json::to_string(&record).unwrap();
    record.push('\n');
    file.write_all(record.as_bytes()).await?;

    let mut res = tide::Response::new(StatusCode::NotFound);
    res.set_body("File not found");
    Ok(res)
}

pub async fn upload(mut req: Request<()>) -> tide::Result<Response> {
    let mut data = req.body_bytes().await?;
    data.pop();
    data.pop();

    let mut boundary = data
        .iter()
        .take_while(|c| c != &&0xA)
        .cloned()
        .skip(2)
        .collect::<Vec<u8>>();
    boundary.pop();

    let mut form = Multipart::with_body(data.as_slice(), String::from_utf8_lossy(&boundary));
    let mut field = form.read_entry().unwrap().unwrap();

    let file_name = field.headers.filename.unwrap().replace(" ", "_");
    let ext = PathBuf::from(file_name.clone());
    let ext = ext.extension().unwrap_or_default().to_string_lossy();
    let id = format!("{}.{}", gen_id(), ext);
    let path = format!("{}/{}", var!(ROMF_SERVE_PATH), id);
    if BLOCKED_EXTS.contains(&ext.as_ref()) {
        return Ok(tide::Response::new(StatusCode::BadRequest));
    }

    const SIZE_LIMIT: u64 = 512 * 1000 * 1000;
    let _file = field
        .data
        .save()
        .size_limit(SIZE_LIMIT)
        .memory_threshold(0)
        .with_path(path);

    println!("Uploaded: {}, {}", id, req.remote().unwrap_or("Unknown"));
    let content = format!("{}/files/{}", var!(ROMF_BASE_URL), id);

    {
        let record = UploadRecord {
            id,
            file_name,
            source: req.query_param("source").unwrap_or(String::from("unknown")),
            address: req.remote().unwrap_or("unknown").to_string(),
            timestamp: chrono::Local::now().to_rfc3339(),
        };
        let mut data = serde_json::to_string(&record).unwrap();
        data.push('\n');
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(UPLOAD_LOG)
            .await?;
        file.write_all(data.as_bytes()).await?;
    }

    let mut res = tide::Response::new(StatusCode::Ok);
    res.set_body(content.as_str());
    Ok(res)
}

fn gen_id() -> String {
    let alpha: Vec<char> = ('a'..'z').collect();
    let num: Vec<char> = ('2'..'9').collect();
    let alphanum = alpha.iter().chain(num.iter());
    let mut rng = rand::thread_rng();
    let id = alphanum.choose_multiple(&mut rng, 8).into_iter().collect();
    return id;
}
