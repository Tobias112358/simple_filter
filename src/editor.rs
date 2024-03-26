use std::sync::Arc;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{create_vizia_editor, widgets::*, ViziaState, ViziaTheming};


use crate::SimpleFilterParams;

mod my_assets;

pub mod my_fonts;

#[derive(Lens)]
struct Data {
    params: Arc<SimpleFilterParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (350, 350))
}

pub(crate) fn create(
    params: Arc<SimpleFilterParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {

        cx.add_font_mem(&my_fonts::RED_ROSE_REGULAR);

        my_assets::register_red_rose_regular(cx);
        my_assets::register_red_rose_bold(cx);
        my_assets::register_red_rose_light(cx);
        my_assets::register_red_rose_semi_bold(cx);
        my_assets::register_red_rose_medium(cx);
        my_assets::register_red_rose_variable_weight(cx);

        Data {
            params: params.clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "HELLO worls")
                .font_family(vec![FamilyOwned::Name(String::from(my_assets::RED_ROSE))])
                .font_weight(FontWeightKeyword::Bold)
                .font_size(30.0)
                .height(Pixels(70.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .color(RGBA::rgb(225,255,255));
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0))
        .height(Pixels(350.0))
        .background_color(RGBA::rgb(0,0,0));

        ResizeHandle::new(cx);
    })
    
}