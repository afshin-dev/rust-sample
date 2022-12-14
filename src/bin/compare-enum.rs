#[derive(Debug , PartialEq)]
enum Degree {
    Beginner, // 1
    Intermediate, // 2
    Advanced, // 3
    Professinal, // 4
}
fn main() {
   compare_enum(Degree::Advanced, Degree::Beginner);
   compare_enum(Degree::Advanced, Degree::Advanced);

  // for compareing enum 
  // must derive derivable trait PartialEq 
}

fn compare_enum(lhs: Degree, rhs: Degree) {
    
    if lhs == rhs {
        println!("{:?} and {:?} is equal.", lhs, rhs);
    }
}