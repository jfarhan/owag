use crate::utils::{math,physics};
use crate::renderer::AnimStates;
pub struct ComponentManager{
   pub  free_indices:Vec<usize>,
}

pub struct ComponentMap{
    pub anim_states:Vec<AnimStates>,
    pub free_indices:Vec<usize>,
    pub indices:Vec<bool>,
    pub position_components:Vec<math::Vector2>,
    pub colliders:Vec<physics::ColliderBox>,
}

impl ComponentMap{

    pub fn new()->ComponentMap{
        ComponentMap{
            anim_states:Vec::new(),
            free_indices:Vec::new(),
            indices:Vec::new(),
            position_components:Vec::new(),
            colliders:Vec::new(),
            }
        }

    pub fn create_new_entity(&mut self)->Entity{
        if self.free_indices.is_empty(){
            self.indices.push(true);
            self.load_all();
            Entity{ index:(self.indices.len()-1),components:Vec::new()}
        }else{
            let index=self.free_indices.pop().unwrap();
            self.indices[index]=true;
            Entity{ index:index,components:Vec::new()}
        }
    }

    pub fn load_all(&mut self){
        self.position_components.push(math::Vector2::new());
        self.colliders.push(physics::ColliderBox::new());
        self.anim_states.push(AnimStates::new());
    }

    pub fn murder_entity(&mut self,ent:Entity)->usize{
        self.indices[ent.index]=false;
        self.free_indices.push(ent.index);
        ent.index
    }

    pub fn collider_update(&mut self){
        let length=self.colliders.len();
        for m in 0..length{
            self.colliders[m].x=self.position_components[m].x;
            self.colliders[m].y=self.position_components[m].y;
        }

        for m in 0..(length-1){

                if self.colliders[m].collide(&self.colliders[m+1]){
                    println!("// Debug ecs/mod.rs : 60 : COLLIDED {:?}",(m,m+1));
                    self.anim_states[0].change_state(2);
                    self.position_components[m].x-=self.colliders[m].width/2.0;
                    self.position_components[m].y-=self.colliders[m].height/2.0;
                    self.position_components[m+1].x+=self.colliders[m+1].width/2.0;
                    self.position_components[m+1].y+=self.colliders[m+1].height/2.0;

                }

        }
    }

    pub fn pos_components(&mut self,entity:&Entity)->&mut math::Vector2{
        &mut self.position_components[entity.index]
    }

    pub fn colliders(&mut self,entity:&Entity)->&mut physics::ColliderBox{
        &mut self.colliders[entity.index]
    }


}


pub enum ComponentTypes{
    PositionComponent,
    ColliderComponent,
}

pub struct Entity{
    pub index:usize,
    pub components:Vec<ComponentTypes>
}


#[cfg(test)]
mod test{
    #[test]
    fn entity_test(){
    }
}
