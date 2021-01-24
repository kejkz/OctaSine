use iced_baseview::canvas::{
    Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke, Text, path, event
};
use iced_baseview::{
    Element, Color, Rectangle, Point, Length, Vector, Size
};

use crate::approximations::Log10Table;

use crate::GuiSyncHandle;
use crate::voices::envelopes::VoiceOperatorVolumeEnvelope;
use crate::constants::{ENVELOPE_MIN_DURATION, ENVELOPE_MAX_DURATION};

use super::{FONT_SIZE, LINE_HEIGHT, Message, SnapPoint};


const WIDTH: u16 = LINE_HEIGHT * 16;
const HEIGHT: u16 = LINE_HEIGHT * 5;
const SIZE: Size = Size { width: WIDTH as f32, height: HEIGHT as f32 };

const SUSTAIN_DURATION: f32 = 0.1 / 4.0;
const DRAGGER_RADIUS: f32 = 5.0;

const ENVELOPE_PATH_SCALE_X: f32 = 1.0 - (1.0 / 16.0);
const ENVELOPE_PATH_SCALE_Y: f32 = 1.0 - (1.0 / 8.0) - (1.0 / 16.0);

const TOTAL_DURATION: f32 = 3.0 + SUSTAIN_DURATION;


struct EnvelopeStagePath {
    path: Path,
    end_point: Point,
}


impl EnvelopeStagePath {
    fn new(
        log10_table: &Log10Table,
        size: Size,
        total_duration: f32,
        x_offset: f32,
        start_duration: f32,
        start_value: f32,
        stage_duration: f32,
        stage_end_value: f32,
    ) -> Self {
        let mut path = path::Builder::new();

        let start = Self::calculate_stage_progress_point(
            log10_table,
            size,
            total_duration,
            x_offset,
            start_duration,
            start_value,
            stage_duration,
            stage_end_value,
            0.0
        );
        let control_a = Self::calculate_stage_progress_point(
            log10_table,
            size,
            total_duration,
            x_offset,
            start_duration,
            start_value,
            stage_duration,
            stage_end_value,
            1.0 / 3.0
        );
        let control_b = Self::calculate_stage_progress_point(
            log10_table,
            size,
            total_duration,
            x_offset,
            start_duration,
            start_value,
            stage_duration,
            stage_end_value,
            2.0 / 3.0
        );
        let to = Self::calculate_stage_progress_point(
            log10_table,
            size,
            total_duration,
            x_offset,
            start_duration,
            start_value,
            stage_duration,
            stage_end_value,
            1.0
        );

        path.move_to(start);
        path.bezier_curve_to(control_a, control_b, to);

        Self {
            path: path.build(),
            end_point: to,
        }
    }

    fn calculate_stage_progress_point(
        log10_table: &Log10Table,
        size: Size,
        total_duration: f32,
        x_offset: f32,
        start_duration: f32,
        start_value: f32,
        stage_duration: f32,
        stage_end_value: f32,
        progress: f32,
    ) -> Point {
        let duration = stage_duration * progress;

        let value = VoiceOperatorVolumeEnvelope::calculate_curve(
            log10_table,
            start_value as f64,
            stage_end_value as f64,
            duration as f64,
            stage_duration as f64,
        ) as f32;

        // Watch out for point.y.is_nan() when duration = 0.0 here
        let point = Point::new(
            (x_offset + (start_duration + duration) / total_duration) * size.width,
            size.height * (1.0 - value)
        );

        scale_point(size, point).snap()
    }
}


impl Default for EnvelopeStagePath {
    fn default() -> Self {
        Self {
            path: Path::line(Point::default(), Point::default()),
            end_point: Point::default(),
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
enum EnvelopeDraggerStatus {
    Normal,
    Hover,
    Dragging {
        from: Point,
        original_duration: f32,
        original_end_value: f32,
    },
}


struct EnvelopeDragger {
    center: Point,
    radius: f32,
    pub hitbox: Rectangle,
    pub status: EnvelopeDraggerStatus,
}


impl EnvelopeDragger {
    fn set_center(&mut self, center: Point){
        self.center = center;

        self.hitbox.width = self.radius * 2.0;
        self.hitbox.height = self.radius * 2.0;
        self.hitbox.x = (center.x - self.radius / 2.0).max(0.0);
        self.hitbox.y = (center.y - self.radius / 2.0).max(0.0);
    }

    fn is_dragging(&self) -> bool {
        matches!(self.status, EnvelopeDraggerStatus::Dragging {..})
    }
}


impl Default for EnvelopeDragger {
    fn default() -> Self {
        Self {
            center: Point::default(),
            radius: DRAGGER_RADIUS,
            hitbox: Rectangle::default(),
            status: EnvelopeDraggerStatus::Normal,
        }
    }
}


pub struct Envelope {
    log10_table: Log10Table,
    cache: Cache,
    operator_index: usize,
    attack_duration: f32,
    attack_end_value: f32,
    decay_duration: f32,
    decay_end_value: f32,
    release_duration: f32,
    size: Size,
    viewport_factor: f32,
    x_offset: f32,
    attack_stage_path: EnvelopeStagePath,
    decay_stage_path: EnvelopeStagePath,
    sustain_stage_path: EnvelopeStagePath,
    release_stage_path: EnvelopeStagePath,
    attack_dragger: EnvelopeDragger,
    decay_dragger: EnvelopeDragger,
    release_dragger: EnvelopeDragger,
    last_cursor_position: Point,
    dragging_background_from: Option<(Point, f32)>,
}


impl Envelope {
    pub fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        operator_index: usize,
    ) -> Self {
        let (attack_dur, attack_val, decay_dur, decay_val, release_dur) = match operator_index {
            0 => (10, 11, 12, 13, 14),
            1 => (24, 25, 26, 27, 28),
            2 => (39, 40, 41, 42, 43),
            3 => (54, 55, 56, 57, 58),
            _ => unreachable!(),
        };

        let attack_duration = Self::process_envelope_duration(
            sync_handle.get_parameter(attack_dur)
        );
        let decay_duration = Self::process_envelope_duration(
            sync_handle.get_parameter(decay_dur)
        );
        let release_duration = Self::process_envelope_duration(
            sync_handle.get_parameter(release_dur)
        );

        let mut envelope = Self {
            log10_table: Log10Table::default(),
            cache: Cache::default(),
            operator_index,
            attack_duration,
            attack_end_value: sync_handle.get_parameter(attack_val) as f32,
            decay_duration,
            decay_end_value: sync_handle.get_parameter(decay_val) as f32,
            release_duration,
            size: SIZE,
            viewport_factor: 1.0,
            x_offset: 0.0,
            attack_stage_path: EnvelopeStagePath::default(),
            decay_stage_path: EnvelopeStagePath::default(),
            sustain_stage_path: EnvelopeStagePath::default(),
            release_stage_path: EnvelopeStagePath::default(),
            attack_dragger: EnvelopeDragger::default(),
            decay_dragger: EnvelopeDragger::default(),
            release_dragger: EnvelopeDragger::default(),
            last_cursor_position: Point::new(-1.0, -1.0),
            dragging_background_from: None,
        };

        envelope.update_data();

        envelope
    }

    fn process_envelope_duration(sync_value: f64) -> f32 {
        sync_value.max(ENVELOPE_MIN_DURATION / ENVELOPE_MAX_DURATION) as f32
    }

    pub fn zoom_in(&mut self){
        const MIN: f32 = 1.0 / 128.0;

        if self.viewport_factor > MIN {
            self.viewport_factor = (self.viewport_factor * 0.5).max(MIN);

            // FIXME: update offset
        }

        self.update_data();
    }

    pub fn zoom_out(&mut self){
        if self.viewport_factor < 1.0 {
            self.viewport_factor = (self.viewport_factor * 2.0).min(1.0);

            // FIXME: update offset
        }

        self.update_data();
    }

    pub fn set_attack_duration(&mut self, value: f64){
        if !self.attack_dragger.is_dragging(){
            self.attack_duration = Self::process_envelope_duration(value);

            self.update_data();
        }
    }

    pub fn set_attack_end_value(&mut self, value: f64){
        self.attack_end_value = value as f32;

        self.update_data();
    }

    pub fn set_decay_duration(&mut self, value: f64){
        if !self.decay_dragger.is_dragging(){
            self.decay_duration = Self::process_envelope_duration(value);

            self.update_data();
        }
    }

    pub fn set_decay_end_value(&mut self, value: f64){
        self.decay_end_value = value as f32;

        self.update_data();
    }

    pub fn set_release_duration(&mut self, value: f64){
        if !self.release_dragger.is_dragging(){
            self.release_duration = Self::process_envelope_duration(value);

            self.update_data();
        }
    }

    fn update_data(&mut self){
        self.update_stage_paths();

        self.attack_dragger.set_center(self.attack_stage_path.end_point);
        self.decay_dragger.set_center(self.decay_stage_path.end_point);
        self.release_dragger.set_center(self.release_stage_path.end_point);

        self.cache.clear();
    }

    fn update_stage_paths(&mut self){
        let sustain_duration = SUSTAIN_DURATION;
        let total_duration = self.viewport_factor * TOTAL_DURATION;

        self.attack_stage_path = EnvelopeStagePath::new(
            &self.log10_table,
            self.size,
            total_duration,
            self.x_offset,
            0.0,
            0.0,
            self.attack_duration as f32,
            self.attack_end_value as f32,
        );

        self.decay_stage_path = EnvelopeStagePath::new(
            &self.log10_table,
            self.size,
            total_duration,
            self.x_offset,
            self.attack_duration,
            self.attack_end_value,
            self.decay_duration as f32,
            self.decay_end_value as f32,
        );

        self.sustain_stage_path = EnvelopeStagePath::new(
            &self.log10_table,
            self.size,
            total_duration,
            self.x_offset,
            self.attack_duration + self.decay_duration,
            self.decay_end_value,
            sustain_duration as f32,
            self.decay_end_value,
        );

        self.release_stage_path = EnvelopeStagePath::new(
            &self.log10_table,
            self.size,
            total_duration,
            self.x_offset,
            self.attack_duration + self.decay_duration + sustain_duration,
            self.decay_end_value,
            self.release_duration as f32,
            0.0
        );
    }

    pub fn view(&mut self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Units(WIDTH))
            .height(Length::Units(HEIGHT))
            .into()
    }

    fn draw_time_markers(&self, frame: &mut Frame){
        let total_duration = self.viewport_factor * TOTAL_DURATION;

        let mut time_marker_interval = 0.01 / 4.0;

        loop {
            let num_markers = (total_duration / time_marker_interval) as usize;

            if num_markers <= 110 {
                break;
            } else {
                time_marker_interval *= 10.0;
            }
        };

        let iterations = (TOTAL_DURATION / time_marker_interval) as usize + 1;

        for i in 0..iterations {
            let x = (self.x_offset + (time_marker_interval * i as f32) / total_duration) * self.size.width;

            if x < 0.0 || x > self.size.width {
                continue;
            }

            let top_point = Point::new(x, 0.0);
            let bottom_point = Point::new(x, self.size.height);

            let path = Path::line(
                scale_point_x(self.size, top_point).snap(),
                scale_point_x(self.size, bottom_point).snap(),
            );

            if i % 10 == 0 && i != 0 {
                let text_point = Point::new(x - 10.0, self.size.height);

                let text = Text {
                    content: format!("{:.1}s", time_marker_interval * 4.0 * i as f32),
                    position: scale_point_x(self.size, text_point),
                    size: FONT_SIZE as f32,
                    ..Default::default()
                };
        
                frame.fill_text(text);

                let stroke = Stroke::default()
                    .with_width(1.0)
                    .with_color(Color::from_rgb(0.7, 0.7, 0.7));

                frame.stroke(&path, stroke);
            } else {
                let stroke = Stroke::default()
                    .with_width(1.0)
                    .with_color(Color::from_rgb(0.9, 0.9, 0.9));

                frame.stroke(&path, stroke);
            }
        }
    }

    fn draw_stage_paths(&self, frame: &mut Frame){
        let stroke = Stroke::default()
            .with_width(1.0)
            .with_color(Color::BLACK);
        let sustain_stroke = Stroke::default()
            .with_width(1.0)
            .with_color(Color::from_rgb(0.5, 0.5, 0.5));

        frame.stroke(&self.attack_stage_path.path, stroke);
        frame.stroke(&self.decay_stage_path.path, stroke);
        frame.stroke(&self.sustain_stage_path.path, sustain_stroke);
        frame.stroke(&self.release_stage_path.path, stroke);
    }

    fn draw_dragger(frame: &mut Frame, dragger: &EnvelopeDragger){
        let circle_path = {
            let mut builder = path::Builder::new();

            builder.move_to(dragger.center);
            builder.circle(dragger.center, dragger.radius);

            builder.build()
        };

        let fill_color = match dragger.status {
            EnvelopeDraggerStatus::Normal => Color::from_rgb(1.0, 1.0, 1.0),
            EnvelopeDraggerStatus::Hover => Color::from_rgb(0.0, 0.0, 0.0),
            EnvelopeDraggerStatus::Dragging {..} => Color::from_rgb(0.0, 0.0, 0.0),
        };

        frame.fill(&circle_path, fill_color);

        let stroke = Stroke::default()
            .with_width(1.0)
            .with_color(Color::from_rgb(0.5, 0.5, 0.5));

        frame.stroke(&circle_path, stroke);
    }
}


impl Program<Message> for Envelope {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{
        let geometry = self.cache.draw(bounds.size(), |frame| {
            self.draw_time_markers(frame);
            self.draw_stage_paths(frame);

            Self::draw_dragger(frame, &self.attack_dragger);
            Self::draw_dragger(frame, &self.decay_dragger);
            Self::draw_dragger(frame, &self.release_dragger);
        });

        vec![geometry]
    }

    fn update(
        &mut self,
        event: event::Event,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        match event {
            event::Event::Mouse(iced_baseview::mouse::Event::CursorMoved {x, y}) => {
                self.last_cursor_position = Point::new(x, y);

                if bounds.contains(Point::new(x, y)){
                    let relative_position = Point::new(
                        x - bounds.x,
                        y - bounds.y,
                    );

                    match self.attack_dragger.status {
                        EnvelopeDraggerStatus::Normal => {
                            if self.attack_dragger.hitbox.contains(relative_position){
                                self.attack_dragger.status = EnvelopeDraggerStatus::Hover;

                                self.cache.clear();
                            }
                        },
                        EnvelopeDraggerStatus::Hover => {
                            if !self.attack_dragger.hitbox.contains(relative_position){
                                self.attack_dragger.status = EnvelopeDraggerStatus::Normal;

                                self.cache.clear();
                            }
                        },
                        EnvelopeDraggerStatus::Dragging { from, original_duration, original_end_value} => {
                            self.attack_duration = dragging_to_duration(
                                self.viewport_factor,
                                x,
                                from,
                                original_duration
                            );
                            self.attack_end_value = dragging_to_end_value(y, from, original_end_value);

                            self.update_data();

                            let (dur, val) = match self.operator_index {
                                0 => (10, 11),
                                1 => (24, 25),
                                2 => (39, 40),
                                3 => (54, 55),
                                _ => unreachable!()
                            };

                            let changes = vec![
                                (dur, self.attack_duration as f64),
                                (val, self.attack_end_value as f64),
                            ];

                            return (event::Status::Captured, Some(Message::ParameterChanges(changes)));
                        },
                    }

                    match self.decay_dragger.status {
                        EnvelopeDraggerStatus::Normal => {
                            if self.decay_dragger.hitbox.contains(relative_position){
                                self.decay_dragger.status = EnvelopeDraggerStatus::Hover;

                                self.cache.clear();
                            }
                        },
                        EnvelopeDraggerStatus::Hover => {
                            if !self.decay_dragger.hitbox.contains(relative_position){
                                self.decay_dragger.status = EnvelopeDraggerStatus::Normal;

                                self.cache.clear();
                            }
                        },
                        EnvelopeDraggerStatus::Dragging { from, original_duration, original_end_value} => {
                            self.decay_duration = dragging_to_duration(
                                self.viewport_factor,
                                x,
                                from,
                                original_duration
                            );
                            self.decay_end_value = dragging_to_end_value(y, from, original_end_value);

                            self.update_data();

                            let (dur, val) = match self.operator_index {
                                0 => (12, 13),
                                1 => (26, 27),
                                2 => (41, 42),
                                3 => (56, 57),
                                _ => unreachable!()
                            };

                            let changes = vec![
                                (dur, self.decay_duration as f64),
                                (val, self.decay_end_value as f64),
                            ];

                            return (event::Status::Captured, Some(Message::ParameterChanges(changes)));
                        },
                    }

                    match self.release_dragger.status {
                        EnvelopeDraggerStatus::Normal => {
                            if self.release_dragger.hitbox.contains(relative_position){
                                self.release_dragger.status = EnvelopeDraggerStatus::Hover;

                                self.cache.clear();
                            }
                        },
                        EnvelopeDraggerStatus::Hover => {
                            if !self.release_dragger.hitbox.contains(relative_position){
                                self.release_dragger.status = EnvelopeDraggerStatus::Normal;

                                self.cache.clear();
                            }
                        },
                        EnvelopeDraggerStatus::Dragging { from, original_duration, .. } => {
                            self.release_duration = dragging_to_duration(
                                self.viewport_factor,
                                x,
                                from,
                                original_duration
                            );

                            self.update_data();

                            let parameter_index = match self.operator_index {
                                0 => 14,
                                1 => 28,
                                2 => 43,
                                3 => 58,
                                _ => unreachable!()
                            };

                            return (event::Status::Captured, Some(Message::ParameterChange(parameter_index, self.release_duration as f64)));
                        },
                    }

                    if let Some((from, original_offset)) = self.dragging_background_from {
                        self.x_offset = original_offset + (x - from.x) / WIDTH as f32;

                        self.update_data();
                    }
                    
                    return (event::Status::Captured, None);
                }
            },
            event::Event::Mouse(iced_baseview::mouse::Event::ButtonPressed(iced_baseview::mouse::Button::Left)) => {
                if bounds.contains(self.last_cursor_position){
                    let relative_position = Point::new(
                        self.last_cursor_position.x - bounds.x,
                        self.last_cursor_position.y - bounds.y,
                    );

                    if self.release_dragger.hitbox.contains(relative_position) && !self.release_dragger.is_dragging(){
                        self.release_dragger.status = EnvelopeDraggerStatus::Dragging {
                            from: self.last_cursor_position,
                            original_duration: self.release_duration,
                            original_end_value: 0.0
                        };
                    } else if self.decay_dragger.hitbox.contains(relative_position) && !self.decay_dragger.is_dragging() {
                        self.decay_dragger.status = EnvelopeDraggerStatus::Dragging {
                            from: self.last_cursor_position,
                            original_duration: self.decay_duration,
                            original_end_value: self.decay_end_value
                        };
                    } else if self.attack_dragger.hitbox.contains(relative_position) && !self.attack_dragger.is_dragging() {
                        self.attack_dragger.status = EnvelopeDraggerStatus::Dragging {
                            from: self.last_cursor_position,
                            original_duration: self.attack_duration,
                            original_end_value: self.attack_end_value
                        };
                    } else {
                        self.dragging_background_from = Some((self.last_cursor_position, self.x_offset));
                    }

                    return (event::Status::Captured, None);
                }
            },
            event::Event::Mouse(iced_baseview::mouse::Event::ButtonReleased(iced_baseview::mouse::Button::Left)) => {
                if self.release_dragger.is_dragging() {
                    self.release_dragger.status = EnvelopeDraggerStatus::Normal;

                    return (event::Status::Captured, None);
                }
                if self.decay_dragger.is_dragging() {
                    self.decay_dragger.status = EnvelopeDraggerStatus::Normal;

                    return (event::Status::Captured, None);
                }
                if self.attack_dragger.is_dragging() {
                    self.attack_dragger.status = EnvelopeDraggerStatus::Normal;

                    return (event::Status::Captured, None);
                }

                if self.dragging_background_from.is_some(){
                    self.dragging_background_from = None;
                }
            },
            _ => (),
        };

        (event::Status::Ignored, None)
    }
}


fn scale_point(size: Size, point: Point) -> Point {
    let translation = Vector {
        x: (1.0 - ENVELOPE_PATH_SCALE_X) * size.width / 2.0,
        y: (1.0 - ENVELOPE_PATH_SCALE_Y) * size.height / 2.0
    };

    let scaled = Point {
        x: point.x * ENVELOPE_PATH_SCALE_X,
        y: point.y * ENVELOPE_PATH_SCALE_Y,
    };

    scaled + translation
}


fn scale_point_x(size: Size, point: Point) -> Point {
    let translation = Vector {
        x: (1.0 - ENVELOPE_PATH_SCALE_X) * size.width / 2.0,
        y: 0.0,
    };

    let scaled = Point {
        x: point.x * ENVELOPE_PATH_SCALE_X,
        y: point.y,
    };

    scaled + translation
}


// Almost-correct reverse transformation for envelope dragger to duration
fn dragging_to_duration(
    viewport_factor: f32,
    cursor_x: f32,
    from: Point,
    original_value: f32
) -> f32 {
    let change = (cursor_x - from.x) / WIDTH as f32;
    let change = change / ENVELOPE_PATH_SCALE_X;
    let change = change * viewport_factor * TOTAL_DURATION;

    (original_value + change)
        .min(1.0)
        .max(ENVELOPE_MIN_DURATION as f32)
}


fn dragging_to_end_value(
    cursor_y: f32,
    from: Point,
    original_value: f32
) -> f32 {
    let change = -(cursor_y - from.y) / HEIGHT as f32;
    let change = change / ENVELOPE_PATH_SCALE_Y;

    (original_value + change)
        .min(1.0)
        .max(0.0)
}
