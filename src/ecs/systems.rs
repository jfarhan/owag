use crate::ecs::ComponentMap;
use crate::utils::physics_engine::PhysicsEngine;

fn alter(val:&mut ComponentMap){
    println!("{:?}",String::from("my butt") );
}


pub fn collision_resolution(manager:&mut ComponentMap){
    let e=1.0;
    let len=manager.indices.len();
    for m in 0..len{
    for n in 0..len{
    if manager.indices[m]&&manager.indices[n]{
        if !PhysicsEngine::collision_check(&manager.physics_components[m],&manager.physics_components[n]){
            break
        }
        if m==n{
            break
        }
        let u1=&manager.physics_components[m].velocity;
        let u2=&manager.physics_components[n].velocity;
        let m1=manager.physics_components[m].mass;
        let m2=manager.physics_components[n].mass;
        let mass_sum=m1+m2;
        let v1_x=(1.0/mass_sum)*((m1-e*m2)*u1.x + (m2+e*m1)*u2.x);
        let v1_y=(1.0/mass_sum)*((m1-e*m2)*u1.y + (m2+e*m1)*u2.y);
        let v2_x=(1.0/mass_sum)*((m2-e*m1)*u2.x + (m1+e*m2)*u1.x);
        let v2_y=(1.0/mass_sum)*((m2-e*m1)*u2.y + (m1+e*m2)*u1.y);
        manager.physics_components[m].velocity.x=v1_x;
        manager.physics_components[m].velocity.y=v1_y;
        manager.physics_components[n].velocity.x=v2_x;
        manager.physics_components[n].velocity.y=v2_y;
        let mod_diff_vec=manager.physics_components[m].position-manager.physics_components[n].position;
        let modulus=mod_diff_vec.modulus();
        let change=manager.physics_components[m].collision_bound.radius+manager.physics_components[n].collision_bound.radius-modulus;
        manager.physics_components[m].position=manager.physics_components[m].position+mod_diff_vec*(change/(2.0*modulus));
        manager.physics_components[n].position=manager.physics_components[n].position-mod_diff_vec*(change/(2.0*modulus));
            }
        }
    }
}
