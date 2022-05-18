use mygl::shader::{self, Shader};
use mygl::Error;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

fn main() -> Result<(), Error> {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Triangles");

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

    let vert = shader::Vertex::from_file("examples/shaders/triangle.vert")?;
    let frag = shader::Fragment::from_file("examples/shaders/triangle.frag")?;
    let prog = shader::Program::new(vert, frag)?;
    prog.set_uniform("example", 10)?;

    el.run(move |event, _, control_flow| {
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::MainEventsCleared => {
                mygl::clear(0.8, 0.2, 0.2, 1.0);
                // ...
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
