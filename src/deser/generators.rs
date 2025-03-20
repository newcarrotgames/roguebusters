extern crate xml;

use std::{collections::HashMap, fs};
use std::path::Path;
use walkdir::WalkDir;
use yaserde_derive::{YaDeserialize};

#[derive(Default, Debug)]
pub struct Generators {
    folder: String,
    generators: HashMap<String, HashMap<String, Generator>>,
}

impl Generators {
    pub fn new(folder: &str) -> Self {
        return Generators {
            folder: String::from(folder),
            generators: HashMap::new(),
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
            let gen = Generator::from_xml(entry.path());
            log::debug!("gen {:?}", gen);
            let gmap = self.generators.entry(gen.gen_type.clone()).or_insert_with(HashMap::new);
            gmap.insert(gen.name.clone(), gen);
            log::debug!("done loading {}", entry.path().display());
        }
    }

    pub fn get(&self, gentype: &str, name: &str) -> &Generator {
        let gentypes = self.generators.get(gentype).unwrap();
        gentypes.get(name).unwrap()
    }
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(rename = "generator")]
pub struct Generator {
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(attribute, rename="type")]
    pub gen_type: String,
    #[yaserde(child)]
    pub rules: Rules
}

impl Generator {
	pub fn from_xml(path: &Path) -> Self {
        let xml = fs::read_to_string(path).expect(format!("Error reading prefab file {:?}", path).as_str());
        yaserde::de::from_str::<Generator>(xml.as_str()).unwrap()
    }
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
pub struct Rules {
    #[yaserde(child, rename = "rule")]
    pub rules: Vec<Rule>,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
pub struct Rule {
    #[yaserde(attribute, rename = "type")]
    pub ruletype: String,
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(attribute)]
    pub frequency: String,
    #[yaserde(attribute)]
    pub chance: f32,
}

#[cfg(test)]
mod tests {
    use std::fs;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_prefab_deser() {
        let xml = fs::read_to_string("data/generators/restaurant.generator.xml").expect("Should have been able to read the test file");
        println!("{}", xml);
        let p = yaserde::de::from_str::<Generator>(xml.as_str()).unwrap();
        println!("{:?}", p);
    }

    #[test]
    fn test_load() {
        let mut gens = Generators::new("data/generators");
        gens.load_all();
        println!("{:?}", gens);
    }

    #[test]
    fn test_get() {
        let mut gens = Generators::new("data/generators");
        gens.load_all();
        println!("{:?}", gens.get("building_interior", "restaurant"));
    }
}