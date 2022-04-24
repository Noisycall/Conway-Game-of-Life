mod board;
mod location;
use std::borrow::{Borrow, BorrowMut};
use std::fmt::{Display, Formatter, Write};
use std::fs::copy;
use std::thread::sleep;
use std::time;
use location::Location;
use board::{Board,Cell};
use clearscreen;
fn main() {
    println!("Hello, world!");
    let mut cell:Cell = Cell {alive:true,location: Location(3, 4) };
    println!("{}",&cell.location);
    let mut board = Board{board:Vec::new(),dimensions:Location(5,5)};
    board.init_board();
    board.board[2][2].alive = true;
    board.board[1][2].alive = true;
    board.board[3][2].alive = true;
    draw_board(&board);
    println!();
    let iter = 5;
    loop{
        let cloned_board = board.clone();
        for row in board.board.iter_mut() {
            for col in row.iter_mut(){
                col.alive = next_cell_state(&cloned_board, &col);
            }
        }
        draw_board(&board);
        println!()
    }
    // let neighbours = get_bounds_checked_neighbours(&board,&board.board[0][0]);
    // neighbours.iter().for_each(|loc|{println!("x:{},y:{}",loc.0,loc.1)});
    // println!("{}",active_neighbour_count(&board,&neighbours));
    // get_bounds_checked_neighbours(&board,&cell).iter().for_each(|x| println!("{}",x));
}




fn next_cell_state(board:&Board,cell:&Cell)->bool{
    let neighbours = get_bounds_checked_neighbours(&board,&cell);
    let active_neighbours = active_neighbour_count(&board,&neighbours);
    if cell.alive{
        if active_neighbours<2{
            return false;
        }
        if active_neighbours>3{
            return  false;
        }
    }
    else {
        if active_neighbours==3{
            return true;
        }
    }
    return cell.alive;
}

fn active_neighbour_count(board:&Board,neighbours:&Vec<Location>)->i32{
    let mut active_neighbours = 0;
    neighbours.iter().for_each(|location|{
        if board.board[location.0 as usize][location.1 as usize].alive {
            active_neighbours +=1;
        }
    });
    active_neighbours
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
    if loc.0>=0&&loc.1>=0&&loc.0<bounds.0&&loc.1<bounds.1 {
        return true
    }
    false
}

fn calc_offset(board:&Board,loc:&Location)->i16{
    loc.0 * board.dimensions.1 + loc.1
}

fn draw_board(board:&Board){
    sleep(time::Duration::from_secs(1));
    clearscreen::clear().expect("Error in clear");
    for row in &board.board{
        for col in row{
            let mut st="O";
            if col.alive {
                st = "X";
            }
            print!("{} ",st);
        }
        println!()
    }

}