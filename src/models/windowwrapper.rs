#![allow(dead_code)]
use crate::xlibwrapper::util::*;
use crate::xlibwrapper::xlibmodels::*;
use super::rect::*;
use super::WindowState;

#[derive(Copy, Clone)]
pub struct WindowWrapper {
    dec: Option<Window>,
    window: Window,
    window_rect: Rect,
    dec_rect: Option<Rect>,
    desktop: u32,
    restore_position: Position,
    restore_size: Size,
    current_state: WindowState,
    previous_state: WindowState,
    
}

impl WindowWrapper {
    pub fn new(window: Window, window_rect: Rect, desktop: u32) -> Self {
        let restore_size = window_rect.get_size();
        Self {
            dec: None,
            window,
            window_rect,
            dec_rect: None,
            desktop: desktop,
            restore_position: Position { x: 0, y: 0 },
            restore_size,
            current_state: WindowState::Free,
            previous_state: WindowState::Free,
        }
    }
    
    
    pub fn get_window_state(&self) -> WindowState {
        self.current_state.clone()
    }

    pub fn set_window_state(&mut self, state: WindowState) {
        self.previous_state = self.current_state;
        self.current_state = state;
    }
    
    pub fn restore_prev_state(&mut self) {
        let temp = self.current_state.clone();
        self.current_state = self.previous_state;
        self.previous_state = temp; 
    }

    pub fn decorated(&self) -> bool {
        match self.dec {
            Some(_) => true,
            _ => false
        }
    }

    pub fn set_decoration(&mut self, dec: Window, dec_rect: Rect) {
        self.dec = Some(dec);
        self.dec_rect = Some(dec_rect);
    }

    pub fn get_dec(&self) -> Option<Window> {
        match self.dec {
            Some(_) => self.dec.clone(),
            None => None
        }
    }
    pub fn set_dec_size(&mut self, size: Size) {
        match self.dec_rect {
            Some(rect) => {
                self.dec_rect = Some(Rect::new(rect.get_position(), size));
            }
            None => {}
        }
    }

    pub fn set_dec_position(&mut self, position: Position) {
        match self.dec_rect {
            Some(rect) => {
                self.dec_rect = Some(Rect::new(position, rect.get_size()));
            }
            None => {}
        }
    }
    pub fn get_dec_rect(&self) -> Option<Rect> {
        self.dec_rect.clone()
    }
    
    pub fn set_position(&mut self, pos: Position) {
        match self.dec_rect {
            Some(dec_rect) => self.dec_rect = Some(Rect::new(pos, dec_rect.get_size())),
            None => self.window_rect = Rect::new(pos, self.window_rect.get_size())
        }
    }

    pub fn window(&self) -> Window {
        self.window
    }

    pub fn dec_window(&self) -> Option<Window> {
        self.dec
    }
    
    pub fn set_inner_size(&mut self, size: Size) {
        self.window_rect = Rect::new(self.window_rect.get_position(), size);
    }

    pub fn get_inner_rect(&self) -> Rect {
        self.window_rect.clone()
    }

    /*
     * Will set total size of window, so the outmost boundaries (excluding window borders)
     */
    pub fn set_size(&mut self, size: Size) {
        match self.dec_rect {
            Some(_rect) => self.set_dec_size(size),
            None => self.set_inner_size(size)
        }
    }
    
    pub fn get_size(&self) -> Size {
        Size { width: self.get_width(), height: self.get_height() }
    }

    pub fn get_width(&self) -> u32 {
        match self.dec_rect {
            Some(rect) => rect.get_size().width,
            None => self.window_rect.get_size().width
        }
    }

    pub fn get_height(&self) -> u32 {
        match self.dec_rect {
            Some(rect) => rect.get_size().height,
            None => self.window_rect.get_size().height
        }
    }

    pub fn get_position(&self) -> Position {
        match self.dec_rect {
            Some(dec_rect) => dec_rect.get_position(),
            None => self.window_rect.get_position()
        }
    }
    
    pub fn save_restore_position(&mut self) {
        self.restore_position = self.get_position();
    }

    pub fn get_restore_position(&self) -> Position {
        self.restore_position.clone()
    }

    pub fn save_restore_size(&mut self) {
        self.restore_size = self.get_size();
    }

    pub fn get_restore_size(&self) -> Size {
        self.restore_size.clone()
    }
    
    pub fn set_desktop(&mut self, desktop: u32) {
        self.desktop = desktop;
    }

    pub fn get_desktop(&self) -> u32 {
        self.desktop
    }

}
