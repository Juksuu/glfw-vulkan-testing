use std::sync::mpsc::Receiver;

use glfw::Context;

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_all_polling(true);
        window.make_current();

        Self {
            glfw,
            window,
            events,
        }
    }

    pub fn close(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn poll_events(&mut self) -> Vec<glfw::WindowEvent> {
        self.glfw.poll_events();
        glfw::flush_messages(&self.events).map(|(_, e)| e).collect()
    }
}
