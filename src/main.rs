extern crate rand;
use std::mem::swap;

const BOARD_SIZE : usize = 200;

fn is_ressurectable(num_neighbours: u8) -> bool {
    num_neighbours == 3
}
fn is_over_populated(num_neighbours: u8) -> bool {
    num_neighbours > 3
}
fn is_under_populated(num_neighbours: u8) -> bool {
    num_neighbours < 2 
}
fn next_cell_state(cell_state: bool, neighbour_count : u8) -> bool{
    match cell_state {
        false => is_ressurectable(neighbour_count),
        true => !is_over_populated(neighbour_count) && !is_under_populated(neighbour_count)
    } 
}
fn is_index_out_of_bounds(index_1: isize, index_2: isize, array_length: usize) -> bool {
    index_1 < 0 || index_1 >= array_length as isize || index_2 < 0 || index_2 >= array_length as isize 
}

const NEIGHBOUR_LOCATIONS : [(isize, isize); 8] = [
    (-1, -1),
    (-1,  0),
    (-1,  1),
    (0,  -1),
    (0,   1),
    (1,  -1),
    (1,   0),
    (1,   1),
];

fn count_alive_neighbours(board: &[bool], x: usize, y: usize) -> u8{
    NEIGHBOUR_LOCATIONS.iter().fold(0, |acc, &location| {
        let index_x : isize = x as isize + location.0; 
        let index_y : isize = y as isize + location.1; 
        let is_out_of_bounds = is_index_out_of_bounds(index_x, index_y, BOARD_SIZE);

        acc + match is_out_of_bounds {
            false => {
                let board_index : usize = index_x as usize + BOARD_SIZE * index_y as usize;
                match board[board_index] {
                    true => 1,
                    false => 0
                }
            },
            _ => 0
        }

    })
}

fn calculate_next_board(current_board: &[bool], next_board :& mut [bool]){

    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let neighbour_count = count_alive_neighbours(current_board, x, y);
            next_board[x + BOARD_SIZE * y] = next_cell_state(current_board[x + BOARD_SIZE * y], neighbour_count);
        }
    }

}

fn seed_board(board: & mut [bool]){
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            board[x + BOARD_SIZE * y] = rand::random();
        }
    }
}

fn print_board(board: &[bool]){
    print!("{}[2J", 27 as char);
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let cell = match board[x + BOARD_SIZE * y]{
                true => 'x',
                false => '_'
            };
            print!("{}", cell);
        }
        print!("\n");
    }
}

fn main() {
    let mut board_one: [bool; BOARD_SIZE * BOARD_SIZE] = [false; BOARD_SIZE * BOARD_SIZE];
    let mut board_two: [bool; BOARD_SIZE * BOARD_SIZE] = [false; BOARD_SIZE * BOARD_SIZE];

    seed_board(& mut board_one); 

    loop{
        print_board(&board_one);
        calculate_next_board(& board_one, & mut board_two);
        swap(& mut board_one, & mut board_two);
    }
}
