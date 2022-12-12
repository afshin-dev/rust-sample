fn main () {
    let retrun_code = RetrunCode::from(0);
    let retrun_code_1 = RetrunCode::from(2);

    println!("{:?}", retrun_code);
    println!("{:?}", retrun_code_1);
}

#[derive(Debug)]
enum RetrunCode {
    Success,
    Failed(u8), // u8 = code number
}

impl From<u8> for RetrunCode {
    fn from(code: u8) -> Self {
        match code {
             0 => Self::Success ,
             code => Self::Failed(code),
        }
    }
}