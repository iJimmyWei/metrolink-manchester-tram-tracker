mod api;
mod parse;
mod logic;
extern crate reqwest;

fn lookup_previous_station<'a>(
    line_stations: &[&'static str; 9],
    data: &'a Vec<parse::StationData>,
    current_station: &parse::StationData,
) -> Option<&'a parse::StationData> {

    let current_station_index = line_stations.iter().position(|&r| r == current_station.location).unwrap();
    let mut next_station_lookup_direction;

    // Logic to determine which direction of that platform we need to look at
    // End of station circuit, flip direction so we can continue traversing the stations
    if current_station_index == 0 {
        if current_station.direction == parse::Direction::Incoming {
            next_station_lookup_direction = parse::Direction::Outgoing;
        } else {
            next_station_lookup_direction = parse::Direction::Incoming;
        }
    } else {
        // Defaults lookup to the direction of travel
        match current_station.direction {
            parse::Direction::Incoming => next_station_lookup_direction = parse::Direction::Incoming,
            _ => next_station_lookup_direction = parse::Direction::Outgoing
        }
    }
    
    // Get station to lookup (always opposite of the direction)
    let next_station_lookup_name: Option<&&str>;
    if next_station_lookup_direction == parse::Direction::Incoming {
        next_station_lookup_name = line_stations.get(current_station_index + 1);
    } else {
        next_station_lookup_name = line_stations.get(current_station_index - 1);
    }

    match next_station_lookup_name {
        // Select the train data with the given station name
        Some(name) => {
            // Some stations only go "outwards"
            if *name == "MediaCityUK" {
                next_station_lookup_direction = parse::Direction::Outgoing
            };

            let next_station_data = data.iter().find(
                |&d| d.location == *name && d.direction == next_station_lookup_direction
            ).unwrap();

            Some(next_station_data)
        },
        _ => None
    }
}

fn main() {
    let client = reqwest::Client::new();
    let res = api::get(client);

    match res {
        Ok(response) => {
            // Parse the response to something we can use
            let data = parse::parse(response);

            let eccles_line_stations: [&'static str; 9] = ["Eccles", "Ladywell", "Weaste", "Langworthy", "Broadway", "MediaCityUK", "Harbour City", "Anchorage", "Exchange Quay"];
            
            // Loop through the data to get all train data
            for current_station in data.iter() {
                if eccles_line_stations.contains(&&*current_station.location) {
                    // let is_end_of_circuit = (eccles_line_stations.iter().position(|&r| r == current_station.location).unwrap()) == 0;

                    // // Backtrace to find the trams current position, while backtracing remove that tram
                    // let uuid = Uuid::new_v4();

                    if current_station.train_data.len() != 0 {
                        let previous_station = lookup_previous_station(&eccles_line_stations, &data, &current_station);

                        match previous_station {
                            Some(previous_station) => {
                                let trams = logic::get_trams_between_stations(&current_station, &previous_station);
                                match trams {
                                    Some(trams) => println!("{:#?}", trams),
                                    _ => {}
                                }
                            },
                            None => {
                                // println!("No more stations")
                            }
                        }
                    }
                    
                }
            }
        },
        Err(err) => api::handler(err)
    }
}
