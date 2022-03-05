use error_chain::error_chain;
use reqwest;
use serde_derive::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Material {
    category: String,
    name: String,
    shortcut_name: String,
    id: String,
    tier: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct  Listing {
    listings: u64,
    unit_price: u64,
    quantity: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct MaterialListing {
    id : u64,
    buys: Vec<Listing>,
    sells: Vec<Listing>
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() {
    let materials = fs::read_to_string("materials.json").expect("Unable to read file");
    let materials_list: Vec<Material> =
        serde_json::from_str(&materials).expect("Cannot parse JSON");

    let base_url = "https://api.guildwars2.com/v2";
    let endpoint_selected = "commerce/listings";

    for material in materials_list {
        let url_request = format!(
            "{}/{}?ids={}",
            base_url, endpoint_selected, material.id
        );
        let response = reqwest::get(&url_request).await.unwrap();

        let listings: Vec<MaterialListing> = response.json::<Vec<MaterialListing>>().await.unwrap();
        println!("Material: {:?}",material.name);
        println!("ID: {:?}", listings[0].id);
        println!("Buy price:{:?}", listings[0].buys[0].unit_price);
        println!("Sell price:{:?}", listings[0].sells[0].unit_price);
        println!();
     }

    let _a = 1;
}
