use gpui::{Div, Render};
use story::Story;

use crate::prelude::*;
use crate::Input;

pub struct InputStory;

impl Render for InputStory {
    type Output = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Output {
        Story::container()
            .child(Story::title_for::<Input>())
            .child(Story::label("Default"))
            .child(div().flex().child(Input::new("Search")))
    }
}