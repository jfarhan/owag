pub struct World{
    pub n_blocks_x:usize,
    pub n_blocks_y:usize,
    block_width:u32,
    block_height:u32,
    state:State
}

pub struct State{
    weather:usize
    // for the time being 
}


// create  method for rendering state 
// state requires something that displays an overlay 
// sunny / windy etc 
// requires such methods 
