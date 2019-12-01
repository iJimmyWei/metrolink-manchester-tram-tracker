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
        let current_station_train_data = vec![
            metrolib::parse::TrainData {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 1,
            }
        ];

        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            train_data: current_station_train_data.clone(),
            ..setup_station()
        };

        let previous_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            train_data: vec![],
            ..setup_station()
        };

        let is_tram_coming = metrolib::logic::get_trams_between_stations(&current_station, &previous_station);
        let expected_res = current_station_train_data;
        assert_eq!(is_tram_coming, Some(expected_res));
    }

    #[test]
    fn test_one_incoming_but_one_previous_exists() {
        let current_station_train_data = vec![
            metrolib::parse::TrainData {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 1,
            }
        ];
        
        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            train_data: current_station_train_data.clone(),
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
        let expected_res = current_station_train_data;
        assert_eq!(is_tram_coming, Some(expected_res));
    }

    #[test]
    fn test_two_incoming_but_one_previous_exists() {
        let current_station_train_data = vec![
            metrolib::parse::TrainData {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 1,
            },
            metrolib::parse::TrainData {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 2,
            }
        ];
        
        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            train_data: current_station_train_data.clone(),
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
        let expected_res = current_station_train_data;
        assert_eq!(is_tram_coming, Some(expected_res));
    }

    #[test]
    fn test_two_incoming_but_two_previous_exists() {
        let current_station_train_data = vec![
            metrolib::parse::TrainData {
                destination: "MediaCityUK".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 3,
            },
            metrolib::parse::TrainData {
                destination: "MediaCityUK".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 5,
            },
        ];

        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            train_data: current_station_train_data.clone(),
            ..setup_station()
        };

        let previous_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            train_data: vec![
                metrolib::parse::TrainData {
                    destination: "MediaCityUK".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 10,
                },
                metrolib::parse::TrainData {
                    destination: "MediaCityUK".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 20,
                }
            ],
            ..setup_station()
        };

        let is_tram_coming = metrolib::logic::get_trams_between_stations(&current_station, &previous_station);
        let expected_res = current_station_train_data;
        assert_eq!(is_tram_coming, Some(expected_res));
    }

    #[test]
    fn test_three_incoming_but_one_previous_exists() {
        let current_station_train_data = vec![
            metrolib::parse::TrainData {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 1,
            },
            metrolib::parse::TrainData {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 2,
            },
            metrolib::parse::TrainData {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 3,
            }
        ];
        
        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            train_data: current_station_train_data.clone(),
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
        let expected_res = current_station_train_data;
        assert_eq!(is_tram_coming, Some(expected_res));
    }

    #[test]
    fn test_three_exists_but_three_previous_exists() {
        let current_station_train_data = vec![
            metrolib::parse::TrainData {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 10,
            },
            metrolib::parse::TrainData {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 20,
            },
            metrolib::parse::TrainData {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 30,
            }
        ];
        
        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            train_data: current_station_train_data.clone(),
            ..setup_station()
        };

        let previous_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            train_data: vec![
                metrolib::parse::TrainData {
                    destination: "Eccles".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 5,
                },
                metrolib::parse::TrainData {
                    destination: "Eccles".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 10,
                },
                metrolib::parse::TrainData {
                    destination: "Eccles".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 15,
                }
            ],
            ..setup_station()
        };

        let is_tram_coming = metrolib::logic::get_trams_between_stations(&current_station, &previous_station);
        assert_eq!(is_tram_coming, None);
    }

    #[test]
    fn test_one_exists_but_one_previous_exists() {
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