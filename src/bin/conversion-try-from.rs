use std::convert::TryFrom ;
fn main() {
    let signal = Signal::try_from(3);
    match signal {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{:?}", e)
    }
}

#[derive(Debug)]
enum Signal {
    Kill,
    Interrupt,
    Stop,
    Start
}

#[derive(Debug)]
enum SignalError {
    NotExists,
}

impl TryFrom<u8> for Signal {
    type Error = SignalError ;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Signal::Kill),
            2 => Ok(Signal::Interrupt),
            3 => Ok(Signal::Start) ,
            4 => Ok(Self::Stop),
            _ => Err(SignalError::NotExists)
        }
    }
}