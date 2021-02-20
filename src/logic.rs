use crate::parse;
use crate::parse::StationHelper;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Tram {
    pub next_station: String,
    pub previous_station: String,
    pub metadata: parse::ApproachingTram,
}

pub struct Line {
    pub stations: Vec<String>
}

// Takes the current station, and previous station
// Returns back all trams in between these 2 stations
// This is possible as the previous station has information on approaching trams to itself
pub fn get_trams_between_current_and_previous_station(
    current_station: &parse::StationData,
    previous_station: &parse::StationData
) -> Option<Vec<Tram>> {
    let matched_prev_station_train_indexes: Vec<usize> = Vec::new();
    let mut trams: Vec<Tram> = Vec::new();
    let mut last_search_was_skipped = false;

    // Compare each train and match them appropriately (using same dest, carriage etc.. basically same tram meta data)
    for current_train in current_station.approaching_trams.iter() {
        // println!("current st train: {:#?} {:#?} {:#?}, prev stat data: {:#?}", current_station.location, current_station.direction, current_train, previous_station);
        
        // No trams found at previous station..
        // 2 possibilities
        // -- trams are backed up to the previous stations station
        // -- night time, no more trams (most likely)
        if previous_station.approaching_trams.len() == 0 {
            trams.push(Tram {
                next_station: current_station.location.clone(),
                previous_station: previous_station.location.clone(),
                metadata: current_train.clone()
            });
            break;
        } else {
            // println!("prev station train possibilities\n--------");
            for (i, prev_train) in previous_station.approaching_trams.iter().enumerate() {
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
                    trams.push(Tram {
                        next_station: current_station.location.clone(),
                        previous_station: previous_station.location.clone(),
                        metadata: current_train.clone()
                    });
                }

                break;
            }

            // We ran out of trams.. probably was night time and no more to come
            if last_search_was_skipped {
                println!("ran out of trams here, likely it's night time");
                trams.push(Tram {
                    next_station: current_station.location.clone(),
                    previous_station: previous_station.location.clone(),
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

pub fn locate_trams_for_line(
    response_data: Vec<parse::StationData>,
    line: &Line
) -> Vec<Tram> {
    let mut trams_between_stations: Vec<Tram> = Vec::new();

     for station in response_data.iter() {
        if line.stations.contains(&station.location) {
            if station.approaching_trams.len() != 0 {
                let previous_station = station.get_previous_station(
                    &response_data,
                    &line.stations,
                );

                match previous_station {
                    Some(previous_station) => {
                        let trams = get_trams_between_current_and_previous_station(
                            &station,
                            &previous_station
                        );

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