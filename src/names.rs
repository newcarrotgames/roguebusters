use std::fs;

use rand::Rng;

use crate::deser::templates::{Templates, self};

pub enum NameType {
    FemaleFirstName,
    MaleFirstName,
    LastName,
    FemaleFullName,
    MaleFullName,
    AnyFullName,
    RestaurantName,
    BuildingName,
    StreetName,
    Direction
}

pub struct Names {
    female_first_names: Vec<String>,
    male_first_names: Vec<String>,
    last_names: Vec<String>,
    templates: Templates,
}

impl Names {
    pub fn new() -> Self {
        let female_first_names = Names::read_names_file("data/names/female_names.txt");
        let male_first_names = Names::read_names_file("data/names/male_names.txt");
        let last_names = Names::read_names_file("data/names/last_names.txt");
        let mut templates = Templates::new("data/templates");
		templates.load_all();
        Names {
            female_first_names,
            male_first_names,
            last_names,
            templates,
        }
    }

    fn read_names_file(path: &str) -> Vec<String> {
        let names = fs::read_to_string(path)
            .expect(format!("Error reading names file {:?}", path).as_str());
        // split names by newlines
        let names: Vec<String> = names
            .split("\r")
            .map(|s| Names::filter_newline(s.to_string()))
            .collect();
        // convert to strings
        let names: Vec<String> = names.iter().map(|s| s.to_string()).collect();
        names
    }

    pub fn get_random_name(&self, name_type: NameType) -> String {
        let mut rng = rand::thread_rng();
        match name_type {
            NameType::FemaleFirstName => {
                let i = rng.gen_range(0..self.female_first_names.len());
                self.female_first_names[i].clone()
            }
            NameType::MaleFirstName => {
                let i = rng.gen_range(0..self.male_first_names.len());
                self.male_first_names[i].clone()
            }
            NameType::LastName => {
                let i = rng.gen_range(0..self.last_names.len());
                self.last_names[i].clone()
            }
            NameType::MaleFullName => {
                let first_name = self.get_random_name(NameType::MaleFirstName);
                let last_name = self.get_random_name(NameType::LastName);
                format!("{} {}", first_name, last_name)
            }
            NameType::FemaleFullName => {
                let first_name = self.get_random_name(NameType::FemaleFirstName);
                let last_name = self.get_random_name(NameType::LastName);
                format!("{} {}", first_name, last_name)
            }
            NameType::AnyFullName => {
                if rng.gen_bool(0.5) {
                    self.get_random_name(NameType::FemaleFullName)
                } else {
                    self.get_random_name(NameType::MaleFullName)
                }
            }
            NameType::RestaurantName => {
                let last_name = self.get_random_name(NameType::LastName);
                let template = self
                    .templates
                    .get_random_template("business", "restaurant");
                self.process_template(template, TemplateData {
					last_name: Some(last_name),
                    street_name: None,
                    direction: None,
				})
            }
            NameType::StreetName => {
                let num = rng.gen_range(1..100);
                self.number_to_position(num)
            }
            NameType::BuildingName => {
                let last_name = self.get_random_name(NameType::LastName);
                let street_name = self.get_random_name(NameType::StreetName);
                let direction = self.get_random_name(NameType::Direction);
                let template = self
                    .templates
                    .get_random_template("building", "building");
                self.process_template(template, TemplateData {
					last_name: Some(last_name),
                    street_name: Some(street_name),
                    direction: Some(direction),
				})
            }
            NameType::Direction => {
                match rng.gen_range(0..4) {
                    0 => String::from("North"),
                    1 => String::from("South"),
                    2 => String::from("East"),
                    3 => String::from("West"),
                    _ => String::from(""),
                }
            }
            _ => String::from(""),
        }
    }

    fn filter_newline(to_string: String) -> String {
        to_string.replace("\r", "").replace("\n", "")
    }

    fn process_template(&self, template: &templates::Template, data: TemplateData) -> String {
        let last_name = data.last_name.as_deref().unwrap_or("");
        let street_name = data.street_name.as_deref().unwrap_or("");
        let direction = data.direction.as_deref().unwrap_or("");
        let mut s = template.text.clone();
        s = s.replace("[LAST_NAME]", last_name);
        s = s.replace("[NUMBERED_STREET]", street_name);
        s = s.replace("[DIRECTION]", direction);
        s
	}

    fn number_to_position(&self, n: i32) -> String {
        let suffix = match (n % 10, n % 100) {
            (1, 1) => "st",
            (2, 2) => "nd",
            (3, 3) => "rd",
            _ => "th",
        };
        format!("{}{}", n, suffix)
    }
}

struct TemplateData {
    last_name: Option<String>,
    street_name: Option<String>,
    direction: Option<String>,
}

#[cfg(test)]
mod tests {
    use std::fs;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_names() {
        let n = Names::new();
        println!("Full name: {}", n.get_random_name(NameType::AnyFullName));
        println!("Full name: {}", n.get_random_name(NameType::AnyFullName));
        println!("Full name: {}", n.get_random_name(NameType::AnyFullName));
        println!("Full name: {}", n.get_random_name(NameType::AnyFullName));
        println!("Full name: {}", n.get_random_name(NameType::AnyFullName));
		println!("Restaurant name: {}", n.get_random_name(NameType::RestaurantName));
		println!("Building name: {}", n.get_random_name(NameType::BuildingName));
    }

    #[test]
    fn test_num_to_position() {
        let names = Names::new();
        let numbers = vec![1, 2, 3, 4, 10, 11, 12, 13, 21, 22, 23, 101, 102, 103];
        for n in numbers {
            println!("{}", names.number_to_position(n));
        }
    }
}
