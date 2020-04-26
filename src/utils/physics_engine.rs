use crate::utils::math::Vector2;
pub trait Collidable{
    fn collision_params(&self)->[f32];
}

pub struct PhysicsObject{
    pub collision_bound:circle_colliders::CollisionCircle,
    pub position : Vector2,
    pub velocity : Vector2,
    pub acceleration : Vector2,
    pub mass:f32
}
impl PhysicsObject{
    pub fn new()->PhysicsObject{
        PhysicsObject{
            collision_bound:circle_colliders::CollisionCircle{radius:1.0},
            position:Vector2::new(),
            velocity:Vector2::new(),
            acceleration:Vector2::new(),
            mass:1.0
        }
    }
}


pub struct PhysicsEngine{
    pub gravity:f32,
    pub restitution:f32,
}

impl PhysicsEngine{

    pub fn collision_check(obj1:&PhysicsObject,obj2:&PhysicsObject)->bool{
        let x_diff= (obj1.position.x-obj2.position.x).abs();
        let y_diff= (obj1.position.y-obj2.position.y).abs();
        let radius_sum=obj1.collision_bound.radius + obj2.collision_bound.radius;
        if x_diff*x_diff + y_diff*y_diff < radius_sum*radius_sum{
            true
        } else
        {   false   }
    }

    pub fn resolve_collisions(obj1:&mut PhysicsObject,obj2:&mut PhysicsObject){
        let e=Self::get_restn_coeff(&obj1, &obj2);
        let u1=&obj1.velocity;
        let u2=&obj2.velocity;
        let m1=obj1.mass;
        let m2=obj2.mass;
        let mass_sum=m1+m2;
        let v1_x=(1.0/mass_sum)*((m1-e*m2)*u1.x + (m2+e*m1)*u2.x);
        let v1_y=(1.0/mass_sum)*((m1-e*m2)*u1.y + (m2+e*m1)*u2.y);
        let v2_x=(1.0/mass_sum)*((m2-e*m1)*u2.x + (m1+e*m2)*u1.x);
        let v2_y=(1.0/mass_sum)*((m2-e*m1)*u2.y + (m1+e*m2)*u1.y);
        obj1.velocity.x=v1_x;
        obj1.velocity.y=v1_y;
        obj2.velocity.x=v2_x;
        obj2.velocity.y=v2_y;
        let mod_diff_vec=obj1.position-obj2.position;
        let modulus=mod_diff_vec.modulus();
        let change=obj1.collision_bound.radius+obj2.collision_bound.radius-modulus;
        obj1.position=obj1.position+mod_diff_vec*(change/(2.0*modulus));
        obj2.position=obj2.position-mod_diff_vec*(change/(2.0*modulus));
    }

    fn get_restn_coeff(_obj1:&PhysicsObject,_obj2:&PhysicsObject)->f32{
            1.0
    }

    pub fn get_display_format(obj:&PhysicsObject)->(i32,i32,u32,u32){
        ((obj.position.x-obj.collision_bound.radius) as i32
        ,(obj.position.y-obj.collision_bound.radius) as i32
        ,(obj.collision_bound.radius*2.0) as u32
        ,(obj.collision_bound.radius*2.0) as u32)
    }
}

pub mod circle_colliders{
    pub struct CollisionCircle{
        pub radius:f32
    }
}

mod rect_structs{
    pub struct CollisionRect{
        pub is_static:bool,
        pub width:f32,
        pub height:f32,
    }
}
