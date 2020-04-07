use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, PossiblyCurrent, WindowedContext};

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("I have no idea what I am doing");
    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    el.run(move |event, _, control_flow| {
        process_input(&event);
        update_game_state();
        render(&event, control_flow, &windowed_context);
    });
}
fn process_input(event: &Event<()>) {
    println!("{:?}", event);
}

fn update_game_state() {}

fn render(
    event: &Event<()>,
    control_flow: &mut ControlFlow,
    windowed_context: &WindowedContext<PossiblyCurrent>,
) {
    *control_flow = ControlFlow::Wait;
    match event {
        Event::LoopDestroyed => return,
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(physical_size) => windowed_context.resize(*physical_size),
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::RedrawRequested(_) => {
            windowed_context.swap_buffers().unwrap();
        }
        _ => (),
    }
}
