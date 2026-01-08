use crate::config;

define_bool_toggle!(handle_cpu, "CPU", "CPU DISPLAY: {}", "CPU DISPLAY: OFF", "CPU DISPLAY: ON", config::save_show_cpu);

define_bool_toggle!(handle_bpm, "BPM", "BPM: {}", "BPM: OFF", "BPM: ON", config::save_show_bpm);

define_bool_toggle!(handle_load_rst, "LOAD.RST", "LOAD.RST: {}", "LOAD.RST: OFF (PERSIST PARAMS)", "LOAD.RST: ON (RESET BEFORE LOAD)", config::save_load_rst);

define_bool_toggle!(handle_load_clr, "LOAD.CLR", "LOAD.CLR: {}", "LOAD.CLR: OFF", "LOAD.CLR: ON (CLEAR ON LOAD)", config::save_load_clr);

define_bool_toggle!(handle_autoload, "AUTOLOAD", "AUTOLOAD: {}", "AUTOLOAD: OFF", "AUTOLOAD: ON (LOAD LAST SCENE)", config::save_autoload);

define_bool_toggle!(handle_out_err, "OUT.ERR", config::save_out_err);

define_bool_toggle!(handle_out_ess, "OUT.ESS", config::save_out_ess);

define_bool_toggle!(handle_out_qry, "OUT.QRY", config::save_out_qry);

define_bool_toggle!(handle_out_cfm, "OUT.CFM", config::save_out_cfm);

define_bool_toggle!(handle_cfm_quit, "CFM.QUIT", "CFM.QUIT: {}", "CFM.QUIT: OFF", "CFM.QUIT: ON (CONFIRM QUIT)", config::save_confirm_quit_unsaved);

define_bool_toggle!(handle_cfm_save, "CFM.SAVE", "CFM.SAVE: {}", "CFM.SAVE: OFF", "CFM.SAVE: ON (CONFIRM OVERWRITE)", config::save_confirm_overwrite_scene);
