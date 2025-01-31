use std::any::Any;
use std::io::{self, Read};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
}

#[derive(Clone, Copy)]
enum State {
    Closed {
        moving: bool,
        direction: Option<Direction>,
        current_floor: Option<i8>,
        dest_floor: Option<i8>,
        door_open: bool,
    },
    Open {
        moving: bool,
        direction: Option<Direction>,
        current_floor: Option<i8>,
        dest_floor: Option<i8>,
        door_open: bool,
    },
    Moving {
        moving: bool,
        direction: Option<Direction>,
        current_floor: Option<i8>,
        dest_floor: Option<i8>,
        door_open: bool,
    },
}

#[derive(Clone, Copy)]
struct Elevator {
    state: State
}

impl Elevator {
    fn new() -> Self {
        Self {
            state: State::Closed {
                moving: false,
                direction: None,
                current_floor: Some(0),
                dest_floor: None,
                door_open: false,
            }
        }
    }

    fn transition(&mut self, dest_floor: Option<i8>, direction: Option<Direction>) {
        self.state = match &self.state {
            &State::Closed { 
                moving: false,
                direction: None,
                current_floor: Some(0), 
                dest_floor: None, 
                door_open: false 
            } => match direction {
                Some(Direction::Up) => State::Open { 
                    moving: false, 
                    direction: Some(Direction::Up), 
                    current_floor: Some(0), 
                    dest_floor: None, 
                    door_open: true, 
                },
                Some(Direction::Down) => State::Open { 
                    moving: false, 
                    direction: Some(Direction::Down), 
                    current_floor: Some(0), 
                    dest_floor: None, 
                    door_open: true, 
                },
                _ => State::Closed { 
                    moving: false, 
                    direction: None, 
                    current_floor: Some(0), 
                    dest_floor: None, 
                    door_open: false, 
                },
            },
            &State::Open { 
                moving: false, 
                direction: None, 
                current_floor: Some(0), 
                dest_floor: None, 
                door_open: true,
            } => match dest_floor {
                Some(0) => State::Closed { 
                    moving: false, 
                    direction: direction, 
                    current_floor: Some(0), 
                    dest_floor: dest_floor, 
                    door_open: false 
                },
                Some(1) => State::Closed { 
                    moving: false, 
                    direction: direction, 
                    current_floor: Some(0), 
                    dest_floor: dest_floor, 
                    door_open: false 
                },
                Some(2) => State::Closed { 
                    moving: false, 
                    direction: direction, 
                    current_floor: Some(0), 
                    dest_floor: dest_floor, 
                    door_open: false 
                },
                Some(3) => State::Closed { 
                    moving: false, 
                    direction: direction, 
                    current_floor: Some(0), 
                    dest_floor: dest_floor, 
                    door_open: false 
                },
                _ => State::Closed { 
                    moving: false, 
                    direction: None, 
                    current_floor: Some(0), 
                    dest_floor: None, 
                    door_open: false, 
                },
            },
            &State::Moving { 
                moving: true, 
                direction: None, 
                current_floor: None, 
                dest_floor: None, 
                door_open: false,
            } => match dest_floor {
                _ => State::Closed { 
                    moving: false, 
                    direction: direction, 
                    current_floor: dest_floor, 
                    dest_floor: None, 
                    door_open: false 
                },    
            },
            _ => self.state.clone(),
        }
    }
}

fn main() {

    let mut elevator = Elevator::new();

    while true {
        println!("Press up or down button: ");

        let mut select_button = String::new();
        io::stdin().read_line(&mut select_button).expect("Invalid input");
        let select_button = select_button.trim().to_lowercase();

        let direction: Option<Direction> = match select_button.as_str() {
            "up" => Some(Direction::Up),
            "down" => Some(Direction::Down),
            _ => None,
        };

        println!("Select a floor: ");

        let mut floor: String = String::new();
        io::stdin().read_line(&mut floor).expect("Invalid input");
        let floor = floor.trim();

        let dest_floor: Option<i8> = match floor {
            "G" => Some(0),
            "0" => Some(0),
            "1" => Some(1),
            "2" => Some(2),
            "3" => Some(3),
            _ => None,
        };

        elevator.transition(dest_floor, direction);
    }

    
}
