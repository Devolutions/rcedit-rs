#![allow(dead_code)]

use std::ffi::c_void;

pub type ResourceUpdater = *mut c_void;

extern "C" {
    pub fn ResourceUpdater_New() -> ResourceUpdater;
    pub fn ResourceUpdater_Free(ctx: ResourceUpdater);

    pub fn ResourceUpdater_Load(ctx: ResourceUpdater, w_file_path: *const u16) -> bool;
    pub fn ResourceUpdater_Commit(ctx: ResourceUpdater) -> bool;

    pub fn ResourceUpdater_SetIcon(ctx: ResourceUpdater, w_icon_path: *const u16) -> bool;
    pub fn ResourceUpdater_ChangeRcdata(ctx: ResourceUpdater, id: u32, w_rcdata_path: *const u16) -> bool;
    pub fn ResourceUpdater_ChangeString(ctx: ResourceUpdater, id: u32, w_rcdata_path: *const u16) -> bool;
}
