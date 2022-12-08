use smpl::math::add ;
use smpl::config::get_env;
fn main() {
    println!("{}", add(1, 1));
    println!("environment = {}", get_env());
}