use crate::prelude::DataTypes;
use crate::utils::physics_engine;
use crate::renderer::AnimStates;
use crate::utils::interactions;
use std::collections::HashMap;
pub mod systems;

pub struct ComponentMap{
    pub data_bank:Vec<DataTypes>,
    pub anim_states:Vec<AnimStates>,
    pub free_indices:Vec<usize>,
    pub indices:Vec<bool>,
    pub physics_components:Vec<physics_engine::PhysicsObject>,
    pub interaction_blocks:Vec<interactions::InteractionRect>,
    pub states:Vec<states::EntityStates>
}

impl ComponentMap{

    pub fn new()->ComponentMap{
        ComponentMap{
            data_bank:Vec::with_capacity(50),
            anim_states:Vec::with_capacity(50),
            free_indices:Vec::with_capacity(50),
            indices:Vec::with_capacity(50),
            physics_components:Vec::with_capacity(50),
            interaction_blocks:Vec::with_capacity(50),
            states:Vec::with_capacity(50),
            }
        }

    pub fn create_new_entity(&mut self)->Entity{
        if self.free_indices.is_empty(){
            self.indices.push(true);
            self.load_all();
            Entity{ index:(self.indices.len()-1),components:Vec::new(),data:Vec::new()}
        }else{
            let index=self.free_indices.pop().unwrap();
            self.indices[index]=true;
            Entity{ index:index,components:Vec::new(),data:Vec::new()}
        }
    }

    pub fn load_all(&mut self){
        self.physics_components.push(physics_engine::PhysicsObject::new());
        self.anim_states.push(AnimStates::new());
        self.states.push(states::EntityStates::new())
    }

    pub fn destroy_entity(&mut self,ent:Entity)->usize{
        self.indices[ent.index]=false;
        self.free_indices.push(ent.index);
        ent.index
    }

    pub fn unsafe_destroy_entity_given_index(&mut self,index:usize)->usize{
        self.indices[index]=false;
        self.free_indices.push(index);
        index
    }

    pub fn update_states(&mut self){

    }
    pub fn test_system(&mut self){
        systems::collision_resolution(self)
    }

}

pub enum ComponentTypes{
    PositionComponent,
    ColliderComponent,
}

pub struct Entity{
    pub index:usize,
    pub components:Vec<ComponentTypes>,
    pub data:Vec<(String,usize)>,
}

mod states{
    pub struct EntityStates{
        state_stack:Vec<usize>,
        current_time:f32,
        revert_time:Option<f32>
    }

    impl EntityStates{
        pub fn new()->EntityStates{
            EntityStates{
                state_stack:Vec::with_capacity(5),
                current_time:0.0,
                revert_time:None
            }
        }
    }

    pub struct GameState{
        state:usize,
        events:usize
    }
}





#[cfg(test)]
mod test{
    #[test]
    fn entity_test(){
    }
}
