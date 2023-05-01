use std::{fs, path::Path, collections::HashMap};

use rand::Rng;
use walkdir::WalkDir;
use yaserde_derive::YaDeserialize;

#[derive(Debug)]
pub struct Items {
    items: Vec<ItemData>,
    items_by_name: HashMap<String, ItemData>,
}

impl Items {
    pub fn new() -> Items {
        Items { 
            items: Vec::new(),
            items_by_name: HashMap::new(),
        }
    }

    pub fn load_all(&mut self, folder: &str) {
        for entry in WalkDir::new(folder)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_dir() {
                continue;
            }
            log::info!("loading {}", entry.path().display());
            let items = ItemCollection::from_xml(entry.path());
            self.items.extend(items.items);
        }
        self.items_by_name = self.items.iter().map(|i| (i.name.clone(), i.clone())).collect();
    }

    pub fn get_item(&self, name: &str) -> Option<&ItemData> {
        self.items_by_name.get(name)
    }

    pub fn random_item(&self) -> &ItemData {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.items.len());
        &self.items[i]
    }
}

#[derive(YaDeserialize, Debug, PartialEq)]
#[yaserde(rename = "items")]
pub struct ItemCollection {
    #[yaserde(rename = "item")]
    pub items: Vec<ItemData>,
}

impl ItemCollection {
	pub fn from_xml(path: &Path) -> Self {
        let xml = fs::read_to_string(path).expect(format!("Error reading prefab file {:?}", path).as_str());
        yaserde::de::from_str::<ItemCollection>(xml.as_str()).unwrap()
    }
}

#[derive(YaDeserialize, Debug, PartialEq, Clone)]
pub struct ItemData {
    pub name: String,
    #[yaserde(rename = "type")]
    pub item_type: String,
    pub subtype: String,
    pub range: u32,
    pub damage: u32,
    pub rate: u32,
    pub accuracy: f32,
    pub ammo: u32,
    pub price: f32,
    pub char: u8,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_item_deser() {
        let xml = r#"
        <items>
            <item>
                <name>.45 Revolver</name>
                <type>weapon</type>
                <subtype>handgun</subtype>
                <range>50</range>
                <damage>5</damage>
                <rate>1</rate>
                <accuracy>0.8</accuracy>
                <ammo>6</ammo>
            </item>
        </items>
    "#;

        let items: ItemCollection = yaserde::de::from_str(&xml).unwrap();
        println!("Deserialized items: {:?}", items);
    }

    #[test]
    fn test_load_add() {
        let mut items = Items::new();
        items.load_all("data/items");
        for item in &items.items {
            println!("Loaded item: {:?}", item);
        }

        // iterate self.items_by_name
        for (name, item) in &items.items_by_name {
            println!("Loaded item by name - Name: {}, Item: {:?}", name, item);
        }
    }
}
