mod board;
mod location;
use std::borrow::Borrow;
use std::fmt::{Display, Formatter, Write};
use std::fs::copy;
use location::Location;
use board::{Board,Cell};

fn main() {
    println!("Hello, world!");
    let mut cell:Cell = Cell {alive:true,location: Location(3, 4) };
    println!("{}",&cell.location);
    let mut board = Board{board:Vec::new(),dimensions:Location(5,5)};
    board.init_board();
    board.board[0][0].alive = true;
    draw_board(&board);
    // get_bounds_checked_neighbours(&board,&cell).iter().for_each(|x| println!("{}",x));
}




fn next_cell_state(board:&Board,cell:&Cell)->bool{

    return false;
}

fn get_bounds_checked_neighbours(board:&Board,cell:&Cell)->Vec<Location>{
    let offsets:[(i16,i16);8] = [(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1),(0,-1),(-1,-1)];
    let mut neighbours:Vec<Location> = Vec::new();
    for i in offsets{
        if is_in_bounds(&board.dimensions, &Location(i.0+cell.location.0, i.1+cell.location.1)){
            neighbours.push(Location(i.0+cell.location.0,i.1+cell.location.1))
        }
    }
    neighbours
}

fn is_in_bounds(bounds:&Location,loc:&Location)->bool{
    if loc.0<bounds.0&&loc.1<bounds.1 {
        return true
    }
    false
}

fn calc_offset(board:&Board,loc:&Location)->i16{
    loc.0 * board.dimensions.1 + loc.1
}

fn draw_board(board:&Board){
    for row in &board.board{
        for col in row{
            print!("{} ",col.alive);
        }
        println!()
    }
}