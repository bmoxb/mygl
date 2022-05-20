use mygl::shaders::{FragmentShader, Shader, ShaderProgram, VertexShader};
use mygl::vao::{
    AttribPointerType, BufferUsageHint, DrawMode, VertexArrayObjectBuilder, VertexBufferObject,
};

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, ContextWrapper, GlRequest, PossiblyCurrent};

fn main() {
    env_logger::init();

    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Triangles");

    let windowed_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    if let Err(e) = run(el, windowed_context) {
        eprintln!("{}", e);
    }
}

fn run(
    el: EventLoop<()>,
    windowed_context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
) -> Result<(), mygl::Error> {
    gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

    let vert = VertexShader::from_file("examples/shaders/triangle.vert")?;
    let frag = FragmentShader::from_file("examples/shaders/triangle.frag")?;
    let prog = ShaderProgram::new(vert, frag)?;

    let uniform = [0.0, 0.2, 0.9];
    prog.set_uniform("myColour", &uniform)?;

    let data: [f32; 9] = [-0.5, -0.5, 0.0, -0.5, 0.5, 0.0, 0.5, -0.5, 0.0];
    let vbo = VertexBufferObject::new(BufferUsageHint::Static, data);

    let vao = VertexArrayObjectBuilder::new()
        .attrib_pointer(vbo, 0, 3, AttribPointerType::Float, false, 3 * 4)
        .build();

    //mygl::enable_wireframe_rendering();

    el.run(move |event, _, control_flow| match event {
        Event::LoopDestroyed => return,
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::MainEventsCleared => {
            mygl::clear(0.8, 0.2, 0.2, 1.0);

            prog.use_program();
            vao.draw_arrays(DrawMode::Triangles, 0, 6);

            windowed_context.swap_buffers().unwrap();
        }

        _ => (),
    });
}
