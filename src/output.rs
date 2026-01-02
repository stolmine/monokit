use crate::types::{OutputCategory, TIER_CONFIRMS, TIER_ERRORS, TIER_ESSENTIAL, TIER_QUERIES};

pub trait OutputDecider {
    fn debug_level(&self) -> u8;
    fn out_err(&self) -> bool;
    fn out_ess(&self) -> bool;
    fn out_qry(&self) -> bool;
    fn out_cfm(&self) -> bool;

    fn should_output(&self, category: OutputCategory) -> bool {
        let tier = match category {
            OutputCategory::Error => TIER_ERRORS,
            OutputCategory::Essential => TIER_ESSENTIAL,
            OutputCategory::Query => TIER_QUERIES,
            OutputCategory::Confirm => TIER_CONFIRMS,
            OutputCategory::Verbose => return false,
        };

        self.debug_level() >= tier
            || match category {
                OutputCategory::Error => self.out_err(),
                OutputCategory::Essential => self.out_ess(),
                OutputCategory::Query => self.out_qry(),
                OutputCategory::Confirm => self.out_cfm(),
                OutputCategory::Verbose => false,
            }
    }

    fn output<F>(&self, category: OutputCategory, message: String, mut output: F)
    where
        F: FnMut(String),
    {
        if self.should_output(category) {
            output(message);
        }
    }
}
