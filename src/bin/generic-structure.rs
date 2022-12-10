fn main() {
    println!("generic structure");
}

trait Trait1 {

}

trait Trait2 {
    
}

trait Trait3 {

}


struct Struct<T, U> 
    where  
        T: Trait1 + Trait2 ,
        U: Trait3
{
    f1 : T,
    f2 : U ,
} 



