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
    selected_element: element::Element,
    previous_mouse_x: usize,
    previous_mouse_y: usize
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
            previous_mouse_y: 0
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
    pub fn set_mouse(&mut self, mouse_pos_x:usize, mouse_pos_y:usize){
        self.previous_mouse_x = mouse_pos_x;
        self.previous_mouse_y = mouse_pos_y;
    }

    #[wasm_bindgen]
    pub fn draw_mouse(& mut self, mouse_pos_x: usize, mouse_pos_y: usize) {       
        
        // const brush_offsets: [(isize, isize); 7] = [
        //     (0, 0),
        //     (1, 0),
        //     (1, 1),
        //     (0, 1),
        //     (-1, 0),
        //     (-1, -1),
        //     (0, -1),
        // ];

        let pointsss = self.draw_line(self.previous_mouse_x as isize, self.previous_mouse_y as isize, mouse_pos_x as isize, mouse_pos_y as isize);

        let brush_offsets: [(isize, isize); 21] = [
            (-2, 0), (-2, 1), (-2, -1),           // Three points to the left
            (-1, 2), (-1, 1), (-1, 0), (-1, -1), (-1, -2), // Five points diagonally left
            (0, 2), (0, 1), (0, -1), (0, -2),     // Middle vertical
            (1, 2), (1, 1), (1, 0), (1, -1), (1, -2),     // Five points diagonally right
            (2, 0), (2, 1), (2, -1),              // Three points to the right
            (0,0),                                  // fill center
        ];
        for point in pointsss{
            let x1 = point.0;
            let y1 = point.1;
            for offset in brush_offsets.iter() {
                let new_x = (x1 as isize + offset.0) as usize;
                let new_y = (y1 as isize + offset.1) as usize;
                if (self.get(Vector2{x: new_x, y: new_y}) == element::NOTHING){
                    self.set(Vector2 { x: new_x, y: new_y }, self.selected_element);
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
            _ => {self.selected_element = element::STONE}
        }
    }

}