use std::sync::Arc;

use iced_baseview::{executor, Application, Command};
use iced_baseview::{
    Container, Column, Element, Row, Text,
};
use iced_audio::{
    knob, Normal, NormalParam
};

use crate::SyncOnlyState;


#[derive(Debug, Clone)]
pub enum Message {
    ParameterChange(usize, Normal),
}


trait ParameterWidget {
    fn view(&mut self) -> Element<Message>;
}


#[derive(Debug, Clone)]
struct OctaSineKnob {
    knob_state: knob::State,
    title: String,
    parameter_index: usize,
}


impl OctaSineKnob {
    fn new(
        sync_only: &Arc<SyncOnlyState>,
        title: String,
        parameter_index: usize
    ) -> Self {
        let value = Normal::new(sync_only.presets.get_parameter_value_float(
            parameter_index
        ) as f32);

        let normal_param = NormalParam {
            value,
            default: value, // FIXME
        };
        
        Self {
            knob_state: knob::State::new(normal_param),
            title,
            parameter_index
        }
    }
}


impl ParameterWidget for OctaSineKnob {
    fn view(&mut self) -> Element<Message> {
        let value_str = format!(
            "{:.4}",
            self.knob_state.normal_param.value.as_f32()
        );

        let title = Text::new(self.title.clone()).size(12);
        let value = Text::new(value_str).size(12);

        let parameter_index = self.parameter_index;

        let knob = knob::Knob::new(
            &mut self.knob_state,
            move |value| Message::ParameterChange(parameter_index, value)
        );

        Column::new()
            .push(Container::new(title).padding(4))
            .push(Container::new(knob).padding(4))
            .push(Container::new(value).padding(4))
            .into()
    }
}


pub(super) struct OctaSineIcedApplication {
    sync_only: Arc<SyncOnlyState>,
    master_volume: OctaSineKnob,
    master_frequency: OctaSineKnob,
}


impl Application for OctaSineIcedApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Arc<SyncOnlyState>;

    fn new(sync_only: Self::Flags) -> (Self, Command<Self::Message>) {
        let master_volume = OctaSineKnob::new(
            &sync_only,
            "Master\nvolume".to_string(),
            0
        );
        let master_frequency = OctaSineKnob::new(
            &sync_only,
            "Master\nfrequency".to_string(),
            1
        );

        let app = Self {
            sync_only,
            master_volume,
            master_frequency
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        crate::PLUGIN_NAME.into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ParameterChange(index, value) => {
                self.sync_only.presets.set_parameter_value_float(
                    index,
                    value.as_f32() as f64
                )
            },
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let master_volume = self.master_volume.view();
        let master_frequency = self.master_frequency.view();
        
        Column::new()
            .push(
                Row::new()
                    .padding(24)
                    .push(master_volume)
                    .push(master_frequency)
            )
            .into()
    }
}