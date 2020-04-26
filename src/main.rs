use owag::utils::interactions::Interactable;
use owag::ecs::{Entity,ComponentMap};
use owag::utils::physics_engine::PhysicsEngine;
use owag::utils::math::Vector2;
use std::collections::HashSet;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;




fn main() {


    //  SDL initialisations
    let sdl_context=sdl2::init().unwrap();
    let video_subsystem=sdl_context.video().unwrap();
    let window=video_subsystem.window("test",1024,768)
    .position_centered().build().unwrap();
    let mut canvas=window.into_canvas().build().unwrap();
    let mut events=sdl_context.event_pump().unwrap();
    canvas.set_draw_color(Color::RGB(0,230,0));
    let texture_creator=canvas.texture_creator();
    let mut textures:Vec<sdl2::render::Texture>=Vec::new();
    loader::load_texture(&mut textures, &texture_creator,"assets/sprites/hero.png");
    loader::load_texture(&mut textures, &texture_creator,"assets/sprites/npc.png");


    // Game Stuff
    let mut manager=owag::ecs::ComponentMap::new();

    let hero=manager.create_new_entity();
    loader::load_player(&mut manager,&hero);
    manager.physics_components[hero.index].collision_bound.radius=32.0;
    manager.physics_components[hero.index].position=Vector2{x:100.0,y:100.0};



    let enemy=manager.create_new_entity();
    loader::load_enemy(&mut manager,&enemy);
    manager.physics_components[enemy.index].collision_bound.radius=32.0;
    manager.physics_components[enemy.index].position=Vector2{x:500.0,y:500.0};
    manager.physics_components[enemy.index].velocity=Vector2{x:0.5,y:0.5};


    let enemy_2=manager.create_new_entity();
    loader::load_enemy(&mut manager,&enemy_2);
    manager.physics_components[enemy_2.index].collision_bound.radius=32.0;
    manager.physics_components[enemy_2.index].position=Vector2{x:300.0,y:700.0};
    manager.physics_components[enemy_2.index].velocity=Vector2{x:0.5,y:0.5};

    let mut bullets=Vec::new();
    let bullet_lifetime=1.5;
    let firing_interval=1.5;
    let mut bullet_life=std::time::Instant::now();
    let mut fire_time_interval=std::time::Instant::now();
    let mut render_time=std::time::Instant::now();






    // Main Event Loop
    let mut running =true;
    while running{
        canvas.set_draw_color(Color::RGB(0,230,0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(235,230,230));
        for event in events.poll_iter() {
              match event {
                  Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape),..}=>
                    {   running = false },
                  _=>(),
              }
          }
        let key_presses:HashSet<Keycode>=events
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode).collect();

    resolve(key_presses,&mut manager,&hero);



    // Game Update loop
    let temp=manager.physics_components[hero.index].position-manager.physics_components[enemy.index].position;
    let temp_2=manager.physics_components[hero.index].position-manager.physics_components[enemy_2.index].position;
    manager.physics_components[enemy.index].velocity=
    temp * (manager.physics_components[enemy.index].velocity.modulus()/temp.modulus());

    manager.physics_components[enemy_2.index].velocity=
    temp_2 * (manager.physics_components[enemy_2.index].velocity.modulus()/temp_2.modulus());

    let time_now=std::time::Instant::now();
    if (time_now-bullet_life).as_secs_f32()>bullet_lifetime{
        if !bullets.is_empty()
        {manager.destroy_entity(bullets.remove(0));
        manager.destroy_entity(bullets.remove(0));}
        bullet_life=time_now
    }
    if (time_now-fire_time_interval).as_secs_f32()>firing_interval{
        let bullet=manager.create_new_entity();
        manager.physics_components[bullet.index].position=manager.physics_components[enemy.index].position;
        manager.physics_components[bullet.index].velocity=manager.physics_components[enemy.index].velocity*10.0;
        manager.physics_components[bullet.index].collision_bound.radius=16.0;


        let other_bullet=manager.create_new_entity();
        manager.physics_components[other_bullet.index].position=manager.physics_components[enemy_2.index].position;
        manager.physics_components[other_bullet.index].velocity=manager.physics_components[enemy_2.index].velocity*10.0;
        manager.physics_components[other_bullet.index].collision_bound.radius=16.0;
        loader::load_bullet(&mut manager,&bullet);
        loader::load_bullet(&mut manager,&other_bullet);
        bullets.push(bullet);
        bullets.push(other_bullet);
        fire_time_interval=time_now;
    }

    for bullet in &bullets{
    if manager.indices[bullet.index]{
    if PhysicsEngine::collision_check(&manager.physics_components[bullet.index],
                                          &manager.physics_components[hero.index]){
            manager.unsafe_destroy_entity_given_index(bullet.index);
            }
        }
    }

        update(&mut manager);
        render(&mut canvas,&mut manager,&textures,&mut render_time);
        canvas.present();
            manager.physics_components[hero.index].velocity.x/=1.05;
            manager.physics_components[hero.index].velocity.y/=1.05;

        std::thread::sleep_ms(16);
    }
}






fn resolve(keys:HashSet<sdl2::keyboard::Keycode>,manager:&mut ComponentMap,player:&Entity){
    let phys_comps=& mut manager.physics_components[player.index];
    if keys.contains(&Keycode::Up){
        phys_comps.velocity.y-=0.3;
    }
    if keys.contains(&Keycode::Down){
        phys_comps.velocity.y+=0.3
    }
    if keys.contains(&Keycode::Left){
        phys_comps.velocity.x-=0.3
    }
    if keys.contains(&Keycode::Right){
        phys_comps.velocity.x+=0.3
    }
}


fn render(canvas:&mut Canvas<Window>,manager:&mut ComponentMap
        ,textures:&Vec<sdl2::render::Texture>,time:&mut std::time::Instant){
    use sdl2::rect::Rect;
    let present_time=std::time::Instant::now();
    let diff=(present_time-*time).as_secs_f32();
    let mut can_render=false;
    if diff>0.1{
        *time=present_time;
        can_render=true
    }

    for m in 0..manager.indices.len(){
        let mut f=manager.anim_states[m].get_frame();
        if can_render{ f=manager.anim_states[m].next_frame()}
        let r=PhysicsEngine::get_display_format(&manager.physics_components[m]);
        if manager.indices[m]
        {   canvas.copy(&textures[manager.anim_states[m].texture_id]
            ,Rect::new(f.0,f.1,16,16)
            ,Rect::new(r.0,r.1,r.2,r.3)).unwrap()
        }
    }
}


fn update(manager:&mut ComponentMap){
    for m in 0..manager.indices.len(){
        if manager.indices[m]{
            manager.physics_components[m].position=
            manager.physics_components[m].position
            + manager.physics_components[m].velocity;
        }
    }
    manager.test_system();
}


pub mod loader{
    use sdl2::image::LoadTexture;
    use std::path::Path;
    use owag::ecs::{ComponentMap,Entity};
    use owag::renderer::AnimStates;
    pub fn load_player(manager:&mut ComponentMap,entity:&Entity){
        manager.anim_states[entity.index]=
        AnimStates{
            frames:vec![
            vec![(0,32),(16,32),(32,32)],
            vec![(0,16),(16,16),(32,16)],
            vec![(0,48),(16,48),(32,48)]
            ],
            frame:0,
            state:0,
            texture_id:0
        };
    }

    pub fn load_enemy(manager:&mut ComponentMap,entity:&Entity){
        manager.anim_states[entity.index]=
        AnimStates{
            frames:vec![
            vec![(0,0),(16,0),(32,0)],
            ],
            frame:0,
            state:0,
            texture_id:1
        };



    }
    pub fn load_bullet(manager:&mut ComponentMap,entity:&Entity){
        manager.anim_states[entity.index]=
        AnimStates{
            frames:vec![
            vec![(0,1)]
            ],
            frame:0,
            state:0,
            texture_id:1
        }
    }

    pub fn load_texture(textures:&mut Vec<sdl2::render::Texture>,
                        texture_creator:&sdl2::render::TextureCreator<sdl2::video::WindowContext>
                        ,file_name:&str){
        let path=Path::new(file_name);
        textures.push(texture_creator.load_texture(path).unwrap());
    }
}
