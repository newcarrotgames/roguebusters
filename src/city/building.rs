use super::city::{Coord, Direction, Grid, Rect, DIRECTIONS};
use crate::{city::city::{Tile, TileId}, deser::{prefabs::{self, Prefabs, Prefab}, generators::{Generator, Generators}}};
use rand::Rng;
use std::fmt;
use log::{debug, error};

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

const NULLCHAR: char = 0 as char;

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
    pub interior_type: String,
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
        Building { id, interior_type: String::new(), floors }
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
            debug!("{}", floor);
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
    //     log::debug!("populating space");
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
    entrance_wall: Option<usize>,
    pub name: String,
    pub interior_type: String,
}

impl Space {
    pub fn new(rect: Rect, building_id: i32) -> Self {
        Space {
            rect,
            walls: Wall::all(),
            partitions: Vec::new(),
            building_id,
            entrance_wall: None,
            name: String::new(),
            interior_type: String::new(),
        }
    }

    pub fn with_walls(rect: Rect, building_id: i32, walls: [bool; 4]) -> Self {
        Space {
            rect,
            walls,
            partitions: Vec::new(),
            building_id,
            entrance_wall: None,
            name: String::new(),
            interior_type: String::new(),
        }
    }

    pub fn partition_point(&self, axis: bool) -> i32 {
        let mut rng = rand::thread_rng();
        let s = match axis {
            VERTICAL => self.rect.width() as i32,
            HORIZONTAL => self.rect.height() as i32,
        };
        let f = s / 4;
        debug!(
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
        debug!(
            "subdividing space: {}, width: {}, height: {}",
            self,
            self.rect.width(),
            self.rect.height()
        );
        let mut rng = rand::thread_rng();

        if depth > 0 && rng.gen_range(0..depth) > 0 {
            debug!("space missed coin toss, not subdividing");
            return;
        }

        // check if the space is large enough to subdivide
        if self.rect.size() <= SUBDIVISION_SIZE_LIMIT {
            debug!("space is too small to subdivide");
            return;
        }

        if self.rect.width() <= SUBDIVISION_WIDTH_LIMIT {
            debug!("space is beyond width limit");
            return;
        }

        if self.rect.height() <= SUBDIVISION_HEIGHT_LIMIT {
            debug!("space is beyond height limit");
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

        debug!("axis: {}", axis);
        debug!("point: {}", point);

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
            debug!(
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
            debug!(
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
            debug!(
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
            debug!(
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
            debug!("one or both subdivided spaces were too small");
            return;
        }

        if space1.rect.height() <= SUBDIVISION_HEIGHT_LIMIT
            || space2.rect.height() <= SUBDIVISION_HEIGHT_LIMIT
        {
            debug!("one or both subdivided spaces were too small");
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
            debug!("space has no exterior walls");
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
            debug!("space has no exterior walls");
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
                                door_x = self.rect.x1 + self.partition_point(VERTICAL);
                                door_y = self.rect.y1;
                            }
                            Direction::EAST => {
                                door_x = self.rect.x2;
                                door_y = self.rect.y1 + self.partition_point(HORIZONTAL);
                            }
                            Direction::SOUTH => {
                                door_x = self.rect.x1 + self.partition_point(VERTICAL);
                                door_y = self.rect.y2;
                            }
                            Direction::WEST => {
                                door_x = self.rect.x1;
                                door_y = self.rect.y1 + self.partition_point(HORIZONTAL);
                            }
                        }
                        data[door_y as usize][door_x as usize] =
                            Tile::door(self.building_id, DIRECTIONS[i]);
                        self.entrance_wall = Some(i);
                        has_door = true;
                    }
                }
            }
        }
    }

    /// Compute (x, y) anchor for a prefab using its alignment relative to the
    /// entrance wall.  "top" = near entrance, "bottom" = far from entrance.
    /// For east/west entrances the depth/lateral axes swap so the mapping
    /// stays entrance-relative even though the prefab grid is not rotated.
    fn compute_aligned_position(&self, prefab: &Prefab, entrance_wall: usize) -> (i32, i32) {
        let h_align = prefab.placement.alignment.horizontal.as_str();
        let v_align = prefab.placement.alignment.vertical.as_str();

        let margin = 1;
        let min_x = self.rect.x1 + margin;
        let min_y = self.rect.y1 + margin;
        let max_x = (self.rect.x2 - margin - prefab.width).max(min_x);
        let max_y = (self.rect.y2 - margin - prefab.height).max(min_y);
        let mid_x = (min_x + max_x) / 2;
        let mid_y = (min_y + max_y) / 2;

        match entrance_wall {
            DIR_NORTH | DIR_SOUTH => {
                let near_y = if entrance_wall == DIR_NORTH { min_y } else { max_y };
                let far_y  = if entrance_wall == DIR_NORTH { max_y } else { min_y };

                let y = match v_align {
                    "top"    => near_y,
                    "bottom" => far_y,
                    _        => mid_y,
                };
                let x = match h_align {
                    "left"  => min_x,
                    "right" => max_x,
                    _       => mid_x,
                };
                (x, y)
            }
            DIR_EAST | DIR_WEST => {
                let near_x = if entrance_wall == DIR_WEST { min_x } else { max_x };
                let far_x  = if entrance_wall == DIR_WEST { max_x } else { min_x };

                let x = match v_align {
                    "top"    => near_x,
                    "bottom" => far_x,
                    _        => mid_x,
                };
                let y = match h_align {
                    "left"  => min_y,
                    "right" => max_y,
                    _       => mid_y,
                };
                (x, y)
            }
            _ => (mid_x, mid_y),
        }
    }

    pub fn fill_space(&mut self, gen: &Generator, prefabs: &Prefabs, data: &mut Grid) {
        let mut rng = rand::thread_rng();
        for rule in gen.rules.rules.iter() {
            let Some(prefab) = prefabs.get(rule.name.as_str()) else { continue };
            match rule.frequency.as_str() {
                "one" => {
                    let has_alignment = !prefab.placement.alignment.vertical.is_empty()
                        || !prefab.placement.alignment.horizontal.is_empty();

                    let mut placed = false;

                    if has_alignment {
                        if let Some(ew) = self.entrance_wall {
                            let (ax, ay) = self.compute_aligned_position(prefab, ew);
                            if self.can_place_prefab(data, ax, ay, prefab) {
                                self.draw_prefab(ax, ay, prefab, self.building_id, data);
                                placed = true;
                            } else {
                                'search: for radius in 1..=5_i32 {
                                    for dy in -radius..=radius {
                                        for dx in -radius..=radius {
                                            if dx.abs() != radius && dy.abs() != radius {
                                                continue;
                                            }
                                            let x = ax + dx;
                                            let y = ay + dy;
                                            if self.can_place_prefab(data, x, y, prefab) {
                                                self.draw_prefab(x, y, prefab, self.building_id, data);
                                                placed = true;
                                                break 'search;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if !placed {
                        let x_range = self.rect.x2 - 2 - self.rect.x1;
                        let y_range = self.rect.y2 - 2 - self.rect.y1;
                        if x_range > 0 && y_range > 0 {
                            let mut tries = 0;
                            loop {
                                let x = rng.gen_range(self.rect.x1..self.rect.x2 - 2);
                                let y = rng.gen_range(self.rect.y1..self.rect.y2 - 2);
                                if self.can_place_prefab(data, x, y, prefab) {
                                    self.draw_prefab(x, y, prefab, self.building_id, data);
                                    break;
                                }
                                tries += 1;
                                if tries > 10 {
                                    break;
                                }
                            }
                        }
                    }
                }
                "many" => {
                    for y in self.rect.y1..self.rect.y2 - 2 {
                        for x in self.rect.x1..self.rect.x2 - 2 {
                            let range_limit = (1.0 / rule.chance) as usize;
                            if rng.gen_range(0..range_limit) == 0 {
                                if self.can_place_prefab(data, x, y, prefab) {
                                    self.draw_prefab(x, y, prefab, self.building_id, data);
                                }
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    fn draw_prefab(&mut self, x: i32, y: i32, prefab: &Prefab, building_id: i32, data: &mut Grid) {
        for (py, row) in prefab.data.rows.iter().enumerate() {
            for (px, cell) in row.cells.iter().enumerate() {
                data[y as usize + py][x as usize + px] = Tile::from_cell(cell, building_id);
            }
        }
    }

    fn can_place_prefab(&self, data: &mut Grid, x: i32, y: i32, prefab: &Prefab) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        for py in 0..prefab.height + 1 {
            for px in 0..prefab.width + 1 {
                let ty = (y + py) as usize;
                let tx = (x + px) as usize;
                if ty >= data.len() || tx >= data[ty].len() {
                    return false;
                }
                if data[ty][tx].tile_id != TileId::Interior {
                    return false;
                }
            }
        }
        true
    }

    pub fn fill(&mut self, generators: &Generators, fallback: &Generator, prefabs: &Prefabs, data: &mut Grid) {
        for space in self.partitions.iter_mut() {
            if space.partitions.len() > 0 {
                log::debug!("space has partitions, continuing to traverse partitions tree...");
                space.fill(generators, fallback, prefabs, data);
            } else {
                log::debug!("space has no partitions, filling with room generator for '{}'", space.interior_type);
                let gen = generators
                    .get_opt("room", &space.interior_type)
                    .filter(|g| g.rules.rules.iter().any(|r| prefabs.get(&r.name).is_some()))
                    .unwrap_or(fallback);
                space.fill_space(gen, prefabs, data);
            }
        }
    }

    /// Recursively visits every leaf space (no partitions) and calls `f` to
    /// obtain a `(name, interior_type)` pair to assign to that space.
    pub fn assign_leaf_businesses_with<F: FnMut() -> (String, String)>(&mut self, f: &mut F) {
        if self.partitions.is_empty() {
            let (n, t) = f();
            self.name = n;
            self.interior_type = t;
        } else {
            for partition in self.partitions.iter_mut() {
                partition.assign_leaf_businesses_with(f);
            }
        }
    }

    /// Searches the leaf-space tree for a space whose rect boundary contains
    /// the point `(x, y)`, and returns its `(name, interior_type)` if found.
    pub fn find_space_info_at(&self, x: i32, y: i32) -> Option<(&str, &str)> {
        if self.partitions.is_empty() {
            let r = &self.rect;
            let on_h_wall = (x == r.x1 || x == r.x2) && y >= r.y1 && y <= r.y2;
            let on_v_wall = (y == r.y1 || y == r.y2) && x >= r.x1 && x <= r.x2;
            if (on_h_wall || on_v_wall) && !self.name.is_empty() {
                return Some((&self.name, &self.interior_type));
            }
            return None;
        }
        for partition in &self.partitions {
            if let Some(info) = partition.find_space_info_at(x, y) {
                return Some(info);
            }
        }
        None
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
