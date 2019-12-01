mod api;
mod parse;
use uuid::Uuid;
extern crate reqwest;

fn lookup_previous_station<'a>(
    line_stations: &[&'static str; 4],
    data: &'a Vec<parse::StationData>,
    current_station: &parse::StationData,
) -> Option<&'a parse::StationData> {

    let current_station_index = line_stations.iter().position(|&r| r == current_station.location).unwrap();
    let next_station_lookup_direction;

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
            let mut data = parse::parse(response);

            let eccles_line_stations: [&'static str; 4] = ["Eccles", "Ladywell", "Weaste", "Langworthy"];
            let trams_in_existence: Vec<Uuid> = Vec::new(); // holds all trams in existence using our own generated tram ids

            // Loop through the data to get all train data
            for current_station in data.iter() {
                if eccles_line_stations.contains(&&*current_station.location) {
                    // Backtrace to find the trams current position, while backtracing remove that tram
                    // println!("Platform: {} ({:#?})-> {:#?}", current_station.location, current_station.direction, current_station.train_data);
                    let uuid = Uuid::new_v4();

                    if current_station.train_data.len() != 0 {
                        let previous_station = lookup_previous_station(&eccles_line_stations, &data, &current_station);

                        match previous_station {
                            Some(previous_station) => {
                                // current_station.train_data[0].train_id = Some(uuid);

                                // Get immediate previous station data, if wait time is of that is less than this, then the train is between these 2 stations
                                if current_station.train_data[0].estimated_wait_time < previous_station.train_data[0].estimated_wait_time {
                                    println!("there is a train between {} -> {} heading to the {} direction", previous_station.location, current_station.location, current_station.train_data.get(0).unwrap().destination);
                                }

                                        
                                // println!("{:#?} prev ", previous_station);
                                println!("------------------ ");
                            },
                            None => {
                                // println!("No more stations")
                            }
                        }

                        // lookup_previous_station(&line_stations, data, next_station_data, next_station_lookup_direction);

                    }
                    
                }
            }
        },
        Err(err) => api::handler(err)
    }
}
