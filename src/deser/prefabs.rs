extern crate xml;

use std::{collections::HashMap, fs};
use std::path::Path;
use walkdir::WalkDir;
use yaserde_derive::{YaDeserialize};

#[derive(Default)]
pub struct Prefabs {
    folder: String,
    prefabs: HashMap<String, Prefab>,
}

impl Prefabs {
    pub fn new(folder: &str) -> Self {
        return Prefabs {
            folder: String::from(folder),
            prefabs: HashMap::new(),
        };
    }

    pub fn load_all(&mut self) {
        for entry in WalkDir::new(self.folder.as_str())
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_dir() {
                continue;
            }
            log::debug!("loading {}", entry.path().display());
            let prefab = Prefab::from_xml(entry.path());
            self.prefabs.insert(String::from(prefab.name.as_str()), prefab); // quite sure this is wrong
        }
    }

    pub fn get(&self, name: &str) -> &Prefab {
        return self.prefabs.get(name).unwrap();
    }
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(rename = "prefab")]
pub struct Prefab {
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(attribute)]
    pub width: i32,
    #[yaserde(attribute)]
    pub height: i32,
    #[yaserde(child)]
    pub placement: Placement,
    #[yaserde(child)]
    pub data: Data,
}

impl Prefab {
	pub fn from_xml(path: &Path) -> Self {
        let xml = fs::read_to_string(path).expect(format!("Error reading prefab file {:?}", path).as_str());
        yaserde::de::from_str::<Prefab>(xml.as_str()).unwrap()
    }
}


#[derive(Default, PartialEq, Debug, YaDeserialize)]
pub struct Placement {
    #[yaserde(attribute)]
    pub mode: String,
    #[yaserde(child)]
    pub alignment: Alignment,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
pub struct Alignment {
    #[yaserde(attribute)]
    pub horizontal: String,
    #[yaserde(attribute)]
    pub vertical: String,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
pub struct Data {
    #[yaserde(child, rename = "row")]
    pub rows: Vec<Row>,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
pub struct Row {
    #[yaserde(child, rename = "cell")]
    pub cells: Vec<Cell>,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
pub struct Cell {
    #[yaserde(attribute)]
    pub blocked: bool,
    #[yaserde(child)]
    pub ascii: u8,
    #[yaserde(child)]
    pub fgd: String,
    #[yaserde(child)]
    pub bkg: String,
}

#[cfg(test)]
mod tests {
    use std::fs;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_prefab_deser() {
        let xml = fs::read_to_string("prefabs/prefab.template.xml").expect("Should have been able to read the test file");
        println!("{}", xml);
        let p = yaserde::de::from_str::<Prefab>(xml.as_str()).unwrap();
        println!("{:?}", p);
    }
}