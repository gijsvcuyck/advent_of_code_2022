pub mod num_traits;

use std::fmt::{Display, Formatter};
use num_traits::Zero;

///Module for a two dimensional grid that expands itself when required.

///(0,0) is top left corner. all opertions are mutable because acces out of bounds results in an increase of the bounds.
/// board is stored in row mayor order.
pub struct Layout<T>{
    board: Vec<Vec<T>>,
    width:usize,
    height:usize
}

impl <T: Zero + Clone> Layout<T>{

    pub fn new(width:usize,height:usize)->Layout<T>{
        Layout{board:vec![vec![T::zero();width];height],width,height }
    }

    pub fn set_val(&mut self,x:usize,y:usize,val:T){
        //println!("debug: now setting {x},{y}");
        self.check_coords(x,y);
        //println!("debug: final acces check: accessing {x},{y} with board of {} by {}",self.board[0].len(),self.board.len());
        // let row = &self.board[y];
        //println!("debug: full width check");
        // for row in self.board.iter(){
        //     println!("debug: width here is {}",row.len())
        // }
        // let temp = &row[x];
        self.board[y][x] = val;
    }

    ///get the textual representation of a space of the board.
    pub fn get_val(&mut self,x:usize,y:usize)->&T{
        self.check_coords(x,y);
        return &self.board[y][x];

    }

    fn check_coords(&mut self,x:usize,y:usize){
        // println!("debug: now checking size");
        if x >= self.width{
            // println!("debug: too shallow");
            self.add_space_right(1+x-self.width)
        }
        if y>= self.height{
            // println!("debug: too short");
            // println!("debug: amount calc: y:{y} height:{}, res:{}",self.height,1+y-self.height);
            self.add_space_below(1+y-self.height)
        }
    }

    ///Adds more space to right end of the board, which means for higher x values.
    fn add_space_right(&mut self,amount:usize){
        self.width+=amount;
        for col in self.board.iter_mut(){
            col.reserve(amount);
            for _ in 0..amount {
                col.push(T::zero());
            }
        }
    }
    /// Adds more space to left end of the board, which means for lower x values.
    /// Shifts the entire board, which invalidates all previously obtained coordinates.
    #[allow(dead_code)]
    fn add_space_left(&mut self,amount:usize){
        self.add_space_right(amount);
        for col in self.board.iter_mut(){
            col.rotate_right(amount);
        }

    }

    ///Adds more space to lower end of the board, which means for higher y values.
    fn add_space_below(&mut self,amount:usize){
        // println!("debug: pre_add space listed:{} actual:{}",self.height,self.board.len());
        // println!("debug: adding amount: {}",amount);
        self.height+=amount;
        self.board.reserve(amount);
        for _ in 0..amount{
            self.board.push(vec![T::zero();self.width]);
        }
        // println!("debug: post_add space listed:{} actual:{}",self.height,self.board.len());


    }
    /// Adds more space to upper end of the board, which means for lower y values.
    /// Shifts the entire board, which invalidates all previously obtained coordinates.
    #[allow(dead_code)]
    fn add_space_above(&mut self,amount:usize){
        self.add_space_below(amount);
        self.board.rotate_right(amount);
    }
}

impl <T> Layout<T>{

    pub fn unsafe_get(&self,x:usize,y:usize)->Option<&T>{
        self.board.get(y)?.get(x)
    }

    pub fn get_height(&self) ->usize{
        self.height
    }

    pub fn get_width(&self)->usize{
        self.width
    }
}

//Probably does not belong here, as it is to specific to the sand puzzle.
impl <T: Zero + PartialEq + Clone> Layout<T>{
    pub fn get_top_empty(&mut self,x:usize,y:usize)->(usize,usize){
        // println!("debug: now checking {x},{y}. height:{}",self.height);
        if y==self.height-1{
            return (x,y);
        }
        if *self.get_val(x,y+1)==T::zero(){
            self.get_top_empty(x,y+1)
        } else if *self.get_val(x-1,y+1) == T::zero() {
            self.get_top_empty(x-1,y+1)
        } else if *self.get_val(x+1,y+1) == T::zero() {
            self.get_top_empty(x+1,y+1)
        } else {
            (x,y)
        }
    }
}

impl <T:Display> Display for Layout<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height{
            for x in 0..self.width-1 {
                write!(f, "{}, ", self.unsafe_get(x,y).ok_or(std::fmt::Error)?)?;
            }
            write!(f,"{}\n",self.unsafe_get(self.width-1,y).ok_or(std::fmt::Error)?)?
        }
        Ok(())
    }
}

