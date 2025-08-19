use std::{ffi::OsString, os::windows::ffi::OsStringExt};

use log::{trace, warn};
use windows::{
    core::{PCWSTR, PWSTR},
    Win32::{
        Foundation::{GetLastError, HMODULE, HWND, LPARAM, WPARAM},
        System::LibraryLoader::*,
        UI::{Controls::EM_SETREADONLY, WindowsAndMessaging::*},
    },
};

use crate::{
    ctype::HWnd,
    dbc::connect::dsn::get_private_profile_string,
    setup::{dsn_dialog_value::DsnDialogValue, installer::valid_dsn},
    util::string_to_utf16,
};

const IDD_DSN_DIALOG: i32 = 101;
const IDC_DATA_SOURCE_NAME: i32 = 1001;
const IDC_ENDPOINT: i32 = 1002;

const IDS_INVALID_DSN: u32 = 201;
const IDS_OVERWRITE_DSN: u32 = 202;

pub(crate) fn dsn_dialog(
    parent_hwnd: HWnd,
    dialog_value: DsnDialogValue,
) -> Result<Option<DsnDialogValue>, ()> {
    const FUNCTION_NAME: &str = "dsn_dialog()";
    trace!(
        "{FUNCTION_NAME} start. parent_hwnd={:?}, dialog_value={:?}",
        parent_hwnd,
        dialog_value
    );

    let hmodule = match get_current_module() {
        Ok(module) => module,
        Err(e) => {
            warn!("{FUNCTION_NAME} error. get_current_module failed. {:?}", e);
            return Err(());
        }
    };
    let dialog_id = PCWSTR(IDD_DSN_DIALOG as *const u16);
    let dialog_value_ptr = Box::into_raw(Box::new(dialog_value));

    let rc = unsafe {
        DialogBoxParamW(
            Some(hmodule.into()),
            dialog_id,
            Some(HWND(parent_hwnd)),
            Some(dialog_proc),
            LPARAM(dialog_value_ptr as isize),
        )
    };

    let dialog_value = unsafe { Box::from_raw(dialog_value_ptr) };

    if rc < 0 {
        warn!(
            "{FUNCTION_NAME} error. DialogBoxParamW failed. {:?}",
            unsafe { GetLastError() }
        );
        return Err(());
    }

    trace!(
        "{FUNCTION_NAME} end. rc={}, dialog_value={:?}",
        rc,
        dialog_value
    );
    if rc == IDOK.0 as isize {
        Ok(Some(*dialog_value))
    } else {
        Ok(None)
    }
}

unsafe extern "system" fn dialog_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> isize {
    const FUNCTION_NAME: &str = "dialog_proc()";
    const TRUE: isize = 1;
    const FALSE: isize = 0;

    let dialog_value_ptr = unsafe {
        if message == WM_INITDIALOG {
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, lparam.0);
            lparam.0 as *mut DsnDialogValue
        } else {
            GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut DsnDialogValue
        }
    };
    let dialog_value = &mut *dialog_value_ptr;

    match message {
        WM_INITDIALOG => {
            let rc = match init_dialog(hwnd, dialog_value) {
                Ok(_) => TRUE,
                Err(e) => {
                    warn!("{FUNCTION_NAME} error. {:?}", e);
                    FALSE
                }
            };
            rc
        }
        WM_COMMAND => {
            let id = wparam.0 as i32;
            if id == IDOK.0 {
                update_dialog_value(hwnd, dialog_value);

                if validate_dsn(dialog_value.data_source_name()).is_err() {
                    message_box_exclamation(hwnd, "Data Source Name Error", IDS_INVALID_DSN);
                    return TRUE;
                }

                if (dialog_value.is_add() || dialog_value.is_rename_dsn())
                    && exists_dsn(dialog_value.data_source_name())
                {
                    trace!(
                        "{FUNCTION_NAME}: DSN {} already exists.",
                        dialog_value.data_source_name()
                    );
                    if message_box_yesno(hwnd, "Overwrite DSN?", IDS_OVERWRITE_DSN) {
                        // fall through
                    } else {
                        return TRUE;
                    }
                }
            }
            if id == IDOK.0 || id == IDCANCEL.0 {
                unsafe {
                    let _ = EndDialog(hwnd, id as isize);
                }
                TRUE
            } else {
                FALSE
            }
        }
        _ => FALSE,
    }
}

fn init_dialog(
    hwnd: HWND,
    dialog_value: &DsnDialogValue,
) -> Result<(), Box<dyn std::error::Error>> {
    set_text_value(hwnd, IDC_DATA_SOURCE_NAME, dialog_value.data_source_name())?;
    set_text_value(hwnd, IDC_ENDPOINT, dialog_value.endpoint())?;

    if !dialog_value.data_source_name_editable() {
        set_editable(hwnd, IDC_DATA_SOURCE_NAME, false)?;
    }

    Ok(())
}

fn set_text_value(hwnd: HWND, id: i32, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut v: Vec<u16> = value.encode_utf16().collect();
    v.push(0); // nul-terminate

    unsafe { SetDlgItemTextW(hwnd, id, PCWSTR(v.as_ptr()))? };
    Ok(())
}

fn set_editable(hwnd: HWND, id: i32, editable: bool) -> Result<(), Box<dyn std::error::Error>> {
    let editable = if editable { 0 } else { 1 };
    unsafe {
        let item = GetDlgItem(Some(hwnd), id)?;
        SendMessageW(item, EM_SETREADONLY, Some(WPARAM(editable)), None);
    }
    Ok(())
}

fn update_dialog_value(hwnd: HWND, dialog_value: &mut DsnDialogValue) {
    dialog_value.set_data_source_name(get_text_value(hwnd, IDC_DATA_SOURCE_NAME));
    dialog_value.set_endpoint(get_text_value(hwnd, IDC_ENDPOINT));
}

fn get_text_value(hwnd: HWND, id: i32) -> String {
    let mut buf = [0u16; 1024];
    let len = unsafe { GetDlgItemTextW(hwnd, id, &mut buf) };
    if len == 0 {
        return String::new();
    }
    let widestring = OsString::from_wide(&buf[..len as usize]);
    let s = widestring.to_string_lossy();
    s.into_owned()
}

fn validate_dsn(dsn: &str) -> Result<(), String> {
    valid_dsn(dsn)
}

fn exists_dsn(dsn: &str) -> bool {
    match get_private_profile_string("ODBC Data Sources", dsn, "") {
        Ok(value) => !value.is_empty(),
        Err(_) => false,
    }
}

fn message_box_exclamation(hwnd: HWND, title: &str, message_id: u32) {
    let title = string_to_utf16(title);

    unsafe {
        let module = get_current_module().unwrap_or_default();
        let mut message = [0u16; 1024];
        LoadStringW(
            Some(module.into()),
            message_id,
            PWSTR(message.as_mut_ptr()),
            message.len() as i32,
        );

        MessageBoxW(
            Some(hwnd),
            PCWSTR(message.as_ptr()),
            PCWSTR(title.as_ptr()),
            MB_OK | MB_ICONEXCLAMATION,
        )
    };
}

fn message_box_yesno(hwnd: HWND, title: &str, message_id: u32) -> bool {
    let title = string_to_utf16(title);

    let rc = unsafe {
        let module = get_current_module().unwrap_or_default();
        let mut message = [0u16; 1024];
        LoadStringW(
            Some(module.into()),
            message_id,
            PWSTR(message.as_mut_ptr()),
            message.len() as i32,
        );

        MessageBoxW(
            Some(hwnd),
            PCWSTR(message.as_ptr()),
            PCWSTR(title.as_ptr()),
            MB_YESNO | MB_ICONQUESTION | MB_DEFBUTTON2,
        )
    };
    matches!(rc, IDYES)
}

fn get_current_module() -> Result<HMODULE, Box<dyn std::error::Error>> {
    let mut module = HMODULE::default();
    unsafe {
        // GetModuleHandleExA(
        //     GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
        //     PCSTR(ODBC_DRIVER_FILE_NAME.as_ptr()),
        //     &mut module,
        // )?;
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            PCWSTR(get_current_module as *const u16),
            &mut module,
        )?;
    }
    Ok(module)
}
