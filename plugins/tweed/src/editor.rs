use std::sync::Arc;

use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};

use crate::TweedParams;

#[derive(Lens)]
struct Data {
    params: Arc<TweedParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (200, 250))
}

pub(crate) fn create(
    params: Arc<TweedParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Tweed GUI");

            Label::new(cx, "Gain");
            ParamSlider::new(cx, Data::params, |params| &params.gain);

            Label::new(cx, "Model Selector");
            ParamSlider::new(cx, Data::params, |params| &params.model);
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
}
