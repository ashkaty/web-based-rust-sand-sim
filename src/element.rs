use std::path::Display;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::Grid;
use crate::Vector2;
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
    velocity_x: isize
}

#[wasm_bindgen]
impl Element {

    // fn new(element_type: ElementType, color: Color, name: &'static str) -> Element {
    //     Element {
    //         element_type,
    //         color: (color),
    //         name,
    //         velocity_x: 0,
    //     }
    // }

    pub fn step(&mut self, grid: &mut Grid, pos: Vector2) {
        if !grid.is_within_bounds(pos) {
            return;
        }
        match self.element_type {
            ElementType::ImmovableSolid => self.step_immoveable_solid(grid, pos),
            ElementType::MoveableSolid => self.step_moveable_solid(grid, pos),
            ElementType::Liquid => self.step_liquid(grid, pos),
            ElementType::Gas => self.step_gas(grid, pos),
            ElementType::PixelGenerator => self.step_pixel_generator(grid, pos),
            _ => {}
        }
    }

    pub fn to_string(&self) -> String {
        self.name.to_string()
    }

    pub fn get_color(&self) -> Color {
        return self.color;
    }

    pub fn get_element_type(&self) -> ElementType {
        return self.element_type;
    }

    fn step_immoveable_solid(&self, grid: &mut Grid, _pos: Vector2) {
        // Immoveable solids don't move, no need for implementation here
    }

    fn step_moveable_solid(&self, grid: &mut Grid, pos: Vector2) {
        // Check if there is space below or liquid to displace
        if pos.y + 1 < grid.height && (grid.get(Vector2 { x: pos.x, y: pos.y + 1 }).element_type == ElementType::Nothing || grid.get(Vector2 { x: pos.x, y: pos.y + 1 }).element_type == ElementType::Liquid) {
            grid.move_element(pos, Vector2 { x: pos.x, y: pos.y + 1 });
        } else {
            // Random movement if no space below
            let mut options = Vec::new();

            if pos.y + 1 < grid.height && pos.x > 0 && grid.get(Vector2 { x: pos.x - 1, y: pos.y + 1 }).element_type == ElementType::Nothing {
                options.push(Vector2 { x: pos.x - 1, y: pos.y + 1 });
            }

            if pos.y + 1 < grid.height && pos.x + 1 < grid.width && grid.get(Vector2 { x: pos.x + 1, y: pos.y + 1 }).element_type == ElementType::Nothing {
                options.push(Vector2 { x: pos.x + 1, y: pos.y + 1 });
            }

            if !options.is_empty() {
                let random_index = thread_rng().gen_range(0..options.len());
                let new_pos = options[random_index];
                grid.move_element(pos, new_pos);
            }
        }
    }

    fn step_liquid(&mut self, grid: &mut Grid, pos: Vector2) {
        let mut rng = rand::thread_rng();
        let direction = rng.gen_range(0..2)*2 -1;
        if pos.y + 1 < grid.height && grid.get(Vector2 { x: pos.x, y: pos.y + 1 }).element_type == ElementType::Nothing {
            let target = Vector2{x: (pos.x as isize + direction) as usize, y: pos.y+1};
            let target_element = grid.get(target);
            if (target_element.element_type == ElementType::Nothing){
                grid.move_element(pos, target);
            }
            else {
                grid.move_element(pos, Vector2{x:pos.x,y:pos.y+1});
            }
        } else {
            let target = Vector2{x: (pos.x as isize + direction) as usize, y: pos.y};
            let target_element = grid.get(target);
            if (target_element.element_type == ElementType::Nothing){
                grid.move_element(pos, target);
            }
        }
    }

    fn step_gas(&mut self, grid: &mut Grid, pos: Vector2) {
        // Gas simulation logic
    }

    fn step_pixel_generator(&self, grid: &mut Grid, pos: Vector2) {
        // Check if there is air below
        if pos.y + 1 < grid.height && grid.get(Vector2 { x: pos.x, y: pos.y + 1 }).element_type == ElementType::Nothing {
            grid.set(Vector2 { x: pos.x, y: pos.y + 1 }, WATER);
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
