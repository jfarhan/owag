use crate::utils::{math,physics,physics_engine};
use crate::renderer::AnimStates;
pub struct ComponentManager{
   pub  free_indices:Vec<usize>,
}

pub struct ComponentMap{
    pub anim_states:Vec<AnimStates>,
    pub free_indices:Vec<usize>,
    pub indices:Vec<bool>,
    pub physics_components:Vec<physics_engine::PhysicsObject>
}

impl ComponentMap{

    pub fn new()->ComponentMap{
        ComponentMap{
            anim_states:Vec::new(),
            free_indices:Vec::new(),
            indices:Vec::new(),
            physics_components:Vec::new()
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
        self.physics_components.push(physics_engine::PhysicsObject::new());
        self.anim_states.push(AnimStates::new());
    }

    pub fn destroy_entity(&mut self,ent:Entity)->usize{
        self.indices[ent.index]=false;
        self.free_indices.push(ent.index);
        ent.index
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
