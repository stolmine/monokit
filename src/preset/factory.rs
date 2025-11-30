use super::Preset;
use std::collections::HashMap;
use std::sync::OnceLock;

static FACTORY_PRESETS: OnceLock<HashMap<String, Preset>> = OnceLock::new();

fn init_factory_presets() -> HashMap<String, Preset> {
    let mut presets = HashMap::new();

    presets.insert("808-kick".to_string(), Preset {
        version: 1,
        name: "808-kick".to_string(),
        category: "drums".to_string(),
        lines: vec![
            "PF 55".to_string(),
            "PA 4".to_string(),
            "PD 60".to_string(),
            "AD 300".to_string(),
            "FC 200".to_string(),
            "FT 0".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Classic 808 kick drum".to_string(),
    });

    presets.insert("punch-kick".to_string(), Preset {
        version: 1,
        name: "punch-kick".to_string(),
        category: "drums".to_string(),
        lines: vec![
            "PF 60".to_string(),
            "PA 3".to_string(),
            "PD 40".to_string(),
            "AD 250".to_string(),
            "DC 4000".to_string(),
            "DM 1".to_string(),
            "DD 50".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Punchy kick with discontinuity".to_string(),
    });

    presets.insert("sub-kick".to_string(), Preset {
        version: 1,
        name: "sub-kick".to_string(),
        category: "drums".to_string(),
        lines: vec![
            "PF 40".to_string(),
            "PA 2".to_string(),
            "PD 100".to_string(),
            "AD 500".to_string(),
            "FC 150".to_string(),
            "FT 0".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Deep sub kick".to_string(),
    });

    presets.insert("basic-snare".to_string(), Preset {
        version: 1,
        name: "basic-snare".to_string(),
        category: "drums".to_string(),
        lines: vec![
            "PF 180".to_string(),
            "AD 150".to_string(),
            "FB 8000".to_string(),
            "FBD 80".to_string(),
            "DC 2000".to_string(),
            "DM 0".to_string(),
            "FC 3000".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Basic snare drum".to_string(),
    });

    presets.insert("snap-snare".to_string(), Preset {
        version: 1,
        name: "snap-snare".to_string(),
        category: "drums".to_string(),
        lines: vec![
            "PF 250".to_string(),
            "AD 80".to_string(),
            "FB 10000".to_string(),
            "FBD 40".to_string(),
            "DC 6000".to_string(),
            "DM 0".to_string(),
            "FT 1".to_string(),
            "FC 2000".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Snappy snare with fast decay".to_string(),
    });

    presets.insert("hat-closed".to_string(), Preset {
        version: 1,
        name: "hat-closed".to_string(),
        category: "drums".to_string(),
        lines: vec![
            "MF 8000".to_string(),
            "MW 3".to_string(),
            "FB 12000".to_string(),
            "AD 30".to_string(),
            "FBD 20".to_string(),
            "FC 6000".to_string(),
            "FT 1".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Closed hi-hat".to_string(),
    });

    presets.insert("hat-open".to_string(), Preset {
        version: 1,
        name: "hat-open".to_string(),
        category: "drums".to_string(),
        lines: vec![
            "MF 7000".to_string(),
            "MW 3".to_string(),
            "FB 14000".to_string(),
            "AD 300".to_string(),
            "FBD 200".to_string(),
            "FC 5000".to_string(),
            "FT 1".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Open hi-hat".to_string(),
    });

    presets.insert("fm-hat".to_string(), Preset {
        version: 1,
        name: "fm-hat".to_string(),
        category: "drums".to_string(),
        lines: vec![
            "PF 800".to_string(),
            "MF 1470".to_string(),
            "FM 10000".to_string(),
            "FA 8000".to_string(),
            "FD 40".to_string(),
            "AD 50".to_string(),
            "FC 6000".to_string(),
            "FT 1".to_string(),
        ],
        j: 0,
        k: 0,
        description: "FM hi-hat".to_string(),
    });

    presets.insert("clap".to_string(), Preset {
        version: 1,
        name: "clap".to_string(),
        category: "drums".to_string(),
        lines: vec![
            "PF 1000".to_string(),
            "AD 200".to_string(),
            "FB 10000".to_string(),
            "FBD 150".to_string(),
            "DC 3000".to_string(),
            "DM 5".to_string(),
            "RV 2000".to_string(),
            "RW 3000".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Clap with reverb".to_string(),
    });

    presets.insert("rim".to_string(), Preset {
        version: 1,
        name: "rim".to_string(),
        category: "drums".to_string(),
        lines: vec![
            "PF 400".to_string(),
            "AD 40".to_string(),
            "PA 2".to_string(),
            "PD 10".to_string(),
            "DC 8000".to_string(),
            "DM 0".to_string(),
            "FC 5000".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Rim shot".to_string(),
    });

    presets.insert("sub-bass".to_string(), Preset {
        version: 1,
        name: "sub-bass".to_string(),
        category: "bass".to_string(),
        lines: vec![
            "PF 55".to_string(),
            "PW 0".to_string(),
            "AD 500".to_string(),
            "FC 400".to_string(),
            "FE 6000".to_string(),
            "FED 200".to_string(),
            "PA 0.5".to_string(),
            "PD 30".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Deep sub bass".to_string(),
    });

    presets.insert("saw-bass".to_string(), Preset {
        version: 1,
        name: "saw-bass".to_string(),
        category: "bass".to_string(),
        lines: vec![
            "PF 65".to_string(),
            "PW 2".to_string(),
            "AD 400".to_string(),
            "FC 600".to_string(),
            "FE 8000".to_string(),
            "FED 250".to_string(),
            "FQ 4000".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Saw wave bass".to_string(),
    });

    presets.insert("fm-bass".to_string(), Preset {
        version: 1,
        name: "fm-bass".to_string(),
        category: "bass".to_string(),
        lines: vec![
            "PF 55".to_string(),
            "FM 8000".to_string(),
            "FA 6000".to_string(),
            "FD 150".to_string(),
            "AD 350".to_string(),
            "FC 800".to_string(),
            "FE 4000".to_string(),
            "FED 200".to_string(),
        ],
        j: 0,
        k: 0,
        description: "FM bass".to_string(),
    });

    presets.insert("saw-lead".to_string(), Preset {
        version: 1,
        name: "saw-lead".to_string(),
        category: "lead".to_string(),
        lines: vec![
            "PF 440".to_string(),
            "PW 2".to_string(),
            "AD 400".to_string(),
            "FC 3000".to_string(),
            "FE 4000".to_string(),
            "FED 200".to_string(),
            "FQ 2000".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Saw wave lead".to_string(),
    });

    presets.insert("fm-lead".to_string(), Preset {
        version: 1,
        name: "fm-lead".to_string(),
        category: "lead".to_string(),
        lines: vec![
            "PF 440".to_string(),
            "FM 6000".to_string(),
            "FA 4000".to_string(),
            "FD 300".to_string(),
            "AD 500".to_string(),
            "FC 4000".to_string(),
            "FQ 1000".to_string(),
        ],
        j: 0,
        k: 0,
        description: "FM lead".to_string(),
    });

    presets.insert("pluck-lead".to_string(), Preset {
        version: 1,
        name: "pluck-lead".to_string(),
        category: "lead".to_string(),
        lines: vec![
            "PF 523".to_string(),
            "PW 1".to_string(),
            "AD 200".to_string(),
            "FC 4000".to_string(),
            "FE 8000".to_string(),
            "FED 100".to_string(),
            "FQ 4000".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Plucked lead".to_string(),
    });

    presets.insert("metal-hit".to_string(), Preset {
        version: 1,
        name: "metal-hit".to_string(),
        category: "percussion".to_string(),
        lines: vec![
            "PF 300".to_string(),
            "MF 500".to_string(),
            "FM 10000".to_string(),
            "AD 150".to_string(),
            "PA 1.5".to_string(),
            "PD 30".to_string(),
            "FC 5000".to_string(),
            "FT 2".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Metallic percussion hit".to_string(),
    });

    presets.insert("conga".to_string(), Preset {
        version: 1,
        name: "conga".to_string(),
        category: "percussion".to_string(),
        lines: vec![
            "PF 200".to_string(),
            "PA 3".to_string(),
            "PD 80".to_string(),
            "AD 200".to_string(),
            "FC 1000".to_string(),
            "FE 2000".to_string(),
            "FED 50".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Conga drum".to_string(),
    });

    presets.insert("tom".to_string(), Preset {
        version: 1,
        name: "tom".to_string(),
        category: "percussion".to_string(),
        lines: vec![
            "PF 120".to_string(),
            "PA 3".to_string(),
            "PD 100".to_string(),
            "AD 250".to_string(),
            "FC 600".to_string(),
            "FE 3000".to_string(),
            "FED 100".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Tom drum".to_string(),
    });

    presets.insert("noise".to_string(), Preset {
        version: 1,
        name: "noise".to_string(),
        category: "fx".to_string(),
        lines: vec![
            "FB 16000".to_string(),
            "FBD 1000".to_string(),
            "AD 500".to_string(),
            "FC 10000".to_string(),
            "FT 0".to_string(),
            "MF 1000".to_string(),
            "MW 3".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Noise sweep".to_string(),
    });

    presets.insert("zap".to_string(), Preset {
        version: 1,
        name: "zap".to_string(),
        category: "fx".to_string(),
        lines: vec![
            "PF 2000".to_string(),
            "PA 16".to_string(),
            "PD 150".to_string(),
            "AD 200".to_string(),
            "FC 8000".to_string(),
            "DC 4000".to_string(),
            "DM 0".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Zap sound effect".to_string(),
    });

    presets.insert("rise".to_string(), Preset {
        version: 1,
        name: "rise".to_string(),
        category: "fx".to_string(),
        lines: vec![
            "PF 100".to_string(),
            "PA 12".to_string(),
            "PD 500".to_string(),
            "AD 600".to_string(),
            "FC 4000".to_string(),
            "FE 4000".to_string(),
            "FED 400".to_string(),
        ],
        j: 0,
        k: 0,
        description: "Rising sweep effect".to_string(),
    });

    presets
}

pub fn get_factory_presets() -> &'static HashMap<String, Preset> {
    FACTORY_PRESETS.get_or_init(init_factory_presets)
}

pub fn get_factory_preset(name: &str) -> Option<Preset> {
    get_factory_presets().get(name).cloned()
}

pub fn list_factory_presets() -> Vec<String> {
    let mut names: Vec<String> = get_factory_presets().keys().cloned().collect();
    names.sort();
    names
}
