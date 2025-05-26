use winit::application::ApplicationHandler;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::event::WindowEvent;


#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
        
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Exiting app");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested =>  {
                self.window.as_ref().unwrap().request_redraw();

            },
            _ => {},
        }
        
    }
}


fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

    let _ = event_loop.run_app(&mut app);
}
