pub mod physics_engine;


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


pub mod physics{
    pub trait Collider{
        fn collide(&self,other:&ColliderBox)->bool;
    }

    pub struct ColliderBox{
        pub x:f32,
        pub y:f32,
        pub width:f32,
        pub height:f32
    }
    impl ColliderBox{
        pub fn new()->ColliderBox{
            ColliderBox{
            x:0.0,
            y:0.0,
            width:0.0,
            height:0.0
            }
        }
    }


    impl  ColliderBox{
        pub fn collide(&self,other:&Self)->bool{
            let x_diff=self.x-other.x;
            let y_diff =self.y-other.y;
            let width_sum=self.width+other.width;
            let height_sum=self.height+other.height;
            if x_diff.abs()<width_sum && y_diff.abs()<height_sum{true}
            else {false}
         }
    }

}
