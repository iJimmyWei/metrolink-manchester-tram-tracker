mod api;
mod parse;
use uuid::Uuid;
extern crate reqwest;

fn lookup_previous_station(
    line_stations: &[&'static str; 3],
    data: &Vec<parse::StationData>,
    current_station: &parse::StationData,
    direction: &parse::Direction,
) {
    // Need to find the next forward station that is incoming
    let current_station_index = line_stations.iter().position(|&r| r == current_station.location).unwrap();
    let next_station_lookup_name: Option<&&str>;
    
    let mut next_station_lookup_direction = &*direction;

    // Reached end of station, flip direction as it's relative to that end of station
    if current_station_index == 0 {
        if next_station_lookup_direction == &parse::Direction::Incoming {
            next_station_lookup_direction = &parse::Direction::Outgoing;
        } else {
            next_station_lookup_direction = &parse::Direction::Incoming;
        }
    }

    if next_station_lookup_direction == &parse::Direction::Incoming {
        next_station_lookup_name = line_stations.get(current_station_index + 1);
    } else {
        next_station_lookup_name = line_stations.get(current_station_index - 1);
    }

    match next_station_lookup_name {
        // Select the train data with the given station name
        Some(name) => {
            let next_station_data = &data.iter().find(|&d| d.location == *name && d.direction == *next_station_lookup_direction);

            match next_station_data {
                Some(station_data) => {
                    println!("found trains {:#?} {:#?}", *name, &station_data.train_data);
        
                    // Get immediate previous station data, if wait time is of that is less than this, then the train is between these 2 stations
                    if current_station.train_data.get(0).unwrap().estimated_wait_time < station_data.train_data.get(0).unwrap().estimated_wait_time {
                        println!("there is a train between {} -> {} heading to the {} direction", station_data.location, current_station.location, current_station.train_data.get(0).unwrap().destination);
                    }
        
                    lookup_previous_station(&line_stations, &data, &station_data, &next_station_lookup_direction);
                }
                _ => println!("error handling {:#?}", next_station_lookup_name)
            }

        },
        _ => println!("no station name found, out of index")
    }
}

fn main() {
    let client = reqwest::Client::new();
    let res = api::get(client);

    match res {
        Ok(response) => {
            // Parse the response to something we can use
            let mut data = parse::parse(response);

            let eccles_line_stations: [&'static str; 3] = ["Eccles", "Ladywell", "Weaste"];
            let trams_in_existence: Vec<Uuid> = Vec::new(); // holds all trams in existence using our own generated tram ids

            // Loop through the data to get all train data
            for d in &data {
                // Only stations on the eccles line
                let uuid = Uuid::new_v4();

                if eccles_line_stations.contains(&&*d.location) {
                    // Backtrace to find the trams current position, while backtracing remove that tram
                    println!("Platform: {} ({:#?})-> {:#?}", d.location, d.direction, d.train_data);

                    if d.train_data.len() != 0 {
                        lookup_previous_station(&eccles_line_stations, &data, &d, &d.direction);

                        println!("------------------");
                    }
                    
                }
            }
        },
        Err(err) => api::handler(err)
    }
}
