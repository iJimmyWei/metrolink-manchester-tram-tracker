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

trait LineHelper {
    fn get_end_stations(&self) -> Vec<String>;
}

impl LineHelper for Line {
    fn get_end_stations(&self) -> Vec<String> {
        let first_station = self.stations.first().unwrap();
        let last_station = self.stations.last().unwrap();

        let mut stations = Vec::new();
        stations.push(first_station.to_lowercase());
        stations.push(last_station.to_lowercase());

        assert_eq!(stations.len(), 2);

        stations
    }
}

// Takes the current station, and previous station
// Returns back all trams in between these 2 stations
// This is possible as the previous station has information on approaching trams to itself
pub fn get_trams_between_current_and_previous_station(
    current_station: &parse::StationData,
    previous_station: &parse::StationData
) -> Option<Vec<Tram>> {
    let mut trams: Vec<Tram> = Vec::new();
    let mut previous_station_has_approaching_trams = true;

    // Compare each train and match them appropriately
    // (using same dest, carriage etc.. basically same tram meta data)
    for current_tram in current_station.approaching_trams.iter() {
        // println!("current st train: {:#?} {:#?} {:#?}, prev stat data: {:#?}", current_station.location, current_station.direction, current_tram, previous_station);
        
        // No approaching trams found at previous station
        // 2 possibilities
        // -- trams are backed up to the previous stations station
        // -- night time, less trams in circulation
        if previous_station.approaching_trams.len() == 0 {
            previous_station_has_approaching_trams = false;
        }

        // println!("prev station train possibilities\n--------");
        for (_index, previous_tram) in previous_station.approaching_trams.iter().enumerate() {
            // println!("prev st train: {:#?}", prev_train);

            // Ensure train meta data matches
            // if !is_end_of_circuit &&
            //     (current_tram.destination != "See Tram Front" && prev_train.destination != "See Tram Front"
            //         && current_tram.destination != prev_train.destination)
            
            //     // || current_tram.carriages != prev_train.carriages

            if current_station.location != "Eccles" &&
                current_tram.destination != previous_tram.destination {
                if current_station.location != "MediaCityUK" &&
                    (
                        current_tram.destination != "Ashton-Under-Lyne" &&
                        previous_tram.destination != "Ashton via MediaCityUK"
                    )
                {
                    // println!("train meta data doesn't match, skipping to next train, {:#?} {:#?}", current_tram.destination, previous_tram.destination);
                    continue;
                }
            }
            
            // Is the tram here inbetween these 2 stations?
            if current_tram.estimated_wait_time < previous_tram.estimated_wait_time {
                // println!("there is a train between {:#?} and {:#?} with times {} and {} heading towards {} (prev t dest: {})",
                //     current_station.location, previous_station.location,
                //     current_tram.estimated_wait_time, prev_train.estimated_wait_time, current_tram.destination, prev_train.destination);
                trams.push(Tram {
                    next_station: current_station.location.clone(),
                    previous_station: previous_station.location.clone(),
                    metadata: current_tram.clone()
                });
            }

            break;
        }

        // We've got a tram coming, but no more detected from previous approaching ones
        if !previous_station_has_approaching_trams {
            trams.push(Tram {
                next_station: current_station.location.clone(),
                previous_station: previous_station.location.clone(),
                metadata: current_tram.clone()
            });
        }
    }
    
    if trams.len() == 0 {
        return None
    }

    return Some(trams)
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
                                let end_stations = line.get_end_stations();

                                // Filter out the trams to only include ones for our line
                                let trams_only_match_line = trams
                                    .into_iter()
                                    .filter(|d| end_stations.contains(
                                        &d.metadata.destination.to_lowercase()
                                    ))
                                    .collect::<Vec<Tram>>();

                                trams_between_stations.extend(trams_only_match_line)
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