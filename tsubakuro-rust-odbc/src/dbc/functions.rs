use std::{collections::HashSet, sync::OnceLock};

use log::trace;

use crate::{
    ctype::{SqlReturn, SqlUSmallInt, SQL_FALSE, SQL_TRUE},
    handle::hdbc::HDbc,
};

const SQL_API_ALL_FUNCTIONS: SqlUSmallInt = 0;
const SQL_API_ODBC3_ALL_FUNCTIONS: SqlUSmallInt = 999;
const SQL_API_ODBC3_ALL_FUNCTIONS_SIZE: usize = 250;

const SQL_API_SQLALLOCHANDLE: SqlUSmallInt = 1001;
const SQL_API_SQLBINDCOL: SqlUSmallInt = 4;
const SQL_API_SQLBINDPARAMETER: SqlUSmallInt = 72;
const SQL_API_SQLCOLATTRIBUTE: SqlUSmallInt = 6;
const SQL_API_SQLCOLUMNS: SqlUSmallInt = 40;
const SQL_API_SQLDESCRIBECOL: SqlUSmallInt = 8;
const SQL_API_SQLDISCONNECT: SqlUSmallInt = 9;
const SQL_API_SQLDRIVERCONNECT: SqlUSmallInt = 41;
const SQL_API_SQLENDTRAN: SqlUSmallInt = 1005;
const SQL_API_SQLEXECDIRECT: SqlUSmallInt = 11;
const SQL_API_SQLEXECUTE: SqlUSmallInt = 12;
const SQL_API_SQLFETCH: SqlUSmallInt = 13;
const SQL_API_SQLFREEHANDLE: SqlUSmallInt = 1006;
const SQL_API_SQLGETCONNECTATTR: SqlUSmallInt = 1007;
const SQL_API_SQLGETDATA: SqlUSmallInt = 43;
const SQL_API_SQLGETDIAGREC: SqlUSmallInt = 1011;
const SQL_API_SQLGETENVATTR: SqlUSmallInt = 1012;
const SQL_API_SQLGETFUNCTIONS: SqlUSmallInt = 44;
const SQL_API_SQLGETINFO: SqlUSmallInt = 45;
const SQL_API_SQLGETSTMTATTR: SqlUSmallInt = 1014;
const SQL_API_SQLGETTYPEINFO: SqlUSmallInt = 47;
const SQL_API_SQLNUMRESULTCOLS: SqlUSmallInt = 18;
const SQL_API_SQLPREPARE: SqlUSmallInt = 19;
const SQL_API_SQLPRIMARYKEYS: SqlUSmallInt = 65;
const SQL_API_SQLROWCOUNT: SqlUSmallInt = 20;
const SQL_API_SQLSETCONNECTATTR: SqlUSmallInt = 1016;
const SQL_API_SQLSETENVATTR: SqlUSmallInt = 1019;
const SQL_API_SQLSETSTMTATTR: SqlUSmallInt = 1020;
const SQL_API_SQLTABLES: SqlUSmallInt = 54;

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
    FUNCTION_SET.get_or_init(|| {
        HashSet::from([
            SQL_API_SQLALLOCHANDLE,
            SQL_API_SQLBINDCOL,
            SQL_API_SQLBINDPARAMETER,
            SQL_API_SQLCOLATTRIBUTE,
            SQL_API_SQLCOLUMNS,
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
    })
}
