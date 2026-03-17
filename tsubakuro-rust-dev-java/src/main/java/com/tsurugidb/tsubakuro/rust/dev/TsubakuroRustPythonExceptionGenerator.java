package com.tsurugidb.tsubakuro.rust.dev;

import java.util.ArrayList;
import java.util.Map;

import com.tsurugidb.tsubakuro.sql.SqlServiceCode;
import com.tsurugidb.tsubakuro.sql.SqlServiceException;

public class TsubakuroRustPythonExceptionGenerator {

    public static void main(String[] args) {
        sqlServiceException();
    }

    static void sqlServiceException() {
        System.out.println("---- tsubakuro-rust-python/src/error.rs ----");
        System.out.println("// SqlServiceException");

        var parentMap = Map.of( //
                "ConstraintViolationException", "IntegrityError", //
                "CompileException", "ProgrammingError", //
                "ParameterException", "ProgrammingError", //
                "EvaluationException", "DataError", //
                "SqlLimitReachedException", "DataError", //
                "UnsupportedCompilerFeatureException", "NotSupportedError", //
                "UnsupportedRuntimeFeatureException", "NotSupportedError", //
                "InternalException", "InternalError" //
        );

        for (var code : SqlServiceCode.values()) {
            var exception = SqlServiceException.of(code);
            String simpleName = exception.getClass().getSimpleName();
            String parent;
            {
                parent = parentMap.get(simpleName);
                if (parent == null) {
                    parent = exception.getClass().getSuperclass().getSimpleName();
                }
            }
            String doc = "Tsurugi %s".formatted(simpleName);
            System.out.printf("""
                    create_exception!(tsurugi_dbapi.error, %s, %s, "%s");
                    """, simpleName, parent, doc);
        }

        System.out.println("---- server_error_to_pyerr() ----");
        System.out.println("// SqlServiceException");

        for (var code : SqlServiceCode.values()) {
            var exception = SqlServiceException.of(code);
            System.out.printf("\"%s\" => return server_error!(%s, code, message),%n", code.getStructuredCode(),
                    exception.getClass().getSimpleName());
        }

        System.out.println("---- tsubakuro-rust-python/src/error.rs export ----");
        System.out.println("// SqlServiceException");
        var list = new ArrayList<String>();
        for (var code : SqlServiceCode.values()) {
            var exception = SqlServiceException.of(code);
            list.add(exception.getClass().getSimpleName());
        }
        System.out.println("#[pymodule_export]");
        System.out.printf("use super::{ %s };%n", String.join(", ", list));
    }
}
