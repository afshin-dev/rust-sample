fn main() {
    const ALTER_PORT : i32= 29 ;
    let ssh_port: Option<i32> = Some(22) ;
    let port: i32 = ssh_port.unwrap_or_else(|| ALTER_PORT) ;
    println!("port is {}" , port);

    let ssh_port : Option<i32> = None ;
    let port : i32 = ssh_port.unwrap_or_else(|| ALTER_PORT);
    println!("alter code is {}", port);

}