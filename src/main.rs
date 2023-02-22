use macroquad::prelude::*;
use std::{thread, time};

const CELL_SIZE: f32 = 20.0;

#[derive(Clone, Copy)]
pub enum Cell {
    Alive,
    Dead
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        match self {
            Cell::Alive => true,
            Cell::Dead => false
        }
    }
}

pub struct Grid {
    width: i32, // unit is cells
    height: i32,
    cells: Vec<Vec<Cell>> // two-dimensional vector
}

impl Grid {
    pub fn tick (&mut self) {
        self.update_cells();
        self.draw_cells();
    }

    fn update_cells (&mut self) {
        let mut cells_copy = self.cells.clone(); // changes will be done in this copy so they dont overlap with previous state

        for row in 0..self.cells.len() { // we iterate in the actual state
            for col in 0..self.cells[row].len() {   
                if self.cells[row][col].is_alive() { 
                
                    if alive_neighbors_count(row as i32, col as i32, self) <2 || 
                    alive_neighbors_count(row as i32, col as i32, self) >3 { // underpopulation or overpopulation
                        cells_copy[row][col] = Cell::Dead; // but modify the copy
                    }
                } else {

                    if alive_neighbors_count(row as i32, col as i32, self) == 3 { // reproduction
                        cells_copy[row][col] = Cell::Alive
                    }
                }
            }
        }
        self.cells = cells_copy; // finally we modify the state all at once
    }

    pub fn draw_cells(&self){
        for row in 0..self.cells.len() {
            for col in 0..self.cells[row].len() {

                if self.cells[row][col].is_alive() { // alive cells are black rectangles
                    draw_rectangle((col as f32) * CELL_SIZE, (row as f32) *CELL_SIZE, CELL_SIZE , CELL_SIZE, BLACK);

                } else { // dead cells are white rectangles
                    draw_rectangle((col as f32) * CELL_SIZE, (row as f32) *CELL_SIZE, CELL_SIZE , CELL_SIZE, WHITE);
                }
            }
        }
    }

    pub fn draw_grid(&self) {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;

        while x + CELL_SIZE <= screen_width() {
            draw_line(x, 0.0, x, screen_height(), 1.0, LIGHTGRAY);
            x += CELL_SIZE;
        }
        while y + CELL_SIZE <= screen_height() {
            draw_line(0.0, y, screen_width(), y, 1.0, LIGHTGRAY);
            y += CELL_SIZE;
        }
    }
}

pub fn alive_neighbors_count(row: i32, col: i32, grid: &mut Grid) -> i32 {
    let mut count = 0;
    let neighbors: [(i32, i32); 8] = [(0,-1) , (-1,0), (0,1), (1,0), (1,-1), (-1,1), (-1,-1), (1,1)];

    for (x, y) in neighbors.into_iter() {

        if (row + y < 0) || (row + y >= grid.height) || (col + x < 0) || (col + x >= grid.width) {
            continue; // to avoid non valid indexes since the universe is finite

        } else {
            if grid.cells[(row + y) as usize][(col + x) as usize].is_alive(){
                count += 1;
            }
        }
    }
    count
}

#[macroquad::main("Game of Life")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;
    let x_quantity = w as f32 / CELL_SIZE; // number of cells
    let y_quantity = h as f32 / CELL_SIZE;

    let mut universe = Grid {
        width: x_quantity as i32,
        height: y_quantity as i32,
        cells: get_default_pattern(x_quantity,y_quantity)
    };
    clear_background(WHITE);
    universe.draw_cells(); // to see initial state
    universe.draw_grid();
    next_frame().await;
    thread::sleep(time::Duration::from_secs(1));

    loop {
        universe.tick();
        universe.draw_grid(); 
        next_frame().await; // // submit our render calls to our screen
        thread::sleep(time::Duration::from_secs(1));
    }
}

pub fn get_default_pattern(x_quantity: f32, y_quantity: f32) -> Vec<Vec<Cell>> {
    let mut cells: Vec<Vec<Cell>> = vec![vec![Cell::Dead; x_quantity as usize]; y_quantity as usize];

    // a random but known initial state to test if it works fine
    cells[0][0] = Cell::Alive;
    cells[1][1] = Cell::Alive;
    cells[1][2] = Cell::Alive;  
    cells[2][1] = Cell::Alive;
    cells[5][1] = Cell::Alive; 
    cells[5][2] = Cell::Alive;
    cells[5][4] = Cell::Alive;  
    cells[6][4] = Cell::Alive;
    cells[5][5] = Cell::Alive;
    cells[6][5] = Cell::Alive;
    cells[5][6] = Cell::Alive;
    cells[1][5] = Cell::Alive;  
    cells[2][5] = Cell::Alive;
    cells[3][5] = Cell::Alive;

    cells[9][3] = Cell::Alive;
    cells[10][1] = Cell::Alive;
    cells[10][2] = Cell::Alive;
    cells[14][0] = Cell::Alive;
    cells[14][1] = Cell::Alive;
    cells[14][2] = Cell::Alive;
    cells[2][12] = Cell::Alive;
    cells[3][11] = Cell::Alive;
    cells[3][13] = Cell::Alive;
    cells[3][15] = Cell::Alive;
    cells[4][12] = Cell::Alive;
    cells[4][14] = Cell::Alive;
    cells[4][15] = Cell::Alive;
    cells[8][11] = Cell::Alive;
    cells[9][10] = Cell::Alive;

    cells[9][12] = Cell::Alive;
    cells[9][16] = Cell::Alive;
    cells[8][19] = Cell::Alive;
    cells[8][20] = Cell::Alive;
    cells[9][20] = Cell::Alive;
    cells[2][24] = Cell::Alive;
    cells[3][23] = Cell::Alive;
    cells[3][25] = Cell::Alive;
    cells[4][23] = Cell::Alive;
    cells[4][25] = Cell::Alive;
    cells[5][24] = Cell::Alive;
    cells[7][24] = Cell::Alive;
    cells[7][27] = Cell::Alive;
    cells[7][28] = Cell::Alive;
    cells[7][29] = Cell::Alive;
    cells[8][25] = Cell::Alive;
    cells[8][27] = Cell::Alive;
    cells[9][26] = Cell::Alive;
    cells[14][15] = Cell::Alive;
    cells[14][16] = Cell::Alive;
    cells[14][17] = Cell::Alive;
    cells[13][12] = Cell::Alive;

    cells[14][11] = Cell::Alive;
    cells[15][10] = Cell::Alive;
    cells[16][9] = Cell::Alive;
    cells[17][7] = Cell::Alive;
    cells[17][8] = Cell::Alive;
    cells[12][21] = Cell::Alive;
    cells[12][23] = Cell::Alive;
    cells[13][20] = Cell::Alive;
    cells[13][22] = Cell::Alive;
    cells[13][24] = Cell::Alive;
    cells[14][21] = Cell::Alive;
    cells[14][23] = Cell::Alive;
    cells[15][22] = Cell::Alive;
    cells[22][9] = Cell::Alive;
    cells[23][8] = Cell::Alive;
    cells[23][9] = Cell::Alive;
    cells[24][9] = Cell::Alive;
    cells[25][8] = Cell::Alive;

    cells[25][9] = Cell::Alive;
    cells[20][16] = Cell::Alive;
    cells[21][17] = Cell::Alive;
    cells[22][16] = Cell::Alive;
    cells[21][22] = Cell::Alive;
    cells[22][22] = Cell::Alive;
    cells[22][23] = Cell::Alive;
    cells[27][26] = Cell::Alive;
    cells[27][27] = Cell::Alive;
    cells[27][28] = Cell::Alive;
    cells[27][29] = Cell::Alive;
    cells[27][30] = Cell::Alive;
    cells[27][31] = Cell::Alive;

    cells
}
