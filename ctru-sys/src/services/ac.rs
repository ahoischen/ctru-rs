/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
extern "C" {
    pub fn acInit() -> Result;
    pub fn acExit();
    pub fn acWaitInternetConnection() -> Result;
    pub fn ACU_GetWifiStatus(out: *mut u32_) -> Result;
}
use ::types::*;
