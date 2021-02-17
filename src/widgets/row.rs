#![allow(dead_code)]
use crate::core::length::Length;
use crate::element::Element;

pub struct Row<'a, Message> {
    width: Length,
    height: Length,
    max_width: u32,
    max_height: u32,
    children: Vec<Element<'a, Message>>,
}

impl<'a, Message> Row<'a, Message> {
    pub fn new() -> Self {
        Self::with_children(Vec::new())
    }

    pub fn with_children(children: Vec<Element<'a, Message>>) -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            max_height: u32::MAX,
            max_width: u32::MAX,
            children,
        }
    }
}
