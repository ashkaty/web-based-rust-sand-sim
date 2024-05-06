use std::path::Display;

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

    pub fn step(&mut self, grid: &mut Grid, x:usize, y:usize) {
        if !grid.is_within_bounds(x, y) {
            return;
        }
        match self.element_type {
            ElementType::ImmovableSolid => self.step_immoveable_solid(grid,x, y),
            ElementType::MoveableSolid => self.step_moveable_solid(grid,x, y),
            ElementType::Liquid => self.step_liquid(grid,x, y),
            ElementType::Gas => self.step_gas(grid,x, y),
            ElementType::PixelGenerator => self.step_pixel_generator(grid,x, y),
            _ => {}
        }
    }

    fn step_immoveable_solid(&self, grid: &mut Grid, _x:usize, y:usize) {
        // Immoveable solids don't move, no need for implementation here
    }

    fn step_moveable_solid(&self, grid: &mut Grid, x:usize, y:usize) {
        // Check if there is space below or liquid to displace
        if y + 1 < grid.height && (grid.get(x, y + 1 ).element_type == ElementType::Nothing || grid.get(x, y + 1 ).element_type == ElementType::Liquid) {
            grid.move_element( x,y,x, y + 1 );
        } else {
            // Random movement if no space below
            let mut options = Vec::new();
            if y + 1 < grid.height && x > 0 && grid.get(x - 1, y + 1 ).element_type == ElementType::Nothing {
                options.push((x - 1, y + 1 ));
            }
            if y + 1 < grid.height && x + 1 < grid.width && grid.get(x + 1, y + 1 ).element_type == ElementType::Nothing {
                options.push((x + 1, y + 1 ));
            }
            if !options.is_empty() {
                let random_index = thread_rng().gen_range(0..options.len());
                let new_pos = options[random_index];
                grid.move_element(x, y, new_pos.0, new_pos.1);
            }
        }
    }

    fn step_liquid(&mut self, grid: &mut Grid, x:usize, y:usize) {
        let mut rng = rand::thread_rng();
        let direction = rng.gen_range(0..2)*2 -1;
        if y + 1 < grid.height && grid.get(x, y + 1).element_type == ElementType::Nothing {
            // Random movement if nothing below
            let mut options = Vec::new();
            if y + 1 < grid.height && x > 0 && grid.get(x - 1, y + 1 ).element_type == ElementType::Nothing {
                options.push((x - 1, y + 1 ));
            }
            if y + 1 < grid.height && x > 0 && grid.get(x + 1, y - 1 ).element_type == ElementType::Nothing {
                options.push((x, y + 1 ));
            }
            if y + 1 < grid.height && x + 1 < grid.width && grid.get(x + 1, y + 1 ).element_type == ElementType::Nothing {
                options.push((x + 1, y + 1 ));
            }
            if !options.is_empty() {
                let random_index = thread_rng().gen_range(0..options.len());
                let new_pos = options[random_index];
                grid.move_element(x, y, new_pos.0, new_pos.1);
            }
            else {
                grid.move_element(x, y,x,y+1);
            }
        } else {
            // Attempt to disperse left or right

            let mut rng = rand::thread_rng();
            let direction = rng.gen_range(0..2) * 2 - 1; 
            let mut current_x = x;

            for i in 1..=5 {
                let new_x: usize = (current_x as i32 + direction as i32) as usize;

                if new_x < grid.width && new_x > 0 {
                    let target = grid.get(new_x, y);

                    if target == NOTHING {
                        grid.move_element(current_x, y, new_x, y);
                        current_x = new_x;
                        //we went sideways! increase velocity if not above max
                        // break;
                    }
                    else{
                        break;
                    }
                }   
            }
        }
    }

    fn step_gas(&mut self, grid: &mut Grid, x:usize, y:usize) {
        // Gas simulation logic
    }

    fn step_pixel_generator(&self, grid: &mut Grid, x:usize, y:usize) {
        // Check if there is air below
        if y + 1 < grid.height && grid.get(x, y + 1 ).element_type == ElementType::Nothing {
            grid.set (x, y + 1, WATER);
        }
    }
}

pub static SAND: Element = Element {
    element_type: ElementType::MoveableSolid,
    color: Color { r: 255.0, g: 215.0, b: 0.0 },
    name: "Sand",
    velocity_x: 0,
};

pub static WATER: Element = Element {
    element_type: ElementType::Liquid,
    color: Color { r: 4.0, g: 59.0, b: 92.0 },
    name: "Water",
    velocity_x: 0,
};

pub static STONE: Element = Element {
    element_type: ElementType::ImmovableSolid,
    color: Color { r: 169.0, g: 169.0, b: 169.0 },
    name: "Stone",
    velocity_x: 0,
};

pub static NOTHING: Element = Element {
    element_type: ElementType::Nothing,
    color: Color { r: 0.0, g: 0.0, b: 0.0 },
    name: "Nothing",
    velocity_x: 0,
};
