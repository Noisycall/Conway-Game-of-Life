use crate::location::Location;

pub struct Board{
    pub(crate) board:Vec<Vec<Cell>>,
    pub(crate) dimensions:Location
}
impl Clone for Board{
    fn clone(&self) -> Self {
        let mut board_clone = Vec::new();
        board_clone.clone_from(&self.board);
        let mut clone = Board{board:board_clone,dimensions:self.dimensions.clone()};
        clone
    }
}
impl Board{
    pub(crate) fn init_board(&mut self){
        let mut boar:Vec<Vec<Cell>> = Vec::new();
        for row in 0..self.dimensions.0{
            let mut vec_row:Vec<Cell> = Vec::new();
            for col in 0..self.dimensions.1{
                vec_row.push(Cell {alive:false,location:Location(row,col)})
            }
            boar.push(vec_row);
        }
        self.board = boar;
    }
}

pub struct Cell{
    pub alive:bool,
    pub location:Location
}
impl Cell{
    fn get_loc(&self)->Location{
        let mut loc:Location = self.location.clone();
        loc
    }
}

impl Clone for Cell{
    fn clone(&self) -> Self {
        let clone = Cell {alive:self.alive.clone(),location:self.location.clone()};
        clone
    }
}

