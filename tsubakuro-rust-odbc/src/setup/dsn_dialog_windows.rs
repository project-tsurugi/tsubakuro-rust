use std::{
    ffi::{c_void, OsString},
    os::windows::ffi::OsStringExt,
    time::Duration,
};

use log::{debug, trace, warn};
use tsubakuro_rust_core::prelude::{ConnectionOption, Credential, Session};
use windows::{
    core::{PCWSTR, PWSTR},
    Win32::{
        Foundation::{GetLastError, HMODULE, HWND, LPARAM, WPARAM},
        System::LibraryLoader::*,
        UI::{
            Controls::{CheckRadioButton, IsDlgButtonChecked, BST_CHECKED, EM_SETREADONLY},
            Input::KeyboardAndMouse::EnableWindow,
            Shell::{DragAcceptFiles, DragFinish, DragQueryFileW, HDROP},
            WindowsAndMessaging::*,
        },
    },
};

use crate::{
    ctype::HWnd,
    dbc::connect::dsn::get_private_profile_string,
    setup::{
        dsn_dialog_value::{DsnDialogCredentialRadio, DsnDialogValue},
        installer_api::valid_dsn,
    },
    util::{string_to_utf16, utf16_to_string},
};

const IDD_DSN_DIALOG: i32 = 101;
const IDC_DATA_SOURCE_NAME: i32 = 1001;
const IDC_ENDPOINT: i32 = 1002;
const IDC_CREDENTIAL_NO_AUTH: i32 = 1003;
const IDC_CREDENTIAL_USER_PASSWORD: i32 = 1004;
const IDC_CREDENTIAL_AUTH_TOKEN: i32 = 1005;
const IDC_CREDENTIAL_FILE: i32 = 1006;
const IDC_USER: i32 = 1007;
const IDC_PASSWORD: i32 = 1008;
const IDC_AUTH_TOKEN: i32 = 1009;
const IDC_CREDENTIALS: i32 = 1010;
const IDC_CONNECT_TEST: i32 = 1011;

const IDS_INVALID_DSN: u32 = 201;
const IDS_OVERWRITE_DSN: u32 = 202;
const IDS_CONNECT_TEST_OK: u32 = 203;
const IDS_CONNECT_TEST_NG: u32 = 204;
const IDS_ERROR: u32 = 209;

const DROP_FILES_ACCEPT: bool = false;

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
        WM_INITDIALOG => match init_dialog(hwnd, dialog_value) {
            Ok(_) => TRUE,
            Err(e) => {
                warn!("{FUNCTION_NAME} init_dialog error. {:?}", e);
                FALSE
            }
        },
        WM_DROPFILES => match drop_files(hwnd, wparam) {
            Ok(_) => TRUE,
            Err(e) => {
                warn!("{FUNCTION_NAME} drop_files error. {:?}", e);
                FALSE
            }
        },
        WM_COMMAND => {
            let id = wparam.0 as i32;
            match id {
                IDC_CREDENTIAL_NO_AUTH
                | IDC_CREDENTIAL_USER_PASSWORD
                | IDC_CREDENTIAL_AUTH_TOKEN
                | IDC_CREDENTIAL_FILE => {
                    let radio = match id {
                        IDC_CREDENTIAL_NO_AUTH => DsnDialogCredentialRadio::Null,
                        IDC_CREDENTIAL_USER_PASSWORD => DsnDialogCredentialRadio::UserPassword,
                        IDC_CREDENTIAL_AUTH_TOKEN => DsnDialogCredentialRadio::AuthToken,
                        IDC_CREDENTIAL_FILE => DsnDialogCredentialRadio::File,
                        _ => unreachable!(),
                    };
                    return match set_radio_enable(hwnd, radio) {
                        Ok(_) => TRUE,
                        Err(e) => {
                            warn!("{FUNCTION_NAME} error. {:?}", e);
                            FALSE
                        }
                    };
                }
                IDC_CONNECT_TEST => {
                    connect_test(hwnd);
                    return TRUE;
                }
                _ => {}
            }
            if id == IDOK.0 {
                if let Err(e) = update_dialog_value(hwnd, dialog_value) {
                    warn!("{FUNCTION_NAME} error. {:?}", e);
                    message_box_exclamation(
                        hwnd,
                        "Update DialogValue Error",
                        IDS_ERROR,
                        e.to_string(),
                    );
                    return FALSE;
                }

                if dialog_value.need_check_data_source_name() {
                    if let Err(e) = validate_dsn(dialog_value.data_source_name()) {
                        message_box_exclamation(hwnd, "Data Source Name Error", IDS_INVALID_DSN, e);
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
    if DROP_FILES_ACCEPT {
        unsafe {
            DragAcceptFiles(hwnd, true);
            ChangeWindowMessageFilter(WM_DROPFILES, MSGFLT_ADD)?;
            ChangeWindowMessageFilter(WM_COPYDATA, MSGFLT_ADD)?;
            // ChangeWindowMessageFilter(WM_COPYGLOBALDATA, MSGFLT_ADD)?;
        }
    }

    set_text_value(hwnd, IDC_DATA_SOURCE_NAME, dialog_value.data_source_name())?;
    set_text_value(hwnd, IDC_ENDPOINT, dialog_value.endpoint())?;
    set_text_value(hwnd, IDC_USER, dialog_value.user())?;
    set_text_value(hwnd, IDC_PASSWORD, dialog_value.password())?;
    set_text_value(hwnd, IDC_AUTH_TOKEN, dialog_value.auth_token())?;
    set_text_value(hwnd, IDC_CREDENTIALS, dialog_value.credential_file())?;

    if !dialog_value.data_source_name_editable() {
        set_editable(hwnd, IDC_DATA_SOURCE_NAME, false)?;
    }
    set_radio_select(hwnd, dialog_value.credential_radio())?;
    set_radio_enable(hwnd, dialog_value.credential_radio())?;

    Ok(())
}

fn set_radio_select(
    hwnd: HWND,
    radio: DsnDialogCredentialRadio,
) -> Result<(), Box<dyn std::error::Error>> {
    use DsnDialogCredentialRadio::*;
    let radio = match radio {
        Nothing | Null => IDC_CREDENTIAL_NO_AUTH,
        UserPassword => IDC_CREDENTIAL_USER_PASSWORD,
        AuthToken => IDC_CREDENTIAL_AUTH_TOKEN,
        File => IDC_CREDENTIAL_FILE,
    };
    unsafe {
        CheckRadioButton(hwnd, IDC_CREDENTIAL_NO_AUTH, IDC_CREDENTIAL_FILE, radio)?;
    }
    Ok(())
}

fn set_radio_enable(
    hwnd: HWND,
    radio: DsnDialogCredentialRadio,
) -> Result<(), Box<dyn std::error::Error>> {
    use DsnDialogCredentialRadio::*;
    set_enable(hwnd, IDC_USER, radio == UserPassword)?;
    set_enable(hwnd, IDC_PASSWORD, radio == UserPassword)?;
    set_enable(hwnd, IDC_AUTH_TOKEN, radio == AuthToken)?;
    set_enable(hwnd, IDC_CREDENTIALS, radio == File)?;
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

fn set_enable(hwnd: HWND, id: i32, enable: bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let item = GetDlgItem(Some(hwnd), id)?;
        let _ = EnableWindow(item, enable);
    }
    Ok(())
}

fn drop_files(hwnd: HWND, wparam: WPARAM) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let hdrop = HDROP(wparam.0 as *mut c_void);
        let mut path = [0u16; 4096];
        let count = DragQueryFileW(hdrop, 0, Some(&mut path));
        if count > 0 {
            set_radio_select(hwnd, DsnDialogCredentialRadio::File)?;
            set_radio_enable(hwnd, DsnDialogCredentialRadio::File)?;

            let path = utf16_to_string(&path);
            set_text_value(hwnd, IDC_CREDENTIALS, &path)?;
        }
        DragFinish(hdrop);
    }
    Ok(())
}

fn connect_test(hwnd: HWND) {
    const FUNCTION_NAME: &str = "connect_test()";

    if let Err(e) = connect_test_main(hwnd) {
        debug!("{FUNCTION_NAME} error. {:?}", e);
        message_box_exclamation(
            hwnd,
            "Connect Test Failed",
            IDS_CONNECT_TEST_NG,
            e.to_string(),
        );
    } else {
        message_box_ok(hwnd, "Connect Test Succeeded", IDS_CONNECT_TEST_OK);
    }
}

fn connect_test_main(hwnd: HWND) -> Result<(), Box<dyn std::error::Error>> {
    let mut dialog_value = DsnDialogValue::new();
    update_dialog_value(hwnd, &mut dialog_value)?;

    let mut connection_option = ConnectionOption::new();

    let endpoint = dialog_value.endpoint();
    connection_option.set_endpoint_url(endpoint)?;

    use DsnDialogCredentialRadio::*;
    let credential = match dialog_value.credential_radio() {
        Nothing | Null => Credential::null(),
        UserPassword => {
            Credential::from_user_password(dialog_value.user(), Some(dialog_value.password()))
        }
        AuthToken => Credential::from_auth_token(dialog_value.auth_token()),
        File => Credential::load(dialog_value.credential_file())?,
    };
    connection_option.set_credential(credential);

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let timeout = Duration::from_secs(30);
    let session = runtime.block_on(Session::connect_for(&connection_option, timeout))?;
    runtime.block_on(session.close())?;

    Ok(())
}

fn update_dialog_value(
    hwnd: HWND,
    dialog_value: &mut DsnDialogValue,
) -> Result<(), Box<dyn std::error::Error>> {
    dialog_value.set_data_source_name(get_text_value(hwnd, IDC_DATA_SOURCE_NAME));
    dialog_value.set_endpoint(get_text_value(hwnd, IDC_ENDPOINT));
    dialog_value.set_user(get_text_value(hwnd, IDC_USER));
    dialog_value.set_password(get_text_value(hwnd, IDC_PASSWORD));
    dialog_value.set_auth_token(get_text_value(hwnd, IDC_AUTH_TOKEN));
    dialog_value.set_credential_file(get_text_value(hwnd, IDC_CREDENTIALS));

    let radio = if is_radio_checked(hwnd, IDC_CREDENTIAL_NO_AUTH) {
        DsnDialogCredentialRadio::Null
    } else if is_radio_checked(hwnd, IDC_CREDENTIAL_USER_PASSWORD) {
        DsnDialogCredentialRadio::UserPassword
    } else if is_radio_checked(hwnd, IDC_CREDENTIAL_AUTH_TOKEN) {
        DsnDialogCredentialRadio::AuthToken
    } else if is_radio_checked(hwnd, IDC_CREDENTIAL_FILE) {
        DsnDialogCredentialRadio::File
    } else {
        DsnDialogCredentialRadio::Nothing
    };
    dialog_value.set_credential_radio(radio);

    Ok(())
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

fn is_radio_checked(hwnd: HWND, id: i32) -> bool {
    let checked = unsafe { IsDlgButtonChecked(hwnd, id) };
    checked == BST_CHECKED.0
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

fn message_box_exclamation(hwnd: HWND, title: &str, message_id: u32, cause_message: String) {
    let title = string_to_utf16(title);
    let message = get_resource_string(message_id);
    let message = format!("{}\r\n\r\n{}", utf16_to_string(&message), cause_message);
    let message = string_to_utf16(&message);

    unsafe {
        MessageBoxW(
            Some(hwnd),
            PCWSTR(message.as_ptr()),
            PCWSTR(title.as_ptr()),
            MB_OK | MB_ICONEXCLAMATION,
        );
    }
}

fn message_box_yesno(hwnd: HWND, title: &str, message_id: u32) -> bool {
    let title = string_to_utf16(title);
    let message = get_resource_string(message_id);

    let rc = unsafe {
        MessageBoxW(
            Some(hwnd),
            PCWSTR(message.as_ptr()),
            PCWSTR(title.as_ptr()),
            MB_YESNO | MB_ICONQUESTION | MB_DEFBUTTON2,
        )
    };
    matches!(rc, IDYES)
}

fn message_box_ok(hwnd: HWND, title: &str, message_id: u32) {
    let title = string_to_utf16(title);
    let message = get_resource_string(message_id);

    unsafe {
        MessageBoxW(
            Some(hwnd),
            PCWSTR(message.as_ptr()),
            PCWSTR(title.as_ptr()),
            MB_OK | MB_ICONINFORMATION,
        );
    }
}

fn get_resource_string(message_id: u32) -> [u16; 256] {
    let mut buffer = [0u16; 256];
    unsafe {
        let module = get_current_module().unwrap_or_default();
        LoadStringW(
            Some(module.into()),
            message_id,
            PWSTR(buffer.as_mut_ptr()),
            buffer.len() as i32,
        );
    }
    buffer
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
