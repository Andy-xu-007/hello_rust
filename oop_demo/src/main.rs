use std::{path::Component, result};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                heigth: 10,
                options: vec![
                    String::from("YES"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),

            Box::new(Button {
                width: 50,
                heigth: 10,
                label: String::from("OK"),
            })
        ],
    };

    screen.run();
}

pub trait Draw {
    fn draw(&self) ;
}

pub struct Button {
    pub width: u32,
    pub heigth: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self)  {
        
    }
}

struct SelectBox {
    width: u32,
    heigth: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self)  {
        todo!()
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}