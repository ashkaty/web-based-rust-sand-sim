// use std::path::Display;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::Grid;
use ::rand::{thread_rng, Rng};

#[derive(Clone, Copy, PartialEq)]
#[wasm_bindgen]
pub enum ElementType {
    ImmovableSolid,
    MoveableSolid,
    Liquid,
    Gas,
    PixelGenerator,
    Nothing,
    Magic,
    Fire,
    Maze,
}
#[derive(Clone, Copy, PartialEq)]
#[wasm_bindgen]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
#[derive(Clone, Copy, PartialEq)]
#[wasm_bindgen]
pub struct Element {
    pub element_type: ElementType,
    pub color: Color,
    name: &'static str,
    velocity_x: isize,
}

#[wasm_bindgen]
impl Element {
    // fn new(element_type: ElementType, color: Color, name: &'static str) -> Element {
    //     Element {
    //         element_type,
    //         color: (color),
    //         name,
    //         velocity_0,
    //     }
    // }

    pub fn step(&mut self, grid: &mut Grid, x: usize, y: usize) {
        if !grid.is_within_bounds(x, y) {
            return;
        }
        match self.element_type {
            ElementType::ImmovableSolid => {},//self.step_immoveable_solid(grid, x, y),
            ElementType::MoveableSolid => self.step_moveable_solid(grid, x, y),
            ElementType::Liquid => self.step_liquid(grid, x, y),
            ElementType::Gas => {},//self.step_gas(grid, x, y),
            ElementType::PixelGenerator => self.step_pixel_generator(grid, x, y),
            ElementType::Magic => self.step_magic(grid, x, y),
            ElementType::Fire => self.step_fire(grid, x, y),
            ElementType::Maze => self.step_maze(grid, x, y),
            _ => {}
        }
    }

    // fn step_immoveable_solid(&self, grid: &mut Grid, _x: usize, y: usize) {
    //     // Immoveable solids don't move, no need for implementation here
    // }

    fn step_moveable_solid(&self, grid: &mut Grid, x: usize, y: usize) {
        // Check if there is space below or liquid to displace
        if y + 1 < grid.height
            && (grid.get(x, y + 1).element_type == ElementType::Nothing
                || grid.get(x, y + 1).element_type == ElementType::Liquid)
        {
            grid.move_element(x, y, x, y + 1);
        } else {
            // Random movement if no space below
            let mut options = Vec::new();
            if y + 1 < grid.height
                && x > 0
                && grid.get(x - 1, y + 1).element_type == ElementType::Nothing
            {
                options.push((x - 1, y + 1));
            }
            if y + 1 < grid.height
                && x + 1 < grid.width
                && grid.get(x + 1, y + 1).element_type == ElementType::Nothing
            {
                options.push((x + 1, y + 1));
            }
            if !options.is_empty() {
                let random_index = thread_rng().gen_range(0..options.len());
                let new_pos = options[random_index];
                grid.move_element(x, y, new_pos.0, new_pos.1);
            }
        }
    }

    fn step_liquid(&mut self, grid: &mut Grid, x: usize, y: usize) {
        if y + 1 < grid.height && grid.get(x, y + 1).element_type == ElementType::Nothing {
            // Random movement if nothing below
            let mut options = Vec::new();
            if y + 1 < grid.height
                && x > 0
                && grid.get(x - 1, y + 1).element_type == ElementType::Nothing
            {
                options.push((x - 1, y + 1));
            }
            if y + 1 < grid.height
                && x > 0
                && grid.get(x + 1, y - 1).element_type == ElementType::Nothing
            {
                options.push((x, y + 1));
            }
            if y + 1 < grid.height
                && x + 1 < grid.width
                && grid.get(x + 1, y + 1).element_type == ElementType::Nothing
            {
                options.push((x + 1, y + 1));
            }
            if !options.is_empty() {
                let random_index = thread_rng().gen_range(0..options.len());
                let new_pos = options[random_index];
                grid.move_element(x, y, new_pos.0, new_pos.1);
            } else {
                grid.move_element(x, y, x, y + 1);
            }
        } else {
            // Attempt to disperse left or right

            let mut rng = rand::thread_rng();
            let direction = rng.gen_range(0..2) * 2 - 1;
            let mut current_x = x;

            for _i in 0..=5 {
                let new_x: usize = (current_x as i32 + direction as i32) as usize;

                if new_x < grid.width && new_x > 0 {
                    let target = grid.get(new_x, y);

                    if target == NOTHING {
                        grid.move_element(current_x, y, new_x, y);
                        current_x = new_x;
                        //we went sideways! increase velocity if not above max
                        // break;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn step_magic(&mut self, grid: &mut Grid, x: usize, y: usize) {
        let mut offset: (isize, isize) = (0, 0);
        if y > 1 && y < grid.height {
            let above = grid.get(x, y - 1);
            if above.element_type == ElementType::Nothing
                || above.element_type == ElementType::Liquid
            {
                offset = (0, -1);
            }
        }
        let mut rng = rand::thread_rng();
        let direction = rng.gen_range(0..2) * 2 - 1;
        let new_x: isize = x as isize + direction;
        if new_x >= 0 && new_x <= (grid.width - 1) as isize {
            let target = grid.get(new_x as usize, (y as isize + offset.1) as usize);
            if target.element_type == ElementType::Nothing
                || target.element_type == ElementType::Liquid
            {
                offset = (new_x, offset.1);
            }
        }
        let (x2, y2) = (x as isize + offset.0, y as isize + offset.1);
        let target = grid.get(x2 as usize, y2 as usize);
        grid.set(x, y, target);
        grid.set(x2 as usize, y2 as usize, MAGIC);
    }

    // fn step_gas(&mut self, grid: &mut Grid, x: usize, y: usize) {
    //     // Gas simulation logic
    // }

    fn step_pixel_generator(&self, grid: &mut Grid, x: usize, y: usize) {
        // Check if there is air below
        if y + 1 < grid.height && grid.get(x, y + 1).element_type == ElementType::Nothing {
            grid.set(x, y + 1, WATER);
        }
    }

    pub fn step_fire(&mut self, grid: &mut Grid, x: usize, y: usize) {
        let mut rng = thread_rng();
        let upward_chance = 0.7;

        // Check if the pixel above is empty and within grid bounds
        if y > 0 && grid.get(x, y - 1) == NOTHING {
            // Move upward with a chance based on upward_chance
            if rng.gen::<f32>() < upward_chance {
                grid.move_element(x, y, x, y - 1);
                return; // Fire moves only once per step
            }
        }

        // If no upward movement occurred, the fire drifts randomly
        let drift_direction = rng.gen_range(-1..=1); // -1 for left, 0 for no drift, 1 for right
        let new_x = (x as i32 + drift_direction) as usize;

        // Check if the new position is within grid bounds and empty
        if new_x < grid.width && grid.get(new_x, y) == NOTHING {
            grid.move_element(x, y, new_x, y);
        } else {
            // If no movement is possible, the fire dies out, turning into NOTHING
            grid.set(x, y, NOTHING);
        }
    }

    pub fn step_maze(&mut self, grid: &mut Grid, x: usize, y: usize) {
        // Check all neighboring cells
        for dx in -1..=1 {
            for dy in -1..=1 {
                // Skip the current cell
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                // Check if the neighboring cell is within the grid bounds
                if nx >= 0 && nx < grid.width as i32 && ny >= 0 && ny < grid.height as i32 {
                    let neighbor = grid.get(nx as usize, ny as usize);

                    // Check if the neighboring cell is not a maze cell
                    if neighbor != MAZE {
                        // Check if the neighboring cell has 3 neighbors
                        let neighbor_neighbors = self.count_maze_neighbors(grid, nx as usize, ny as usize);

                        // Set the neighboring cell to maze if it has 3 neighbors
                        if neighbor_neighbors == 3 {
                            grid.set(nx as usize, ny as usize, MAZE);
                        }
                    }
                }
            }
        }
        // Check the current cell
        if grid.get(x, y) == MAZE {
            let current_neighbors = self.count_maze_neighbors(grid, x, y);

            // Set the current cell to nothing if it has less than 1 or more than 5 neighbors
            if current_neighbors < 1 || current_neighbors > 5 {
                grid.set(x, y, NOTHING);
            }
        }
    }
    
    fn count_maze_neighbors(&mut self, grid: &Grid, x: usize, y: usize) -> usize {
        let mut neighbor_neighbors = 0;
    
        for dx in -1..=1 {
            for dy in -1..=1 {
                // Skip the current cell and the neighboring cell
                if (dx == 0 && dy == 0) || (dx == 0 && dy == 0) {
                    continue;
                }
    
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
    
                // Check if the neighbor of the current cell is within the grid bounds
                if nx >= 0 && nx < grid.width as i32 && ny >= 0 && ny < grid.height as i32 {
                    let neighbor = grid.get(nx as usize, ny as usize);
    
                    // Check if the neighbor of the current cell is a maze cell
                    if neighbor == MAZE {
                        neighbor_neighbors += 1;
                    }
                }
            }
        }
    
        neighbor_neighbors
    }
}



pub static SAND: Element = Element {
    element_type: ElementType::MoveableSolid,
    color: Color {
        r: 255.0,
        g: 215.0,
        b: 0.0,
    },
    name: "Sand",
    velocity_x: 0,
};

pub static WATER: Element = Element {
    element_type: ElementType::Liquid,
    color: Color {
        r: 4.0,
        g: 59.0,
        b: 92.0,
    },
    name: "Water",
    velocity_x: 0,
};

pub static STONE: Element = Element {
    element_type: ElementType::ImmovableSolid,
    color: Color {
        r: 169.0,
        g: 169.0,
        b: 169.0,
    },
    name: "Stone",
    velocity_x: 0,
};

pub static NOTHING: Element = Element {
    element_type: ElementType::Nothing,
    color: Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    },
    name: "Nothing",
    velocity_x: 0,
};

pub static MAGIC: Element = Element {
    element_type: ElementType::Magic,
    color: Color {
        r: 0.0,
        g: 255.0,
        b: 0.0,
    },
    name: "Magic",
    velocity_x: 0,
};

pub static FIRE: Element = Element {
    element_type: ElementType::Fire,
    color: Color {
        r: 255.0,
        g: 0.0,
        b: 0.0,
    },
    name: "Fire",
    velocity_x: 0,
};

pub static MAZE: Element = Element {
    element_type: ElementType::Maze,
    color: Color {
        r: 255.0,
        g: 255.0,
        b: 255.0,
    },
    name: "Maze",
    velocity_x: 0,
};
