fn add (x: i32 , y: i32) -> i32 {
    x + y 
}

fn main() {

}

#[cfg(test)]
mod test {
    #[test] 
    fn test_add_fn(){
        // Arange 
        let arg1 = 1 ; 
        let arg2 = 1 ;
        let sum = arg1 + arg2 ;

        // act 
        let res = crate::add(arg1, arg2);

        // assert 
        assert_eq!(sum, res);
    }
 }