use wasm_bindgen::prelude::wasm_bindgen;


use crate::element_type::{
    step_gas, step_immoveable_solid, step_liquid, step_moveable_solid, step_pixel_generator,
    ElementType,
};
use crate::Grid;
use crate::Vector2;

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
}

#[wasm_bindgen]
impl Element {

    fn new(element_type: ElementType, color: Color, name: &'static str) -> Element {
        Element {
            element_type,
            color: (color),
            name,
        }
    }

    pub fn step(&self, grid: &mut Grid, pos: Vector2) {
        if !grid.is_within_bounds(pos) {
            return;
        }
        match self.element_type {
            ElementType::ImmovableSolid => step_immoveable_solid(grid, pos),
            ElementType::MoveableSolid => step_moveable_solid(grid, pos),
            ElementType::Liquid => step_liquid(grid, pos, 4),
            ElementType::Gas => step_gas(grid, pos, 1),
            ElementType::PixelGenerator => step_pixel_generator(grid, pos),
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


    pub fn sand() -> Element {
        Element::new(ElementType::MoveableSolid, Color { r: 255.0, g: 215.0, b: 0.0 }, "Sand")
    }

    pub fn nothing() -> Element {
        Element::new(ElementType::Nothing, Color{ r: 0.0, g: 0.0, b: 0.0 }, "Nothing")
    }

    pub fn water() -> Element {
        Element::new(ElementType::Liquid, Color { r: 4.0, g: 59.0, b: 92.0 }, "Water")
    }
}


pub static AIR: Element = Element {
    element_type: ElementType::Gas,
    color: Color{ r: 135.0, g: 206.0, b: 235.0 },
    name: "Air",
};

pub static SAND: Element = Element {
    element_type: ElementType::MoveableSolid,
    color: Color { r: 255.0, g: 215.0, b: 0.0 },
    name: "Sand",
};

pub static WATER: Element = Element {
    element_type: ElementType::Liquid,
    color: Color { r: 4.0, g: 59.0, b: 92.0 },
    name: "Water",
};

pub static STONE: Element = Element {
    element_type: ElementType::ImmovableSolid,
    color: Color { r: 169.0, g: 169.0, b: 169.0 },
    name: "Stone",
};

pub static FAUCET: Element = Element {
    element_type: ElementType::PixelGenerator,
    color: Color { r: 255.0, g: 255.0, b: 255.0 },
    name: "Faucet",
};

pub static CLAY: Element = Element {
    element_type: ElementType::MoveableSolid,
    color: Color { r: 165.0, g: 42.0, b: 42.0 },
    name: "Clay",
};

pub static NOTHING: Element = Element {
    element_type: ElementType::Nothing,
    color: Color{ r: 0.0, g: 0.0, b: 0.0 },
    name: "Nothing",
};
