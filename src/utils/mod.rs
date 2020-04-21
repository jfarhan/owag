pub mod math{
    pub struct Vector2{
        pub x:f32,
        pub y:f32
    }
    impl Vector2{
        pub fn new()->Vector2{
            Vector2{x:0.0,y:0.0}
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
