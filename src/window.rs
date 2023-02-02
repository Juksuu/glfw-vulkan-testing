use std::sync::mpsc;

pub struct Window {
    glfw_window: glfw::Window,
    events: mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new(glfw: &mut glfw::Glfw, width: u32, height: u32, title: &str) -> Self {
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
        glfw.window_hint(glfw::WindowHint::Resizable(false));
        glfw.window_hint(glfw::WindowHint::ContextNoError(true));
        glfw.window_hint(glfw::WindowHint::ContextCreationApi(
            glfw::ContextCreationApi::Native,
        ));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_all_polling(true);
        // window.make_current();

        Self {
            glfw_window: window,
            events,
        }
    }

    pub fn get_raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        use raw_window_handle::HasRawDisplayHandle;
        self.glfw_window.raw_display_handle()
    }

    pub fn get_raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        use raw_window_handle::HasRawWindowHandle;
        self.glfw_window.raw_window_handle()
    }

    pub fn close(&mut self) {
        self.glfw_window.set_should_close(true);
    }

    pub fn should_close(&self) -> bool {
        self.glfw_window.should_close()
    }

    pub fn poll_events(&mut self) -> Vec<glfw::WindowEvent> {
        glfw::flush_messages(&self.events).map(|(_, e)| e).collect()
    }
}
