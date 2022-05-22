pub use glutin::event::{Event, WindowEvent};
pub use glutin::event_loop::{ControlFlow, EventLoop};
pub use glutin::window::WindowBuilder;
pub use glutin::{Api, ContextBuilder, ContextWrapper, GlRequest, PossiblyCurrent};

#[allow(unused_macros)]
macro_rules! example {
    ($callback:ident) => {
        fn main() {
            env_logger::init();

            let el = EventLoop::new();
            let wb = WindowBuilder::new().with_title("Triangles");

            let windowed_context = ContextBuilder::new()
                .with_gl(GlRequest::Specific(Api::OpenGl, (4, 3)))
                .with_vsync(true)
                .build_windowed(wb, &el)
                .unwrap();
            let windowed_context = unsafe { windowed_context.make_current().unwrap() };

            gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

            if let Err(e) = $callback(el, windowed_context) {
                eprintln!("{}", e);
            }
        }
    };
}

#[allow(dead_code)]
fn main() {}

#[allow(unused_imports)]
pub(crate) use example;
