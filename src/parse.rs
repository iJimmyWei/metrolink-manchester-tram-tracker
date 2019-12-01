use crate::api::ResponseDto;

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Incoming,
    Outgoing,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Carriages {
    Single,
    Double,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Departing,
    Arriving,
    Due,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrainData {
    pub destination: String,
    pub carriages: Carriages,
    pub status: Status,
    pub estimated_wait_time: i32,
}

#[derive(Debug, Clone)]
pub struct StationData {
    pub id: i32,
    pub line: String,
    pub tla_ref: String,
    pub pid_ref: String,
    pub location: String,
    pub atco_code: String,
    pub direction: Direction,
    pub train_data: Vec<TrainData>,
    pub message_board: String,
    pub last_updated: String,
}

pub fn parse(response: ResponseDto) -> Vec<StationData> {
    let mut all_station_data: Vec<StationData> = Vec::new();

    // Loops through all station data
    for station in response.value {
        // Map to new nested structure
        let mut train_data: Vec<TrainData> = Vec::new();

        // All stations provide exactly 4 traindata objects, but some may be null
        if station.Dest0 != "" {
            train_data.push(TrainData {
                destination: station.Dest0.clone().replace("MCUK", "MediaCityUK"),
                status: match station.Status0.as_str() { 
                    "Departing" => Status::Departing,
                    "Arriving" => Status::Arriving,
                    _ => Status::Due,
                },
                estimated_wait_time: station.Wait0.parse::<i32>().unwrap(),
                carriages: match station.Carriages0.as_str() { 
                    "Double" => Carriages::Double,
                    _ => Carriages::Single,
                },
            })
        }

        if station.Dest1 != "" {
            train_data.push(TrainData {
                destination: if station.Dest1 == "See Tram Front" {
                    station.Dest0.clone().replace("MCUK", "MediaCityUK")
                } else {
                    station.Dest1.clone().replace("MCUK", "MediaCityUK")
                },
                status: match station.Status1.as_str() { 
                    "Departing" => Status::Departing,
                    "Arriving" => Status::Arriving,
                    _ => Status::Due,
                },
                estimated_wait_time: station.Wait1.parse::<i32>().unwrap(),
                carriages: match station.Carriages1.as_str() { 
                    "Double" => Carriages::Double,
                    _ => Carriages::Single,
                },
            })
        }

        if station.Dest2 != "" {
            train_data.push(TrainData {
                destination: if station.Dest2 == "See Tram Front" {
                    station.Dest0.clone().replace("MCUK", "MediaCityUK")
                } else {
                    station.Dest2.clone().replace("MCUK", "MediaCityUK")
                },
                status: match station.Status2.as_str() { 
                    "Departing" => Status::Departing,
                    "Arriving" => Status::Arriving,
                    _ => Status::Due,
                },
                estimated_wait_time: station.Wait2.parse::<i32>().unwrap(),
                carriages: match station.Carriages2.as_str() { 
                    "Double" => Carriages::Double,
                    _ => Carriages::Single,
                },
            })
        }

        if station.Dest3 != "" {
            train_data.push(TrainData {
                destination: if station.Dest3 == "See Tram Front" {
                    station.Dest0.replace("MCUK", "MediaCityUK")
                } else {
                    station.Dest3.replace("MCUK", "MediaCityUK")
                },
                status: match station.Status3.as_str() { 
                    "Departing" => Status::Departing,
                    "Arriving" => Status::Arriving,
                    _ => Status::Due,
                },
                estimated_wait_time: station.Wait3.parse::<i32>().unwrap(),
                carriages: match station.Carriages3.as_str() { 
                    "Double" => Carriages::Double,
                    _ => Carriages::Single,
                },
            })
        }

        let data = StationData {
            id: station.Id,
            line: station.Line,
            tla_ref: station.TLAREF,
            pid_ref: station.PIDREF,
            location: station.StationLocation,
            atco_code: station.AtcoCode,
            direction: match station.Direction.as_str() { 
                "Incoming" => Direction::Outgoing,
                _ => Direction::Incoming,
            },
            train_data,
            message_board: station.MessageBoard,
            last_updated: station.LastUpdated,
        };

        all_station_data.push(data);
    }

    all_station_data
}