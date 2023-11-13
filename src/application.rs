extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use crate::scene::Scene;

pub struct Application
{
    title: String,
    width: u32,
    height: u32,
    scene: Option<Box<dyn Scene>>,
}

impl Application
{
    pub fn new(title: String, width: u32, height: u32) -> Self
    {
        Self { title, width, height, scene: None }
    }

    pub fn set_scene(&mut self, scene: Box<dyn Scene>)
    {
        self.scene = Some(scene);
    }

    pub fn run(&mut self)
    {
        self.scene.as_ref().expect("No Scene To Run!");

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem.window(self.title.as_str(), self.width, self.height)
            .opengl()
            .build()
            .unwrap();

        // Unlike the other example above, nobody created a context for your window, so you need to create one.
        let ctx = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        debug_assert_eq!(gl_attr.context_version(), (3, 3));

        let mut event_pump = sdl_context.event_pump().unwrap();

        let scene = self.scene.as_mut().unwrap();
        scene.init();

        'running: loop {

            scene.render();

            window.gl_swap_window();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
