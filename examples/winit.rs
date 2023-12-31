// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::{Point2D, Size2D};
use pittore::{
    PittoreColor,
    PittoreContextBuilder,
    PittoreRect,
    PittoreShape,
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

    let image = render_target.load_bitmap_from_file("C:/Windows/SysWOW64/DefaultAccountTile.png")
        .expect("Failed to load standard Windows image");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,

            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                render_target.resize(size.width, size.height)
                    .expect("Failed to resize render target");
            }

            Event::RedrawRequested(..) => {
                let window_size = window.inner_size()
                    .to_logical::<f32>(window.scale_factor());

                render_target.begin_render_pass(&mut |render_pass| {
                    render_pass.clear(PittoreColor::RED);

                    render_pass.fill(PittoreColor::BLUE.into(), PittoreShape::Rectangle(
                        PittoreRect::new(
                            Point2D::new(10.0, 10.0),
                            Size2D::new(window_size.width - 20.0, window_size.height - 20.0)
                        )
                    ));

                    render_pass.fill(PittoreColor::MAGENTA.into(), PittoreShape::Ellipse {
                        center: Point2D::new(window_size.width / 2.0, window_size.height / 2.0),
                        radius: Point2D::new(20.0, 20.0),
                    });

                    render_pass.fill(PittoreColor::WHITE.into(), PittoreShape::Rectangle(PittoreRect::new(
                        Point2D::new(30.0, 30.0),
                        Size2D::new(364.0, 364.0)
                    )));
                    render_pass.fill(image.into(), PittoreShape::Rectangle(PittoreRect::new(
                        Point2D::new(30.0, 30.0),
                        Size2D::new(364.0, 364.0)
                    )));
                }).unwrap();
            }

            _ => (),
        }
    });
}
