use crate::prelude::InteractionParams;

#[derive(Copy,Clone)]
pub struct InteractionRect{
    pub params:InteractionParams,
    pub assn:usize,
    pub rigid:bool,
}

impl InteractionRect{
    pub fn new(x:f32,y:f32,width:f32,height:f32)->InteractionRect{
        InteractionRect{
            params:InteractionParams{x:x,y:y,width:width,height:height},
            assn:0,
            rigid:false
        }
    }
    pub fn is_rigid(&self)->bool{
        self.rigid
    }

    pub fn did_rigid_collide(&self,other:&mut InteractionParams){
        if self.did_interact(*other)&&self.is_rigid(){
            let diff_x=self.params.x-other.x;
            let diff_y=self.params.y-other.y;
            let wt_sum=(self.params.width + other.width)/2.0;
            let ht_sum=(self.params.height + other.height)/2.0;
            other.x+=(diff_x-wt_sum)*diff_x.signum();
            other.y+=(diff_y-ht_sum)*diff_y.signum();
        }
    }
}


impl Interactable for InteractionRect{
    fn interaction_params(&self) ->InteractionParams {
        self.params
    }
}



pub trait Interactable{
    fn did_interact(&self,other:impl Interactable)->bool{
        let a=self.interaction_params();
        let b=other.interaction_params();
        if (a.x-b.x).abs()<(a.width/2.0+b.width/2.0)&&
        (a.y-b.y).abs()<(a.height/2.0+b.height/2.0){
            true    }else
        {   false   }
    }
    fn interaction_params(&self)->InteractionParams;
}

impl Interactable for InteractionParams{
    fn interaction_params(&self) -> InteractionParams {
        *self
    }
}
