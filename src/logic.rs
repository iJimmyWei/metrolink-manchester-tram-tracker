use crate::parse;

#[derive(Debug, PartialEq)]
pub struct TramBetweenStation {
    pub station_1: String,
    pub station_2: String,
    pub metadata: parse::TrainData,
}

// Returns a string of destination names if exists
pub fn get_trams_between_stations(
    current_station: &parse::StationData,
    previous_station: &parse::StationData
) -> Option<Vec<TramBetweenStation>> {
    let matched_prev_station_train_indexes: Vec<usize> = Vec::new();
    let mut trams: Vec<TramBetweenStation> = Vec::new();
    let mut last_search_was_skipped = false;

    // Compare each train and match them appropriately (using same dest, carriage etc.. basically same tram meta data)
    for current_train in current_station.train_data.iter() {
        // println!("current st train: {:#?} {:#?} {:#?}, prev stat data: {:#?}", current_station.location, current_station.direction, current_train, previous_station);
        
        // No trams found at previous station..
        // 2 possibilities
        // -- trams are backed up to the previous stations station
        // -- night time, no more trams (most likely)
        if previous_station.train_data.len() == 0 {
            trams.push(TramBetweenStation {
                station_1: current_station.location.clone(),
                station_2: previous_station.location.clone(),
                metadata: current_train.clone()
            });
            break;
        } else {
            // println!("prev station train possibilities\n--------");
            for (i, prev_train) in previous_station.train_data.iter().enumerate() {
                last_search_was_skipped = false;
                // println!("prev st train: {:#?}", prev_train);

                // Ensure train meta data matches
                // if !is_end_of_circuit &&
                //     (current_train.destination != "See Tram Front" && prev_train.destination != "See Tram Front"
                //         && current_train.destination != prev_train.destination)
                
                //     // || current_train.carriages != prev_train.carriages

                if current_station.location != "Eccles" &&
                current_train.destination != prev_train.destination
                {
                    if current_station.location != "MediaCityUK" && (current_train.destination != "Ashton-Under-Lyne" && prev_train.destination != "Ashton via MediaCityUK") {
                        // println!("train meta data doesn't match, skipping to next train, {:#?} {:#?}", current_train.destination, prev_train.destination);
                        continue;
                    }
                }
                

                if matched_prev_station_train_indexes.contains(&i) {
                    // println!("match already found for this train, skipping to next train");
                    last_search_was_skipped = true;
                    continue;
                }

                // Is the tram here inbetween these 2 stations?
                if current_train.estimated_wait_time < prev_train.estimated_wait_time {
                    // println!("there is a train between {:#?} and {:#?} with times {} and {} heading towards {} (prev t dest: {})",
                    //     current_station.location, previous_station.location,
                    //     current_train.estimated_wait_time, prev_train.estimated_wait_time, current_train.destination, prev_train.destination);
                    trams.push(TramBetweenStation {
                        station_1: current_station.location.clone(),
                        station_2: previous_station.location.clone(),
                        metadata: current_train.clone()
                    });
                }

                break;
            }

            // We ran out of trams.. probably was night time and no more to come
            if last_search_was_skipped {
                println!("ran out of trams here, likely it's night time");
                trams.push(TramBetweenStation {
                    station_1: current_station.location.clone(),
                    station_2: previous_station.location.clone(),
                    metadata: current_train.clone()
                });
            }
        }
    }
    
    if trams.len() > 0 {
        return Some(trams)
    } else {
        return None
    }
}

// Gets the previous station's data given a station within a line 
pub fn lookup_previous_station<'a>(
    line_stations: &[&'static str],
    data: &'a Vec<parse::StationData>,
    current_station: &parse::StationData,
) -> Option<&'a parse::StationData> {
    let current_station_index = line_stations.iter().position(|&r| r == current_station.location).unwrap();
    let mut previous_station_direction;
    
    // Logic to determine which direction of that platform we need to look at
    // End of station circuit, flip direction so we can continue traversing the stations
    if current_station_index == 0 || current_station_index == line_stations.len() {
        if current_station.direction == parse::Direction::Incoming {
            previous_station_direction = parse::Direction::Outgoing;
        } else {
            previous_station_direction = parse::Direction::Incoming;
        }
    } else {
        // Defaults lookup to the direction of travel
        match current_station.direction {
            parse::Direction::Incoming => previous_station_direction = parse::Direction::Incoming,
            _ => previous_station_direction = parse::Direction::Outgoing
        }
    }
    
    // Get station to lookup (always opposite of the direction)
    let previous_station_name: Option<&&str>;
    if previous_station_direction == parse::Direction::Incoming {
        previous_station_name = line_stations.get(current_station_index + 1);
    } else {
        previous_station_name = line_stations.get(current_station_index - 1);
    }

    match previous_station_name {
        // Select the train data with the given station name
        Some(name) => {
            // Some stations only go "outwards"
            if *name == "MediaCityUK" || *name == "Ashton-Under-Lyne" {
                previous_station_direction = parse::Direction::Outgoing
            };

            let next_station_data = data.iter().find(
                |&d| d.location == *name && d.direction == previous_station_direction
            ).unwrap();

            Some(next_station_data)
        },
        _ => None
    }
}

pub fn locate_trams(response_data: Vec<parse::StationData>, line_stations: &[&'static str]) -> Vec<TramBetweenStation> {
    let mut trams_between_stations: Vec<TramBetweenStation> = Vec::new();

     // Loop through the data to get all train data
     for station in response_data.iter() {
        if line_stations.contains(&&*station.location) {
            if station.train_data.len() != 0 {
                let previous_station = lookup_previous_station(&line_stations, &response_data, &station);

                match previous_station {
                    Some(previous_station) => {
                        let trams = get_trams_between_stations(&station, &previous_station);
                        match trams {
                            Some(trams) => {
                                for tram in trams {
                                    trams_between_stations.push(tram)
                                }
                            },
                            _ => {} // No trams found
                        }
                    },
                    _ => {} // No previous station (station sharing multiple lines)
                }
            }
            
        }
    }

    trams_between_stations
}