use crate::{
    constants::{LFO_SHAPE_STEPS, LFO_TARGET_CONTEXT_STEPS},
    voices::lfos::VoiceLfo,
};

pub trait CalculateCurve: PartialEq + Copy {
    fn calculate(self, phase: Phase) -> f64;
    fn steps() -> &'static [Self];
}

/// Phase. value >= 0.0 && value < 1.0
#[derive(Debug, Copy, Clone)]
pub struct Phase(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct SampleRate(pub f64);

impl Default for SampleRate {
    fn default() -> Self {
        Self(44100.0)
    }
}

impl Into<TimePerSample> for SampleRate {
    fn into(self) -> TimePerSample {
        TimePerSample(1.0 / self.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TimePerSample(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BeatsPerMinute(pub f64);

impl Default for BeatsPerMinute {
    fn default() -> Self {
        Self(120.0)
    }
}

pub trait ModTarget: Copy {
    fn set_index(&mut self, index: usize, value: bool);
    fn index_active(&self, index: usize) -> bool;
    fn as_string(&self) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModTargetStorage<const N: usize>([bool; N]);

impl<const N: usize> ModTargetStorage<N> {
    pub fn active_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.0
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(index, active)| if active { Some(index) } else { None })
    }
}

impl ModTargetStorage<1> {
    pub fn permutations() -> &'static [Self] {
        &[ModTargetStorage([true]), ModTargetStorage([false])]
    }
}

impl ModTargetStorage<2> {
    pub fn permutations() -> &'static [Self] {
        &[
            ModTargetStorage([false, false]),
            ModTargetStorage([true, false]),
            ModTargetStorage([false, true]),
            ModTargetStorage([true, true]),
        ]
    }
}

impl ModTargetStorage<3> {
    pub fn permutations() -> &'static [Self] {
        &[
            ModTargetStorage([true, false, false]),
            ModTargetStorage([true, true, false]),
            ModTargetStorage([true, false, true]),
            ModTargetStorage([true, true, true]),
            ModTargetStorage([false, true, false]),
            ModTargetStorage([false, false, true]),
            ModTargetStorage([false, true, true]),
            ModTargetStorage([false, false, true]),
            ModTargetStorage([false, false, false]),
        ]
    }
}

impl Default for ModTargetStorage<1> {
    fn default() -> Self {
        Self([true])
    }
}

impl Default for ModTargetStorage<2> {
    fn default() -> Self {
        Self([false, true])
    }
}

impl Default for ModTargetStorage<3> {
    fn default() -> Self {
        Self([false, false, true])
    }
}

impl<const N: usize> ModTarget for ModTargetStorage<N> {
    fn set_index(&mut self, index: usize, value: bool) {
        self.0[index] = value;
    }

    fn index_active(&self, index: usize) -> bool {
        self.0[index]
    }

    fn as_string(&self) -> String {
        let mut output = String::new();

        for (index, active) in self.0.into_iter().enumerate() {
            if active {
                let operator_number = index + 1;

                if output.is_empty() {
                    output.push_str(&format!("{}", operator_number))
                } else {
                    output.push_str(&format!(", {}", operator_number))
                }
            }
        }

        output
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnvelopeStage {
    Attack,
    Decay,
    Sustain,
    Release,
    Ended,
    Restart,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WaveType {
    Sine,
    WhiteNoise,
}

impl Default for WaveType {
    fn default() -> Self {
        Self::Sine
    }
}
impl CalculateCurve for WaveType {
    fn calculate(self, phase: Phase) -> f64 {
        match self {
            Self::Sine => crate::voices::lfos::sine(phase),
            Self::WhiteNoise => {
                // Ensure same numbers are generated each time for GUI
                // consistency. This will however break if fastrand changes
                // its algorithm.
                let seed = phase.0.to_bits() + 2;

                (fastrand::Rng::with_seed(seed).f64() - 0.5) * 2.0
            }
        }
    }
    fn steps() -> &'static [Self] {
        &[Self::Sine, Self::WhiteNoise]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LfoTargetMasterParameter {
    Volume,
    Frequency,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LfoTargetOperatorParameter {
    Volume,
    Panning,
    MixOut,
    ModOut,
    Feedback,
    FrequencyRatio,
    FrequencyFree,
    FrequencyFine,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LfoTargetLfoParameter {
    Shape,
    FrequencyRatio,
    FrequencyFree,
    Amount,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LfoTargetParameter {
    Master(LfoTargetMasterParameter),
    Operator(usize, LfoTargetOperatorParameter),
    Lfo(usize, LfoTargetLfoParameter),
}

impl std::fmt::Display for LfoTargetParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LfoTargetParameter::Master(p) => {
                write!(f, "Master {}", format!("{:?}", p).to_lowercase())
            }
            LfoTargetParameter::Operator(n, p) => {
                use LfoTargetOperatorParameter::*;

                let p = match p {
                    Volume => "volume",
                    Panning => "pan",
                    MixOut => "mix out",
                    ModOut => "mod out",
                    Feedback => "feedback",
                    FrequencyRatio => "freq ratio",
                    FrequencyFree => "freq free",
                    FrequencyFine => "freq fine",
                };

                write!(f, "Op. {} {}", n + 1, p)
            }
            LfoTargetParameter::Lfo(n, p) => {
                use LfoTargetLfoParameter::*;

                let p = match p {
                    Shape => "shape",
                    FrequencyRatio => "freq ratio",
                    FrequencyFree => "freq free",
                    Amount => "amount",
                };

                write!(f, "LFO {} {}", n + 1, p)
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LfoShape {
    Saw,
    ReverseSaw,
    Triangle,
    ReverseTriangle,
    Square,
    ReverseSquare,
    Sine,
    ReverseSine,
}

impl CalculateCurve for LfoShape {
    fn calculate(self, phase: Phase) -> f64 {
        VoiceLfo::calculate_curve(self, phase)
    }
    fn steps() -> &'static [Self] {
        &LFO_SHAPE_STEPS
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LfoMode {
    Once,
    Forever,
}

pub fn get_lfo_target_parameters(lfo_index: usize) -> &'static [LfoTargetParameter] {
    let end = match lfo_index {
        0 => 33,
        1 => 37,
        2 => 41,
        3 => 45,
        _ => unreachable!(),
    };

    &LFO_TARGET_CONTEXT_STEPS[..end]
}
