// fun example of typestate pattern 
fn main() {
    use human::{Body, SoulType} ;
    let body = Body {name: "afshin".to_owned(), health:92, soul_type:SoulType::Alpha } ;
    let soul = body.die();

    println!("after die: {:?} with {:?} soul ", soul.dead_owner, soul.typ);
}


mod human {

    #[derive(Debug)]
    pub enum SoulType {
        Alpha, 
        Meta,
    }
    pub struct Body {
        pub name: String,
        pub health : i32 ,
        pub soul_type : SoulType,
    }
    impl Body {
        pub fn die(self) -> Soul {
            // change soul type to meta 
            // and return soul
            Soul { typ: SoulType::Meta, dead_owner: self.name }
        }
    }
    pub struct  Soul {
        pub typ: SoulType,
        pub dead_owner: String
    }
}