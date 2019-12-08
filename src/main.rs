mod api;
mod parse;
mod logic;
extern crate reqwest;
use std::thread;
use std::sync::{Arc, RwLock, Mutex};
use std::time::Duration;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn trams(data: web::Data<Arc<Mutex<Vec<logic::TramBetweenStation>>>>) -> impl Responder {
    let guard = &**data;
    let json = guard.lock().unwrap();
    let body = &*json;
    // println!("actix_data: {:?}", );
    HttpResponse::Ok().json(body)
}

fn main() {
    let global_data: Mutex<Vec<logic::TramBetweenStation>> = Mutex::new(Vec::new());
    let arc = Arc::new(global_data);
    let arc2 = arc.clone();

    // Metrolink data fetching & parsing thread
    thread::spawn(move || {
        let client = reqwest::Client::new();
        loop {
            let res = api::get(&client);
            println!("New data loaded!");
            
            match res {
                Ok(response) => {
                    // Parse the response to something we can use
                    let response_data = parse::parse(response);
                    let eccles_line_stations: [&'static str; 27] = [
                        "Eccles", "Ladywell", "Weaste", "Langworthy", "Broadway",
                        "MediaCityUK", "Harbour City", "Anchorage", "Exchange Quay",
                        "Pomona", "Cornbrook", "Deansgate - Castlefield", "St Peter's Square",
                        "Piccadilly Gardens", "Piccadilly", "New Islington", "Holt Town",
                        "Etihad Campus", "Velopark", "Clayton Hall", "Edge Lane",
                        "Cemetery Road", "Droylsden", "Audenshaw", "Ashton Moss",
                        "Ashton West", "Ashton-Under-Lyne"];
                        
                        let mut trams_between_stations = logic::locate_trams(response_data, &eccles_line_stations);
                        
                        trams_between_stations.dedup();
                        // println!("Trams Count: {:?} {:#?}", &trams_between_stations.len(), trams_between_stations);

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
            // Store `MyData` in application storage.
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
