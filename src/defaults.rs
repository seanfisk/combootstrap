use anyhow::{Result, Context};
use std::ffi::CString;

mod sys {
    include!(concat!(env!("OUT_DIR"), "/defaults.rs"));
}

pub(crate) fn set_bool(app_id: &str, key: &str, value: bool) -> Result<()> {
    let c_app_id = to_cstring(&app_id)?;
    let c_key = to_cstring(&key)?;
    unsafe {
        sys::defaults_set_bool(c_app_id.as_ptr(), c_key.as_ptr(), value)
    }
    Ok(())
}

fn to_cstring(str: &str) -> Result<CString> {
    CString::new(str).with_context(|| format!("Converting to string {:?} to CString", str))
}
