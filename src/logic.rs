use crate::parse;

#[derive(Debug, PartialEq)]
pub struct TramDetails {
    pub destination: String,
    pub carriages: parse::Carriages,
}

// Returns a string of destination names if exists
pub fn get_trams_between_stations(
    current_station: &parse::StationData,
    previous_station: &parse::StationData
) -> Option<Vec<TramDetails>> {
    let mut trams_between_station: Vec<TramDetails> = Vec::new();
    let mut matched_prev_station_train_indexes: Vec<usize> = Vec::new();

    // Compare each train and match them appropriately (using same dest, carriage etc.. basically same tram meta data)
    for current_train in current_station.train_data.iter() {
        println!("current st train: {:#?} {:#?} {:#?}, prev stat data: {:#?}", current_station.location, current_station.direction, current_train, previous_station);
        
        // No trams found at previous station..
        // 2 possibilities
        // -- trams are backed up to the previous stations station
        // -- night time, no more trams (most likely)
        if previous_station.train_data.len() == 0 {
            trams_between_station.push(
                TramDetails {
                    destination: current_train.destination.clone(),
                    carriages: current_train.carriages.clone()
                }
            );
            break;
        }

        // println!("prev station train possibilities\n--------");
        for (i, prev_train) in previous_station.train_data.iter().enumerate() {
            println!("prev st train: {:#?}", prev_train);


            // Ensure train meta data matches
            // if !is_end_of_circuit &&
            //     (current_train.destination != "See Tram Front" && prev_train.destination != "See Tram Front"
            //         && current_train.destination != prev_train.destination)
            
            //     // || current_train.carriages != prev_train.carriages
            // {
            //     println!("train meta data doesn't match, skipping to next train, {:#?} {:#?}", current_train.destination, prev_train.destination);
            //     continue;
            // }

            if matched_prev_station_train_indexes.contains(&i) {
                // println!("match already found for this train, skipping to next train");
                continue;
            }

            matched_prev_station_train_indexes.push(i);

            // Is the tram here inbetween these 2 stations?
            if current_train.estimated_wait_time < previous_station.train_data[i].estimated_wait_time {
                println!("there is a train between {} and {} heading towards {}", current_station.location, previous_station.location, current_train.destination);
                trams_between_station.push(
                    TramDetails {
                        destination: current_train.destination.clone(),
                        carriages: current_train.carriages.clone()
                    }
                );
            } else {
                break;
            }
        }
        
        // println!("-----------------------eo train poss");
    }
    
    if trams_between_station.len() > 0 {
        return Some(trams_between_station)
    } else {
        return None
    }

}