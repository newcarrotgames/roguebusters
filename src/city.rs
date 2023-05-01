use crate::{
    components::position::Position,
    deser::{
        generators::Generators,
        prefabs::{Cell, Prefab, Prefabs},
    },
};
use rand::Rng;
use std::collections::HashMap;
use tcod::{
    colors::{BLACK, BLUE, DARKEST_GREY, DARK_AMBER, LIGHT_GREY, WHITE},
    Color,
};

type Coord = [f32; 2];

const CITY_BLOCK_SIZE: i32 = 80;

const BUILDING_TEMPLATE_A: [Coord; 6] = [
    [0.0, 0.0],
    [1.0, 0.0],
    [1.0, 0.5],
    [0.5, 0.5],
    [0.5, 1.0],
    [0.0, 1.0],
];

const BUILDING_TEMPLATE_B: [Coord; 6] = [
    [0.0, 0.0],
    [1.0, 0.0],
    [1.0, 1.0],
    [0.5, 1.0],
    [0.5, 0.5],
    [0.0, 0.5],
];

const BUILDING_TEMPLATE_C: [Coord; 6] = [
    [0.0, 0.0],
    [0.5, 0.0],
    [0.5, 0.5],
    [1.0, 0.5],
    [1.0, 1.0],
    [0.0, 1.0],
];

const BUILDING_TEMPLATE_D: [Coord; 6] = [
    [0.5, 0.0],
    [1.0, 0.0],
    [1.0, 1.0],
    [0.0, 1.0],
    [0.0, 0.5],
    [0.5, 0.5],
];

pub type Rect = [i32; 4];
pub type Grid = Vec<Vec<Tile>>;

pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub building_id: i32,
    pub char: char,
    pub exterior_char: char,
    pub bg_color: Color,
    pub fg_color: Color,
    pub seen: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            building_id: 0,
            bg_color: DARKEST_GREY,
            fg_color: WHITE,
            char: 0 as char,
            exterior_char: 0 as char,
            seen: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
            building_id: 0,
            bg_color: Color::new(245, 245, 245),
            fg_color: Color::new(235, 235, 235),
            char: 219 as char,
            exterior_char: 0 as char,
            seen: false,
        }
    }

    pub fn sidewalk() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            building_id: 0,
            bg_color: LIGHT_GREY,
            fg_color: WHITE,
            char: ' ',
            exterior_char: 0 as char,
            seen: false,
        }
    }

    pub fn door(building_id: i32, dir: Direction) -> Self {
        let char;
        match dir {
            Direction::NORTH => char = 220 as char,
            Direction::EAST => char = 221 as char,
            Direction::SOUTH => char = 223 as char,
            Direction::WEST => char = 222 as char,
        }

        Tile {
            blocked: false,
            block_sight: true,
            building_id,
            bg_color: LIGHT_GREY,
            fg_color: DARK_AMBER,
            char,
            exterior_char: 0 as char,
            seen: false,
        }
    }

    pub fn water() -> Self {
        Tile {
            blocked: true,
            block_sight: false,
            building_id: 0,
            bg_color: BLUE,
            fg_color: WHITE,
            char: ' ',
            exterior_char: 0 as char,
            seen: false,
        }
    }

    pub fn interior(building_id: i32) -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            building_id,
            bg_color: BLACK,
            fg_color: WHITE,
            char: 0 as char,
            exterior_char: 178 as char,
            seen: false,
        }
    }

    fn from_cell(cell: &Cell, building_id: i32) -> Tile {
        Tile {
            blocked: cell.blocked,
            block_sight: false,
            building_id,
            bg_color: Tile::parse_hex_color(cell.bkg.as_str()),
            fg_color: Tile::parse_hex_color(cell.fgd.as_str()),
            char: cell.ascii as char,
            exterior_char: 178 as char,
            seen: false,
        }
    }

    // assumes "#RRGGBB" format
    fn parse_hex_color(hex: &str) -> Color {
        let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
        let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
        let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
        Color::new(r, g, b)
    }
}

pub struct Building {
    id: i32,
}

impl Building {
    fn new(id: i32) -> Self {
        Building { id: id }
    }
}

#[derive(Default)]
pub struct City {
    pub width: i32,
    pub height: i32,
    pub data: Grid,
    pub buildings: HashMap<i32, Building>,
}

/*

A: ██
   █

B: ██
    █

C: █
   ██

D:  █
   ██

 */

#[derive(PartialEq, Clone, Copy, Debug)]
enum BuildingOrientation {
    Vertical,
    Horizontal,
    A,
    B,
    C,
    D,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum BuildingType {
    Empty,
    Single,
    Double(BuildingOrientation), // true horizontal, false vertical
    Triple(BuildingOrientation),
}

impl BuildingType {
    fn random() -> BuildingType {
        let mut rng = rand::thread_rng();
        let btype = rng.gen_range(0..100);
        if (70..80).contains(&btype) {
            return BuildingType::Double(BuildingOrientation::Horizontal);
        } else if (80..90).contains(&btype) {
            return BuildingType::Double(BuildingOrientation::Vertical);
        } else if (90..100).contains(&btype) {
            let subbtype = rng.gen_range(0..4);
            if subbtype == 0 {
                return BuildingType::Triple(BuildingOrientation::A);
            } else if subbtype == 1 {
                return BuildingType::Triple(BuildingOrientation::B);
            } else if subbtype == 2 {
                return BuildingType::Triple(BuildingOrientation::C);
            } else if subbtype == 3 {
                return BuildingType::Triple(BuildingOrientation::D);
            }
        }
        BuildingType::Single
    }
}

struct BuildingGuide {
    building_type: BuildingType,
    x: i32,
    y: i32,
}

impl BuildingGuide {
    fn place(blocks: &mut Vec<Vec<BuildingType>>) -> BuildingGuide {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..blocks[0].len() - 2) as i32;
        let y = rng.gen_range(0..blocks.len() - 2) as i32;

        // pick building type
        let building_type = BuildingType::random();
        let mut points: Vec<[i32; 2]> = Vec::new();
        match building_type {
            BuildingType::Single => points.push([x, y]),
            BuildingType::Double(orientation) => match orientation {
                BuildingOrientation::Vertical => {
                    points.push([x, y]);
                    points.push([x, y + 1]);
                }
                BuildingOrientation::Horizontal => {
                    points.push([x, y]);
                    points.push([x + 1, y]);
                }
                _ => log::error!(
                    "building orientation is not vertical or horizontal for building type double"
                ),
            },
            BuildingType::Triple(orientation) => match orientation {
                BuildingOrientation::A => {
                    points.push([x, y]);
                    points.push([x + 1, y]);
                    points.push([x, y + 1]);
                }
                BuildingOrientation::B => {
                    points.push([x, y]);
                    points.push([x + 1, y]);
                    points.push([x + 1, y + 1]);
                }
                BuildingOrientation::C => {
                    points.push([x, y]);
                    points.push([x, y + 1]);
                    points.push([x + 1, y + 1]);
                }
                BuildingOrientation::D => {
                    points.push([x + 1, y]);
                    points.push([x, y + 1]);
                    points.push([x + 1, y + 1]);
                }
                _ => log::error!(
                    "building orientation is not A, B, C, or D for building type triple"
                ),
            },
            BuildingType::Empty => log::error!("guide is empty!"),
        }

        let mut is_empty = true;
        for point in &points {
            if point[0] >= blocks[0].len() as i32 - 2 || point[1] >= blocks.len() as i32 - 2 {
                is_empty = false;
                break;
            }
            if blocks[point[1] as usize][point[0] as usize] != BuildingType::Empty {
                is_empty = false;
                break;
            }
        }

        if is_empty {
            for point in &points {
                blocks[point[1] as usize][point[0] as usize] = building_type;
            }
            return BuildingGuide {
                building_type,
                x,
                y,
            };
        }

        BuildingGuide {
            building_type: BuildingType::Empty,
            x: 0,
            y: 0,
        }
    }
}

impl City {
    pub fn new(width: i32, height: i32) -> Self {
        let data = vec![vec![Tile::empty(); height as usize]; width as usize];
        let buildings = HashMap::new();
        City {
            width,
            height,
            data,
            buildings,
        }
    }

    pub fn build(&mut self, prefabs: Prefabs) {
        // add water
        for y in 0..self.height {
            for x in self.width - 100..self.width {
                self.data[y as usize][x as usize] = Tile::water();
            }
        }

        let mut rng = rand::thread_rng();

        let mut horizontal_guides: Vec<i32> = Vec::new();
        let mut vertical_guides: Vec<i32> = Vec::new();

        horizontal_guides.push(0);
        vertical_guides.push(0);

        let mut x = 0;
        while x < self.width - 200 {
            // try not to generate buildings out in the water
            let g = CITY_BLOCK_SIZE + rng.gen_range(-9..10);
            x += g;
            horizontal_guides.push(x);
        }

        let mut y = 0;
        while y < self.height - 100 {
            let g = CITY_BLOCK_SIZE + rng.gen_range(-9..10);
            y += g;
            vertical_guides.push(y);
        }

        // city block grid
        let mut empty_blocks = ((vertical_guides.len() - 1) * (horizontal_guides.len() - 1)) as i32;
        let mut blocks_grid: Vec<Vec<BuildingType>> = Vec::new();
        for _ in 0..vertical_guides.len() + 1 {
            let mut row = Vec::new();
            for _ in 0..horizontal_guides.len() + 1 {
                row.push(BuildingType::Empty);
            }
            blocks_grid.push(row);
        }

        let mut building_guides: Vec<BuildingGuide> = Vec::new();
        let mut previous_empty_blocks = empty_blocks;
        while empty_blocks > 0 {
            // attempt to place building
            let building_guide = BuildingGuide::place(&mut blocks_grid);
            match building_guide.building_type {
                BuildingType::Empty => (),
                BuildingType::Single => empty_blocks -= 1,
                BuildingType::Double(_) => empty_blocks -= 2,
                BuildingType::Triple(_) => empty_blocks -= 3,
            }

            if previous_empty_blocks != empty_blocks {
                log::info!("empty blocks: {}", empty_blocks);
                previous_empty_blocks = empty_blocks;
            }

            if building_guide.building_type != BuildingType::Empty {
                building_guides.push(building_guide);
            }
        }

        // draw buildings

        let mut generators = Generators::new("data/generators");
        generators.load_all();

        log::info!("horizontal guides: {:?}", horizontal_guides);
        log::info!("vertical guides: {:?}", vertical_guides);

        let mut building_id = 0;

        for building_guide in building_guides {
            building_id += 1;
            match building_guide.building_type {
                BuildingType::Empty => log::error!("building guide type is empty!"),
                BuildingType::Single => {
                    let x1 = horizontal_guides[building_guide.x as usize];
                    let y1 = vertical_guides[building_guide.y as usize];
                    let x2 = horizontal_guides[(building_guide.x + 1) as usize];
                    let y2 = vertical_guides[(building_guide.y + 1) as usize];

                    let mut offset = 10;

                    // sidewalks
                    self.rect(
                        x1 + offset,
                        y1 + offset,
                        x2 - offset,
                        y2 - offset,
                        Tile::sidewalk(),
                    );

                    offset += 1;
                    self.rect(
                        x1 + offset,
                        y1 + offset,
                        x2 - offset,
                        y2 - offset,
                        Tile::sidewalk(),
                    );

                    // exterior walls
                    offset += 1;
                    self.rect(
                        x1 + offset,
                        y1 + offset,
                        x2 - offset,
                        y2 - offset,
                        Tile::wall(),
                    );

                    // interior
                    offset += 1;
                    self.filled_rect(
                        x1 + offset,
                        y1 + offset,
                        x2 - offset,
                        y2 - offset,
                        Tile::interior(building_id),
                    );
                }
                BuildingType::Double(orientation) => {
                    let x1 = horizontal_guides[building_guide.x as usize];
                    let y1 = vertical_guides[building_guide.y as usize];
                    let mut x2 = 0;
                    let mut y2 = 0;
                    match orientation {
                        BuildingOrientation::Vertical => {
                            x2 = horizontal_guides[(building_guide.x + 1) as usize];
                            y2 = vertical_guides[(building_guide.y + 2) as usize];
                        }
                        BuildingOrientation::Horizontal => {
                            x2 = horizontal_guides[(building_guide.x + 2) as usize];
                            y2 = vertical_guides[(building_guide.y + 1) as usize];
                        }
                        _ => log::error!(
                            "building type double has wrong orientation {:?}",
                            orientation
                        ),
                    }

                    let mut offset = 10;

                    // sidewalks
                    self.rect(
                        x1 + offset,
                        y1 + offset,
                        x2 - offset,
                        y2 - offset,
                        Tile::sidewalk(),
                    );

                    offset += 1;
                    self.rect(
                        x1 + offset,
                        y1 + offset,
                        x2 - offset,
                        y2 - offset,
                        Tile::sidewalk(),
                    );

                    // buildings
                    offset += 1;
                    self.rect(
                        x1 + offset,
                        y1 + offset,
                        x2 - offset,
                        y2 - offset,
                        Tile::wall(),
                    );

                    // interior
                    offset += 1;
                    self.filled_rect(
                        x1 + offset,
                        y1 + offset,
                        x2 - offset,
                        y2 - offset,
                        Tile::interior(building_id),
                    );
                }
                BuildingType::Triple(orientation) => {
                    log::info!("-------------------------------- TRIPLE START --------------------------------");
                    let x1 = horizontal_guides[building_guide.x as usize];
                    let y1 = vertical_guides[building_guide.y as usize];
                    let x2 = horizontal_guides[(building_guide.x + 2) as usize];
                    let y2 = vertical_guides[(building_guide.y + 2) as usize];

                    log::info!("x1: {}, y1: {}, x2: {}, y2: {}", x1, y1, x2, y2);

                    let building_width = x2 - x1;
                    let building_height = y2 - y1;

                    log::info!("width: {}, height: {}", building_width, building_height);

                    let mut offset = 10;
                    
                    let template = match orientation {
                        BuildingOrientation::A => BUILDING_TEMPLATE_A,
                        BuildingOrientation::B => BUILDING_TEMPLATE_B,
                        BuildingOrientation::C => BUILDING_TEMPLATE_C,
                        BuildingOrientation::D => BUILDING_TEMPLATE_D,
                        BuildingOrientation::Vertical => todo!(),
                        BuildingOrientation::Horizontal => todo!(),
                    };

                    log::info!("template: {:?}", template);

                    for i in 0..3 {
                        log::info!("i: {}", i);
                        let t:Tile = match i {
                            0 => Tile::sidewalk(),
                            1 => Tile::sidewalk(),
                            2 => Tile::wall(),
                            _ => Tile::empty(),
                        };
                        // let x = x1 + offset;
                        // let y = y1 + offset;
                        self.draw_template(x1, y1, building_width, building_height, template, t, offset);
                        offset += 1;
                    }

                    log::info!("-------------------------------- TRIPLE END ----------------------------------");
                }
            }
        }

        // let mut rng = rand::thread_rng();

        // let mut last_y = 0;
        // for cy in vertical_guides.iter() {
        //     let mut last_x = 0;
        //     for cx in horizontal_guides.iter() {
        //         // don't render buildings off the map
        //         if *cx > self.width || *cy > self.height {
        //             continue;
        //         }

        //         let building_id = (self.buildings.len() + 1) as i32;
        //         let building = Building::new(building_id);
        //         self.buildings.insert(building.id, building);

        //         let mut offset = 8;

        //         // sidewalks
        //         self.rect(
        //             last_x + offset,
        //             last_y + offset,
        //             *cx - offset,
        //             *cy - offset,
        //             Tile::sidewalk(),
        //         );

        //         offset += 1;
        //         self.rect(
        //             last_x + offset,
        //             last_y + offset,
        //             *cx - offset,
        //             *cy - offset,
        //             Tile::sidewalk(),
        //         );

        //         // buildings
        //         offset += 1;
        //         self.rect(
        //             last_x + offset,
        //             last_y + offset,
        //             *cx - offset,
        //             *cy - offset,
        //             Tile::wall(),
        //         );

        //         // let building_x = last_x + offset;
        //         // let building_y = last_y + offset;
        //         let building_width = (*cx - offset) - (last_x + offset);
        //         let building_height = (*cy - offset) - (last_y + offset);

        //         // entrances
        //         let side: i32 = rng.gen_range(0..4);
        //         let mut door: Position = Position::zero();
        //         let dir: Direction;
        //         match side {
        //             0 => {
        //                 door.x = (last_x + offset) as f32; // left wall
        //                 door.y = ((last_y + offset) + building_height / 2) as f32;
        //                 dir = Direction::WEST;
        //             }
        //             1 => {
        //                 door.x = ((last_x + offset) + building_width / 2) as f32; // top wall
        //                 door.y = (last_y + offset) as f32;
        //                 dir = Direction::NORTH;
        //             }
        //             2 => {
        //                 door.x = (*cx - offset) as f32; // right wall
        //                 door.y = ((last_y + offset) + building_height / 2) as f32;
        //                 dir = Direction::EAST;
        //             }
        //             3 => {
        //                 door.x = ((last_x + offset) + building_width / 2) as f32; // bottom wall
        //                 door.y = (*cy - offset) as f32;
        //                 dir = Direction::SOUTH;
        //             }
        //             _ => unreachable!(),
        //         }

        //         self.data[door.y as usize][door.x as usize] = Tile::door(building_id, dir);

        //         offset += 1;
        //         let interior: Rect = [last_x + offset, last_y + offset, *cx - offset, *cy - offset];
        //         self.filled_rect(
        //             last_x + offset,
        //             last_y + offset,
        //             *cx - offset,
        //             *cy - offset,
        //             Tile::interior(building_id),
        //         );

        //         let gen = generators.get("building_interior", "restaurant");

        //         for rule in gen.rules.rules.iter() {
        //             let prefab = prefabs.get(rule.name.as_str());
        //             match rule.frequency.as_str() {
        //                 "one" => loop {
        //                     let x = rng.gen_range(interior[0]..interior[2] - 2);
        //                     let y = rng.gen_range(interior[1]..interior[3] - 2);
        //                     if self.can_place_prefab(x, y, prefab) {
        //                         self.draw_prefab(x, y, prefab, building_id);
        //                         break;
        //                     }
        //                 },
        //                 "many" => {
        //                     for y in interior[1]..interior[3] - 2 {
        //                         for x in interior[0]..interior[2] - 2 {
        //                             let range_limit = (1.0 / rule.chance) as usize;
        //                             if rng.gen_range(0..range_limit) == 0 {
        //                                 if self.can_place_prefab(x, y, prefab) {
        //                                     self.draw_prefab(x, y, prefab, building_id);
        //                                 }
        //                             }
        //                         }
        //                     }
        //                 }
        //                 _ => unreachable!(),
        //             }
        //         }
        //         last_x = *cx;
        //     }
        //     last_y = *cy;
        // }
    }

    pub fn rect(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, tile: Tile) {
        for y in y1..y2 + 1 {
            for x in x1..x2 + 1 {
                if x == x1 || x == x2 || y == y1 || y == y2 {
                    // shame shame shame
                    self.data[y as usize][x as usize] = tile;
                }
            }
        }
    }

    pub fn filled_rect(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, tile: Tile) {
        for y in y1..y2 + 1 {
            for x in x1..x2 + 1 {
                self.data[y as usize][x as usize] = tile;
            }
        }
    }

    pub fn get_random_target(&self) -> Position {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(100..self.width - 100);
            let y = rng.gen_range(100..self.height - 100);
            if self.data[y as usize][x as usize].blocked
                || self.data[y as usize][x as usize].building_id > 0
            {
                continue;
            }
            return Position {
                x: x as f32,
                y: y as f32,
            };
        }
    }

    fn draw_prefab(&mut self, x: i32, y: i32, prefab: &Prefab, building_id: i32) {
        for (py, row) in prefab.data.rows.iter().enumerate() {
            for (px, cell) in row.cells.iter().enumerate() {
                self.data[y as usize + py][x as usize + px] = Tile::from_cell(cell, building_id);
            }
        }
    }

    fn can_place_prefab(&self, x: i32, y: i32, prefab: &Prefab) -> bool {
        for py in 0..prefab.height + 1 {
            for px in 0..prefab.width + 1 {
                let tile = self.data[(y + py) as usize][(x + px) as usize];
                if tile.char != 0 as char {
                    return false;
                }
            }
        }
        return true;
    }

    fn draw_template(&mut self, x: i32, y: i32, width: i32, height: i32, template: [[f32; 2]; 6], t: Tile, offset: i32) {
        let w = width - offset * 2;
        let h = height - offset * 2;

        log::info!("w: {:?}, h: {:?}", w, h);

        for i in 0..5 {
            let p1 = template[i];
            let p2 = template[i + 1];

            log::info!("p1: {:?}, p2: {:?}", p1, p2);

            let x1 = x as f32 + offset as f32 + p1[0] * w as f32;
            let y1 = y as f32 + offset as f32 + p1[1] * h as f32;

            let x2 = x as f32 + offset as f32 + p2[0] * w as f32;
            let y2 = y as f32 + offset as f32 + p2[1] * h as f32;

            self.line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, t);
        }

        // final line
        let p1 = template[5];
        let p2 = template[0];

        let x1 = x as f32 + offset as f32 + p1[0] * w as f32;
        let y1 = y as f32 + offset as f32 + p1[1] * h as f32;

        let x2 = x as f32 + offset as f32 + p2[0] * w as f32;
        let y2 = y as f32 + offset as f32 + p2[1] * h as f32;

        self.line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, t);
    }

    fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, t: Tile) {
        if x1 == x2 {
            self.vertical_line(x1, y1, y2, t)
        } else {
            self.horizontal_line(x1, y1, x2, t);
        }
    }

    fn horizontal_line(&mut self, x1: i32, y: i32, x2: i32, t: Tile) {
        log::info!("horizontal_line - x1: {}, y: {}, x2: {}", x1, y, x2);
        let mut x_1 = x1;
        let mut x_2 = x2;
        if x1 > x2 {
            x_1 = x2;
            x_2 = x1;
        }
        for x in x_1..x_2 + 1 {
            self.data[y as usize][x as usize] = t;
        }
    }

    fn vertical_line(&mut self, x: i32, y1: i32, y2: i32, t: Tile) {
        log::info!("vertical_line - x: {}, y1: {}, y2: {}", x, y1, y2);
        let mut y_1 = y1;
        let mut y_2 = y2;
        if y1 > y2 {
            y_1 = y2;
            y_2 = y1;
        }
        for y in y_1..y_2 + 1 {
            self.data[y as usize][x as usize] = t;
        }
    }
}
