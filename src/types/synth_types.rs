pub const PRIMARY_BUS: i32 = 16;
pub const MOD_BUS: i32 = 17;
pub const NOISE_BUS: i32 = 18;
pub const PLAITS_MAIN_BUS: i32 = 19;
pub const PLAITS_AUX_BUS: i32 = 20;

pub const NOISE_NODE_ID: i32 = 1000;
pub const MOD_NODE_ID: i32 = 1001;
pub const PRIMARY_NODE_ID: i32 = 1002;
pub const MAIN_NODE_ID: i32 = 1003;
pub const PLAITS_NODE_ID: i32 = 1004;

pub const SPECTRUM_NODE_ID: i32 = 1010;
pub const SCOPE_NODE_ID: i32 = 1011;

pub struct VoiceSynths {
    pub noise_node: i32,
    pub mod_node: i32,
    pub primary_node: i32,
    pub main_node: i32,
    pub plaits_node: i32,
}

impl VoiceSynths {
    pub fn new() -> Self {
        VoiceSynths {
            noise_node: NOISE_NODE_ID,
            mod_node: MOD_NODE_ID,
            primary_node: PRIMARY_NODE_ID,
            main_node: MAIN_NODE_ID,
            plaits_node: PLAITS_NODE_ID,
        }
    }
}

pub fn route_param_to_node(param: &str) -> i32 {
    match param {
        "nw" | "nv" => NOISE_NODE_ID,

        "mf" | "mw" | "mv" | "fb" | "fba" | "fbd" | "nm" | "md" => MOD_NODE_ID,

        "mb" | "mba" | "mbd" => MOD_NODE_ID,

        "pf" | "pw" | "pv" | "fm" | "fa" | "fd" | "pa" | "pd" | "tk" | "np" | "mp" | "mt" => PRIMARY_NODE_ID,
        "dc" | "dm" | "dd" | "da" => MAIN_NODE_ID,

        "pitch" | "detune" | "engine" | "harmonics" | "timbre" | "morph" | "decay" | "lpg" | "plv" | "pav" | "pl_gate" | "t_gate_plaits" => PLAITS_NODE_ID,

        "s_rate" | "s_pitch" | "s_fine" | "s_direction" | "s_loop" |
        "s_startFrame" | "s_endFrame" | "s_atk" | "s_dec" | "s_rel" | "s_sust" |
        "s_volume" | "s_pan" | "s_fx" | "s_ratemod" | "s_pitchmod" |
        "s_bufnum" | "t_gate_sampler" => super::sampler_types::SAMPLER_NODE_ID,

        "sf_cut" | "sf_res" | "sf_type" |
        "sf_bits" | "sf_rate" | "sf_deci" |
        "sf_prob" | "sf_mult" | "sf_glit" => super::sampler_types::SAMPLER_NODE_ID,

        _ => MAIN_NODE_ID,
    }
}

pub fn route_param_to_nodes(param: &str) -> Vec<i32> {
    match param {
        "mb" | "mba" | "mbd" => vec![MOD_NODE_ID, PRIMARY_NODE_ID],

        _ => vec![route_param_to_node(param)],
    }
}
