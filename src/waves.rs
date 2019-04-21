use crate::constants::*;
use crate::utils::*;


#[derive(Debug, Copy, Clone)]
pub struct WaveDuration(pub f64);


#[derive(Debug, Copy, Clone)]
pub struct WaveVolume(pub f64);

impl WaveVolume {
    pub fn from_host_value(&self, value: f64) -> f64 {
        value * 2.0
    }
    pub fn get_default_host_value(&self) -> f64 {
        WAVE_DEFAULT_VOLUME / 2.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WaveRatio(pub f64);

impl WaveRatio {
    pub fn from_host_value(&self, value: f64) -> f64 {
        map_host_param_value_to_step(&WAVE_RATIO_STEPS[..], value)
    }
    pub fn get_default_host_value(&self) -> f64 {
        get_host_value_for_default_step(&WAVE_RATIO_STEPS[..], 1.0)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WaveFrequencyFree(pub f64);

impl WaveFrequencyFree {
    pub fn from_host_value(&self, value: f64) -> f64 {
        (value + 0.5).powf(3.0)
    }
    pub fn get_default_host_value(&self) -> f64 {
        0.5
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WaveFeedback(pub f64);

impl WaveFeedback {
    pub fn from_host_value(&self, value: f64) -> f64 {
        value * 5.0
    }
    pub fn get_default_host_value(&self) -> f64 {
        WAVE_DEFAULT_FEEDBACK / 5.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WaveBeta(pub f64);

impl WaveBeta {
    pub fn from_host_value(&self, value: f64) -> f64 {
        map_host_param_value_to_step_smooth(&WAVE_BETA_STEPS[..], value)
    }
    pub fn get_default_host_value(&self) -> f64 {
        get_host_value_for_default_step(&WAVE_BETA_STEPS[..], WAVE_DEFAULT_BETA)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Wave {
    pub duration: WaveDuration,
    pub volume: WaveVolume,
    pub ratio: WaveRatio,
    pub frequency_free: WaveFrequencyFree,
    pub feedback: WaveFeedback,
    pub beta: WaveBeta,
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            duration: WaveDuration(0.0),
            volume: WaveVolume(WAVE_DEFAULT_VOLUME),
            ratio: WaveRatio(WAVE_DEFAULT_RATIO),
            frequency_free: WaveFrequencyFree(WAVE_DEFAULT_FREQUENCY_FREE),
            feedback: WaveFeedback(WAVE_DEFAULT_FEEDBACK),
            beta: WaveBeta(WAVE_DEFAULT_BETA),
        }
    }
}