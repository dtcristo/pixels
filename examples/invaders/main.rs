use pixels::{Error, Pixels, SurfaceTexture};
use winit::event;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();

    let (window, surface, width, height) = {
        let window = winit::window::Window::new(&event_loop).unwrap();
        let surface = wgpu::Surface::create(&window);
        let size = window.inner_size().to_physical(window.hidpi_factor());

        (window, surface, size.width as u32, size.height as u32)
    };

    let surface_texture = SurfaceTexture::new(width, height, &surface);
    let mut fb = Pixels::new(320, 240, surface_texture)?;

    event_loop.run(move |event, _, control_flow| match event {
        event::Event::WindowEvent { event, .. } => match event {
            event::WindowEvent::KeyboardInput {
                input:
                    event::KeyboardInput {
                        virtual_keycode: Some(event::VirtualKeyCode::Escape),
                        state: event::ElementState::Pressed,
                        ..
                    },
                ..
            }
            | event::WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            event::WindowEvent::RedrawRequested => fb.render(),
            _ => (),
        },
        event::Event::EventsCleared => window.request_redraw(),
        _ => (),
    });
}
