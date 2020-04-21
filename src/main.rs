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



    // Entity Initialisations
    let mut time_manager=std::time::Instant::now();
    let mut manager=ComponentMap::new();
    let player=manager.create_new_entity();
    let enemy=manager.create_new_entity();
    //let enemy2=manager.create_new_entity();
    //let enemy3=manager.create_new_entity();
    //let enemy4=manager.create_new_entity();
    manager.pos_components(&enemy).x=300.0;
    manager.pos_components(&enemy).y=300.0;
    //manager.pos_components(&enemy2).x=600.0;
    //manager.pos_components(&enemy2).y=600.0;
    //manager.pos_components(&enemy3).x=800.0;
    //manager.pos_components(&enemy3).y=800.0;
    manager.pos_components(&player).x=100.0;
    manager.pos_components(&player).y=100.0;

    loader::load_texture(& mut textures,&texture_creator,"assets/sprites/hero.png");
    loader::load_texture(&mut textures,&texture_creator,"assets/sprites/npc.png");
    loader::load_player(&mut manager,&player);
    loader::load_enemy(&mut manager,&enemy);
    //loader::load_enemy(&mut manager,&enemy2);
    //loader::load_enemy(&mut manager,&enemy3);
    //loader::load_enemy(&mut manager,&enemy4);
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

        let keypresses:HashSet<Keycode>=events.keyboard_state()
                                            .pressed_scancodes()
                                            .filter_map(Keycode::from_scancode)
                                            .collect();
        resolve(keypresses,&mut manager,&player);
        update(&mut manager,&player);
        render(&mut canvas,&mut manager,&textures,& mut time_manager);
        canvas.present();
        }
}






fn resolve(keys:HashSet<sdl2::keyboard::Keycode>,manager:&mut ComponentMap,player:&Entity){
    let phys_comps=manager.pos_components(player);
    if keys.contains(&Keycode::Up){
        phys_comps.y-=0.3
    }
    if keys.contains(&Keycode::Down){
        phys_comps.y+=0.3
    }
    if keys.contains(&Keycode::Left){
        phys_comps.x-=0.3
    }
    if keys.contains(&Keycode::Right){
        phys_comps.x+=0.3
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

    for anim in 0..manager.anim_states.len()
    {
        let mut frame=manager.anim_states[anim].get_frame();
        if cond{
                frame=manager.anim_states[anim].next_frame();
                println!("// Debug : main.rs : 89 time_diff : CHANGE_FRAME_w_CC");
            }
            canvas.copy(&textures[manager.anim_states[anim].texture_id]
                    , Rect::new(frame.0,frame.1,16,16)
                    , Rect::new(manager.position_components[anim].x as i32
                    ,manager.position_components[anim].y as i32,64,64)).unwrap();
    }

}


fn update(manager:&mut ComponentMap,player:&Entity){
    let player_index=player.index;
    (0..manager.position_components.len()).for_each(|m|{
        if m!=player_index{
            let diff_vec_Y=manager.pos_components(&player).y-manager.position_components[m].y;
            let diff_vec_X=manager.pos_components(&player).x-manager.position_components[m].x;
            let modulus=(diff_vec_X*diff_vec_X + diff_vec_Y*diff_vec_Y).sqrt();
            manager.position_components[m].x+=0.1*diff_vec_X/modulus;
            manager.position_components[m].y+=0.1*diff_vec_Y/modulus;
        }
    });
    manager.collider_update();
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
        manager.colliders[entity.index].width=16.0;
        manager.colliders[entity.index].height=16.0;
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
        manager.colliders[entity.index].width=16.0;
        manager.colliders[entity.index].height=16.0;
    }



    pub fn load_texture(textures:&mut Vec<sdl2::render::Texture>,
                        texture_creator:&sdl2::render::TextureCreator<sdl2::video::WindowContext>
                        ,file_name:&str){
        let path=Path::new(file_name);
        textures.push(texture_creator.load_texture(path).unwrap());
    }
}
