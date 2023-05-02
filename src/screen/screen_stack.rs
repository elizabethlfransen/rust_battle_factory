use std::mem;
use crate::screen::screen::Screen;

enum ScreenStackNode {
    Filled(Box<dyn Screen>, Box<ScreenStackNode>),
    Empty,
}


pub struct ScreenStack {
    head: ScreenStackNode,
}


impl ScreenStack {
    pub fn get_current_screen(&self) -> Option<&dyn Screen> {
        match self.head {
            ScreenStackNode::Empty => None,
            ScreenStackNode::Filled(ref screen, _) => Some(screen.as_ref())
        }
    }

    pub fn get_current_screen_as_mut(&mut self) -> Option<&mut dyn Screen> {
        match self.head {
            ScreenStackNode::Empty => None,
            ScreenStackNode::Filled(ref mut screen, _) => Some(screen.as_mut())
        }
    }

    pub fn push(&mut self, screen: impl Screen + 'static) {
        let prev_head = mem::replace(
            &mut self.head,
            ScreenStackNode::Empty,
        );
        self.head = ScreenStackNode::Filled(Box::new(screen), Box::new(prev_head));
    }

    pub fn new(initial_screen: impl Screen + 'static) -> Self {
        let mut result = ScreenStack {
            head: ScreenStackNode::Empty
        };
        result.push(initial_screen);
        return result;
    }
}