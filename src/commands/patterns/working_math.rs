use anyhow::Context;
use crate::commands::patterns::common::{
    define_pattern_op_1val, define_pattern_op_2val,
    pattern_add_impl, pattern_sub_impl, pattern_mul_impl,
    pattern_div_impl, pattern_mod_impl, pattern_scale_impl,
};

define_pattern_op_1val!(handle_pattern_add, handle_pn_add, pattern_add_impl, "ADD", direct);
define_pattern_op_1val!(handle_pattern_sub, handle_pn_sub, pattern_sub_impl, "SUB", direct);
define_pattern_op_1val!(handle_pattern_mul, handle_pn_mul, pattern_mul_impl, "MUL", direct);
define_pattern_op_1val!(handle_pattern_div, handle_pn_div, pattern_div_impl, "DIV", result);
define_pattern_op_1val!(handle_pattern_mod, handle_pn_mod, pattern_mod_impl, "MOD", result);
define_pattern_op_2val!(handle_pattern_scale, handle_pn_scale, pattern_scale_impl, "SCALE", "MIN AND MAX VALUES", i16);
