package com.tsurugidb.tsubakuro.rust.java.session;

import java.lang.foreign.MemorySegment;
import java.nio.file.Path;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiCredential extends TgFfiObject {

    public static TgFfiCredential nullCredential(TgFfiContext context) {
        Objects.requireNonNull(context, "context must not be null");
        return nullCredential(context.manager(), context);
    }

    public static TgFfiCredential nullCredential(TgFfiObjectManager manager) {
        return nullCredential(manager, null);
    }

    public static TgFfiCredential nullCredential(TgFfiObjectManager manager, TgFfiContext context) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return nullCredentialMain(manager, context);
            }
        } else {
            return nullCredentialMain(manager, null);
        }
    }

    private static TgFfiCredential nullCredentialMain(TgFfiObjectManager manager, TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_credential_null(ctx, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiCredential(manager, outHandle);
    }

    public static TgFfiCredential fromUserPassword(TgFfiContext context, String user, String password) {
        Objects.requireNonNull(context, "context must not be null");
        return fromUserPassword(context.manager(), context, user, password);
    }

    public static TgFfiCredential fromUserPassword(TgFfiObjectManager manager, String user, String password) {
        return fromUserPassword(manager, null, user, password);
    }

    public static TgFfiCredential fromUserPassword(TgFfiObjectManager manager, TgFfiContext context, String user, String password) {
        Objects.requireNonNull(manager, "manager must not be null");
        Objects.requireNonNull(user, "user must not be null");

        if (context != null) {
            synchronized (context) {
                return fromUserPasswordMain(manager, context, user, password);
            }
        } else {
            return fromUserPasswordMain(manager, null, user, password);
        }
    }

    private static TgFfiCredential fromUserPasswordMain(TgFfiObjectManager manager, TgFfiContext context, String user, String password) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(user);
        var arg2 = manager.allocateString(password);
        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_credential_from_user_password(ctx, arg1, arg2, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiCredential(manager, outHandle);
    }

    public static TgFfiCredential fromAuthToken(TgFfiContext context, String token) {
        Objects.requireNonNull(context, "context must not be null");
        return fromAuthToken(context.manager(), context, token);
    }

    public static TgFfiCredential fromAuthToken(TgFfiObjectManager manager, String token) {
        return fromAuthToken(manager, null, token);
    }

    public static TgFfiCredential fromAuthToken(TgFfiObjectManager manager, TgFfiContext context, String token) {
        Objects.requireNonNull(manager, "manager must not be null");
        Objects.requireNonNull(token, "token must not be null");

        if (context != null) {
            synchronized (context) {
                return fromAuthTokenMain(manager, context, token);
            }
        } else {
            return fromAuthTokenMain(manager, null, token);
        }
    }

    private static TgFfiCredential fromAuthTokenMain(TgFfiObjectManager manager, TgFfiContext context, String token) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg = manager.allocateString(token);
        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_credential_from_auth_token(ctx, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiCredential(manager, outHandle);
    }

    public static TgFfiCredential load(TgFfiContext context, Path path) {
        Objects.requireNonNull(context, "context must not be null");
        return load(context.manager(), context, path);
    }

    public static TgFfiCredential load(TgFfiObjectManager manager, Path path) {
        return load(manager, null, path);
    }

    public static TgFfiCredential load(TgFfiObjectManager manager, TgFfiContext context, Path path) {
        Objects.requireNonNull(manager, "manager must not be null");
        Objects.requireNonNull(path, "path must not be null");

        if (context != null) {
            synchronized (context) {
                return loadMain(manager, context, path);
            }
        } else {
            return loadMain(manager, null, path);
        }
    }

    private static TgFfiCredential loadMain(TgFfiObjectManager manager, TgFfiContext context, Path path) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg = manager.allocateString(path.toString());
        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_credential_load(ctx, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiCredential(manager, outHandle);
    }

    TgFfiCredential(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_credential_dispose(handle);
    }
}
