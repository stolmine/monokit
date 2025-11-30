use super::common::{
    define_pattern_query, define_pattern_query_1val,
};

define_pattern_query!(handle_pattern_min, handle_pn_min, pattern_min_impl, "MIN", result);
define_pattern_query!(handle_pattern_max, handle_pn_max, pattern_max_impl, "MAX", result);
define_pattern_query!(handle_pattern_sum, handle_pn_sum, pattern_sum_impl, "SUM", direct);
define_pattern_query!(handle_pattern_avg, handle_pn_avg, pattern_avg_impl, "AVG", result);
define_pattern_query_1val!(handle_pattern_fnd, handle_pn_fnd, pattern_fnd_impl, "FND");
