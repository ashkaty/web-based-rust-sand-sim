use element::{Color, Element, WATER};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;



use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;
mod element;
mod element_type;



pub const GRID_WIDTH: usize = 226;
pub const GRID_HEIGHT: usize = 126;



#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: usize,
    pub y: usize,

}

#[wasm_bindgen]
impl Vector2 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: usize, y: usize) -> Vector2 {
        Vector2 { x, y }
    }
}

#[wasm_bindgen]

pub struct Grid {
    width: usize,
    height: usize,
    elements: Vec<element::Element>,
    selected_element: element::Element
}


#[wasm_bindgen]

impl Grid {
    // Create a new grid with the given width and height

    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            elements: vec![element::NOTHING; width * height],
            selected_element: element::Element::water()
        }
    }
    // Get the element at the given position

    
    pub fn get(&self, pos: Vector2) -> element::Element {
        if pos.x < self.width && pos.y < self.height {
            self.elements[pos.y * self.width + pos.x]
        } else {
            element::NOTHING
        }
    }
    
    #[wasm_bindgen]
    pub fn test(&self){ 
        console::log_1(&"Hello using web-sys".into()); 
    } 

    pub fn set(&mut self, pos: Vector2, value: element::Element) {
        if pos.x < self.width && pos.y < self.height {
            self.elements[pos.y * self.width + pos.x] = value;
        }
    }

    // Move the element at the given position to the new position

   
    pub fn move_element(&mut self, pos: Vector2, new_pos: Vector2) {
        let element = self.get(pos);
        self.set(pos, element::NOTHING);
        self.set(new_pos, element);
    }

    // Swap the elements at the given positions


    pub fn swap_elements(&mut self, pos: Vector2, new_pos: Vector2) {
        let element1 = self.get(pos);
        let element2 = self.get(new_pos);
        self.set(pos, element2);
        self.set(new_pos, element1);
    }

    // Update the grid


    pub fn update(&mut self) {
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let element = self.get(Vector2 { x, y });
                element.step(self, Vector2 { x, y });
            }
        }
    }



    // Apply the function to each element in between two positions

    // pub fn traverse_line<F>(&mut self, start: Vector2, end: Vector2, mut f: F)
    // where
    //     F: FnMut(usize, usize),
    // {
    //     let dx = end.0 as isize - start.0 as isize;
    //     let dy = end.1 as isize - start.1 as isize;
    //     let steps = if dx.abs() > dy.abs() {
    //         dx.abs()
    //     } else {
    //         dy.abs()
    //     } as f32;
    //     let x_increment = dx as f32 / steps;
    //     let y_increment = dy as f32 / steps;
    //     let mut x = start.0 as f32;
    //     let mut y = start.1 as f32;
    //     for _ in 0..steps as usize {
    //         f(x as usize, y as usize);
    //         x += x_increment;
    //         y += y_increment;
    //     }
    // }


    pub fn is_within_bounds(&self, pos: Vector2) -> bool {
        pos.x < self.width && pos.y < self.height
    }

    pub fn reset(&mut self) {
        self.elements = vec![element::NOTHING; self.width * self.height];
    }

    #[wasm_bindgen]
    pub fn render(&mut self, context: &CanvasRenderingContext2d, cell_size: f64) {
        self.update();
        for y in 0..self.height {
            for x in 0..self.width {
                let element = self.get(Vector2{x, y});
                let color = element.get_color();
                let color_string = format!("rgb({}, {}, {})", color.r, color.g, color.b);
                context.set_fill_style(&JsValue::from_str(&color_string));
                context.fill_rect((x as f64) * cell_size, (y as f64) * cell_size, cell_size, cell_size);
            }
        }
    }

    #[wasm_bindgen]
    pub fn draw_mouse(& mut self, ctx: &CanvasRenderingContext2d, mouse_pos_x: usize, mouse_pos_y: usize) {       
        // const brush_offsets: [(isize, isize); 7] = [
        //     (0, 0),
        //     (1, 0),
        //     (1, 1),
        //     (0, 1),
        //     (-1, 0),
        //     (-1, -1),
        //     (0, -1),
        // ];

        let brush_offsets: [(isize, isize); 20] = [
            (-2, 0), (-2, 1), (-2, -1),           // Three points to the left
            (-1, 2), (-1, 1), (-1, 0), (-1, -1), (-1, -2), // Five points diagonally left
            (0, 2), (0, 1), (0, -1), (0, -2),     // Middle vertical
            (1, 2), (1, 1), (1, 0), (1, -1), (1, -2),     // Five points diagonally right
            (2, 0), (2, 1), (2, -1)              // Three points to the right
        ];
        for offset in brush_offsets.iter() {
            let new_x = (mouse_pos_x as isize + offset.0) as usize;
            let new_y = (mouse_pos_y as isize + offset.1) as usize;
            self.set(Vector2 { x: new_x, y: new_y }, self.selected_element);
        }
    }
}
