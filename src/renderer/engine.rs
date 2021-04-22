extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::path::Path;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;
use crate::core::input::UiCommand;
// use simulation::{self, Exit, InitializedSimulationBackend, Perf, Renderer, Simulation};

/*
use amethyst::utils::ortho_camera::*;

fn point_eye_of_god(&world) -> Camera{
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Transform::default())
        .with(Camera::standard_2d(1920.0, 1080.0))
        .with(CameraOrtho::normalized(CameraNormalizeMode::Contain))
        .build();
}
*/

pub fn look_into_the_mishmash() {
    println!("linked sdl2_ttf: {}", sdl2::ttf::get_linked_version());
    let ttf_context = match sdl2::ttf::init().map_err(|e| e.to_string()) {
        Ok(x) => x,
        Err(why) => panic!("failure initializing ttf_context because {}", why) 
    };

    // Open a window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("opened_eye demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));

    let font_path: &Path = Path::new("/Users/christopherquion/Documents/GitHub/ogre-romance/assets/fonts/Bitstream-Vera-Sans-Mono/VeraMono.ttf");
    println!("{:?}", font_path);
    let texture_creator = canvas.texture_creator();

    // Load a font
    let mut font = match ttf_context.load_font(font_path, 64 ) {
        Ok(x) => x,
        Err(why) => panic!("failure loading text context because: {}", why)
    };
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    // render a surface, and convert it to a texture bound to the canvas
    let surface = font
     .render("Hello!")
     .blended(Color::RGBA(255, 0, 0, 255))
     .map_err(|e| e.to_string());

    let surface = match surface {
        Ok(s) => s, 
        Err(why) => panic!("surface failure because {}", why)
    };

    let texture = match texture_creator.create_texture_from_surface(&surface)
     .map_err(|e| e.to_string()) {
        Ok(x) => x,
        Err(e) => panic!("texture failure due to {}", e)
     };

    let TextureQuery { width, height, .. } = texture.query();

    // If the example text is too big for the screen, downscale it (and center irregardless)
    let padding = 64;
    let target = get_centered_rect(
        width,
        height,
        SCREEN_WIDTH - padding,
        SCREEN_HEIGHT - padding,
    );
    println!("target: {:?}", target);

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.clear();
        canvas.copy(&texture, None, Some(target));

        // The rest of the game loop goes here...
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

pub fn write_something_to_the_screen() -> Result<(), String> {
    println!("linked sdl2_ttf: {}", sdl2::ttf::get_linked_version());

    Ok(())
}