mod api;
mod parse;
mod logic;
extern crate reqwest;
use crate::logic::Line;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct ResponseDto {
    tram_count: usize,
    trams: Vec<logic::Tram>,
}

fn trams(data: web::Data<Arc<Mutex<Vec<logic::Tram>>>>) -> impl Responder {
    let guard = &**data;
    let json = guard.lock().unwrap();
    let body = &*json;
    // println!("actix_data: {:?}", );
    HttpResponse::Ok().json(ResponseDto {
        tram_count: body.len(),
        trams: body.clone()
    })
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn main() {
    let global_data: Mutex<Vec<logic::Tram>> = Mutex::new(Vec::new());
    let arc = Arc::new(global_data);
    let arc2 = arc.clone();

    let eccles_line = Line {
        stations: vec_of_strings![
            "Eccles", "Ladywell", "Weaste", "Langworthy", "Broadway",
            "MediaCityUK", "Harbour City", "Anchorage", "Exchange Quay",
            "Pomona", "Cornbrook", "Deansgate - Castlefield", "St Peter's Square",
            "Piccadilly Gardens", "Piccadilly", "New Islington", "Holt Town",
            "Etihad Campus", "Velopark", "Clayton Hall", "Edge Lane",
            "Cemetery Road", "Droylsden", "Audenshaw", "Ashton Moss",
            "Ashton West", "Ashton-Under-Lyne"
        ]
    };

    // Metrolink data fetching & parsing thread
    thread::spawn(move || {
        loop {
            let client = reqwest::Client::new();
            let res = api::get(client);
            
            match res {
                Ok(response) => {
                    let response_data = parse::parse(response);
                        
                    let mut trams_between_stations = logic::locate_trams_for_line(
                        response_data,
                        &eccles_line
                    );

                    trams_between_stations.dedup();

                    // Send to rx
                    // tx.send(trams_between_stations).unwrap();

                    let mut guard = arc.lock().unwrap();
                    *guard = trams_between_stations;
                    
                    // println!("{:#?}", global_data);
                },
                Err(err) => api::handler(err)
            }

            thread::sleep(Duration::from_secs(5));
        }
    });

    // Web API
    HttpServer::new(move || {
        App::new()
            .data(arc2.clone())
            .service(
                web::resource("/").route(
                    web::get().to(trams)
                )
            )
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();

    thread::park();
}
