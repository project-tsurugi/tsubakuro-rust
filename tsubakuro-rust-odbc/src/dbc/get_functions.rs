use std::{collections::HashSet, sync::OnceLock};

use log::trace;

use crate::{
    ctype::{SqlReturn, SqlUSmallInt, SQL_FALSE, SQL_TRUE},
    handle::hdbc::HDbc,
};

const SQL_API_ALL_FUNCTIONS: SqlUSmallInt = 0;
const SQL_API_ODBC3_ALL_FUNCTIONS: SqlUSmallInt = 999;
const SQL_API_ODBC3_ALL_FUNCTIONS_SIZE: usize = 250;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
enum SqlApiFunction {
    SQL_API_SQLALLOCHANDLE = 1001,
    SQL_API_SQLBINDCOL = 4,
    SQL_API_SQLBINDPARAMETER = 72,
    SQL_API_SQLCOLATTRIBUTE = 6,
    SQL_API_SQLCOLUMNS = 40,
    SQL_API_SQLCONNECT = 7,
    SQL_API_SQLDESCRIBECOL = 8,
    SQL_API_SQLDISCONNECT = 9,
    SQL_API_SQLDRIVERCONNECT = 41,
    SQL_API_SQLENDTRAN = 1005,
    SQL_API_SQLEXECDIRECT = 11,
    SQL_API_SQLEXECUTE = 12,
    SQL_API_SQLFETCH = 13,
    SQL_API_SQLFREEHANDLE = 1006,
    SQL_API_SQLGETCONNECTATTR = 1007,
    SQL_API_SQLGETDATA = 43,
    SQL_API_SQLGETDIAGFIELD = 1010,
    SQL_API_SQLGETDIAGREC = 1011,
    SQL_API_SQLGETENVATTR = 1012,
    SQL_API_SQLGETFUNCTIONS = 44,
    SQL_API_SQLGETINFO = 45,
    SQL_API_SQLGETSTMTATTR = 1014,
    SQL_API_SQLGETTYPEINFO = 47,
    SQL_API_SQLNUMRESULTCOLS = 18,
    SQL_API_SQLPREPARE = 19,
    SQL_API_SQLPRIMARYKEYS = 65,
    SQL_API_SQLROWCOUNT = 20,
    SQL_API_SQLSETCONNECTATTR = 1016,
    SQL_API_SQLSETENVATTR = 1019,
    SQL_API_SQLSETSTMTATTR = 1020,
    SQL_API_SQLTABLES = 54,
}

#[no_mangle]
pub extern "system" fn SQLGetFunctions(
    hdbc: HDbc,
    function_id: SqlUSmallInt,
    supported: *mut SqlUSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetFunctions()";
    trace!(
        "{FUNCTION_NAME} start. hdbc={:?}, function_id={:?}, supported={:?}",
        hdbc,
        function_id,
        supported
    );

    if supported.is_null() {
        return SqlReturn::SQL_ERROR;
    }

    let rc = match function_id {
        SQL_API_ALL_FUNCTIONS => get_all_functions(supported),
        SQL_API_ODBC3_ALL_FUNCTIONS => get_odbc3_all_functions(supported),
        _ => exists_function(function_id, supported),
    };

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn get_all_functions(supported: *mut SqlUSmallInt) -> SqlReturn {
    unsafe {
        for i in 0..100 {
            *supported.add(i) = 0;
        }

        let set = get_function_set();
        for id in set {
            if *id < 100 {
                let i = *id as usize;
                *supported.add(i) = 1;
            }
        }
    }

    SqlReturn::SQL_SUCCESS
}

fn get_odbc3_all_functions(supported: *mut SqlUSmallInt) -> SqlReturn {
    unsafe {
        for i in 0..SQL_API_ODBC3_ALL_FUNCTIONS_SIZE {
            *supported.add(i) = 0;
        }

        let set = get_function_set();
        for id in set {
            let i = (*id >> 4) as usize;
            let b = *id & 0xf;

            if i < SQL_API_ODBC3_ALL_FUNCTIONS_SIZE {
                *supported.add(i) |= 1 << b;
            }
        }
    }

    SqlReturn::SQL_SUCCESS
}

fn exists_function(function_id: SqlUSmallInt, supported: *mut SqlUSmallInt) -> SqlReturn {
    let set = get_function_set();
    let exists = set.contains(&function_id);
    unsafe {
        *supported = if exists {
            SQL_TRUE as SqlUSmallInt
        } else {
            SQL_FALSE as SqlUSmallInt
        };
    }

    SqlReturn::SQL_SUCCESS
}

static FUNCTION_SET: OnceLock<HashSet<SqlUSmallInt>> = OnceLock::new();

fn get_function_set() -> &'static HashSet<SqlUSmallInt> {
    use SqlApiFunction::*;
    FUNCTION_SET.get_or_init(|| {
        HashSet::from([
            SQL_API_SQLALLOCHANDLE,
            SQL_API_SQLBINDCOL,
            SQL_API_SQLBINDPARAMETER,
            SQL_API_SQLCOLATTRIBUTE,
            SQL_API_SQLCOLUMNS,
            SQL_API_SQLCONNECT,
            SQL_API_SQLDESCRIBECOL,
            SQL_API_SQLDISCONNECT,
            SQL_API_SQLDRIVERCONNECT,
            SQL_API_SQLENDTRAN,
            SQL_API_SQLEXECDIRECT,
            SQL_API_SQLEXECUTE,
            SQL_API_SQLFETCH,
            SQL_API_SQLFREEHANDLE,
            SQL_API_SQLGETCONNECTATTR,
            SQL_API_SQLGETDATA,
            SQL_API_SQLGETDIAGFIELD,
            SQL_API_SQLGETDIAGREC,
            SQL_API_SQLGETENVATTR,
            SQL_API_SQLGETFUNCTIONS,
            SQL_API_SQLGETINFO,
            SQL_API_SQLGETSTMTATTR,
            SQL_API_SQLGETTYPEINFO,
            SQL_API_SQLNUMRESULTCOLS,
            SQL_API_SQLPREPARE,
            SQL_API_SQLPRIMARYKEYS,
            SQL_API_SQLROWCOUNT,
            SQL_API_SQLSETCONNECTATTR,
            SQL_API_SQLSETENVATTR,
            SQL_API_SQLSETSTMTATTR,
            SQL_API_SQLTABLES,
        ])
        .iter()
        .map(|e| *e as SqlUSmallInt)
        .collect()
    })
}
