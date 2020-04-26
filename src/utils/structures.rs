use std::cmp::Ord;

pub struct QuadTree<T>{
    data:Vec<T>,
    depth:u32
}

impl <T>QuadTree<T>{
    pub fn new()->QuadTree<T>{
        QuadTree{
            data:Vec::new(),
            depth:0
        }
    }
    pub fn breadth_scan(&self,row:usize)->&[T]{
        let pow=4_u32.pow(row as u32);
        let start=(pow-1)/3;
        let end=start + pow;
        &self.data[(start as usize)..(end as usize)]
    }

    pub fn branch_scan(&self,branch:u32,mut depth:u32)->Result<Vec<&T>,&str>{
        // running in debug mode
        if branch>=4 {  return Err("length must be less than 4")    }
        let mut result=Vec::new();
        let mut pow=4_u32.pow(depth);
        while depth<self.depth{
            let start=(pow-1)/3 + branch*(pow/4);
            let length=pow/4;
            (0..length).for_each(|v| result.push(&self.data[(start+v) as usize]));
            pow*=4;
            depth+=1;
        }
        Ok(result)
    }
    pub fn sort(&mut self) where T:Ord{ todo!() }
}




// Tile System is created for small tile Based Games
mod tile_system{
    use crate::utils::structures::QuadTree;

pub struct Tile{
        x:i32,
        y:i32,
        width:i32,
        height:i32
    }

    pub struct TileMap {
        tiles:QuadTree<Tile>
    }
    impl TileMap{
        pub fn new()->Self{
            TileMap{
                tiles:QuadTree::new()
            }
        }
    }
}
