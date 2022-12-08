fn main() {
    let mut role : Role = Role::DbManager ;
    if let Role::Admin = role  {
        println!("access granted");
    }
    role = Role::Admin ;
    if let Role::Admin = role  {
        println!("access granted");
    }
}

#[derive(Debug)]
enum Role {
 Admin,
 DbManager,
 Developer,
 DevOpsEngineer
}