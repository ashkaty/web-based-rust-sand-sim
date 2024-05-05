use crate::element::{NOTHING, WATER};
use crate::Grid;
use crate::Vector2;
use wasm_bindgen::prelude::wasm_bindgen;

use ::rand::{thread_rng, Rng};




pub const GRID_WIDTH: usize = 226;
pub const GRID_HEIGHT: usize = 126;

#[derive(Clone, Copy, PartialEq)]

#[wasm_bindgen]
pub enum ElementType {
    ImmovableSolid,
    MoveableSolid,
    Liquid,
    Gas,
    PixelGenerator,
    Nothing,
}

pub fn step_moveable_solid(grid: &mut Grid, pos: Vector2) {
    // Check if there is air below
    if pos.y + 1 < GRID_HEIGHT && grid.get(Vector2 { x: pos.x, y: pos.y + 1 }) == NOTHING {
        // Fall down
        grid.move_element(Vector2 { x: pos.x, y: pos.y }, Vector2 { x: pos.x, y: pos.y + 1 });
    } else if pos.y + 1 < GRID_HEIGHT && grid.get(Vector2 { x: pos.x, y: pos.y + 1 }) == WATER {
        // Swap with water below
        grid.swap_elements(Vector2 { x: pos.x, y: pos.y}, Vector2 { x: pos.x, y: pos.y + 1 });
    } else {
        let mut options = Vec::new();

        if pos.y + 1 < GRID_HEIGHT && pos.x > 0 && grid.get(Vector2 { x: pos.x - 1, y: pos.y + 1}) == NOTHING {
            options.push((pos.x - 1, pos.y + 1));
        }

        if pos.y + 1 < GRID_HEIGHT
            && pos.x + 1 < GRID_WIDTH
            && grid.get(Vector2 { x: pos.x + 1, y: pos.y + 1}) == NOTHING
        {
            options.push((pos.x + 1, pos.y + 1));
        }

        if !options.is_empty() {
            let random_index = thread_rng().gen_range(0..options.len());
            let (new_x, new_y) = options[random_index];
            grid.move_element(Vector2 { x: pos.x, y: pos.y}, Vector2 { x: new_x, y: new_y});
        }
    }
}

pub fn step_immoveable_solid(grid: &mut Grid, pos: Vector2) {
    if pos.y + 1 < GRID_HEIGHT && grid.get(Vector2 { x: pos.x, y: pos.y + 1}) == WATER {
        grid.swap_elements(Vector2 { x: pos.x, y: pos.y}, Vector2 { x: pos.x, y: pos.y + 1});
    }
}

pub fn step_gas(grid: &mut Grid, pos:Vector2, diffusion_rate: usize) {
    if pos.y > 0 {
        let above = grid.get(Vector2 { x: pos.x, y: pos.y - 1});
        if above == NOTHING {
            grid.move_element(Vector2 { x: pos.x, y: pos.y}, Vector2 { x: pos.x, y: pos.y - 1});
        } else {
            // Attempt to disperse left or right

            // let direction = rand::gen_range(0, 2) * 2;

            let direction = 1 * 2;

            for i in 1..=diffusion_rate {
                let new_x = (pos.x as usize + direction * i as usize) as usize;

                if new_x < GRID_WIDTH {
                    if thread_rng().gen_range(0..100) < diffusion_rate * 10 {
                        let target = grid.get(Vector2 { x: new_x, y: pos.y});

                        if target == NOTHING {
                            grid.move_element(Vector2 { x: pos.x, y: pos.y}, Vector2 { x: new_x, y: pos.y});
                            break;
                        }
                    }
                }
            }
        }
    }
}

// This implementation looks pretty good, but is very poor for a couple reasons.
// 1. The water has a weird tendency to flow left.
// 2. We generate a new random number every single time instead of just using a preexisting pseudo-random number, like the
//      frame count.
// 3. The dispersion rate is buggy asf but does finally work.
pub fn step_liquid(grid: &mut Grid, pos:Vector2, dispersion_rate: usize) {
    // Check if the water can fall down
    // If it can, move the water down
    // Otherwise, attempt to disperse left or right
    if pos.y < GRID_HEIGHT - 1 {
        let below = grid.get(Vector2 { x: pos.x, y: pos.y + 1});
        if below == NOTHING {
            grid.move_element(Vector2 { x: pos.x, y: pos.y}, Vector2 { x: pos.x, y: pos.y + 1});
        } else {
            // Attempt to disperse left or right

            // let direction = rand::gen_range(0, 2) * 2 - 1;

            let direction = 1 * 2 - 1;

            for i in 1..=dispersion_rate {
                let new_x = (pos.x as i32 + direction * i as i32) as usize;

                if new_x < GRID_WIDTH {
                    let target = grid.get(Vector2 { x: new_x, y: pos.y});

                    if target == NOTHING {
                        grid.move_element(Vector2 { x: pos.x, y: pos.y}, Vector2 { x: new_x, y: pos.y});
                        break;
                    }
                }
            }
        }
    }
}

pub fn step_pixel_generator(grid: &mut Grid, pos:Vector2) {
    // Check if there is air below
    if pos.y + 1 < GRID_HEIGHT && grid.get(Vector2 { x: pos.x, y: pos.y + 1}) == NOTHING {
        grid.set(Vector2 { x: pos.x, y: pos.y + 1}, WATER);
    }
}
