/// Authoritative list of parameters in order
pub const PARAMETERS: &[Parameter] = &[
    Parameter::Master(MasterParameter::Volume),
    Parameter::Master(MasterParameter::Frequency),
    Parameter::Operator(0, OperatorParameter::Volume),
    Parameter::Operator(0, OperatorParameter::Active),
    Parameter::Operator(0, OperatorParameter::MixOut),
    Parameter::Operator(0, OperatorParameter::Panning),
    Parameter::Operator(0, OperatorParameter::WaveType),
    Parameter::Operator(0, OperatorParameter::Feedback),
    Parameter::Operator(0, OperatorParameter::FrequencyRatio),
    Parameter::Operator(0, OperatorParameter::FrequencyFree),
    Parameter::Operator(0, OperatorParameter::FrequencyFine),
    Parameter::Operator(0, OperatorParameter::AttackDuration),
    Parameter::Operator(0, OperatorParameter::DecayDuration),
    Parameter::Operator(0, OperatorParameter::SustainVolume),
    Parameter::Operator(0, OperatorParameter::ReleaseDuration),
    Parameter::Operator(0, OperatorParameter::EnvelopeLockGroup),
    Parameter::Operator(1, OperatorParameter::Volume),
    Parameter::Operator(1, OperatorParameter::Active),
    Parameter::Operator(1, OperatorParameter::MixOut),
    Parameter::Operator(1, OperatorParameter::Panning),
    Parameter::Operator(1, OperatorParameter::WaveType),
    Parameter::Operator(1, OperatorParameter::ModTargets),
    Parameter::Operator(1, OperatorParameter::ModOut),
    Parameter::Operator(1, OperatorParameter::Feedback),
    Parameter::Operator(1, OperatorParameter::FrequencyRatio),
    Parameter::Operator(1, OperatorParameter::FrequencyFree),
    Parameter::Operator(1, OperatorParameter::FrequencyFine),
    Parameter::Operator(1, OperatorParameter::AttackDuration),
    Parameter::Operator(1, OperatorParameter::DecayDuration),
    Parameter::Operator(1, OperatorParameter::SustainVolume),
    Parameter::Operator(1, OperatorParameter::ReleaseDuration),
    Parameter::Operator(1, OperatorParameter::EnvelopeLockGroup),
    Parameter::Operator(2, OperatorParameter::Volume),
    Parameter::Operator(2, OperatorParameter::Active),
    Parameter::Operator(2, OperatorParameter::MixOut),
    Parameter::Operator(2, OperatorParameter::Panning),
    Parameter::Operator(2, OperatorParameter::WaveType),
    Parameter::Operator(2, OperatorParameter::ModTargets),
    Parameter::Operator(2, OperatorParameter::ModOut),
    Parameter::Operator(2, OperatorParameter::Feedback),
    Parameter::Operator(2, OperatorParameter::FrequencyRatio),
    Parameter::Operator(2, OperatorParameter::FrequencyFree),
    Parameter::Operator(2, OperatorParameter::FrequencyFine),
    Parameter::Operator(2, OperatorParameter::AttackDuration),
    Parameter::Operator(2, OperatorParameter::DecayDuration),
    Parameter::Operator(2, OperatorParameter::SustainVolume),
    Parameter::Operator(2, OperatorParameter::ReleaseDuration),
    Parameter::Operator(2, OperatorParameter::EnvelopeLockGroup),
    Parameter::Operator(3, OperatorParameter::Volume),
    Parameter::Operator(3, OperatorParameter::Active),
    Parameter::Operator(3, OperatorParameter::MixOut),
    Parameter::Operator(3, OperatorParameter::Panning),
    Parameter::Operator(3, OperatorParameter::WaveType),
    Parameter::Operator(3, OperatorParameter::ModTargets),
    Parameter::Operator(3, OperatorParameter::ModOut),
    Parameter::Operator(3, OperatorParameter::Feedback),
    Parameter::Operator(3, OperatorParameter::FrequencyRatio),
    Parameter::Operator(3, OperatorParameter::FrequencyFree),
    Parameter::Operator(3, OperatorParameter::FrequencyFine),
    Parameter::Operator(3, OperatorParameter::AttackDuration),
    Parameter::Operator(3, OperatorParameter::DecayDuration),
    Parameter::Operator(3, OperatorParameter::SustainVolume),
    Parameter::Operator(3, OperatorParameter::ReleaseDuration),
    Parameter::Operator(3, OperatorParameter::EnvelopeLockGroup),
    Parameter::Lfo(0, LfoParameter::Target),
    Parameter::Lfo(0, LfoParameter::BpmSync),
    Parameter::Lfo(0, LfoParameter::FrequencyRatio),
    Parameter::Lfo(0, LfoParameter::FrequencyFree),
    Parameter::Lfo(0, LfoParameter::Mode),
    Parameter::Lfo(0, LfoParameter::Shape),
    Parameter::Lfo(0, LfoParameter::Amount),
    Parameter::Lfo(0, LfoParameter::Active),
    Parameter::Lfo(1, LfoParameter::Target),
    Parameter::Lfo(1, LfoParameter::BpmSync),
    Parameter::Lfo(1, LfoParameter::FrequencyRatio),
    Parameter::Lfo(1, LfoParameter::FrequencyFree),
    Parameter::Lfo(1, LfoParameter::Mode),
    Parameter::Lfo(1, LfoParameter::Shape),
    Parameter::Lfo(1, LfoParameter::Amount),
    Parameter::Lfo(1, LfoParameter::Active),
    Parameter::Lfo(2, LfoParameter::Target),
    Parameter::Lfo(2, LfoParameter::BpmSync),
    Parameter::Lfo(2, LfoParameter::FrequencyRatio),
    Parameter::Lfo(2, LfoParameter::FrequencyFree),
    Parameter::Lfo(2, LfoParameter::Mode),
    Parameter::Lfo(2, LfoParameter::Shape),
    Parameter::Lfo(2, LfoParameter::Amount),
    Parameter::Lfo(2, LfoParameter::Active),
    Parameter::Lfo(3, LfoParameter::Target),
    Parameter::Lfo(3, LfoParameter::BpmSync),
    Parameter::Lfo(3, LfoParameter::FrequencyRatio),
    Parameter::Lfo(3, LfoParameter::FrequencyFree),
    Parameter::Lfo(3, LfoParameter::Mode),
    Parameter::Lfo(3, LfoParameter::Shape),
    Parameter::Lfo(3, LfoParameter::Amount),
    Parameter::Lfo(3, LfoParameter::Active),
    Parameter::Lfo(0, LfoParameter::KeySync),
    Parameter::Lfo(1, LfoParameter::KeySync),
    Parameter::Lfo(2, LfoParameter::KeySync),
    Parameter::Lfo(3, LfoParameter::KeySync),
    Parameter::Master(MasterParameter::PitchBendRangeUp),
    Parameter::Master(MasterParameter::PitchBendRangeDown),
    Parameter::Master(MasterParameter::VelocitySensitivityVolume),
    Parameter::Operator(0, OperatorParameter::VelocitySensitivityModOut),
    Parameter::Operator(0, OperatorParameter::VelocitySensitivityFeedback),
    Parameter::Operator(0, OperatorParameter::AftertouchSensitivityVolume),
    Parameter::Operator(1, OperatorParameter::VelocitySensitivityModOut),
    Parameter::Operator(1, OperatorParameter::VelocitySensitivityFeedback),
    Parameter::Operator(1, OperatorParameter::AftertouchSensitivityVolume),
    Parameter::Operator(2, OperatorParameter::VelocitySensitivityModOut),
    Parameter::Operator(2, OperatorParameter::VelocitySensitivityFeedback),
    Parameter::Operator(2, OperatorParameter::AftertouchSensitivityVolume),
    Parameter::Operator(3, OperatorParameter::VelocitySensitivityModOut),
    Parameter::Operator(3, OperatorParameter::VelocitySensitivityFeedback),
    Parameter::Operator(3, OperatorParameter::AftertouchSensitivityVolume),
    Parameter::Master(MasterParameter::VoiceMode),
    Parameter::Master(MasterParameter::GlideActive),
    Parameter::Master(MasterParameter::GlideTime),
    Parameter::Master(MasterParameter::GlideBpmSync),
    Parameter::Master(MasterParameter::GlideMode),
    Parameter::Master(MasterParameter::GlideRetrigger),
];

/// Parameter enum used to abstract over parameter indices
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Parameter {
    /// Only used in LFO targetting
    None,
    Master(MasterParameter),
    Operator(u8, OperatorParameter),
    Lfo(u8, LfoParameter),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MasterParameter {
    Volume,
    Frequency,
    PitchBendRangeUp,
    PitchBendRangeDown,
    VelocitySensitivityVolume,
    VoiceMode,
    GlideActive,
    GlideTime,
    GlideBpmSync,
    GlideMode,
    GlideRetrigger,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OperatorParameter {
    Volume,
    Active,
    MixOut,
    Panning,
    WaveType,
    ModTargets,
    ModOut,
    Feedback,
    FrequencyRatio,
    FrequencyFree,
    FrequencyFine,
    AttackDuration,
    DecayDuration,
    SustainVolume,
    ReleaseDuration,
    EnvelopeLockGroup,
    VelocitySensitivityModOut,
    VelocitySensitivityFeedback,
    AftertouchSensitivityVolume,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LfoParameter {
    Target,
    BpmSync,
    FrequencyRatio,
    FrequencyFree,
    Mode,
    Shape,
    Amount,
    Active,
    /// Sync LFO phase to key presses. If turned off, start at random phase
    KeySync,
}
