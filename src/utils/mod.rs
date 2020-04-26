use crate::prelude::InteractionParams;
pub mod physics_engine;
pub mod interactions;
pub mod structures;


pub mod math{
    #[derive(Copy,Clone)]
    pub struct Vector2{
        pub x:f32,
        pub y:f32
    }
    impl Vector2{
        pub fn new()->Vector2{
            Vector2{x:0.0,y:0.0}
        }
        pub fn modulus(&self)->f32{
            (self.x*self.x+self.y*self.y).sqrt()
        }
    }

    impl core::ops::Add for Vector2{
        type Output=Vector2;
        fn add(self, other:Self) -> <Self as std::ops::Add<Self>>::Output {
            Vector2{
                x:self.x+other.x,
                y:self.y+other.y
            }
        }
    }

    impl core::ops::Sub for Vector2{
        type Output=Vector2;
        fn sub(self,other:Self) -> <Self as std::ops::Sub<Self>>::Output {
            Vector2{
                x:self.x-other.x,
                y:self.y-other.y
            }
        }
    }
    impl core::ops::Mul<f32> for Vector2{
        type Output=Vector2;
        fn mul(self,cons:f32) -> <Self as std::ops::Mul<f32>>::Output {
            Vector2{
                x:self.x*cons,
                y:self.y*cons
            }
        }
    }

    impl core::ops::Div<f32> for Vector2{
        type Output=Vector2;
        fn div(self,cons:f32) -> <Self as std::ops::Mul<f32>>::Output {
            Vector2{
                x:self.x/cons,
                y:self.y/cons
            }
        }
    }
}





impl interactions::Interactable for physics_engine::PhysicsObject{
    fn interaction_params(&self) -> crate::prelude::InteractionParams {
        InteractionParams{
            x:self.position.x,
            y:self.position.y,
            width:self.collision_bound.radius,
            height:self.collision_bound.radius
        }
    }
}
