package com.tsurugidb.tsubakuro.rust.odbc.handle;

import java.lang.foreign.MemorySegment;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcFunction;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlReturn;

public class TgOdbcEnvHandle extends TgOdbcHandle {

    public static TgOdbcEnvHandle allocEnvHandle(TgOdbcManager manager) {
        return sqlAllocHandle(manager, HandleType.SQL_HANDLE_ENV, null, TgOdbcEnvHandle::new);
    }

    public TgOdbcEnvHandle(TgOdbcManager manager, MemorySegment handleAddress) {
        super(manager, handleAddress);
    }

    @Override
    protected HandleType handleType() {
        return HandleType.SQL_HANDLE_ENV;
    }

    public void setEnvAttr(int attribute, long value) {
        MemorySegment henv = handleAddress();
        MemorySegment valuePtr = MemorySegment.ofAddress(value);
        try {
            short result = (short) OdbcFunction.sqlSetEnvAttr.invoke(henv, attribute, valuePtr, 0);
            SqlReturn.check("SQLSetEnvAttr", result);
        } catch (RuntimeException e) {
            throw e;
        } catch (Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public TgOdbcDbcHandle allocDbcHandle() {
        return TgOdbcDbcHandle.allocDbcHandle(this);
    }
}
