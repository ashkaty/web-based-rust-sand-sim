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
    pub velocity: Vector2, // Add velocity field
}

#[wasm_bindgen]
impl Element {

    fn new(element_type: ElementType, color: Color, name: &'static str) -> Element {
        Element {
            element_type,
            color: (color),
            name,
            velocity: Vector2{x: 0, y: 0},
        }
    }

    pub fn step(&self, grid: &mut Grid, pos: Vector2) {
        if !grid.is_within_bounds(pos) {
            return;
        }
        match self.element_type {
            ElementType::ImmovableSolid => self.step_immoveable_solid(grid, pos),
            ElementType::MoveableSolid => self.step_moveable_solid(grid, pos),
            ElementType::Liquid => self.step_liquid(grid, pos, 8),
            ElementType::Gas => self.step_gas(grid, pos, 1),
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

    pub fn step_immoveable_solid(&self, grid: &mut Grid, pos: Vector2) {
    }

    pub fn step_moveable_solid(&self,grid: &mut Grid, pos: Vector2) {
    // Check if there is air below
    if pos.y + 1 < grid.height && grid.get(Vector2 { x: pos.x, y: pos.y + 1 }) == NOTHING {
        // Fall down
        grid.move_element(Vector2 { x: pos.x, y: pos.y }, Vector2 { x: pos.x, y: pos.y + 1 });
    } else if pos.y + 1 < grid.height && grid.get(Vector2 { x: pos.x, y: pos.y + 1 }) == WATER {
        // Swap with water below
        grid.swap_elements(Vector2 { x: pos.x, y: pos.y}, Vector2 { x: pos.x, y: pos.y + 1 });
    } else {
        let mut options = Vec::new();

        if pos.y + 1 < grid.height && pos.x > 0 && grid.get(Vector2 { x: pos.x - 1, y: pos.y + 1}) == NOTHING {
            options.push((pos.x - 1, pos.y + 1));
        }

        if pos.y + 1 < grid.height
            && pos.x + 1 < grid.width
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

    pub fn step_gas(&self, grid: &mut Grid, pos:Vector2, diffusion_rate: usize) {
            //removed gas
    }

    pub fn step_liquid(&self, grid: &mut Grid, pos:Vector2, dispersion_rate: usize) {
        // Check if the water can fall down
        // If it can, move the water down
        // Otherwise, attempt to disperse left or right
        if pos.y < grid.height - 1 {
            let below = grid.get(Vector2 { x: pos.x, y: pos.y + 1});
            if below == NOTHING {
                grid.move_element(Vector2 { x: pos.x, y: pos.y}, Vector2 { x: pos.x, y: pos.y + 1});
            } else {
                // Attempt to disperse left or right

                let mut rng = rand::thread_rng();
                let direction = rng.gen_range(0..2) * 2 - 1; 
                let mut current_x = pos.x;

                for i in 1..=dispersion_rate {
                    let new_x: usize = (current_x as i32 + direction as i32) as usize;

                    if new_x < grid.width && new_x > 0 {
                        let target = grid.get(Vector2 { x: new_x, y: pos.y});

                        if target == NOTHING {
                            grid.move_element(Vector2 { x: current_x, y: pos.y}, Vector2 { x: new_x, y: pos.y});
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
    }

    pub fn step_pixel_generator(&self, grid: &mut Grid, pos:Vector2) {
        // Check if there is air below
        if pos.y + 1 < grid.height && grid.get(Vector2 { x: pos.x, y: pos.y + 1}).element_type == ElementType::Nothing {
            grid.set(Vector2 { x: pos.x, y: pos.y + 1}, WATER);
        }
    }
}


pub static SAND: Element = Element {
    element_type: ElementType::MoveableSolid,
    color: Color { r: 255.0, g: 215.0, b: 0.0 },
    name: "Sand",
    velocity: Vector2{x: 0, y: 0},
};

pub static WATER: Element = Element {
    element_type: ElementType::Liquid,
    color: Color { r: 4.0, g: 59.0, b: 92.0 },
    name: "Water",
    velocity: Vector2{x: 0, y: 0},
};

pub static STONE: Element = Element {
    element_type: ElementType::ImmovableSolid,
    color: Color { r: 169.0, g: 169.0, b: 169.0 },
    name: "Stone",
    velocity: Vector2{x: 0, y: 0},
};

pub static NOTHING: Element = Element {
    element_type: ElementType::Nothing,
    color: Color{ r: 0.0, g: 0.0, b: 0.0 },
    name: "Nothing",
    velocity: Vector2{x: 0, y: 0},
};

// pub static AIR: Element = Element {
//     element_type: ElementType::Gas,
//     color: Color{ r: 135.0, g: 206.0, b: 235.0 },
//     name: "Air",
//     velocity: Vector2{x: 0, y: 0},
// };

// pub static FAUCET: Element = Element {
//     element_type: ElementType::PixelGenerator,
//     color: Color { r: 255.0, g: 255.0, b: 255.0 },
//     name: "Faucet",
//     velocity: Vector2{x: 0, y: 0},
// };

// pub static CLAY: Element = Element {
//     element_type: ElementType::MoveableSolid,
//     color: Color { r: 165.0, g: 42.0, b: 42.0 },
//     name: "Clay",
//     velocity: Vector2{x: 0, y: 0},
// };
