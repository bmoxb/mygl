use mygl::{Shader, FragmentShader, VertexShader, ShaderProgram};

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Triangles");

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    if let Err(e) = run(el, windowed_context) {
        eprintln!("{}", e);
    }
}

fn run(el: EventLoop<()>, windowed_context: ContextWrapper<PossiblyCurrent, glutin::window::Window>) -> Result<(), mygl::Error> {
    gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

    let vert = VertexShader::from_file("examples/shaders/triangle.vert")?;
    let frag = FragmentShader::from_file("examples/shaders/triangle.frag")?;
    let prog = ShaderProgram::new(vert, frag)?;

    let uniform = nalgebra::SVector::from([1.0, 0.8, 0.5]);
    prog.set_uniform("myColour", uniform.as_ref())?;

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
