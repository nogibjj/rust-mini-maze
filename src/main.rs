use std::io::stdin;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Road {
    name: String,
    path: String,
}

fn main() {
    let data = r#"
    [
    {
        "name": "Route 1",
        "path": "Fort Kent, ME - Key West, FL"
    },
    {
        "name": "Route 66",
        "path": "Chicago, IL - Santa Monica, CA"
    },
    {
        "name": "Interstate 5",
        "path": "Blaine, WA - San Ysidro, CA"
    },
    {
        "name": "Interstate 10",
        "path": "Santa Monica, CA - Jacksonville, FL"
    },
    {
        "name": "Interstate 20",
        "path": "Kent, TX - Florence, SC"
    },
    {
        "name": "Interstate 40",
        "path": "Barstow, CA - Wilmington, NC"
    },
    {
        "name": "Interstate 70",
        "path": "Cove Fort, UT - Baltimore, MD"
    },
    {
        "name": "Interstate 80",
        "path": "San Francisco, CA - Teaneck, NJ"
    },
    {
        "name": "Interstate 90",
        "path": "Seattle, WA - Boston, MA"
    },
    {
        "name": "US 1",
        "path": "Fort Kent, ME - Key West, FL"
    },
    {
        "name": "US 20",
        "path": "Boston, MA - Newport, OR"
    },
    {
        "name": "US 50",
        "path": "Ocean City, MD - West Sacramento, CA"
    },
    {
        "name": "US 101",
        "path": "Olympic National Park, WA - Los Angeles, CA"
    },
    {
        "name": "US 129",
        "path": "Chattanooga, TN - Deals Gap, NC"
    },
    {
        "name": "US 191",
        "path": "Douglas, AZ - Choteau, MT"
    },
    {
        "name": "US 212",
        "path": "Yellowstone National Park, WY - Sioux Falls, SD"
    },
    {
        "name": "US 550",
        "path": "Montrose, CO - Bernalillo, NM"
    }
]
"#;


    let roads: Vec<Road> = serde_json::from_str(data).expect("Unable to parse JSON");

    println!("Enter a road name:");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Unable to read line");

    let query = input.trim().to_lowercase();

    let matching_roads: Vec<&Road> = roads
        .iter()
        .filter(|road| road.name.to_lowercase().contains(&query))
        .collect();

    if matching_roads.is_empty() {
        println!("No roads found");
    } else {
        for (i, road) in matching_roads.iter().enumerate() {
            println!("{}. {}", i + 1, road.name);
        }

        println!("Enter a road number:");

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Unable to read line");

        let index = input.trim().parse::<usize>().expect("Invalid input");

        if index > matching_roads.len() {
            println!("Invalid road number");
        } else {
            let road = &matching_roads[index - 1];
            println!("{}: {}", road.name, road.path);
        }
    }
}
