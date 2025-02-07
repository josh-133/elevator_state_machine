use std::fmt;
use std::io::{self, Read};
use std::thread::current;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Down => write!(f, "Down"),
            Direction::Up => write!(f, "Up"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
struct Elevator {
    state: State
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Closed { 
                moving, 
                direction, 
                current_floor, 
                dest_floor, 
                door_open 
            } => {
                write!(
                    f,
                    "Closed (moving: {}, direction: {}, current floor: {}, destination: {}, door open: {})",
                    moving, 
                    direction.as_ref().map_or("None".to_string(), |d| d.to_string()), 
                    current_floor.as_ref().map_or("None".to_string(), |c| c.to_string()), 
                    dest_floor.as_ref().map_or("None".to_string(), |d| d.to_string()), 
                    door_open,
                )
            },
            State::Open { 
                moving, 
                direction, 
                current_floor, 
                dest_floor, 
                door_open 
            } => {
                write!(
                    f,
                    "Open (moving: {}, direction: {}, current floor: {}, destination: {}, door open: {})",
                    moving, 
                    direction.as_ref().map_or("None".to_string(), |d| d.to_string()), 
                    current_floor.as_ref().map_or("None".to_string(), |c| c.to_string()), 
                    dest_floor.as_ref().map_or("None".to_string(), |d| d.to_string()), 
                    door_open,
                )
            },
            State::Moving { 
                moving, 
                direction, 
                current_floor, 
                dest_floor, 
                door_open 
            } => {
                write!(
                    f,
                    "Moving (moving: {}, direction: {}, current floor: {}, destination: {}, door open: {})",
                    moving, 
                    direction.as_ref().map_or("None".to_string(), |d| d.to_string()), 
                    current_floor.as_ref().map_or("None".to_string(), |c| c.to_string()), 
                    dest_floor.as_ref().map_or("None".to_string(), |d| d.to_string()), 
                    door_open,
                )
            }
        }
    }
}

impl fmt::Display for Elevator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.state)
    }
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
            State::Closed { 
                moving: false,
                direction, 
                current_floor, 
                dest_floor,
                door_open: false, 
            } => State::Open { 
                moving: false, 
                direction: *direction, 
                current_floor: *current_floor, 
                dest_floor: None, 
                door_open: true,
            },
            
            State::Open { 
                moving: false,
                direction,
                current_floor, 
                dest_floor, 
                door_open: true, 
            } => State::Closed { 
                    moving: false, 
                    direction: *direction, 
                    current_floor: *current_floor, 
                    dest_floor: *dest_floor, 
                    door_open: false, 
            },

            State::Closed { 
                moving: true, 
                direction, 
                current_floor, 
                dest_floor, 
                door_open: false,
            } if dest_floor.is_some() => State::Moving {
                moving: true, 
                direction: *direction, 
                current_floor: *current_floor, 
                dest_floor: *dest_floor, 
                door_open: false 
            },
            
            State::Moving { 
                moving: true, 
                direction, 
                current_floor,
                dest_floor,
                door_open,
            } => {
                if current_floor == dest_floor {
                    State::Open {
                        moving: false, 
                        direction: *direction, 
                        current_floor: *current_floor,
                        dest_floor: *dest_floor,
                        door_open: true,
                    }
                } else {
                    let next_floor = if dest_floor.unwrap() > current_floor.unwrap() {
                        current_floor.unwrap() + 1
                    } else {
                        current_floor.unwrap() - 1
                    };

                    State::Moving { 
                        moving: true, 
                        direction: *direction, 
                        current_floor: *current_floor,
                        dest_floor: *dest_floor,
                        door_open: false,
                    }                    
                } 
            }
            _ => self.state.clone(),
        };
    }
}

fn main() {

    let mut elevator = Elevator::new();

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
    print!("{}\n", elevator.state);
    elevator.transition(dest_floor, direction);
    print!("{}\n", elevator.state);
    elevator.transition(dest_floor, direction);
    print!("{}\n", elevator.state);
    elevator.transition(dest_floor, direction);
    print!("{}\n", elevator.state);    
}
