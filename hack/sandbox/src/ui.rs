// SPDX-License-Identifier: GPL-3.0

use libui::gui;

#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl gui::Draw for SelectBox {
    fn draw(&self) {
        println!("{self:?}");
    }
}
