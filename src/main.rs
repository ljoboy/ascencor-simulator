use std::io::{stdin};
use std::thread::sleep as sys_sleep;

struct Ascencor {
    min_floor: i8,
    max_floor: i8,
    depart: i8,
}

trait AscencorTrait {
    fn new(min_floor: i8, max_floor: i8) -> Self;
    fn check_floor(self, floor: i8) -> bool;
    fn move_to_floor(self, floor: i8) -> bool;
}

impl Ascencor {
    fn new(min_floor: i8, max_floor: i8) -> Self {
        if min_floor > max_floor {
            panic!("The min floor must be less than the max floor");
        }
        Ascencor { min_floor, max_floor, depart: 0 }
    }

    // function to check if the floor is valid and do not take the ownership of the floor
    fn check_floor(&self, floor: &i8) -> bool {
        if floor < &self.min_floor || floor > &self.max_floor {
            return false;
        }
        true
    }

    // function to move to the floor from the depart floor to the floor by printing every floor between them, up or down
    fn move_to_floor(&mut self, floor: i8) {
        if !self.check_floor(&floor) {
            panic!("The floor is not valid");
        }
        if self.depart < floor {
            for i in self.depart..floor {
                println!("Floor {}", i);
                // sleep for 2 second
                sys_sleep(std::time::Duration::from_secs(2));
            }
        } else {
            for i in (floor..self.depart).rev() {
                println!("Floor {}", i);
                // sleep for 2 second
                sys_sleep(std::time::Duration::from_secs(2));
            }
        }
        self.depart = floor;
        // print that the ascencor is arrived to the floor
        println!("Arrived to floor {}", floor);
    }
}

fn get_floor() -> i8 {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}

fn main() {
    let mut _ascencor = Ascencor::new(-4, 12);
    // Call the ascencor
    println!("Please enter the floor where you are : ");
    _ascencor.move_to_floor(get_floor());
    // Go where you want
    println!("Please enter the floor where you want to go : ");
    _ascencor.move_to_floor(get_floor());
}