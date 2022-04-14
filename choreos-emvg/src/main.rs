use gstreamer_gl::gst;
use gstreamer_gl::GLDisplay;
use gstreamer_gl::GLWindow;

fn main() {
    match gst::init() {
        Ok(_) => {
    		let disp = GLDisplay::new();
    		let win = GLWindow::new(&disp);
    		println!("OK!");
        },
        Err(_) => println!("POOP!"),
    }
}
