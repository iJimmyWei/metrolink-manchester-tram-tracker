#[cfg(test)]
extern crate metrolib;

mod tests {
    fn setup_station() -> metrolib::parse::StationData {
        metrolib::parse::StationData {
            id: 830,
            line: "North Pole".to_string(),
            tla_ref: "LDW".to_string(),
            pid_ref: "LDW-TPTD01".to_string(),
            location: "Ladywell".to_string(),
            atco_code: "9400ZZMALDY2".to_string(),
            direction: metrolib::parse::Direction::Outgoing,
            approaching_trams: vec![],
            message_board: "None".to_string(),
            last_updated: "2019-12-01T16:11:20Z".to_string(),
        }
    }

    #[test]
    fn test_one_approaching_no_previous_approaching() {
        let approaching_trams = vec![
            metrolib::parse::ApproachingTram {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 1,
            }
        ];

        let current_station = metrolib::parse::StationData {
            approaching_trams: approaching_trams.clone(),
            ..setup_station()
        };

        let previous_station = metrolib::parse::StationData {
            approaching_trams: vec![],
            ..setup_station()
        };

        let trams = metrolib::logic::get_trams_between_current_and_previous_station(
            &current_station,
            &previous_station
        );

        let mut expected_trams: Vec<metrolib::logic::Tram> = Vec::new();
        for tram_data in approaching_trams.iter() {
            expected_trams.push(metrolib::logic::Tram {
                next_station: current_station.location.clone(),
                previous_station: previous_station.location.clone(),
                metadata: tram_data.clone()
            });
        }
       
        assert_eq!(trams, Some(expected_trams));
    }

    #[test]
    fn test_one_approaching_one_previous_approaching() {
        let current_station_train_data = vec![
            metrolib::parse::ApproachingTram {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 1,
            }
        ];
        
        let current_station = metrolib::parse::StationData {
            approaching_trams: current_station_train_data.clone(),
            ..setup_station()
        };

        let previous_station = metrolib::parse::StationData {
            approaching_trams: vec![
                metrolib::parse::ApproachingTram {
                    destination: "Eccles".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 5,
                }
            ],
            ..setup_station()
        };

        let trams = metrolib::logic::get_trams_between_current_and_previous_station(
            &current_station,
            &previous_station
        );

        let mut expected_trams: Vec<metrolib::logic::Tram> = Vec::new();
        for tram_data in current_station_train_data.iter() {
            expected_trams.push(metrolib::logic::Tram {
                next_station: current_station.location.clone(),
                previous_station: previous_station.location.clone(),
                metadata: tram_data.clone()
            });
        }

        assert_eq!(trams, Some(expected_trams));
    }

    #[test]
    fn test_one_approaching_two_previous_approaching() {
        let current_station_train_data = vec![
            metrolib::parse::ApproachingTram {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 2,
            },
            metrolib::parse::ApproachingTram {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 7,
            },
        ];
        
        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            approaching_trams: current_station_train_data.clone(),
            ..setup_station()
        };

        let previous_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            approaching_trams: vec![
                metrolib::parse::ApproachingTram {
                    destination: "Eccles".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 5,
                },
                metrolib::parse::ApproachingTram {
                    destination: "Eccles".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 15,
                },
            ],
            ..setup_station()
        };

        let mut expected_trams: Vec<metrolib::logic::Tram> = Vec::new();
        
        expected_trams.push(metrolib::logic::Tram {
            next_station: current_station.location.clone(),
            previous_station: previous_station.location.clone(),
            metadata: current_station_train_data[0].clone()
        });
       
        let trams = metrolib::logic::get_trams_between_current_and_previous_station(
            &current_station,
            &previous_station
        );
        assert_eq!(trams, Some(expected_trams));
    }

    #[test]
    fn test_two_approaching_one_previous_approaching() {
        let current_station_train_data = vec![
            metrolib::parse::ApproachingTram {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 1,
            },
            metrolib::parse::ApproachingTram {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 2,
            }
        ];
        
        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            approaching_trams: current_station_train_data.clone(),
            ..setup_station()
        };

        let previous_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            approaching_trams: vec![
                metrolib::parse::ApproachingTram {
                    destination: "Eccles".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 5,
                }
            ],
            ..setup_station()
        };

        let trams = metrolib::logic::get_trams_between_current_and_previous_station(
            &current_station,
            &previous_station
        );

        let mut expected_trams: Vec<metrolib::logic::Tram> = Vec::new();
        for tram_data in current_station_train_data.iter() {
            expected_trams.push(metrolib::logic::Tram {
                next_station: current_station.location.clone(),
                previous_station: previous_station.location.clone(),
                metadata: tram_data.clone()
            });
        }
       
        assert_eq!(trams, Some(expected_trams));
    }


    #[test]
    fn test_two_approaching_two_previous_approaching() {
        let current_station_train_data = vec![
            metrolib::parse::ApproachingTram {
                destination: "MediaCityUK".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 3,
            },
            metrolib::parse::ApproachingTram {
                destination: "MediaCityUK".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 5,
            },
        ];

        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            approaching_trams: current_station_train_data.clone(),
            ..setup_station()
        };

        let previous_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            approaching_trams: vec![
                metrolib::parse::ApproachingTram {
                    destination: "MediaCityUK".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 10,
                },
                metrolib::parse::ApproachingTram {
                    destination: "MediaCityUK".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 20,
                }
            ],
            ..setup_station()
        };

        let trams = metrolib::logic::get_trams_between_current_and_previous_station(
            &current_station,
            &previous_station
        );

        let mut expected_trams: Vec<metrolib::logic::Tram> = Vec::new();
        for tram_data in current_station_train_data.iter() {
            expected_trams.push(metrolib::logic::Tram {
                next_station: current_station.location.clone(),
                previous_station: previous_station.location.clone(),
                metadata: tram_data.clone()
            });
        }
       
        assert_eq!(trams, Some(expected_trams));
    }

    #[test]
    fn test_three_approaching_one_previous_approaching() {
        let current_station_train_data = vec![
            metrolib::parse::ApproachingTram {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 1,
            },
            metrolib::parse::ApproachingTram {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 2,
            },
            metrolib::parse::ApproachingTram {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 3,
            }
        ];
        
        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            approaching_trams: current_station_train_data.clone(),
            ..setup_station()
        };

        let previous_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            approaching_trams: vec![
                metrolib::parse::ApproachingTram {
                    destination: "Eccles".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 5,
                }
            ],
            ..setup_station()
        };

        let trams = metrolib::logic::get_trams_between_current_and_previous_station(
            &current_station,
            &previous_station
        );
        
        let mut expected_res: Vec<metrolib::logic::Tram> = Vec::new();
        for tram_data in current_station_train_data.iter() {
            expected_res.push(metrolib::logic::Tram {
                next_station: current_station.location.clone(),
                previous_station: previous_station.location.clone(),
                metadata: tram_data.clone()
            });
        }

        assert_eq!(trams, Some(expected_res));
    }

    #[test]
    fn test_three_approaching_three_previous_approaching_expect_none() {
        let current_station_train_data = vec![
            metrolib::parse::ApproachingTram {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 10,
            },
            metrolib::parse::ApproachingTram {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 20,
            },
            metrolib::parse::ApproachingTram {
                destination: "Eccles".to_string(),
                carriages: metrolib::parse::Carriages::Double,
                status: metrolib::parse::Status::Arriving,
                estimated_wait_time: 30,
            }
        ];
        
        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            approaching_trams: current_station_train_data.clone(),
            ..setup_station()
        };

        let previous_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            approaching_trams: vec![
                metrolib::parse::ApproachingTram {
                    destination: "Eccles".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 5,
                },
                metrolib::parse::ApproachingTram {
                    destination: "Eccles".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 10,
                },
                metrolib::parse::ApproachingTram {
                    destination: "Eccles".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 15,
                }
            ],
            ..setup_station()
        };

        let trams = metrolib::logic::get_trams_between_current_and_previous_station(
            &current_station,
            &previous_station
        );
        assert_eq!(trams, None);
    }

    #[test]
    fn test_one_approaching_one_previous_approaching_expect_none() {
        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            approaching_trams: vec![
                metrolib::parse::ApproachingTram {
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
            approaching_trams: vec![
                metrolib::parse::ApproachingTram {
                    destination: "MediaCityUK".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 1,
                }
            ],
            ..setup_station()
        };

        let trams = metrolib::logic::get_trams_between_current_and_previous_station(
            &current_station,
            &previous_station
        );
        assert_eq!(trams, None);
    }

    #[test]
    fn test_one_approaching_backlog_three_previous_approaching_expect_none() {
        let current_station = metrolib::parse::StationData {
            line: "Eccles".to_string(),
            approaching_trams: vec![
                metrolib::parse::ApproachingTram {
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
            approaching_trams: vec![
                metrolib::parse::ApproachingTram {
                    destination: "North Pole".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 2,
                },
                metrolib::parse::ApproachingTram {
                    destination: "North Pole".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 5,
                },
                metrolib::parse::ApproachingTram {
                    destination: "North Pole".to_string(),
                    carriages: metrolib::parse::Carriages::Double,
                    status: metrolib::parse::Status::Arriving,
                    estimated_wait_time: 10,
                }
            ],
            ..setup_station()
        };

        let trams = metrolib::logic::get_trams_between_current_and_previous_station(
            &current_station,
            &previous_station
        );
        assert_eq!(trams, None);
    }
}
