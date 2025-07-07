package com.tsurugidb.tsubakuro.rust.odbc.handle;

import java.lang.foreign.AddressLayout;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.util.function.BiFunction;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcResource;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcFunction;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlReturn;

public abstract class TgOdbcHandle extends TgOdbcResource {
    private static final Logger LOG = LoggerFactory.getLogger(TgOdbcHandle.class);

    protected static <T extends TgOdbcHandle> T sqlAllocHandle(TgOdbcManager manager, HandleType handleType, TgOdbcHandle handle, BiFunction<TgOdbcManager, MemorySegment, T> outputHandleGenerator) {
        short handleTypeValue = handleType.value();
        MemorySegment handleAddress = (handle != null) ? handle.handleAddress() : MemorySegment.NULL;
        MemorySegment outputHandlePtr = manager.allocateAddress();
        try {
            short result = (short) OdbcFunction.sqlAllocHandle.invoke(handleTypeValue, handleAddress, outputHandlePtr);
            if (handle == null) {
                SqlReturn.check("SQLAllocHandle", result);
            } else {
                SqlReturn.check("SQLAllocHandle", result, handle);
            }

            MemorySegment address = outputHandlePtr.get(AddressLayout.ADDRESS, 0);
            LOG.debug("SQLAllocHandle({})={}", handleType, address);
            T outputHandle = outputHandleGenerator.apply(manager, address);
            manager.add(outputHandle);

            return outputHandle;
        } catch (RuntimeException e) {
            throw e;
        } catch (Error e) {
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
        return getDiagRecW(recNumber);
    }

    synchronized TgOdbcDiagRec getDiagRecA(int recNumber) {
        short handleType = handleType().value();
        short recNumberShort = (short) recNumber;
        MemorySegment handle = handleAddress();
        MemorySegment sqlStatePtr = manager.allocateBytes(6);
        MemorySegment nativeErrorPtr = manager.allocateInt();
        short bufferSize = 1024;
        MemorySegment messagePtr = manager.allocateBytes(bufferSize);
        MemorySegment messageLengthPtr = manager.allocateShort();
        try {
            short result = (short) OdbcFunction.sqlGetDiagRecA.invoke(handleType, handle, recNumberShort, sqlStatePtr, nativeErrorPtr, messagePtr, bufferSize, messageLengthPtr);
            if (result == SqlReturn.SQL_NO_DATA) {
                return null;
            }
            SqlReturn.check("SQLGetDiagRecA", result);

            String sqlState = TgOdbcManager.stringFromUtf8(sqlStatePtr);
            int nativeError = nativeErrorPtr.get(ValueLayout.JAVA_INT, 0);
            String message = TgOdbcManager.stringFromUtf8(messagePtr, messageLengthPtr);
            return new TgOdbcDiagRec(recNumberShort, sqlState, nativeError, message);
        } catch (RuntimeException e) {
            throw e;
        } catch (Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    synchronized TgOdbcDiagRec getDiagRecW(int recNumber) {
        short handleType = handleType().value();
        short recNumberShort = (short) recNumber;
        MemorySegment handle = handleAddress();
        MemorySegment sqlStatePtr = manager.allocateBytes(6 * 2);
        MemorySegment nativeErrorPtr = manager.allocateInt();
        short bufferSize = 1024;
        MemorySegment messagePtr = manager.allocateBytes(bufferSize * 2);
        MemorySegment messageLengthPtr = manager.allocateShort();
        try {
            short result = (short) OdbcFunction.sqlGetDiagRecW.invoke(handleType, handle, recNumberShort, sqlStatePtr, nativeErrorPtr, messagePtr, bufferSize, messageLengthPtr);
            if (result == SqlReturn.SQL_NO_DATA) {
                return null;
            }
            SqlReturn.check("SQLGetDiagRecW", result);

            String sqlState = TgOdbcManager.stringFromUtf16(sqlStatePtr);
            int nativeError = nativeErrorPtr.get(ValueLayout.JAVA_INT, 0);
            String message = TgOdbcManager.stringFromUtf16(messagePtr, messageLengthPtr);
            return new TgOdbcDiagRec(recNumberShort, sqlState, nativeError, message);
        } catch (RuntimeException e) {
            throw e;
        } catch (Error e) {
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
            short result = (short) OdbcFunction.sqlEndTran.invoke(handleType, handle, completionTypeValue);
            SqlReturn.check("SQLEndTran", result, this);
        } catch (RuntimeException e) {
            throw e;
        } catch (Error e) {
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
        } catch (RuntimeException e) {
            throw e;
        } catch (Error e) {
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
