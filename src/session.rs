use crate::prelude::*;

pub trait TSession: TSessionCommon + TSessionElement + TSessionLocation + TSessionModule {
    fn clone_box(&self) -> Box<dyn TSession>;
}
