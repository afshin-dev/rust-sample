// simulate command line for changing 
// power state of the system
fn main() {
    io::print_message("type a power state ");
    let state = io::read_line();
    let state_analize_result = power::get_state(state);
    io::print_option(state_analize_result);
}

mod power {

    #[derive(Debug)]
    pub enum State {
        Sleep,
        Off,
        Reboot,
        Shutdown,
        Hibernate,
    }
    pub fn get_state(state: String) -> Option<State> {
        match state.as_str() {
            "sleep" => {
                return Some(State::Sleep) ;
            },
            "off" => {
                return Some(State::Off);
            }, 
            "reboot" => {
                return Some(State::Reboot);
            },
            "shutdown" => {
                return Some(State::Shutdown);
            },
            "hibernate" => {
                return Some(State::Hibernate);
            },
            _ => {
                return None ;
            }
        }
    }
}

mod io {
    use std::io ;

    use crate::power;

    pub fn print_message(msg: &str) -> () {
        println!("\n{}:", msg);
    }
    pub fn read_line() -> String {
        let mut buffer = String::new() ;
        let line = io::stdin().read_line(&mut buffer);
        match line {
            Ok(_) => {
                return buffer.trim().to_owned() ;
            },
            Err(e) => {
                println!("{}",e);
                return "".to_owned();
            }
        }
    }
    pub fn print_option(o: Option<power::State>) {
        match o {
            Some(s) => {
                println!("machine state change to {:?} state", s);
            },
            None => {
                println!("current supported state: [off, sleep, hibernate, reboot, shutdown ]");
            }
        }
    }
}