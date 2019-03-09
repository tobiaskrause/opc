#![allow(non_camel_case_types, non_snake_case, unused, non_upper_case_globals)]
use super::winapi::shared::guiddef::GUID;
use super::winapi::shared::minwindef::UINT;
use super::winapi::shared::winerror::HRESULT;
use super::winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use super::winapi::um::oaidl::{IDispatch, IDispatchVtbl, LPDISPATCH, VARIANT};
use super::winapi::um::unknwnbase::{IUnknown, IUnknownVtbl, LPUNKNOWN};
use super::winapi::shared::wtypes::DATE;
use super::winapi::um::oaidl::SAFEARRAY;

include!(concat!(env!("OUT_DIR"), "/gbdaaut.rs"));