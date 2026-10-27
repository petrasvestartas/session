#[cfg(test)]
impl Lane {
    pub(crate) fn use_legacy_fixture(&mut self){self.finite=None;}
    pub(crate) fn finite_colour_ready(&self)->bool {self.finite.as_ref().is_some_and(crate::finite_ink::Ink::colour_ready)}
}
