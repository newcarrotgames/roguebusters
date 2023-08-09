use super::city::{Coord, Direction, Grid, Rect, DIRECTIONS};
use crate::{city::city::Tile, deser::prefabs};
use rand::Rng;
use std::fmt;
use log::{info, error};

pub const EXTERIOR: bool = true;
pub const INTERIOR: bool = false;
pub const VERTICAL: bool = true;
pub const HORIZONTAL: bool = false;
pub const DIR_NORTH: usize = 0;
pub const DIR_EAST: usize = 1;
pub const DIR_SOUTH: usize = 2;
pub const DIR_WEST: usize = 3;

const SUBDIVISION_SIZE_LIMIT: i32 = 32;
const SUBDIVISION_WIDTH_LIMIT: i32 = 5;
const SUBDIVISION_HEIGHT_LIMIT: i32 = 5;

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

#[rustfmt::skip]
pub const BUILDING_TEMPLATE_A: [Coord; 7] = [
    [ 0.0,  0.0],
    [ 0.0,  1.0],
    [-1.0,  1.0],
    [-1.0, -1.0],
    [ 1.0, -1.0],
    [ 1.0,  0.0],
    [ 0.0,  0.0],
];

#[rustfmt::skip]
pub const BUILDING_TEMPLATE_B: [Coord; 7] = [
    [ 0.0,  0.0],
    [-1.0,  0.0],
    [-1.0, -1.0],
    [ 1.0, -1.0],
    [ 1.0,  1.0],
    [ 0.0,  1.0],
    [ 0.0,  0.0],
];

#[rustfmt::skip]
pub const BUILDING_TEMPLATE_C: [Coord; 7] = [
    [ 0.0,  0.0],
    [ 1.0,  0.0],
    [ 1.0,  1.0],
    [-1.0,  1.0],
    [-1.0, -1.0],
    [ 0.0, -1.0],
    [ 0.0,  0.0],
];

#[rustfmt::skip]
pub const BUILDING_TEMPLATE_D: [Coord; 7] = [
    [ 0.0,  0.0],
    [ 0.0, -1.0],
    [ 1.0, -1.0],
    [ 1.0,  1.0],
    [-1.0,  1.0],
    [-1.0,  0.0],
    [ 0.0,  0.0],
];

#[derive(PartialEq, Clone, Debug)]
pub struct Building {
    pub id: i32,
    pub floors: Vec<Space>,
}

impl Building {
    pub fn new(id: i32, rect: Rect) -> Self {
        let mut rng = rand::thread_rng();
        let mut floors: Vec<Space> = Vec::new();
        let num_floors = rng.gen_range(0..20) + 1;
        for _ in 0..num_floors {
            floors.push(Space::new(rect, id));
        }
        Building { id, floors }
    }

    pub fn root(&mut self) -> &mut Space {
        return &mut self.floors[0];
    }

    pub fn subdivide_space(space: &mut Space, data: &mut Grid, depth: i32) {
        space.subdivide(data, depth);
        if space.partitions.len() == 0 {
            return;
        }
        for space in space.partitions.iter_mut() {
            Building::subdivide_space(space, data, depth + 1);
        }
    }

    pub fn add_doors(space: &mut Space, data: &mut Grid) {
        if space.partitions.len() > 0 {
            for space in space.partitions.iter_mut() {
                Building::add_doors(space, data);
            }
        } else {
            space.add_doors(data, EXTERIOR);
        }
    }

    pub(crate) fn add_stairs(building: &mut Building, data: &mut Grid) {
        for floor in building.floors.iter_mut() {
            info!("{}", floor);
        }
    }

    // pub fn get_spaces(building: &mut Building, level: i32) -> Vec<Space> {
    //     let spaces:Vec<Space> = Vec::new();
    //     let floor = building.floors[level as usize].clone();

    //     while !done {
    //         for space in floor.partitions {

    //         }
    //     }
    //     for space in floor.partitions {
            
    //     }
    //     spaces
    // }

    // pub(crate) fn populate(space: &mut Space, data: &mut Grid, prefabs: ) {
    //     log::info!("populating space");
    // }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BuildingType {
    Empty,
    Single,
    Double(BuildingOrientation), // true horizontal, false vertical
    Triple(BuildingOrientation),
    Quad,
}

impl BuildingType {
    fn random() -> BuildingType {
        let mut rng = rand::thread_rng();
        let btype = rng.gen_range(0..100);
        if (40..60).contains(&btype) {
            return BuildingType::Double(BuildingOrientation::Horizontal);
        } else if (60..80).contains(&btype) {
            return BuildingType::Double(BuildingOrientation::Vertical);
        // } else if (60..90).contains(&btype) {
        //     let subbtype = rng.gen_range(0..4);
        //     if subbtype == 0 {
        //         return BuildingType::Triple(BuildingOrientation::A);
        //     } else if subbtype == 1 {
        //         return BuildingType::Triple(BuildingOrientation::B);
        //     } else if subbtype == 2 {
        //         return BuildingType::Triple(BuildingOrientation::C);
        //     } else if subbtype == 3 {
        //         return BuildingType::Triple(BuildingOrientation::D);
        //     }
        } else if (90..100).contains(&btype) {
            return BuildingType::Quad;
        }
        BuildingType::Single
    }
}

pub struct BuildingGuide {
    pub building_type: BuildingType,
    pub x: i32,
    pub y: i32,
}

impl BuildingGuide {
    pub fn place(blocks: &mut Vec<Vec<BuildingType>>) -> BuildingGuide {
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
                _ => error!(
                    "building orientation is not vertical or horizontal for building type double"
                ),
            },
            BuildingType::Triple(_) => {
                points.push([x, y]);
                points.push([x + 1, y]);
                points.push([x, y + 1]);
                points.push([x + 1, y + 1]);
            }
            BuildingType::Quad => {
                points.push([x, y]);
                points.push([x + 1, y]);
                points.push([x, y + 1]);
                points.push([x + 1, y + 1]);
            }
            BuildingType::Empty => error!("guide is empty!"),
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

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BuildingOrientation {
    Vertical,
    Horizontal,
    A,
    B,
    C,
    D,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Space {
    rect: Rect,
    walls: [bool; 4], // interior/exterior
    pub partitions: Vec<Space>,
    building_id: i32,
}

impl Space {
    pub fn new(rect: Rect, building_id: i32) -> Self {
        Space {
            rect,
            walls: Wall::all(),
            partitions: Vec::new(),
            building_id,
        }
    }

    pub fn with_walls(rect: Rect, building_id: i32, walls: [bool; 4]) -> Self {
        Space {
            rect,
            walls,
            partitions: Vec::new(),
            building_id,
        }
    }

    pub fn partition_point(&self, axis: bool) -> i32 {
        let mut rng = rand::thread_rng();
        let s = match axis {
            VERTICAL => self.rect.width() as i32,
            HORIZONTAL => self.rect.height() as i32,
        };
        let f = s / 4;
        info!(
            "a: {}, rect: {}, f: {}/{}, s: {}",
            axis,
            self.rect,
            f,
            f * 2,
            s
        );
        f + rng.gen_range(0..f * 2)
    }

    pub fn subdivide(&mut self, data: &mut Grid, depth: i32) {
        info!(
            "subdividing space: {}, width: {}, height: {}",
            self,
            self.rect.width(),
            self.rect.height()
        );
        let mut rng = rand::thread_rng();

        if depth > 0 && rng.gen_range(0..depth) > 0 {
            info!("space missed coin toss, not subdividing");
            return;
        }

        // check if the space is large enough to subdivide
        if self.rect.size() <= SUBDIVISION_SIZE_LIMIT {
            info!("space is too small to subdivide");
            return;
        }

        if self.rect.width() <= SUBDIVISION_WIDTH_LIMIT {
            info!("space is beyond width limit");
            return;
        }

        if self.rect.height() <= SUBDIVISION_HEIGHT_LIMIT {
            info!("space is beyond height limit");
            return;
        }

        let axis: bool;
        let dim_ratio = self.rect.width() as f32 / self.rect.height() as f32;
        if dim_ratio > 1.25 {
            axis = VERTICAL;
        } else if dim_ratio < 0.75 {
            axis = HORIZONTAL;
        } else {
            axis = rng.gen::<bool>();
        }

        let point = self.partition_point(axis);

        info!("axis: {}", axis);
        info!("point: {}", point);

        // create new spaces from partitions
        let space1: Space;
        let space2: Space;

        // first space
        if axis == HORIZONTAL {
            let x1 = self.rect.x1;
            let mut y1 = self.rect.y1;
            let x2 = self.rect.x2;
            let mut y2 = self.rect.y1 + point;

            let mut walls = self.walls.clone();
            walls[DIR_SOUTH] = INTERIOR;

            space1 = Space::with_walls(Rect { x1, y1, x2, y2 }, self.building_id, walls);
            info!(
                "space1: {}, width: {}, height: {}",
                space1,
                space1.rect.width(),
                space1.rect.height()
            );
            // second space
            y1 = self.rect.y1 + point;
            y2 = self.rect.y2;

            let mut walls = self.walls.clone();
            walls[DIR_NORTH] = INTERIOR;

            space2 = Space::with_walls(Rect { x1, y1, x2, y2 }, self.building_id, walls);
            info!(
                "space2: {}, width: {}, height: {}",
                space2,
                space2.rect.width(),
                space2.rect.height()
            );
        } else {
            let mut x1 = self.rect.x1;
            let y1 = self.rect.y1;
            let mut x2 = self.rect.x1 + point;
            let y2 = self.rect.y2;

            let mut walls = self.walls.clone();
            walls[DIR_EAST] = INTERIOR;

            space1 = Space::with_walls(Rect { x1, y1, x2, y2 }, self.building_id, walls);
            info!(
                "space1: {}, width: {}, height: {}",
                space1,
                space1.rect.width(),
                space1.rect.height()
            );

            // second space
            x1 = self.rect.x1 + point;
            x2 = self.rect.x2;

            let mut walls = self.walls.clone();
            walls[DIR_WEST] = INTERIOR;

            space2 = Space::with_walls(Rect { x1, y1, x2, y2 }, self.building_id, walls);
            info!(
                "space2: {}, width: {}, height: {}",
                space2,
                space2.rect.width(),
                space2.rect.height()
            );
        }

        // if subdivided spaces are too small, don't subdivide
        if space1.rect.width() <= SUBDIVISION_WIDTH_LIMIT
            || space2.rect.width() <= SUBDIVISION_WIDTH_LIMIT
        {
            info!("one or both subdivided spaces were too small");
            return;
        }

        if space1.rect.height() <= SUBDIVISION_HEIGHT_LIMIT
            || space2.rect.height() <= SUBDIVISION_HEIGHT_LIMIT
        {
            info!("one or both subdivided spaces were too small");
            return;
        }

        // check spaces for at least 1 exterior wall
        let mut has_exterior_wall = false;
        for wall in space1.walls {
            if wall {
                has_exterior_wall = true;
                break;
            }
        }

        if !has_exterior_wall {
            info!("space has no exterior walls");
            return;
        }

        let mut has_exterior_wall = false;
        for wall in space2.walls {
            if wall {
                has_exterior_wall = true;
                break;
            }
        }

        if !has_exterior_wall {
            info!("space has no exterior walls");
            return;
        }

        // calculate wall positions and interiors

        // draw partition
        if axis == HORIZONTAL {
            for x in self.rect.x1..self.rect.x2 {
                data[(self.rect.y1 + point) as usize][x as usize] = Tile::wall();
            }
        } else {
            for y in self.rect.y1..self.rect.y2 {
                data[y as usize][(self.rect.x1 + point) as usize] = Tile::wall();
            }
        }

        self.partitions.push(space1);
        self.partitions.push(space2);
    }

    pub fn get_wall_coords(&mut self, wall_dir: Direction) -> Rect {
        match wall_dir {
            Direction::NORTH => Rect {
                x1: self.rect.x1,
                y1: self.rect.y1,
                x2: self.rect.x2,
                y2: self.rect.y1,
            },
            Direction::EAST => Rect {
                x1: self.rect.x2,
                y1: self.rect.y1,
                x2: self.rect.x2,
                y2: self.rect.y2,
            },
            Direction::SOUTH => Rect {
                x1: self.rect.x1,
                y1: self.rect.y2,
                x2: self.rect.x2,
                y2: self.rect.y2,
            },
            Direction::WEST => Rect {
                x1: self.rect.x1,
                y1: self.rect.y1,
                x2: self.rect.x1,
                y2: self.rect.y2,
            },
        }
    }

    pub fn add_doors(&mut self, data: &mut Grid, exterior: bool) {
        let mut rng = rand::thread_rng();
        let mut has_door = false;
        while !has_door {
            for (i, wall) in self.walls.iter().enumerate() {
                if *wall == exterior {
                    if rng.gen_bool(0.5) {
                        let door_x: i32;
                        let door_y: i32;
                        match DIRECTIONS[i] {
                            Direction::NORTH => {
                                door_x = self.rect.x1 + self.partition_point(HORIZONTAL);
                                door_y = self.rect.y1;
                            }
                            Direction::EAST => {
                                door_x = self.rect.x2;
                                door_y = self.rect.y1 + self.partition_point(VERTICAL);
                            }
                            Direction::SOUTH => {
                                door_x = self.rect.x1 + self.partition_point(HORIZONTAL);
                                door_y = self.rect.y2;
                            }
                            Direction::WEST => {
                                door_x = self.rect.x1;
                                door_y = self.rect.y1 + self.partition_point(VERTICAL);
                            }
                        }
                        data[door_y as usize][door_x as usize] =
                            Tile::door(self.building_id, DIRECTIONS[i]);
                        has_door = true;
                    }
                }
            }
        }
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.rect.x1, self.rect.y1, self.rect.x2, self.rect.y2
        )
    }
}

pub struct Wall {}

impl Wall {
    fn all() -> [bool; 4] {
        [true, true, true, true]
    }
}
