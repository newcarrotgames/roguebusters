extern crate xml;

use rand::Rng;
use std::path::Path;
use std::{collections::HashMap, fs};
use walkdir::WalkDir;
use yaserde_derive::YaDeserialize;

/*

The Templates struct contains useful patterns for generating text.

Examples:
    - NPC names
    - Business names
    - Building names
    - Simple dialog between the player and NPCs

*/
#[derive(Default, Debug)]
pub struct Templates {
    folder: String,
    templates: HashMap<String, HashMap<String, Vec<Template>>>,
}

impl Templates {
    pub fn new(folder: &str) -> Self {
        return Templates {
            folder: String::from(folder),
            templates: HashMap::new(),
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
            let tc = TemplatesCollection::from_xml(entry.path());
            for t in tc.templates.iter() {
                let tt = t.template_type.clone();

                // Use the entry API to work with the outer HashMap
                let tmap = self
                    .templates
                    .entry(tt.clone())
                    .or_insert_with(HashMap::new);

                // Use the entry API to work with the nested HashMap
                let ts = tmap.entry(t.subtype.clone()).or_insert_with(Vec::new);

                // Push the new template into the Vec
                ts.push(Template {
                    template_type: t.template_type.clone(),
                    subtype: t.subtype.clone(),
                    text: t.text.clone(),
                });
            }
        }
    }

    pub fn get_random_template(&self, template_type: &str, subtype: &str) -> &Template {
        let subtypes = self.templates.get(template_type).unwrap();
        let templates = subtypes.get(subtype).unwrap();
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..templates.len());
        return templates.get(i).unwrap();
    }
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(rename = "templates")]
pub struct TemplatesCollection {
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(child, rename = "template")]
    pub templates: Vec<Template>,
}

impl TemplatesCollection {
    pub fn from_xml(path: &Path) -> Self {
        let xml = fs::read_to_string(path)
            .expect(format!("Error reading prefab file {:?}", path).as_str());
        yaserde::de::from_str::<TemplatesCollection>(xml.as_str()).unwrap()
    }
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(rename = "template")]
pub struct Template {
    #[yaserde(child, rename = "type")]
    pub template_type: String,
    #[yaserde(child)]
    pub subtype: String,
    #[yaserde(child)]
    pub text: String,
}

#[cfg(test)]
mod tests {
    use std::fs;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_template_deser() {
        let xml = "<template>
		<type>business</type>
		<subtype>restaurant</subtype>
		<text>[LAST_NAME]'s Family Restaurant</text>
	</template>";
        let t = yaserde::de::from_str::<Template>(xml).unwrap();
        assert!(t.template_type == "business", "wrong template type");
        assert!(t.subtype == "restaurant", "wrong template type");
    }

    #[test]
    fn test_load_all() {
        let mut ts = Templates::new("data/templates");
        ts.load_all();
        println!("{:?}", ts);
    }
}
