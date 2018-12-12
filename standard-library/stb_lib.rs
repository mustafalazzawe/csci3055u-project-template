use std::collections::HashMap;
use std::collections::HashSet;

//sets
fn make_set(words: &str) -> HashSet<&str> {
    words.split_whitespace().collect()
}

fn main() {
    //array to vec
    println!("ARRAY CONVERSION");
    let nums = ["5","52","65"];
    let iter = nums.iter().map(|s| s.parse::<i32>());
    let converted: Result<Vec<_>,_> = iter.collect();
    println!("{:?}",converted);

    //maps
    println!("\nMAPS");
    let mut map = HashMap::new();
    map.insert("one",1);
    map.insert("two",2);
    map.insert("three",3);

    println!("before {}", map.get("two").unwrap());

    //block
    {
        let mref = map.get_mut("two").unwrap();
        *mref = 20;
    }

    println!("after {}", map.get("two").unwrap());

    //sets 
    println!("\nSETS:");
    let fruit = make_set("apple orange pear");
    let colours = make_set("brown purple orange yellow");

    for c in fruit.intersection(&colours) {
        println!("{:?}",c);
    }    
}
