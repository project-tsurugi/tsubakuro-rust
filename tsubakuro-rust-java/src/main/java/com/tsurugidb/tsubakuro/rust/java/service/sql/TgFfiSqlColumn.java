package com.tsurugidb.tsubakuro.rust.java.service.sql;

import java.lang.foreign.MemorySegment;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiSqlColumn extends TgFfiObject {

	TgFfiSqlColumn(TgFfiObjectManager manager, MemorySegment handle) {
		super(manager, handle);
	}

	public synchronized String getName(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_name(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToString(out);
	}

	public synchronized TgFfiAtomType getAtomType(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_atom_type(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		int outInt = outToInt(out);
		return TgFfiAtomType.forNumber(outInt);
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_dispose(handle);
	}
}
