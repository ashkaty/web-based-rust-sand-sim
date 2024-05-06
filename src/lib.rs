use element::*;
use wasm_bindgen::JsValue;
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;
mod element;

pub const GRID_WIDTH: usize = 226;
pub const GRID_HEIGHT: usize = 126;
// #[wasm_bindgen]
// #[derive(Clone, Copy, PartialEq)]
// pub struct Vector2 {
//     pub x: usize,
//     pub y: usize,
// }

// #[wasm_bindgen]
// impl Vector2 {
//     #[wasm_bindgen(constructor)]
//     pub fn new(x: usize, y: usize) -> Vector2 {
//         Vector2 { x, y }
//     }
// }

#[wasm_bindgen]
pub struct Grid {
    width: usize,
    height: usize,
    elements: Vec<element::Element>,
    selected_element: element::Element,
    previous_mouse_x: usize,
    previous_mouse_y: usize,
    brush_size: usize,
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
            selected_element: element::WATER,
            previous_mouse_x: 0,
            previous_mouse_y: 0,
            brush_size: 3
        }
    }
    // Get the element at the given position
    pub fn get(&self, x: usize, y: usize) -> element::Element {
        if x < self.width && y < self.height {
            self.elements[y * self.width + x]
        } else {
            element::NOTHING
        }
    }
    
    #[wasm_bindgen]
    pub fn test(&self){ 
        console::log_1(&"Hello using web-sys".into()); 
    } 

    pub fn set(&mut self, x: usize, y: usize, value: element::Element) {
        if x < self.width && y < self.height {
            self.elements[y * self.width + x] = value;
        }
    }

    // Move the element at the given position to the new position   
    pub fn move_element(&mut self, x: usize, y: usize, new_x:usize, new_y: usize ) {
        let element = self.get(x, y);
        self.set(x, y, element::NOTHING);
        self.set(new_x,new_y, element);
    }

    // Swap the elements at the given positions
    pub fn swap_elements(&mut self, x: usize, y:usize, new_x:usize, new_y: usize ) {
        let element1 = self.get(x, y);
        let element2 = self.get(new_x, new_y);
        self.set(x, y, element2);
        self.set(new_x, new_y, element1);
    }

    // Update the grid
    pub fn update(&mut self) {
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let mut element = self.get( x, y );
                element.step(self, x, y);
            }
        }
    }

    fn draw_line(&self, x0: isize, y0: isize, x1: isize, y1: isize) -> Vec<(isize, isize)> {
        let mut points = Vec::new();
        let mut x = x0;
        let mut y = y0;
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;  // error value e_xy
        loop {
            points.push((x, y)); // Collect the current point
            if x == x1 && y == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy { 
                err += dy;
                x += sx;
            }
            if e2 <= dx { 
                err += dx; 
                y += sy; 
            }
        }
        points
    }

    pub fn is_within_bounds(&self, x:usize, y:usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn reset(&mut self) {
        self.elements = vec![element::NOTHING; self.width * self.height];
    }

    #[wasm_bindgen]
    pub fn render(&mut self, context: &CanvasRenderingContext2d, cell_size: f64) {
        self.update();
        for y in 0..self.height {
            for x in 0..self.width {
                let element = self.get(x,y);
                let color = element.color;
                let color_string = format!("rgb({}, {}, {})", color.r, color.g, color.b);
                context.set_fill_style(&JsValue::from_str(&color_string));
                context.fill_rect((x as f64) * cell_size, (y as f64) * cell_size, cell_size, cell_size);
            }
        }
    }

    #[wasm_bindgen]
    pub fn set_mouse(&mut self, mouse_pos_x:usize, mouse_pos_y:usize){
        self.previous_mouse_x = mouse_pos_x;
        self.previous_mouse_y = mouse_pos_y;
    }

    #[wasm_bindgen]
    pub fn draw_mouse(& mut self, mouse_pos_x: usize, mouse_pos_y: usize) {  
        let mut brush_offsets = vec![(0,0)];


        match self.brush_size {
            2 => {
                brush_offsets.push((1, 0));
                brush_offsets.push((1, 1));
                brush_offsets.push((0, 1));
                brush_offsets.push((-1, 0));
                brush_offsets.push((-1, -1));
                brush_offsets.push((0, -1));},
            3 => {
                brush_offsets.push((1, 0));
                brush_offsets.push((1, 1));
                brush_offsets.push((0, 1));
                brush_offsets.push((-1, 0));
                brush_offsets.push((-1, -1));
                brush_offsets.push((0, -1));
                brush_offsets.push((-2, 0));
                brush_offsets.push((-2, 1));
                brush_offsets.push((-2, -1));
                brush_offsets.push((-1, 2));
                brush_offsets.push((-1, 1));
                brush_offsets.push((-1, -2));
                brush_offsets.push((0, 2));
                brush_offsets.push((0, -1));
                brush_offsets.push((0, -2));
                brush_offsets.push((1, 2));
                brush_offsets.push((1, 0));
                brush_offsets.push((1, -1));
                brush_offsets.push((1, -2));
                brush_offsets.push((2, 0));
                brush_offsets.push((2, 1));
                brush_offsets.push((2, -1));
            },
            _ => {},
        }
        let points_on_line = self.draw_line(self.previous_mouse_x as isize, self.previous_mouse_y as isize, mouse_pos_x as isize, mouse_pos_y as isize);
        for point in points_on_line{
            let x1 = point.0;
            let y1 = point.1;
            for offset in brush_offsets.iter() {
                let new_x:usize = (x1 + offset.0) as usize;
                let new_y = (y1 as isize + offset.1) as usize;
                if self.get(new_x,new_y) == element::NOTHING || self.selected_element == element::NOTHING{
                    self.set(new_x, new_y, self.selected_element);
                }
            }
        }
        self.previous_mouse_x = mouse_pos_x;
        self.previous_mouse_y = mouse_pos_y;
    }

    #[wasm_bindgen]
    pub fn update_selected_element(& mut self, e:Element){
        self.selected_element = e;
    }

    #[wasm_bindgen]
    pub fn handle_input(&mut self, gk:&str){
        match gk {
            "q" => self.selected_element = element::SAND,
            "w" => self.selected_element = element::WATER,
            "e" => self.selected_element = element::STONE,
            "r" => self.selected_element = element::MAGIC,
            "t" => self.selected_element = element::NOTHING,
            "y" => self.selected_element = element::FIRE,
            "m" => self.selected_element = element::MAZE,
            "[" => {
                if self.brush_size > 1 {
                    self.brush_size -= 1;
                }
            },
            "]" => {
                if self.brush_size < 3{
                    self.brush_size += 1;
                }
            }
            "z" => self.reset(),
            _ => {self.selected_element = element::STONE}
        }
    }

}