use crate::{
    core_foundation_sys::base::CFTypeID,
    core_graphics::display::CGDirectDisplayID,
    CVReturn, CVTime,
};

#[derive(Debug, Copy, Clone)]
pub enum __CVDisplayLink { }
pub type CVDisplayLinkRef = *mut __CVDisplayLink;

extern "C" {
    pub fn CVDisplayLinkGetTypeID() -> CFTypeID;
    pub fn CVDisplayLinkCreateWithCGDisplay(displayID: CGDirectDisplayID, displayLinkOut: *mut CVDisplayLinkRef) -> CVReturn;
    pub fn CVDisplayLinkGetNominalOutputVideoRefreshPeriod(displayLink: CVDisplayLinkRef) -> CVTime;
    pub fn CVDisplayLinkRelease(displayLink: CVDisplayLinkRef);
}
