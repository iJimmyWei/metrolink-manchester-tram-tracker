use crate::parse;

// Returns a string of destination names if exists
pub fn get_trams_between_stations(
    current_station: &parse::StationData,
    previous_station: &parse::StationData
) -> Option<Vec<parse::TrainData>> {
    let mut trams_between_station: Vec<parse::TrainData> = Vec::new();
    let matched_prev_station_train_indexes: Vec<usize> = Vec::new();

    let mut last_search_was_skipped = false;

    // Compare each train and match them appropriately (using same dest, carriage etc.. basically same tram meta data)
    for current_train in current_station.train_data.iter() {
        // println!("current st train: {:#?} {:#?} {:#?}, prev stat data: {:#?}", current_station.location, current_station.direction, current_train, previous_station);
        
        // No trams found at previous station..
        // 2 possibilities
        // -- trams are backed up to the previous stations station
        // -- night time, no more trams (most likely)
        if previous_station.train_data.len() == 0 {
            trams_between_station.push(
                current_train.clone()
            );
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
                    println!("there is a train between {:#?} and {:#?} with times {} and {} heading towards {} (prev t dest: {})",
                        current_station.location, previous_station.location,
                        current_train.estimated_wait_time, prev_train.estimated_wait_time, current_train.destination, prev_train.destination);
                    trams_between_station.push(
                        current_train.clone()
                    );
                }

                break;
            }

            // We ran out of trams.. probably was night time and no more to come
            if last_search_was_skipped {
                println!("ran out of trams here, likely it's night time");
                trams_between_station.push(
                    current_train.clone()
                );
            }
        }
    }
    
    if trams_between_station.len() > 0 {
        return Some(trams_between_station)
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
    let mut next_station_lookup_direction;
    
    // Logic to determine which direction of that platform we need to look at
    // End of station circuit, flip direction so we can continue traversing the stations
    if current_station_index == 0 || current_station_index == line_stations.len() {
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
            if *name == "MediaCityUK" || *name == "Ashton-Under-Lyne" {
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