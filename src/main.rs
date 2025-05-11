<<<<<<< HEAD
use endoflife::rust;

use serde_json;

fn main() {
    let json_str: &str = r#"{
    "releaseDate": "2024-05-02",
    "eol": false,
    "latest": "1.78.0",
    "latestReleaseDate": "2024-05-02",
    "lts": false
    }"#;

    let json_object: Result<RustSingleCycle,Error> = serde_json::from_str::<rust::RustSingleCycle>(json_str);

    println!(
    "{:?}",
    json_object
    );
}
=======
fn main() {
    let mut x = 300;
    let mut even_numbers = Vec::new(); 
    while x > 0 {
        if x % 2 == 0 {
            println!("{x}");
            even_numbers.push(x); 
        }
        x -= 1;
    }

    println!("Even numbers: {:?}", even_numbers);
}
>>>>>>> 603bfc9 (Just a try)
