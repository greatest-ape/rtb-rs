use crate::element::Element;
use crate::platform::PlatformWindow;
use crate::mouse::MouseHandler;

use std::ffi::c_void;

pub struct Size {
    pub width: i32,
    pub height: i32,
}

pub struct Window {
    platform_window: Box<WindowImpl>,
}

impl Window {
    pub fn attach(parent: *mut c_void, _dimensions: Size, _title: &str) -> Self {
        Window  {
            platform_window: Box::new(PlatformWindow {
                id: parent
            })
        }
    }
    pub fn close(self) {
        //TODO: cleanup
        //drop
    }
    pub fn draw(&mut self, force_redraw: bool) -> bool {
        self.platform_window.draw(force_redraw)
    }
    pub fn focus_element(&mut self, element: &mut Element) {
        self.platform_window.focus_element(element);
    }
    pub fn lock(&mut self) {
        self.platform_window.lock();
    }
    pub fn unlock(&mut self) {
        self.platform_window.unlock();
    }
    pub fn reinit(&mut self) {
        unimplemented!();
    }
}

pub trait WindowImpl: Drop + MouseHandler {
    fn draw(&mut self, force_redraw: bool) -> bool;
    fn focus_element(&mut self, element: &mut Element);
    fn lock(&mut self);
    fn unlock(&mut self);
    fn reinit(&mut self);
    fn resize(&mut self, _size: Size) { unimplemented!() }
    fn set_title(&mut self, _title: &str) { unimplemented!() }

    fn draw(&mut self, _force_redraw: bool) -> bool { unimplemented!() }
    fn focus_element(&mut self, _element: &mut Element) { unimplemented!() }
    fn lock(&mut self) { unimplemented!() }
    fn unlock(&mut self) { unimplemented!() }
    fn reinit(&mut self) { unimplemented!() }
}
