

pub trait Scene
{
    fn init(&mut self) -> ();
    fn render(&self) -> ();
}

