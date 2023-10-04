// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use pittore::{
    PittoreColor,
    PittoreContextBuilder,
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let context = PittoreContextBuilder::new()
        .build()
        .expect("Failed to create a Pittore Context!");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let render_target = context.attach_to_window(&window)
        .expect("Failed to create a render target (by attaching to the window)");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,

            Event::RedrawRequested(..) => {
                render_target.begin_render_pass(&mut |render_pass| {
                    render_pass.clear(PittoreColor::RED)
                }).unwrap();
            }

            _ => (),
        }
    });
}
