mod fetchdata;
use fetchdata::fetch_data_and_return_percentage_variations;

mod soundplay;
use soundplay::play_audio;

use actix_files::NamedFile;
use actix_web::web::Data;
use actix_web::{web, App, HttpRequest, HttpResponse, Result};
use std::path::PathBuf;
use tera::Tera;

use chrono::NaiveDate;

use std::sync::Mutex;

struct AppStateWithData {
    dates: Mutex<Vec<String>>,
    values: Mutex<Vec<String>>,
}

async fn play(data: web::Data<AppStateWithData>, month: u32) {
    let percentage_variations = fetch_data_and_return_percentage_variations(month)
        .await
        .unwrap();

    let mut dates = Vec::new();
    let mut values = Vec::new();

    let mut index = 29;

    for variation in percentage_variations {
        dates.push(variation.date.clone());
        values.push(variation.percentage_variation.to_string());

        index = (index as i64 + variation.percentage_variation).rem_euclid(63) as usize;
        play_audio(index).unwrap();
        println!(
            "Date: {}, Variation: {}%, Index: {}",
            variation.date, variation.percentage_variation, index
        );
    }

    let mut data_dates = data.dates.lock().unwrap();
    let mut data_values = data.values.lock().unwrap();
    *data_dates = dates;
    *data_values = values;
}

async fn index(data: web::Data<AppStateWithData>, tera: web::Data<Tera>) -> Result<HttpResponse> {
    let dates = data.dates.lock().unwrap();
    let values = data.values.lock().unwrap();

    let mut context = tera::Context::new();
    context.insert("dates", &*dates);
    context.insert("values", &*values);

    let rendered = tera.render("index.html", &context).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let tera = Tera::new("templates/**/*").unwrap();
    let app_state = Data::new(AppStateWithData {
        dates: Mutex::new(Vec::new()),
        values: Mutex::new(Vec::new())
    });

    play(app_state.clone(), 11).await;

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .app_data(Data::new(tera.clone()))
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
