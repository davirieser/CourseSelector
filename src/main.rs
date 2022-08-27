#![allow(non_snake_case, unused, dead_code)]

use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};

use actix_files::{Files, NamedFile};
use actix_web::{get, web, App, Either, HttpResponse, HttpServer, Responder};
use awc::{error::HttpError, http::Uri, Client};
use lazy_static::lazy_static;
use scraper::{Html, Selector};
use serde::{Serialize, Serializer};

const PUBLIC_DIR: &str = "./static/dist";
const LFU_FACULTY_URL: &str = "https://lfuonline.uibk.ac.at/public/lfuonline_lv.home#lv-details";

fn format_lfu_object_url(id: usize) -> String {
    format!(
        "https://lfuonline.uibk.ac.at/public/lfuonline_lv.rubrik_ajax?r={}&l=0",
        id
    )
}

fn format_course_url(id: usize) -> String {
    format!(
        "https://lfuonline.uibk.ac.at/public/lfuonline_lv.details_ajax?id={}",
        id
    )
}

lazy_static! {
    static ref XNODE_SELECTOR: Selector = Selector::parse(".xnode").unwrap();
    static ref TABLE_SELECTOR: Selector = Selector::parse("table").unwrap();
    static ref TR_SELECTOR: Selector = Selector::parse("tr").unwrap();
    static ref TD_SELECTOR: Selector = Selector::parse("td").unwrap();
    static ref A_SELECTOR: Selector = Selector::parse("a").unwrap();
    static ref LV_ROW_SELECTOR: Selector = Selector::parse(".lv-row").unwrap();
    static ref LV_DETAILS_SELECTOR: Selector = Selector::parse(".lv-details").unwrap();
    static ref LV_NO_SELECTOR: Selector = Selector::parse(".lv-no").unwrap();
    static ref LV_TITLE_SELECTOR: Selector = Selector::parse(".lv-title").unwrap();
    static ref LV_TITLE_TEXT_SELECTOR: Selector = Selector::parse(".lv-title-text").unwrap();
}

#[derive(Serialize, Debug)]
struct Response<D, M>
where
    D: Serialize,
    M: Display,
{
    success: bool,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<D>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<M>,
}

impl<D, M> Response<D, M>
where
    D: Serialize,
    M: Display,
{
    fn success(data: D) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    fn error(error: M) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

#[derive(Serialize)]
struct LfuObject {
    id: usize,
    name: String,
}

#[derive(Serialize)]
struct Course {
    id: usize,
    times: Vec<CourseDate>,
}

#[derive(Serialize)]
struct CourseDate {
    // TODO: Add Group
    date: String,
    time: String,
    location: String,
    comment: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .service(index)
            .service(get_faculties)
            .service(get_object)
            .service(get_course)
            .service(Files::new("/public", PUBLIC_DIR).prefer_utf8(true));
        #[cfg(debug_assertions)]
        let app = app
            .service(debug_get_object)
            .service(debug_get_course)
            .service(debug_get_faculties);
        app
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    let path: PathBuf = [PUBLIC_DIR, "index.html"].iter().collect();
    Ok(NamedFile::open(path)?)
}

#[cfg(debug_assertions)]
#[get("/debug/lfu")]
async fn debug_get_faculties() -> actix_web::Result<String> {
    if let Ok(document) = make_request(LFU_FACULTY_URL).await {
        return Ok(document);
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Could not find Objects").into())
}

#[get("/lfu")]
async fn get_faculties() -> web::Json<Response<Objects, &'static str>> {
    if let Ok(document) = get_html_document(LFU_FACULTY_URL).await {
        return match get_html_objects(document, 0) {
            Some(Objects::Object(objects)) => {
                web::Json(Response::success(Objects::Object(objects)))
            }
            Some(Objects::CourseDetails(details)) => {
                web::Json(Response::success(Objects::CourseDetails(details)))
            }
            _ => web::Json(Response::error("Server did not send correct Type")),
        };
    };
    web::Json(Response::error("Could not get Faculties"))
}

#[cfg(debug_assertions)]
#[get("/debug/lfu/{id}")]
async fn debug_get_object(path: web::Path<usize>) -> actix_web::Result<String> {
    let id = path.into_inner();
    if let Ok(document) = make_request(format_lfu_object_url(id)).await {
        return Ok(document);
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Could not find Objects").into())
}

#[get("/lfu/{id}")]
async fn get_object(path: web::Path<usize>) -> web::Json<Response<Objects, &'static str>> {
    let id = path.into_inner();
    if let Ok(document) = get_html_document(format_lfu_object_url(id)).await {
        return match get_html_objects(document, id) {
            Some(Objects::Object(objects)) => {
                web::Json(Response::success(Objects::Object(objects)))
            }
            Some(Objects::CourseDetails(details)) => {
                web::Json(Response::success(Objects::CourseDetails(details)))
            }
            _ => web::Json(Response::error("Server did not send correct Type")),
        };
    }
    web::Json(Response::error("Server did not send correct Type"))
}

#[cfg(debug_assertions)]
#[get("/debug/course/{id}")]
async fn debug_get_course(path: web::Path<usize>) -> actix_web::Result<String> {
    let id = path.into_inner();
    if let Ok(document) = make_request(format_course_url(id)).await {
        return Ok(document);
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Could not find Objects").into())
}

#[get("/course/{id}")]
async fn get_course(path: web::Path<usize>) -> web::Json<Response<Course, &'static str>> {
    let id = path.into_inner();
    if let Ok(document) = get_html_document(format_course_url(id)).await {
        // TODO: Parse all Tables
        if let Some(time_table) = document.select(&TABLE_SELECTOR).next() {
            let mut rows = time_table.select(&TR_SELECTOR);
            // Skip the Header Row
            let rows = rows.skip(2);
            return web::Json(Response::success(Course {
                id,
                times: rows
                    // TODO: Put into seperate Function
                    .map(|e| {
                        let mut e = e.select(&TD_SELECTOR);
                        if let (
                            Some(date),
                            Some(duration),
                            Some(location),
                            Some(_),
                            Some(comment),
                        ) = (e.next(), e.next(), e.next(), e.next(), e.next())
                        {
                            CourseDate {
                                date: date
                                    .text()
                                    .map(|e| e.split(' ').nth(1).unwrap_or("").trim())
                                    .collect(),
                                time: duration.text().map(|e| e.trim()).collect(),
                                location: location
                                    .select(&A_SELECTOR)
                                    .next()
                                    .unwrap()
                                    .text()
                                    .map(|e| e.trim())
                                    .collect(),
                                comment: comment.text().map(|e| e.trim()).collect(),
                            }
                        } else {
                            CourseDate {
                                date: String::new(),
                                time: String::new(),
                                location: String::new(),
                                comment: String::new(),
                            }
                        }
                    })
                    .collect(),
            }));
        }
    }
    web::Json(Response::error("Could not get Course"))
}

#[derive(Serialize)]
struct CourseDetail {
    id: usize,
    lv_number: usize,
    name: String,
    lecturers: String,
}

#[derive(Serialize)]
#[serde(tag = "type", content = "data")]
enum Objects {
    Object(Vec<LfuObject>),
    CourseDetails(Vec<CourseDetail>),
}

fn get_html_objects(document: Html, id: usize) -> Option<Objects> {
    let mut elements = document.select(&XNODE_SELECTOR);
    let objects: Vec<LfuObject> = elements
        .map(|e| LfuObject {
            id: str::parse::<usize>(e.value().attr("data-id").unwrap_or("0")).unwrap_or(0),
            name: e.text().map(|e| e.trim()).collect(),
        })
        .collect();
    if (!objects.is_empty()) {
        return Some(Objects::Object(objects));
    }
    let mut elements = document.select(&LV_ROW_SELECTOR);
    let objects: Vec<CourseDetail> = elements
        .map(|e| {
            let title = e.select(&LV_TITLE_SELECTOR).next().unwrap();
            let title_text = title
                .select(&LV_TITLE_TEXT_SELECTOR)
                .next()
                .unwrap()
                .text()
                .collect();
            let lecturers = String::from(title.text().nth(1).unwrap());
            CourseDetail {
                id: e
                    .select(&LV_DETAILS_SELECTOR)
                    .next()
                    .unwrap()
                    .value()
                    .attr("onclick")
                    .unwrap()
                    .split(',')
                    .nth(1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                lv_number: e
                    .select(&LV_NO_SELECTOR)
                    .next()
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
                name: title_text,
                lecturers,
            }
        })
        .collect();
    if (!objects.is_empty()) {
        return Some(Objects::CourseDetails(objects));
    }
    None
}

async fn get_html_document<U>(url: U) -> Result<Html, ()>
where
    Uri: TryFrom<U>,
    <Uri as TryFrom<U>>::Error: Into<HttpError>,
{
    if let Ok(body) = make_request(url).await {
        Ok(Html::parse_document(&body))
    } else {
        Err(())
    }
}

async fn make_request<U>(url: U) -> Result<String, ()>
where
    Uri: TryFrom<U>,
    <Uri as TryFrom<U>>::Error: Into<HttpError>,
{
    if let Ok(mut response) = Client::default().get(url).send().await {
        if let Ok(mut body) = response.body().limit(usize::MAX).await {
            return Ok(String::from_utf8_lossy(body.as_ref()).into_owned());
        }
    }
    Err(())
}
