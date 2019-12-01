#[cfg(test)]
extern crate metrolib;

mod tests {
    fn setup_station() -> metrolib::parse::StationData {
        metrolib::parse::StationData {
            id: 830,
            line: "ChangeMe".to_string(),
            tla_ref: "LDW".to_string(),
            pid_ref: "LDW-TPTD01".to_string(),
            location: "Ladywell".to_string(),
            atco_code: "9400ZZMALDY2".to_string(),
            direction: metrolib::parse::Direction::Outgoing,
            train_data: vec![],
            message_board: "None".to_string(),
            last_updated: "2019-12-01T16:11:20Z".to_string(),
        }
    }

    #[test]
    fn test_one_incoming_but_none_previous() {
        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            train_data: vec![
                metrolib::parse::TrainData {
                    destination: "Eccles".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 1,
                }
            ],
            ..setup_station()
        };

        let previous_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            train_data: vec![],
            ..setup_station()
        };

        let is_tram_coming = metrolib::logic::get_trams_between_stations(&current_station, &previous_station);
        let expected_res = vec![
            metrolib::logic::TramDetails {
                destination: current_station.train_data[0].destination.clone(),
                carriages: current_station.train_data[0].carriages.clone()
            }
        ];
        assert_eq!(is_tram_coming, Some(expected_res));
    }

    #[test]
    fn test_one_incoming_but_one_previous_exists() {
        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            train_data: vec![
                metrolib::parse::TrainData {
                    destination: "MediaCityUK".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 1,
                }
            ],
            ..setup_station()
        };

        let previous_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            train_data: vec![
                metrolib::parse::TrainData {
                    destination: "MediaCityUK".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 5,
                }
            ],
            ..setup_station()
        };

        let is_tram_coming = metrolib::logic::get_trams_between_stations(&current_station, &previous_station);
        let expected_res = vec![
            metrolib::logic::TramDetails {
                destination: current_station.train_data[0].destination.clone(),
                carriages: current_station.train_data[0].carriages.clone()
            }
        ];
        assert_eq!(is_tram_coming, Some(expected_res));
    }

    #[test]
    fn test_one_exists_but_one_previous_incoming() {
        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            train_data: vec![
                metrolib::parse::TrainData {
                    destination: "MediaCityUK".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 5,
                }
            ],
            ..setup_station()
        };

        let previous_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            train_data: vec![
                metrolib::parse::TrainData {
                    destination: "MediaCityUK".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 1,
                }
            ],
            ..setup_station()
        };

        let is_tram_coming = metrolib::logic::get_trams_between_stations(&current_station, &previous_station);
        assert_eq!(is_tram_coming, None);
    }

}