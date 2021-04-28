use util;

use glium;
use glium::backend::glutin::glutin::event::{Event, WindowEvent};
use glium::Surface;


fn main() {
    use glium::glutin;

    println!("SuperLuminal Performance Enabled: {}", superluminal_perf::enabled());

    // Set name of current thread for superluminal profiler
    superluminal_perf::set_current_thread_name("Main Thread");

    superluminal_perf::begin_event("Initialization OpenGL");

    // Create an EventLoop for handling window and device events.
    let mut event_loop = glium::glutin::event_loop::EventLoop::new();
    // Specify the window parameters.
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Getting Started");
    // Specify Opengl specific attributes like multisampling and/or vsync.
    let cb = glutin::ContextBuilder::new();


    // Create the OpenGL window and throws a custom message if the display failed to create and throws a panic.
    let display = glium::Display::new(wb, cb, &event_loop)
        .expect("Failed to create display");

    superluminal_perf::end_event();


    event_loop.run(move |evt,_, control_flow|{

        superluminal_perf::begin_event("Start_Draw");

        let mut target = display.draw();
        target.clear_color(0.,0., 1.,1.);
        target.finish().unwrap();

        superluminal_perf::end_event();

        superluminal_perf::begin_event("Window_Handle");

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        // Suspense thread when the current thread finishes and wait until next_frame_time to resume.
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match evt {
            Event::WindowEvent {event , .. } => {

                match event{
                    WindowEvent::CloseRequested => {

                        // Stop the loop for event_loop (closes the window)
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                    },
                    _ => {}
                }
            }
            _ =>{}
        }

        superluminal_perf::end_event();
    });


}

