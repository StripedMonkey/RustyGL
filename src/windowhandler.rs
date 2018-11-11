extern crate glium;
use self::glium::{Surface, glutin};

//Window "Class"

pub struct Window {
    events_loop: glutin::EventsLoop,
    window: glutin::WindowBuilder,
    context: glutin::ContextBuilder,
    display: glium::Display,
}

impl Window {
    pub fn new() -> Window {

        let mut events_loop = glutin::EventsLoop::new(); // Creates the loop in the run() function
        let window = glutin::WindowBuilder::new(); //Handles the non-OpenGL part of the window. *Technically* not needed
        let context =  glutin::ContextBuilder::new(); //Handles openGL stuff like Vsync
        let display = glium::Display::new(window, context, &events_loop).unwrap(); //Creates the openGL portion of the window

        Window {
            events_loop,
            window,
            context,
            display,
        }
    }

    fn run(&mut self) {
        let mut closed = false;
        use windowhandler::glium::Surface;
        //Main Loop of the program
        while !closed {
            //creates the next frame of the program
            let mut nframe = self.display.draw();

            //clears the frame to blue
            nframe.clear_color(0.0, 0.0, 0.0, 1.0);

            nframe.finish().unwrap();

            /*Check for events and then do something with them
            *Honestly I don't really know what this does other than it makes a top bar. ¯\_(ツ)_/¯
            *Try commenting it out and see.
            */
            self.events_loop.poll_events(|ev| {
                match ev {
                    glium::glutin::Event::WindowEvent { event, .. } => 
                    match event {
                        glium::glutin::WindowEvent::CloseRequested => closed = true,
                        _ => (),
                    },
                    _ => (),
                }
            });
        }
    }

}
