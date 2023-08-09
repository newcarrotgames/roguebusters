use crate::{
    city::building::{
        BuildingGuide, BuildingOrientation, BuildingType, BUILDING_TEMPLATE_A, BUILDING_TEMPLATE_B,
        BUILDING_TEMPLATE_C, BUILDING_TEMPLATE_D,
    },
    components::position::Position,
    deser::{
        generators::Generators,
        prefabs::{Cell, Prefab, Prefabs},
    },
};
use log::{error, info};
use rand::Rng;
use std::{collections::HashMap, error::Error, fmt};
use tcod::{
    colors::{BLACK, BLUE, DARK_AMBER, LIGHT_GREY, WHITE},
    Color,
};

use super::building::Building;

pub type Coord = [f32; 2];
const NULLCHAR: char = 0 as char;
const CITY_BLOCK_SIZE: i32 = 100;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Rect {
        // // return error when x's or y's are the same
        // if x1 == x2 {
        //     return Err(RectError::new("x1 is equal to x2"));
        // }
        // if y1 == y2 {
        //     return Err(RectError::new("y1 is equal to y2"));
        // }

        // // swap values if first is larger than second
        // if x1 > x2 {
        //     let x_ = x2;
        //     x2 = x1;
        //     x1 = x_;
        // }
        // if y1 > y2 {
        //     let y_ = y2;
        //     y2 = y1;
        //     y1 = y_;
        // }

        Rect { x1, y1, x2, y2 }
    }

    pub fn width(&self) -> i32 {
        self.x2 - self.x1
    }

    pub fn height(&self) -> i32 {
        self.y2 - self.y1
    }

    pub fn size(&self) -> i32 {
        (self.width() * self.height()) as i32
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{},{}", self.x1, self.y1, self.x2, self.y2)
    }
}

#[derive(Debug, Clone)]
pub struct RectError {
    msg: String,
}

impl RectError {
    fn new(msg: &str) -> Self {
        RectError {
            msg: msg.to_string(),
        }
    }
}

impl Error for RectError {}

impl fmt::Display for RectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rect error: {}", self.msg)
    }
}

pub type Grid = Vec<Vec<Tile>>;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

// convenience array to iterate directions
pub const DIRECTIONS: [Direction; 4] = [
    Direction::NORTH,
    Direction::EAST,
    Direction::SOUTH,
    Direction::WEST,
];

pub enum VerticalDirection {
    UP,
    DOWN,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileId {
    Empty,
    Wall,
    Sidewalk,
    Door,
    Stairs,
    Water,
    Interior,
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub tile_id: TileId,
    pub blocked: bool,
    pub block_sight: bool,
    pub building_id: i32,
    pub char: char,
    pub bg_color: Color,
    pub fg_color: Color,
    pub seen: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            tile_id: TileId::Empty,
            blocked: false,
            block_sight: false,
            building_id: 0,
            bg_color: BLACK,
            fg_color: WHITE,
            char: 0 as char,
            seen: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            tile_id: TileId::Wall,
            blocked: true,
            block_sight: true,
            building_id: 0,
            bg_color: Color::new(245, 245, 245),
            fg_color: Color::new(235, 235, 235),
            char: 219 as char,
            seen: false,
        }
    }

    pub fn sidewalk() -> Self {
        Tile {
            tile_id: TileId::Sidewalk,
            blocked: false,
            block_sight: false,
            building_id: 0,
            bg_color: LIGHT_GREY,
            fg_color: WHITE,
            char: ' ',
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
            tile_id: TileId::Door,
            blocked: true,
            block_sight: true,
            building_id,
            bg_color: LIGHT_GREY,
            fg_color: DARK_AMBER,
            char,
            seen: false,
        }
    }

    pub fn stairs(building_id: i32, dir: VerticalDirection) -> Self {
        let char = match dir {
            VerticalDirection::UP => 30 as char,
            VerticalDirection::DOWN => 31 as char,
        };
        Tile {
            tile_id: TileId::Stairs,
            blocked: false,
            block_sight: false,
            building_id,
            bg_color: BLACK,
            fg_color: WHITE,
            char,
            seen: false,
        }
    }

    pub fn water() -> Self {
        Tile {
            tile_id: TileId::Water,
            blocked: true,
            block_sight: false,
            building_id: 0,
            bg_color: BLUE,
            fg_color: WHITE,
            char: ' ',
            seen: false,
        }
    }

    pub fn interior(building_id: i32) -> Self {
        Tile {
            tile_id: TileId::Interior,
            blocked: false,
            block_sight: false,
            building_id,
            bg_color: BLACK,
            fg_color: WHITE,
            char: ' ' as char,
            seen: false,
        }
    }

    fn from_cell(cell: &Cell, building_id: i32) -> Tile {
        Tile {
            tile_id: TileId::Interior,
            blocked: cell.blocked,
            block_sight: false,
            building_id,
            bg_color: Tile::parse_hex_color(cell.bkg.as_str()),
            fg_color: Tile::parse_hex_color(cell.fgd.as_str()),
            char: cell.ascii as char,
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

#[derive(Default)]
pub struct City {
    pub width: i32,
    pub height: i32,
    pub data: Grid,
    pub buildings: HashMap<i32, Building>,
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
        while x < self.width - 250 {
            let g = CITY_BLOCK_SIZE + rng.gen_range(-2..3);
            x += g;
            horizontal_guides.push(x);
        }

        let mut y = 0;
        while y < self.height - 100 {
            let g = CITY_BLOCK_SIZE + rng.gen_range(-2..3);
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
                BuildingType::Triple(_) => empty_blocks -= 4,
                BuildingType::Quad => empty_blocks -= 4,
            }

            if previous_empty_blocks != empty_blocks {
                info!("empty blocks: {}", empty_blocks);
                previous_empty_blocks = empty_blocks;
            }

            if building_guide.building_type != BuildingType::Empty {
                building_guides.push(building_guide);
            }
        }

        // draw buildings
        let mut generators = Generators::new("data/generators");
        generators.load_all();

        info!("horizontal guides: {:?}", horizontal_guides);
        info!("vertical guides: {:?}", vertical_guides);

        let mut building_id = 0;

        for building_guide in building_guides {
            building_id += 1;
            let mut rect: Rect = Rect {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
            };
            match building_guide.building_type {
                BuildingType::Empty => error!("building guide type is empty!"),
                BuildingType::Single => {
                    let x1 = horizontal_guides[building_guide.x as usize];
                    let y1 = vertical_guides[building_guide.y as usize];
                    let x2 = horizontal_guides[(building_guide.x + 1) as usize];
                    let y2 = vertical_guides[(building_guide.y + 1) as usize];

                    let mut offset = 16;
                    for i in 0..5 {
                        let t: Tile = match i {
                            0 | 1 | 2 | 3 => Tile::sidewalk(),
                            4 => Tile::wall(),
                            _ => Tile::empty(), // unnecessary?
                        };
                        self.rect(
                            x1 + offset + i,
                            y1 + offset + i,
                            x2 - offset - i,
                            y2 - offset - i,
                            t,
                        );
                    }

                    rect = Rect::new(
                        x1 + offset + 4,
                        y1 + offset + 4,
                        x2 - offset - 4,
                        y2 - offset - 4,
                    );
                    self.buildings
                        .insert(building_id, Building::new(building_id, rect));

                    // interior
                    offset += 5;
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
                        _ => error!(
                            "building type double has wrong orientation {:?}",
                            orientation
                        ),
                    }

                    let mut offset = 16;
                    for i in 0..5 {
                        let t: Tile = match i {
                            0 | 1 | 2 | 3 => Tile::sidewalk(),
                            4 => Tile::wall(),
                            _ => Tile::empty(), // unnecessary?
                        };
                        self.rect(
                            x1 + offset + i,
                            y1 + offset + i,
                            x2 - offset - i,
                            y2 - offset - i,
                            t,
                        );
                    }

                    rect = Rect::new(
                        x1 + offset + 4,
                        y1 + offset + 4,
                        x2 - offset - 4,
                        y2 - offset - 4,
                    );
                    self.buildings
                        .insert(building_id, Building::new(building_id, rect));

                    // interior
                    offset += 5;
                    self.filled_rect(
                        x1 + offset,
                        y1 + offset,
                        x2 - offset,
                        y2 - offset,
                        Tile::interior(building_id),
                    );
                }
                BuildingType::Triple(orientation) => {
                    let x1 = horizontal_guides[building_guide.x as usize];
                    let y1 = vertical_guides[building_guide.y as usize];
                    let x2 = horizontal_guides[(building_guide.x + 2) as usize];
                    let y2 = vertical_guides[(building_guide.y + 2) as usize];

                    let building_width = x2 - x1;
                    let building_height = y2 - y1;

                    let template = match orientation {
                        BuildingOrientation::A => BUILDING_TEMPLATE_A,
                        BuildingOrientation::B => BUILDING_TEMPLATE_B,
                        BuildingOrientation::C => BUILDING_TEMPLATE_C,
                        BuildingOrientation::D => BUILDING_TEMPLATE_D,
                        BuildingOrientation::Vertical => todo!(),
                        BuildingOrientation::Horizontal => todo!(),
                    };

                    let sidewalk_offset = 16;
                    self.filled_rect(
                        x1 + sidewalk_offset,
                        y1 + sidewalk_offset,
                        x2 - sidewalk_offset,
                        y2 - sidewalk_offset,
                        Tile::sidewalk(),
                    );

                    let offset = 32;
                    self.draw_template(
                        x1,
                        y1,
                        building_width,
                        building_height,
                        template,
                        Tile::wall(),
                        offset,
                        5,
                        orientation,
                    );

                    // floodfill building interior
                    let fill_offset = match orientation {
                        BuildingOrientation::A => (10 + offset, 10 + offset),
                        BuildingOrientation::B => (10 + offset, 10 + offset),
                        BuildingOrientation::C => (10 + offset, 10 + offset),
                        BuildingOrientation::D => {
                            (building_width - 10 - offset, building_height - 10 - offset)
                        }
                        _ => todo!(),
                    };
                    self.fill(
                        x1 + fill_offset.0,
                        y1 + fill_offset.1,
                        Tile::interior(building_id),
                    );
                }
                BuildingType::Quad => {
                    let x1 = horizontal_guides[building_guide.x as usize];
                    let y1 = vertical_guides[building_guide.y as usize];
                    let x2 = horizontal_guides[(building_guide.x + 2) as usize];
                    let y2 = vertical_guides[(building_guide.y + 2) as usize];

                    let mut offset = 16;
                    for i in 0..5 {
                        let t: Tile = match i {
                            0 | 1 | 2 | 3 => Tile::sidewalk(),
                            4 => Tile::wall(),
                            _ => Tile::empty(), // unnecessary?
                        };
                        self.rect(
                            x1 + offset + i,
                            y1 + offset + i,
                            x2 - offset - i,
                            y2 - offset - i,
                            t,
                        );
                    }

                    // interior
                    offset += 5;
                    self.filled_rect(
                        x1 + offset,
                        y1 + offset,
                        x2 - offset,
                        y2 - offset,
                        Tile::interior(building_id),
                    );
                }
            }
        }

        let mut buildings: Vec<&mut Building> = Vec::new();
        for (_, value) in self.buildings.iter_mut() {
            buildings.push(value);
        }

        // subdivide buildings
        for building in buildings.iter_mut() {
            info!("--------------------------- subdividing building ---------------------------");
            Building::subdivide_space(&mut building.root(), &mut self.data, 0);
            Building::add_doors(&mut building.root(), &mut self.data);
            Building::add_stairs(building, &mut self.data);
            // Building::populate(&mut building.root(), &mut self.data);
        }
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

    // todo: add building id
    fn can_place_prefab(&self, x: i32, y: i32, prefab: &Prefab) -> bool {
        for py in 0..prefab.height + 1 {
            for px in 0..prefab.width + 1 {
                let tile = self.data[(y + py) as usize][(x + px) as usize];
                if tile.char != NULLCHAR {
                    return false;
                }
            }
        }
        return true;
    }

    fn draw_template(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        template: [Coord; 7],
        t: Tile,
        offset: i32,
        wall_offset: i32,
        orientation: BuildingOrientation,
    ) {
        let origin = [x + width / 2, y + height / 2];
        let w = (width - offset) / 2;
        let h = (height - offset) / 2;

        let mut points: Vec<Coord> = Vec::new();

        for p in template {
            let (x, y);
            if p[0].abs() == 1.0 && p[1].abs() == 1.0 {
                x = p[0] * w as f32 + origin[0] as f32 + -wall_offset as f32 * p[0];
                y = p[1] * h as f32 + origin[1] as f32 + -wall_offset as f32 * p[1];
            } else {
                #[rustfmt::skip]
                let off = match orientation {
                    BuildingOrientation::A => [-1.0, -1.0],
                    BuildingOrientation::B => [ 1.0, -1.0],
                    BuildingOrientation::C => [-1.0,  1.0],
                    BuildingOrientation::D => [ 1.0,  1.0],
                    _ => todo!(),
                };
                x = p[0] * w as f32 + origin[0] as f32 + (off[0] * wall_offset as f32);
                y = p[1] * h as f32 + origin[1] as f32 + (off[1] * wall_offset as f32);
            }
            points.push([x, y]);
        }
        for i in 0..points.len() - 1 {
            let p1 = points[i];
            let p2 = points[i + 1];
            self.line(p1[0] as i32, p1[1] as i32, p2[0] as i32, p2[1] as i32, t);
        }
    }

    // only draws horizontal and vertical lines
    fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, t: Tile) {
        if x1 == x2 {
            self.vertical_line(x1, y1, y2, t)
        } else {
            self.horizontal_line(x1, y1, x2, t);
        }
    }

    fn horizontal_line(&mut self, x1: i32, y: i32, x2: i32, t: Tile) {
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

    fn fill(&mut self, x: i32, y: i32, tile: Tile) {
        let mut tiles: Vec<FillTile> = Vec::new();
        tiles.push(FillTile::new(x, y));
        while !tiles.is_empty() {
            let t = tiles.pop().unwrap();
            if self.data[t.y as usize][t.x as usize].char == NULLCHAR {
                let mut x_ = t.x - 1;
                while self.data[t.y as usize][x_ as usize].char == NULLCHAR
                    || self.data[t.y as usize][x_ as usize].char == ' '
                {
                    x_ -= 1;
                }

                for xi in x_ + 1..x {
                    self.data[t.y as usize][xi as usize] = tile;
                }

                let mut x_ = t.x;
                while self.data[t.y as usize][x_ as usize].char == NULLCHAR
                    || self.data[t.y as usize][x_ as usize].char == ' '
                {
                    x_ += 1;
                }

                for xi in x..x_ - 1 {
                    self.data[t.y as usize][xi as usize] = tile;
                }

                tiles.push(FillTile::new(t.x, t.y + 1));
                tiles.push(FillTile::new(t.x, t.y - 1));
            }
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct FillTile {
    x: i32,
    y: i32,
}

impl FillTile {
    fn new(x: i32, y: i32) -> Self {
        FillTile { x, y }
    }
}
