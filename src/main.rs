mod window;

use glfw::{Action, Key};

fn main() {
    let mut window = window::Window::new(1280, 720, "This is a test");

    loop {
        if window.should_close() {
            // Do app cleanup here
            break;
        }

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
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.close(),
                _ => {}
            }
        }
    }
}
