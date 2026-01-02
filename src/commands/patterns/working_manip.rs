use crate::commands::patterns::common::{
    define_pattern_op_1val, define_pattern_op_noarg,
    define_pattern_op_idx_val, define_pattern_op_idx,
    define_pattern_pop, define_pattern_rnd,
};
use crate::output::OutputDecider;

define_pattern_op_1val!(handle_pattern_push, handle_pn_push, pattern_push_impl, "PUSH", result);
define_pattern_pop!(handle_pattern_pop, handle_pn_pop, pattern_pop_impl, "POP");
define_pattern_op_idx_val!(handle_pattern_ins, handle_pn_ins, pattern_ins_impl, "INS");
define_pattern_op_idx!(handle_pattern_rm, handle_pn_rm, pattern_rm_impl, "RM");
define_pattern_op_noarg!(handle_pattern_rev, handle_pn_rev, pattern_rev_impl, "REV", direct);
define_pattern_op_1val!(handle_pattern_rot, handle_pn_rot, pattern_rot_impl, "ROT", result);
define_pattern_op_noarg!(handle_pattern_shuf, handle_pn_shuf, pattern_shuf_impl, "SHUF", result_idx);
define_pattern_op_noarg!(handle_pattern_sort, handle_pn_sort, pattern_sort_impl, "SORT", direct);
define_pattern_rnd!(handle_pattern_rnd, handle_pn_rnd, pattern_rnd_impl, "RND");
