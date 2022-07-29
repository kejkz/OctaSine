use iced_baseview::{
    alignment::{Horizontal, Vertical},
    button,
    tooltip::Position,
    Alignment, Button, Column, Container, Element, Length, Row, Space, Text, Tooltip,
};

use crate::{
    get_version_info,
    parameters::{MasterFrequencyValue, MasterVolumeValue},
    sync::GuiSyncHandle,
};

use super::{
    common::{container_l1, container_l2, container_l3, space_l3, triple_container},
    knob::{self, OctaSineKnob},
    mod_matrix::ModulationMatrix,
    patch_picker::PatchPicker,
    style::Theme,
    Message, FONT_SIZE, LINE_HEIGHT,
};

pub struct CornerWidgets {
    pub style: Theme,
    pub master_volume: OctaSineKnob<MasterVolumeValue>,
    pub master_frequency: OctaSineKnob<MasterFrequencyValue>,
    pub modulation_matrix: ModulationMatrix,
    pub patch_picker: PatchPicker,
    toggle_info_state: button::State,
    toggle_style_state: button::State,
    save_patch_button: button::State,
    save_bank_button: button::State,
    load_bank_or_patches_button: button::State,
    rename_patch_button: button::State,
}

impl CornerWidgets {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H) -> Self {
        let style = sync_handle.get_gui_settings().theme;

        let master_volume = knob::master_volume(sync_handle, style);
        let master_frequency = knob::master_frequency(sync_handle, style);
        let modulation_matrix = ModulationMatrix::new(sync_handle, style);
        let patch_picker = PatchPicker::new(sync_handle, style);

        Self {
            style,
            master_volume,
            master_frequency,
            modulation_matrix,
            patch_picker,
            toggle_info_state: button::State::default(),
            toggle_style_state: button::State::default(),
            save_patch_button: Default::default(),
            save_bank_button: Default::default(),
            load_bank_or_patches_button: Default::default(),
            rename_patch_button: Default::default(),
        }
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.master_volume.style = style;
        self.master_frequency.style = style;
        self.modulation_matrix.set_style(style);
        self.patch_picker.style = style;
    }

    pub fn view(&mut self) -> Element<'_, Message> {
        let mod_matrix = Container::new(
            Column::new()
                .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                .push(
                    Row::new()
                        .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                        .push(self.modulation_matrix.view())
                        // Allow room for modulation matrix extra pixel
                        .push(Space::with_width(Length::Units(LINE_HEIGHT - 1))),
                )
                .push(Space::with_height(Length::Units(LINE_HEIGHT))),
        )
        .height(Length::Units(LINE_HEIGHT * 8))
        .width(Length::Units(LINE_HEIGHT * 7))
        .style(self.style.container_l3());

        let master = container_l1(
            self.style,
            container_l2(
                self.style,
                Row::new()
                    .push(container_l3(self.style, self.master_volume.view()))
                    .push(space_l3())
                    .push(container_l3(self.style, self.master_frequency.view()))
                    .push(Space::with_width(Length::Units(LINE_HEIGHT * 3))), // Extend to end
            ),
        );

        let patch_picker = {
            let save_patch_button = Tooltip::new(
                Button::new(
                    &mut self.save_patch_button,
                    Text::new("SAVE")
                        .font(self.style.font_regular())
                        .height(Length::Units(LINE_HEIGHT)),
                )
                .on_press(Message::SavePatch)
                .padding(self.style.button_padding())
                .style(self.style.button()),
                "Save patch",
                Position::Top,
            )
            .style(self.style.tooltip());

            let save_bank_button = Tooltip::new(
                Button::new(
                    &mut self.save_bank_button,
                    Text::new("SAVE ALL")
                        .font(self.style.font_regular())
                        .height(Length::Units(LINE_HEIGHT)),
                )
                .on_press(Message::SaveBank)
                .padding(self.style.button_padding())
                .style(self.style.button()),
                "Save patch bank",
                Position::Top,
            )
            .style(self.style.tooltip());

            let load_button = Tooltip::new(
                Button::new(
                    &mut self.load_bank_or_patches_button,
                    Text::new("OPEN")
                        .font(self.style.font_regular())
                        .height(Length::Units(LINE_HEIGHT)),
                )
                .on_press(Message::LoadBankOrPatch)
                .padding(self.style.button_padding())
                .style(self.style.button()),
                "Open bank or patches",
                Position::Top,
            )
            .style(self.style.tooltip());

            let rename_button = Tooltip::new(
                Button::new(
                    &mut self.rename_patch_button,
                    Text::new("R")
                        .font(self.style.font_regular())
                        .height(Length::Units(LINE_HEIGHT)),
                )
                .on_press(Message::RenamePatch)
                .padding(self.style.button_padding())
                .style(self.style.button()),
                "Rename patch",
                Position::Top,
            )
            .style(self.style.tooltip());

            // Helps with issues arising from use of different font weights
            let button_space = match self.style {
                Theme::Dark => 3,
                Theme::Light => 2,
            };

            Container::new(
                Column::new()
                    .align_items(Alignment::Center)
                    .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                    .push(
                        Text::new("Patch")
                            .size(FONT_SIZE * 3 / 2)
                            .height(Length::Units(FONT_SIZE * 3 / 2))
                            .width(Length::Units(LINE_HEIGHT * 10))
                            .font(self.style.font_heading())
                            .color(self.style.heading_color())
                            .horizontal_alignment(Horizontal::Center),
                    )
                    .push(Space::with_height(Length::Units(LINE_HEIGHT / 4)))
                    .push(
                        Row::new()
                            .push(self.patch_picker.view())
                            .push(Space::with_width(Length::Units(button_space)))
                            .push(rename_button),
                    )
                    .push(Space::with_height(Length::Units(LINE_HEIGHT / 4)))
                    .push(
                        Row::new()
                            .push(load_button)
                            .push(Space::with_width(Length::Units(button_space)))
                            .push(save_patch_button)
                            .push(Space::with_width(Length::Units(button_space)))
                            .push(save_bank_button),
                    ),
            )
            .width(Length::Units(LINE_HEIGHT * 9))
            .height(Length::Units(LINE_HEIGHT * 6))
        };

        let logo = {
            let theme_button = Tooltip::new(
                Button::new(
                    &mut self.toggle_style_state,
                    Text::new("THEME")
                        .font(self.style.font_regular())
                        .height(Length::Units(LINE_HEIGHT)),
                )
                .on_press(Message::SwitchTheme)
                .padding(self.style.button_padding())
                .style(self.style.button()),
                "Switch color theme",
                Position::Top,
            )
            .style(self.style.tooltip());

            let info_button = Tooltip::new(
                Button::new(
                    &mut self.toggle_info_state,
                    Text::new("INFO")
                        .font(self.style.font_regular())
                        .height(Length::Units(LINE_HEIGHT)),
                )
                .on_press(Message::NoOp)
                .padding(self.style.button_padding())
                .style(self.style.button()),
                get_info_text(),
                Position::FollowCursor,
            )
            .style(self.style.tooltip());

            // Helps with issues arising from use of different font weights
            let logo_button_space = match self.style {
                Theme::Dark => 3,
                Theme::Light => 2,
            };

            Container::new(
                Column::new()
                    .align_items(Alignment::Center)
                    .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                    .push(
                        Text::new("OctaSine")
                            .size(FONT_SIZE * 3 / 2)
                            .height(Length::Units(FONT_SIZE * 3 / 2))
                            .width(Length::Units(LINE_HEIGHT * 8))
                            .font(self.style.font_heading())
                            .color(self.style.heading_color())
                            .horizontal_alignment(Horizontal::Center),
                    )
                    .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                    .push(
                        Row::new()
                            .push(theme_button)
                            .push(Space::with_width(Length::Units(logo_button_space)))
                            .push(info_button),
                    ),
            )
            .width(Length::Units(LINE_HEIGHT * 7))
            .height(Length::Units(LINE_HEIGHT * 6))
        };

        Column::new()
            .push(
                Row::new()
                    .push(mod_matrix)
                    .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                    .push(master),
            )
            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
            .push(
                Row::new()
                    .push(triple_container(self.style, patch_picker))
                    .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                    .push(triple_container(self.style, logo)),
            )
            .into()
    }
}

fn get_info_text() -> String {
    format!(
        "OctaSine frequency modulation synthesizer
Site: OctaSine.com
Build: {}
Copyright © 2019-2022 Joakim Frostegård",
        get_version_info()
    )
}
