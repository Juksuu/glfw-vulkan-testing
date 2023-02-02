mod vulkan;
mod window;

pub use window::Window;

fn main() {
    let mut glfw = glfw::init(glfw::LOG_ERRORS).unwrap();
    let mut window = window::Window::new(&mut glfw, 1280, 720, "This is a test");

    let _renderer = vulkan::Renderer::new(&window);

    loop {
        if window.should_close() {
            break;
        }

        glfw.poll_events();
        let events = window.poll_events();
        for event in events {
            match event {
                // glfw::WindowEvent::Pos(_, _) => todo!(),
                // glfw::WindowEvent::Size(_, _) => todo!(),
                // glfw::WindowEvent::Close => todo!(),
                // glfw::WindowEvent::Refresh => todo!(),
                // glfw::WindowEvent::Focus(_) => todo!(),
                // glfw::WindowEvent::FramebufferSize(_, _) => todo!(),
                // glfw::WindowEvent::MouseButton(_, _, _) => todo!(),
                // glfw::WindowEvent::CursorPos(_, _) => todo!(),
                // glfw::WindowEvent::CursorEnter(_) => todo!(),
                // glfw::WindowEvent::Scroll(_, _) => todo!(),
                // glfw::WindowEvent::Key(_, _, _, _) => todo!(),
                // glfw::WindowEvent::Char(_) => todo!(),
                // glfw::WindowEvent::CharModifiers(_, _) => todo!(),
                // glfw::WindowEvent::FileDrop(_) => todo!(),
                // glfw::WindowEvent::Maximize(_) => todo!(),
                // glfw::WindowEvent::ContentScale(_, _) => todo!(),
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.close()
                }
                _ => {}
            }
        }
    }
}
