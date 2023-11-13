
mod application;
mod scene;
mod scenes
{
    pub mod test
    {
        pub mod scene;
    }
}

use scenes::test::scene::TestScene;

fn main() {

    let mut app = application::Application::new(String::from("Project Zeus"), 800, 600);
    let scene = TestScene::new();

    app.set_scene(Box::new(scene));
    app.run();
}