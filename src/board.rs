use crate::location::Location;

pub struct Board{
    pub(crate) board:Vec<Vec<Cell>>,
    pub(crate) dimensions:Location
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

