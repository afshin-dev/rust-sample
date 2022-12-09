fn main () {
    let pkg = Package::default();
    println!("default pkg weight: {}", pkg.weight);
}

struct Package {
    weight: usize // gram
}

impl Default for Package {
    fn default() -> Self {
        Self { weight: 5000 } // default weight of a package is 5000 gram
    }
}