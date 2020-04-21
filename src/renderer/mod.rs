pub struct AnimStates{
    pub frames:Vec<Vec<(i32,i32)>>,
    pub frame:usize,
    pub state:usize,
    pub texture_id:usize
    }

impl AnimStates{
    pub fn new()->AnimStates{
        AnimStates{
            frames:Vec::new(),
            frame:0,
            state:0,
            texture_id:0
        }
    }
    pub fn change_state(&mut self,state:usize){
        self.state=state;
    }
    pub fn next_frame(&mut self)->(i32,i32){
        self.frame=(self.frame+1)%self.frames[self.state].len();
        self.frames[self.state][self.frame]
    }
    pub fn get_frame(&mut self)->(i32,i32){
        self.frames[self.state][self.frame]
    }
}
