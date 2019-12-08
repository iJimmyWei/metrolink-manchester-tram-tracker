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

            let trams_between_stations = logic::locate_trams(response_data, &eccles_line_stations);
            println!("Trams!: {:#?}", trams_between_stations);
        },
        Err(err) => api::handler(err)
    }
}
