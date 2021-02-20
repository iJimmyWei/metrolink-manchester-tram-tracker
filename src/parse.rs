use crate::api::ResponseDto;
use serde::Serialize;

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Incoming,
    Outgoing,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Carriages {
    Single,
    Double,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Status {
    Departing,
    Arriving,
    Due,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Tram {
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
    pub approaching_trams: Vec<Tram>,
    pub message_board: String,
    pub last_updated: String,
}

fn parse_station_name_abbreviation(station_name_maybe_abbreviated: &str) -> String {
    match station_name_maybe_abbreviated {
        "MCUK" => String::from("MediaCityUK"),
        _ => station_name_maybe_abbreviated.to_owned()
    }
}

fn parse_destination(previous_tram_destination: Option<&str>, destination: &str) -> String {
    if previous_tram_destination == None {
        return parse_station_name_abbreviation(destination)
    }

    if destination == "See Tram Front" {
        return parse_station_name_abbreviation(previous_tram_destination.unwrap())
    }

    return destination.to_owned();
}

fn parse_status(status: &str) -> Status {
    match status { 
        "Departing" => Status::Departing,
        "Arriving" => Status::Arriving,
        _ => Status::Due,
    }
}

fn parse_carriages(carriages: &str) -> Carriages {
    match carriages { 
        "Double" => Carriages::Double,
        _ => Carriages::Single,
    }
}

fn parse_direction(direction: &str) -> Direction {
    match direction { 
        "Incoming" => Direction::Outgoing,
        _ => Direction::Incoming,
    }
}

pub fn parse(response: ResponseDto) -> Vec<StationData> {
    let mut all_station_data: Vec<StationData> = Vec::new();

    for station in response.value {
        let mut approaching_trams: Vec<Tram> = Vec::new();

        // All stations provide exactly 4 traindata objects
        if station.Dest0 != "" {
            approaching_trams.push(Tram {
                destination: parse_destination(None, &station.Dest0),
                status: parse_status(&station.Status0),
                estimated_wait_time: station.Wait0.parse::<i32>().unwrap(),
                carriages: parse_carriages(&station.Carriages0)
            })
        }

        if station.Dest1 != "" {
            approaching_trams.push(Tram {
                destination: parse_destination(Some(&station.Dest0), &station.Dest1),
                status: parse_status(&station.Status1),
                estimated_wait_time: station.Wait1.parse::<i32>().unwrap(),
                carriages: parse_carriages(&station.Carriages1)
            })
        }

        if station.Dest2 != "" {
            approaching_trams.push(Tram {
                destination: parse_destination(Some(&station.Dest1), &station.Dest2),
                status: parse_status(&station.Status2),
                estimated_wait_time: station.Wait2.parse::<i32>().unwrap(),
                carriages:  parse_carriages(&station.Carriages2)
            })
        }

        if station.Dest3 != "" {
            approaching_trams.push(Tram {
                destination: parse_destination(Some(&station.Dest2), &station.Dest3),
                status: parse_status(&station.Status3),
                estimated_wait_time: station.Wait3.parse::<i32>().unwrap(),
                carriages: parse_carriages(&station.Carriages3)
            })
        }

        let data = StationData {
            id: station.Id,
            line: station.Line,
            tla_ref: station.TLAREF,
            pid_ref: station.PIDREF,
            location: station.StationLocation,
            atco_code: station.AtcoCode,
            direction: parse_direction(&station.Direction),
            approaching_trams,
            message_board: station.MessageBoard,
            last_updated: station.LastUpdated,
        };

        all_station_data.push(data);
    }

    all_station_data
}