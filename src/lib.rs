use std::path::Path;

use rcedit_sys as sys;
use thiserror::Error;
use widestring::WideCString;

#[derive(Debug, Error)]
pub enum RceditError {
    #[error("Failed to load the specified executable")]
    ExecutableLoadFailed,
    #[error("Failed to commit the executed changes")]
    ExecutableCommitFailed,
    #[error("Failed to update the resource")]
    ResourceUpdateFailure,
    #[error("Failed to convert string encoding")]
    StringConversionFailed,
}

pub type RceditResult<T> = Result<T, RceditError>;

pub struct ResourceUpdater {
    handle: sys::ResourceUpdater,
}

impl ResourceUpdater {
    pub fn new() -> Self {
        let handle = unsafe { sys::ResourceUpdater_New() };
        Self { handle }
    }

    pub fn load(&mut self, executable_path: &Path) -> RceditResult<()> {
        let cstr = path_to_wide_string(executable_path)?;
        if unsafe { sys::ResourceUpdater_Load(self.handle, cstr.as_ptr()) } {
            Ok(())
        } else {
            Err(RceditError::ExecutableLoadFailed)
        }
    }

    pub fn commit(&mut self) -> RceditResult<()> {
        if unsafe { sys::ResourceUpdater_Commit(self.handle) } {
            Ok(())
        } else {
            Err(RceditError::ExecutableCommitFailed)
        }
    }

    pub fn set_icon(&mut self, icon_path: &Path) -> RceditResult<()> {
        let cstr = path_to_wide_string(icon_path)?;
        if unsafe { sys::ResourceUpdater_SetIcon(self.handle, cstr.as_ptr()) } {
            Ok(())
        } else {
            Err(RceditError::ResourceUpdateFailure)
        }
    }

    pub fn set_version_string(&mut self, name: &str, value: &str) -> RceditResult<()> {
        let cstr_name = utf8_to_wide_string(name)?;
        let cstr_value = utf8_to_wide_string(value)?;
        if unsafe {
            sys::ResourceUpdater_SetVersionString(
                self.handle,
                cstr_name.as_ptr(),
                cstr_value.as_ptr(),
            )
        } {
            Ok(())
        } else {
            Err(RceditError::ResourceUpdateFailure)
        }
    }

    pub fn set_file_version(&mut self, v1: u16, v2: u16, v3: u16, v4: u16) -> RceditResult<()> {
        if unsafe { sys::ResourceUpdater_SetFileVersion(self.handle, v1, v2, v3, v4) } {
            Ok(())
        } else {
            Err(RceditError::ResourceUpdateFailure)
        }
    }

    pub fn set_product_version(&mut self, v1: u16, v2: u16, v3: u16, v4: u16) -> RceditResult<()> {
        if unsafe { sys::ResourceUpdater_SetProductVersion(self.handle, v1, v2, v3, v4) } {
            Ok(())
        } else {
            Err(RceditError::ResourceUpdateFailure)
        }
    }

    pub fn set_execution_level(&mut self, level: &str) -> RceditResult<()> {
        let cstr = utf8_to_wide_string(level)?;
        if unsafe { sys::ResourceUpdater_SetExecutionLevel(self.handle, cstr.as_ptr()) } {
            Ok(())
        } else {
            Err(RceditError::ResourceUpdateFailure)
        }
    }

    pub fn set_string(&mut self, string_id: u32, data: &str) -> RceditResult<()> {
        let cstr = utf8_to_wide_string(data)?;
        if unsafe { sys::ResourceUpdater_ChangeString(self.handle, string_id, cstr.as_ptr()) } {
            Ok(())
        } else {
            Err(RceditError::ResourceUpdateFailure)
        }
    }

    pub fn set_rcdata(&mut self, string_id: u32, data_path: &Path) -> RceditResult<()> {
        let cstr = path_to_wide_string(data_path)?;
        if unsafe { sys::ResourceUpdater_ChangeRcdata(self.handle, string_id, cstr.as_ptr()) } {
            Ok(())
        } else {
            Err(RceditError::ResourceUpdateFailure)
        }
    }
}

impl Drop for ResourceUpdater {
    fn drop(&mut self) {
        unsafe { sys::ResourceUpdater_Free(self.handle) };
    }
}

impl Default for ResourceUpdater {
    fn default() -> Self {
        Self::new()
    }
}

fn utf8_to_wide_string(string: &str) -> RceditResult<WideCString> {
    WideCString::from_str(string).map_err(|_| RceditError::StringConversionFailed)
}

fn path_to_wide_string(string: &Path) -> RceditResult<WideCString> {
    utf8_to_wide_string(string.to_str().ok_or(RceditError::StringConversionFailed)?)
}
