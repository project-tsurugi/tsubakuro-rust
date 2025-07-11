package com.tsurugidb.tsubakuro.rust.odbc.handle;

import java.lang.foreign.AddressLayout;
import java.lang.foreign.MemorySegment;
import java.util.function.BiFunction;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcResource;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcFunction;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlReturn;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDiagFieldArgument.DiagIdentifier;

public abstract class TgOdbcHandle extends TgOdbcResource {
    private static final Logger LOG = LoggerFactory.getLogger(TgOdbcHandle.class);

    protected static <T extends TgOdbcHandle> T sqlAllocHandle(TgOdbcManager manager, HandleType handleType, TgOdbcHandle handle, BiFunction<TgOdbcManager, MemorySegment, T> outputHandleGenerator) {
        short handleTypeValue = handleType.value();
        MemorySegment handleAddress = (handle != null) ? handle.handleAddress() : MemorySegment.NULL;
        MemorySegment outputHandlePtr = manager.allocateAddress();
        try {
            short rc = (short) OdbcFunction.sqlAllocHandle.invoke(handleTypeValue, handleAddress, outputHandlePtr);
            if (handle == null) {
                SqlReturn.check("SQLAllocHandle", rc);
            } else {
                SqlReturn.check("SQLAllocHandle", rc, handle);
            }

            MemorySegment address = outputHandlePtr.get(AddressLayout.ADDRESS, 0);
            LOG.debug("SQLAllocHandle({})={}", handleType, address);
            T outputHandle = outputHandleGenerator.apply(manager, address);
            manager.add(outputHandle);

            return outputHandle;
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    private MemorySegment handleAddress;

    public TgOdbcHandle(TgOdbcManager manager, MemorySegment handleAddress) {
        super(manager);
        this.handleAddress = handleAddress;
    }

    protected abstract HandleType handleType();

    public MemorySegment handleAddress() {
        return this.handleAddress;
    }

    public TgOdbcDiagRec getDiagRec(int recNumber) {
        return getDiagRec(recNumber, true);
    }

    public TgOdbcDiagRec getDiagRec(int recNumber, boolean wideChar) {
        var arg = TgOdbcDiagRecArgument.of(manager, 1024, wideChar);

        short rc = getDiagRec0(recNumber, arg);
        if (rc == SqlReturn.SQL_NO_DATA) {
            return null;
        }
        SqlReturn.check(wideChar ? "SQLGetDiagRecW" : "SQLGetDiagRecA", rc);

        return new TgOdbcDiagRec((short) recNumber, arg.sqlState(), arg.nativeError(), arg.messageText());
    }

    private short getDiagRec0(int recNumber, TgOdbcDiagRecArgument arg) {
        short handleType = handleType().value();
        short recNumberShort = (short) recNumber;
        MemorySegment handle = handleAddress();
        try {
            if (arg.wideChar()) {
                return (short) OdbcFunction.sqlGetDiagRecW.invoke(handleType, handle, recNumberShort, arg.sqlStatePtr(), arg.nativeErrorPtr(), arg.messageTextPtr(), arg.bufferLength(),
                        arg.textLengthPtr());
            } else {
                return (short) OdbcFunction.sqlGetDiagRecA.invoke(handleType, handle, recNumberShort, arg.sqlStatePtr(), arg.nativeErrorPtr(), arg.messageTextPtr(), arg.bufferLength(),
                        arg.textLengthPtr());
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public Integer getDiagFieldInteger(int recNumber, DiagIdentifier diagIdentifier, boolean wideChar) {
        var arg = TgOdbcDiagFieldArgument.ofInteger(manager, diagIdentifier);

        short rc = getDiagField0(recNumber, arg, wideChar);
        if (rc == SqlReturn.SQL_NO_DATA) {
            return null;
        }
        SqlReturn.check(wideChar ? "SQLGetDiagFieldW" : "SQLGetDiagFieldA", rc);

        return arg.diagInfoInteger();
    }

    private short getDiagField0(int recNumber, TgOdbcDiagFieldArgument arg, boolean wideChar) {
        short handleType = handleType().value();
        MemorySegment handle = handleAddress();
        short recNumberShort = (short) recNumber;

        try {
            if (wideChar) {
                return (short) OdbcFunction.sqlGetDiagFieldW.invoke(handleType, handle, recNumberShort, arg.diagIdentifier().value(), arg.diagInfoPtr(), arg.bufferLength(), arg.stringLengthPtr());
            } else {
                return (short) OdbcFunction.sqlGetDiagFieldA.invoke(handleType, handle, recNumberShort, arg.diagIdentifier().value(), arg.diagInfoPtr(), arg.bufferLength(), arg.stringLengthPtr());
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public enum CompletionType {
        SQL_COMMIT(0), SQL_ROLLBACK(1);

        private final short value;

        CompletionType(int value) {
            this.value = (short) value;
        }

        public short value() {
            return this.value;
        }
    }

    public void endTran(CompletionType completionType) {
        short handleType = handleType().value();
        MemorySegment handle = handleAddress();
        short completionTypeValue = completionType.value();
        try {
            short rc = (short) OdbcFunction.sqlEndTran.invoke(handleType, handle, completionTypeValue);
            SqlReturn.check("SQLEndTran", rc, this);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public void close() {
        var address = this.handleAddress;
        if (address == null) {
            return;
        }
        this.handleAddress = null;

        short handleType = handleType().value();
        try {
            short result = (short) OdbcFunction.sqlFreeHandle.invoke(handleType, address);
            SqlReturn.check("SQLFreeHandle", result);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public String toString() {
        return getClass().getSimpleName() + "{handleAddress=" + handleAddress + "}";
    }
}
