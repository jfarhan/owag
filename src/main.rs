use owag::ecs::{Entity,ComponentMap};
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


/*
    // Entity Initialisations
    let mut time_manager=std::time::Instant::now();
    let mut manager=ComponentMap::new();
    let player=manager.create_new_entity();
    let enemy=manager.create_new_entity();
    manager.pos_components(&enemy).x=300.0;
    manager.pos_components(&enemy).y=300.0;
    manager.pos_components(&player).x=100.0;
    manager.pos_components(&player).y=100.0;

    loader::load_texture(& mut textures,&texture_creator,"assets/sprites/hero.png");
    loader::load_texture(&mut textures,&texture_creator,"assets/sprites/npc.png");
    loader::load_player(&mut manager,&player);
    loader::load_enemy(&mut manager,&enemy);
*/
    let mut ball=owag::utils::physics_engine::PhysicsObject::new();
    ball.position.x=100.0;
    ball.position.y=100.0;
    ball.velocity.x=1.0;
    ball.velocity.y=0.0;
    ball.collision_bound.radius=25.0;
    ball.mass=100.0;
    //ball.acceleration.y=0.09;

    let mut ball_1=owag::utils::physics_engine::PhysicsObject::new();
    ball_1.position.x=500.0;
    ball_1.position.y=100.0;
    ball_1.velocity.x=-2.3;
    ball_1.velocity.y=0.0;
    ball_1.collision_bound.radius=25.0;
    //ball_1.acceleration.y=0.09;
    let mut running =true;



    // Main Event Loop
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

         canvas.fill_rect(sdl2::rect::Rect::new(ball.position.x as i32+(ball.collision_bound.radius/2.0) as i32,
                                                ball.position.y as i32+(ball.collision_bound.radius/2.0) as i32
                                            ,(ball.collision_bound.radius*2.0) as u32
                                            ,(ball.collision_bound.radius*2.0) as u32));
        canvas.set_draw_color(Color::RGB(100,100,100));

        canvas.fill_rect(sdl2::rect::Rect::new(ball_1.position.x as i32+(ball_1.collision_bound.radius/2.0) as i32
                                            ,ball_1.position.y as i32 + (ball_1.collision_bound.radius/2.0) as i32
                                            ,(ball_1.collision_bound.radius*2.0) as u32
                                            ,(ball_1.collision_bound.radius*2.0) as u32));


        ball_1.velocity=ball_1.velocity + ball_1.acceleration;
        ball.velocity=ball.velocity + ball.acceleration;
        ball_1.position =  ball_1.position + ball_1.velocity ;
        ball.position   =  ball.position   + ball.velocity   ;
        canvas.present();
        if owag::utils::physics_engine::PhysicsEngine::collision_check(&ball,&ball_1){
            owag::utils::physics_engine::PhysicsEngine::resolve_collisions(&mut ball,&mut ball_1)
        }
        std::thread::sleep_ms(16);
    }
}






fn resolve(keys:HashSet<sdl2::keyboard::Keycode>,manager:&mut ComponentMap,player:&Entity){
    //let phys_comps=manager.pos_components(player);
    if keys.contains(&Keycode::Up){

    }
    if keys.contains(&Keycode::Down){
        //phys_comps.y+=0.3
    }
    if keys.contains(&Keycode::Left){
        //phys_comps.x-=0.3
    }
    if keys.contains(&Keycode::Right){
        //phys_comps.x+=0.3
    }
}

fn render(canvas:&mut Canvas<Window>
        ,manager:&mut ComponentMap
        ,textures:&Vec<sdl2::render::Texture>
        ,time:&mut std::time::Instant){

    use sdl2::rect::Rect;
    let present_time=std::time::Instant::now();
    let diff=(present_time-*time).as_secs_f32();
    let mut cond:bool=false;
    if diff>0.1{
        *time=present_time;
        cond=true;
        }

}


fn update(manager:&mut ComponentMap,player:&Entity){

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



    pub fn load_texture(textures:&mut Vec<sdl2::render::Texture>,
                        texture_creator:&sdl2::render::TextureCreator<sdl2::video::WindowContext>
                        ,file_name:&str){
        let path=Path::new(file_name);
        textures.push(texture_creator.load_texture(path).unwrap());
    }
}
