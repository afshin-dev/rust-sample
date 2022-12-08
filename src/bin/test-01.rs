fn main() {

}

fn clamp(n:i32, lbound: i32, ubound: i32) -> i32 {
    if n < lbound {
        return lbound;
    } else if  n > ubound {
        return ubound;
    } 
    n
}

#[cfg(test)]
mod test {
    use crate::{clamp} ;
    
    #[test]
    fn test_clamp() {
        // Arange 
        let mut n = 10; 
        let lbound = 1;
        let ubound = 100 ;

        // act 
        let mut res = clamp(n, lbound, ubound);

        assert_eq!(res, n, "n must be return");

        n = lbound -1 ; // means lower than lower bound 
        res = clamp(n, lbound, ubound);
        assert_eq!(res, lbound, "lower bound must be return ");

        n = ubound + 1 ;
        res = clamp(n, lbound, ubound);
        assert_eq!(res, ubound, "upper bound must be return");
    }
}