use rosc::OscType;
use super::audio_types::{MeterData, VoiceMeterData, SpectrumData, ScopeData, CpuData, CompressorData};
use super::script_types::SyncMode;

#[derive(Debug, Clone)]
pub struct DelayedCommand {
    pub due_at_ms: u64,
    pub command: String,
    pub script_index: usize,
}

#[derive(Debug, Clone)]
pub enum MetroCommand {
    SetInterval(u64),
    SetActive(bool),
    SetScriptIndex(usize),
    SendParam(String, OscType),
    SendTrigger,
    SendPlaitsTrigger,
    SendVolume(f32),
    StartRecording(String),
    StopRecording,
    SetRecordingPath(String),
    SetSlewTime(f32),
    SetParamSlew(String, f32),
    SetGate(f32),
    SetEnvGate(String, f32),
    Shutdown,
    ScheduleDelayed(String, u64, usize),
    ScheduleRepeated(String, i16, u64, usize),
    ClearDelayed,
    SetSyncMode(SyncMode),
    MidiClockTick,
    MidiTransportStart,
    MidiTransportStop,
    EnableMidiTimingDiag,
    DisableMidiTimingDiag,
    PrintMidiTimingReport,
    SendScDiag(i32),
    SendScDiagReport,
    GetTriggerCount,
    ResetTriggerCount,
    SendScopeRate(f32),
    Error(String),
    QueryAudioOutDevices,
    SetAudioOutDevice(String),
}

#[derive(Debug, Clone)]
pub enum MetroEvent {
    ExecuteScript(usize),
    ExecuteDelayed(String, usize),
    MeterUpdate(MeterData),
    VoiceMeterUpdate(VoiceMeterData),
    SpectrumUpdate(SpectrumData),
    ScopeUpdate(ScopeData),
    CpuUpdate(CpuData),
    CompressorUpdate(CompressorData),
    ScReady,
    AudioDeviceList { current: String, devices: Vec<String> },
    RestartScWithDevice(String),
    Error(String),
    StartRecordingDirect(String),
    StopRecordingDirect,
    SetRecordingPathDirect(String),
}

/// Commands for the separate delay thread
#[derive(Debug, Clone)]
pub enum DelayThreadCommand {
    Schedule(String, u64, usize),      // command, due_at_ms, script_index
    ScheduleRepeated(String, i16, u64, usize), // command, count, interval_ms, script_index
    Clear,
    Shutdown,
}

#[derive(Debug, Clone)]
pub struct MetroState {
    pub interval_ms: u64,
    pub active: bool,
    pub script_index: usize,
}

impl Default for MetroState {
    fn default() -> Self {
        Self {
            interval_ms: 500,
            active: false,
            script_index: 8,
        }
    }
}
