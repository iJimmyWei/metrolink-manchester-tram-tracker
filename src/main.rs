mod api;
mod parse;
mod logic;
extern crate reqwest;

fn main() {
    let client = reqwest::Client::new();
    let res = api::get(client);

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
            
            // Loop through the data to get all train data
            for station in response_data.iter() {
                if eccles_line_stations.contains(&&*station.location) {
                    if station.train_data.len() != 0 {
                        let previous_station = logic::lookup_previous_station(&eccles_line_stations, &response_data, &station);

                        match previous_station {
                            Some(previous_station) => {
                                let trams = logic::get_trams_between_stations(&station, &previous_station);
                                match trams {
                                    Some(trams) => println!("{:#?}", trams),
                                    _ => {} // No trams found
                                }
                            },
                            _ => {} // No previous station (station sharing multiple lines)
                        }
                    }
                    
                }
            }
        },
        Err(err) => api::handler(err)
    }
}
