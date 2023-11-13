
use crate::scene::Scene;
extern crate gl;

pub struct TestScene
{

}

impl TestScene
{
    pub fn new() -> Self
    {
        Self { }
    }
}

impl Scene for TestScene
{
    fn init(&mut self) -> ()
    {

    }

    fn render(&self) -> ()
    {
        unsafe {
            gl::ClearColor(0.6, 1.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

