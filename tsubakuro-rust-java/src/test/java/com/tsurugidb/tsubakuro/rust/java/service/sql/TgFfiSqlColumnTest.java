package com.tsurugidb.tsubakuro.rust.java.service.sql;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.lang.foreign.MemorySegment;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiSqlColumnTest extends TgFfiTester {

    @Test
    void int4() {
        var tester = new TestMain("int", TgFfiAtomType.INT4);
        tester.test();
    }

    @Test
    void int8() {
        var tester = new TestMain("bigint", TgFfiAtomType.INT8);
        tester.test();
    }

    @Test
    void float4() {
        var tester = new TestMain("real", TgFfiAtomType.FLOAT4);
        tester.test();
    }

    @Test
    void float8() {
        var tester = new TestMain("double", TgFfiAtomType.FLOAT8);
        tester.test();
    }

    @Test
    void decimal() {
        String sqlType = "decimal";
        var atomType = TgFfiAtomType.DECIMAL;
        ArbitraryInt precision = null;
        ArbitraryInt scale = null;
        var tester = new TestMain(sqlType, atomType, precision, scale) {
            @Override
            protected void assertTableColumn(TgFfiContext context, TgFfiSqlColumn c) {
                assertNull(c.getLength(context));
                assertEquals(ArbitraryInt.of(38), c.getPrecision(context));
                assertEquals(ArbitraryInt.of(0), c.getScale(context));
                assertNull(c.getVarying(context));
                assertNull(c.getDescription(context));
            }
        };
        tester.test();
    }

    @Test
    void decimal10() {
        String sqlType = "decimal(10)";
        var atomType = TgFfiAtomType.DECIMAL;
        ArbitraryInt precision = ArbitraryInt.of(10);
        ArbitraryInt scale = null;
        var tester = new TestMain(sqlType, atomType, precision, scale) {
            @Override
            protected void assertTableColumn(TgFfiContext context, TgFfiSqlColumn c) {
                assertNull(c.getLength(context));
                assertEquals(ArbitraryInt.of(10), c.getPrecision(context));
                assertEquals(ArbitraryInt.of(0), c.getScale(context));
                assertNull(c.getVarying(context));
                assertNull(c.getDescription(context));
            }
        };
        tester.test();
    }

    @Test
    void decimal10_2() {
        String sqlType = "decimal(10,2)";
        var atomType = TgFfiAtomType.DECIMAL;
        ArbitraryInt precision = ArbitraryInt.of(10);
        ArbitraryInt scale = ArbitraryInt.of(2);
        var tester = new TestMain(sqlType, atomType, precision, scale) {
            @Override
            protected void assertTableColumn(TgFfiContext context, TgFfiSqlColumn c) {
                assertNull(c.getLength(context));
                assertEquals(ArbitraryInt.of(10), c.getPrecision(context));
                assertEquals(ArbitraryInt.of(2), c.getScale(context));
                assertNull(c.getVarying(context));
                assertNull(c.getDescription(context));
            }
        };
        tester.test();
    }

    @Test
    void decimalA() {
        String sqlType = "decimal(*)";
        var atomType = TgFfiAtomType.DECIMAL;
        ArbitraryInt precision = ArbitraryInt.ofArbitrary();
        ArbitraryInt scale = null;
        var tester = new TestMain(sqlType, atomType, precision, scale) {
            @Override
            protected void assertTableColumn(TgFfiContext context, TgFfiSqlColumn c) {
                assertNull(c.getLength(context));
                assertEquals(ArbitraryInt.of(38), c.getPrecision(context));
                assertEquals(ArbitraryInt.of(0), c.getScale(context));
                assertNull(c.getVarying(context));
                assertNull(c.getDescription(context));
            }
        };
        tester.test();
    }

    @Test
    void decimalA_2() {
        String sqlType = "decimal(*,2)";
        var atomType = TgFfiAtomType.DECIMAL;
        ArbitraryInt precision = ArbitraryInt.ofArbitrary();
        ArbitraryInt scale = ArbitraryInt.of(2);
        var tester = new TestMain(sqlType, atomType, precision, scale) {
            @Override
            protected void assertTableColumn(TgFfiContext context, TgFfiSqlColumn c) {
                assertNull(c.getLength(context));
                assertEquals(ArbitraryInt.of(38), c.getPrecision(context));
                assertEquals(ArbitraryInt.of(2), c.getScale(context));
                assertNull(c.getVarying(context));
                assertNull(c.getDescription(context));
            }
        };
        tester.test();
    }

    @Test
    void character() {
        String sqlType = "char";
        var atomType = TgFfiAtomType.CHARACTER;
        ArbitraryInt length = null;
        Boolean varying = false;
        var tester = new TestMain(sqlType, atomType, length, varying) {
            @Override
            protected void assertTableColumn(TgFfiContext context, TgFfiSqlColumn c) {
                assertEquals(ArbitraryInt.of(1), c.getLength(context));
                assertNull(c.getPrecision(context));
                assertNull(c.getScale(context));
                assertFalse(c.getVarying(context));
                assertNull(c.getDescription(context));
            }
        };
        tester.test();
    }

    @Test
    void character10() {
        String sqlType = "char(10)";
        var atomType = TgFfiAtomType.CHARACTER;
        ArbitraryInt length = ArbitraryInt.of(10);
        Boolean varying = false;
        var tester = new TestMain(sqlType, atomType, length, varying) {
            @Override
            protected void assertTableColumn(TgFfiContext context, TgFfiSqlColumn c) {
                assertEquals(ArbitraryInt.of(10), c.getLength(context));
                assertNull(c.getPrecision(context));
                assertNull(c.getScale(context));
                assertFalse(c.getVarying(context));
                assertNull(c.getDescription(context));
            }
        };
        tester.test();
    }

    @Test
    void varchar() {
        String sqlType = "varchar";
        var atomType = TgFfiAtomType.CHARACTER;
        ArbitraryInt length = null;
        Boolean varying = true;
        var tester = new TestMain(sqlType, atomType, length, varying) {
            @Override
            protected void assertTableColumn(TgFfiContext context, TgFfiSqlColumn c) {
                assertEquals(ArbitraryInt.ofArbitrary(), c.getLength(context));
                assertNull(c.getPrecision(context));
                assertNull(c.getScale(context));
                assertTrue(c.getVarying(context));
                assertNull(c.getDescription(context));
            }
        };
        tester.test();
    }

    @Test
    void varchar10() {
        String sqlType = "varchar(10)";
        var atomType = TgFfiAtomType.CHARACTER;
        ArbitraryInt length = ArbitraryInt.of(10);
        Boolean varying = true;
        var tester = new TestMain(sqlType, atomType, length, varying) {
            @Override
            protected void assertTableColumn(TgFfiContext context, TgFfiSqlColumn c) {
                assertEquals(ArbitraryInt.of(10), c.getLength(context));
                assertNull(c.getPrecision(context));
                assertNull(c.getScale(context));
                assertTrue(c.getVarying(context));
                assertNull(c.getDescription(context));
            }
        };
        tester.test();
    }

    @Test
    void varcharA() {
        String sqlType = "varchar(*)";
        var atomType = TgFfiAtomType.CHARACTER;
        ArbitraryInt length = ArbitraryInt.ofArbitrary();
        Boolean varying = true;
        var tester = new TestMain(sqlType, atomType, length, varying) {
            @Override
            protected void assertTableColumn(TgFfiContext context, TgFfiSqlColumn c) {
                assertEquals(ArbitraryInt.ofArbitrary(), c.getLength(context));
                assertNull(c.getPrecision(context));
                assertNull(c.getScale(context));
                assertTrue(c.getVarying(context));
                assertNull(c.getDescription(context));
            }
        };
        tester.test();
    }

    @SuppressWarnings("unused")
    private class TestMain {
        private final String sqlType;
        private final TgFfiAtomType atomType;
        private ArbitraryInt length;
        private ArbitraryInt precision;
        private ArbitraryInt scale;
        private Boolean varying;

        private TestMain(String sqlType, TgFfiAtomType atomType) {
            this.sqlType = sqlType;
            this.atomType = atomType;
        }

        public TestMain(String sqlType, TgFfiAtomType atomType, ArbitraryInt precision, ArbitraryInt scale) {
            this.sqlType = sqlType;
            this.atomType = atomType;
            this.precision = precision;
            this.scale = scale;
        }

        public TestMain(String sqlType, TgFfiAtomType atomType, ArbitraryInt length, Boolean varying) {
            this.sqlType = sqlType;
            this.atomType = atomType;
            this.length = length;
            this.varying = varying;
        }

        public void test() {
            test(sqlType, null);
            test(sqlType + " null", true);
            test(sqlType + " not null", false);
        }

        private void test(String columnDefinition, Boolean nullable) {
            dropAndCreateTable("test", """
                    create table test (
                      pk int primary key,
                      value %s
                    )""".formatted(columnDefinition));

            testTableMetadata(nullable);
            testResultSetMetadata(nullable);
        }

        private void testTableMetadata(Boolean nullable) {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager); //
                    var client = createSqlClient()) {
                try (var metadata = client.getTableMetadata(context, "test")) {
                    assertNull(metadata.getDescription(context));

                    var columns = metadata.getColumns(context);
                    assertEquals(2, columns.size());
                    {
                        var c = columns.get(0);
                        assertEquals("pk", c.getName(context));
                        assertEquals(TgFfiAtomType.INT4, c.getAtomType(context));
                        assertFalse(c.getNullable(context));

                        testArgError(c);
                    }
                    {
                        var c = columns.get(1);
                        assertEquals("value", c.getName(context));
                        assertEquals(atomType, c.getAtomType(context));
                        assertTableColumn(context, c);
                        if (nullable == null) {
                            assertTrue(c.getNullable(context));
                        } else {
                            assertEquals(nullable, c.getNullable(context));
                        }
                    }
                }
            }
        }

        protected void assertTableColumn(TgFfiContext context, TgFfiSqlColumn c) {
            assertNull(c.getLength(context));
            assertNull(c.getPrecision(context));
            assertNull(c.getScale(context));
            assertNull(c.getVarying(context));
            assertNull(c.getDescription(context));
        }

        private void testResultSetMetadata(Boolean nullable) {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager); //
                    var client = createSqlClient()) {
                try (var transaction = startOcc(client); //
                        var qr = client.query(context, transaction, "select * from test")) {
                    var metadata = qr.getMetadata(context);
                    var columns = metadata.getColumns(context);
                    assertEquals(2, columns.size());
                    {
                        var c = columns.get(0);
                        assertEquals("pk", c.getName(context));
                        assertEquals(TgFfiAtomType.INT4, c.getAtomType(context));
                    }
                    {
                        var c = columns.get(1);
                        assertEquals("value", c.getName(context));
                        assertEquals(atomType, c.getAtomType(context));
                        assertNull(c.getLength(context));
                        assertNull(c.getPrecision(context));
                        assertNull(c.getScale(context));
                        assertNull(c.getNullable(context));
                        assertNull(c.getVarying(context));
                        // TODO assertEquals(length, c.getLength(context));
                        // TODO assertEquals(precision, c.getPrecision(context));
                        // TODO assertEquals(scale, c.getScale(context));
                        // TODO assertEquals(nullable, c.getNullable(context));
                        // TODO assertEquals(varying, c.getVarying(context));
                        assertNull(c.getDescription(context));
                    }
                }
            }
        }
    }

    private boolean done = false;

    private void testArgError(TgFfiSqlColumn c) {
        if (!this.done) {
            this.done = true;

            get_name_argError(c);
            get_atom_type_argError(c);
            get_length_argError(c);
            get_precision_argError(c);
            get_scale_argError(c);
            get_nullable_argError(c);
            get_varying_argError(c);
            get_description_argError(c);
        }
    }

    private void get_name_argError(TgFfiSqlColumn c) {
        var manager = getFfiObjectManager();
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_atom_type_argError(TgFfiSqlColumn c) {
        var manager = getFfiObjectManager();
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_atom_type(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_atom_type(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_length_argError(TgFfiSqlColumn c) {
        var manager = getFfiObjectManager();
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var providedOut = manager.allocateBooleanOut();
            var valueOut = manager.allocateIntOut();
            var arbitraryOut = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_length(ctx, handle, providedOut, valueOut, arbitraryOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var providedOut = MemorySegment.NULL;
            var valueOut = manager.allocateIntOut();
            var arbitraryOut = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_length(ctx, handle, providedOut, valueOut, arbitraryOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var providedOut = manager.allocateBooleanOut();
            var valueOut = MemorySegment.NULL;
            var arbitraryOut = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_length(ctx, handle, providedOut, valueOut, arbitraryOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var providedOut = manager.allocateBooleanOut();
            var valueOut = manager.allocateIntOut();
            var arbitraryOut = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_length(ctx, handle, providedOut, valueOut, arbitraryOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
        }
    }

    private void get_precision_argError(TgFfiSqlColumn c) {
        var manager = getFfiObjectManager();
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var providedOut = manager.allocateBooleanOut();
            var valueOut = manager.allocateIntOut();
            var arbitraryOut = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_precision(ctx, handle, providedOut, valueOut, arbitraryOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var providedOut = MemorySegment.NULL;
            var valueOut = manager.allocateIntOut();
            var arbitraryOut = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_precision(ctx, handle, providedOut, valueOut, arbitraryOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var providedOut = manager.allocateBooleanOut();
            var valueOut = MemorySegment.NULL;
            var arbitraryOut = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_precision(ctx, handle, providedOut, valueOut, arbitraryOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var providedOut = manager.allocateBooleanOut();
            var valueOut = manager.allocateIntOut();
            var arbitraryOut = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_precision(ctx, handle, providedOut, valueOut, arbitraryOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
        }
    }

    private void get_scale_argError(TgFfiSqlColumn c) {
        var manager = getFfiObjectManager();
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var providedOut = manager.allocateBooleanOut();
            var valueOut = manager.allocateIntOut();
            var arbitraryOut = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_scale(ctx, handle, providedOut, valueOut, arbitraryOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var providedOut = MemorySegment.NULL;
            var valueOut = manager.allocateIntOut();
            var arbitraryOut = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_scale(ctx, handle, providedOut, valueOut, arbitraryOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var providedOut = manager.allocateBooleanOut();
            var valueOut = MemorySegment.NULL;
            var arbitraryOut = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_scale(ctx, handle, providedOut, valueOut, arbitraryOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var providedOut = manager.allocateBooleanOut();
            var valueOut = manager.allocateIntOut();
            var arbitraryOut = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_scale(ctx, handle, providedOut, valueOut, arbitraryOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
        }
    }

    private void get_nullable_argError(TgFfiSqlColumn c) {
        var manager = getFfiObjectManager();
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var providedOut = manager.allocateBooleanOut();
            var valueOut = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_nullable(ctx, handle, providedOut, valueOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var providedOut = MemorySegment.NULL;
            var valueOut = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_nullable(ctx, handle, providedOut, valueOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var providedOut = manager.allocateBooleanOut();
            var valueOut = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_nullable(ctx, handle, providedOut, valueOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    private void get_varying_argError(TgFfiSqlColumn c) {
        var manager = getFfiObjectManager();
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var providedOut = manager.allocateBooleanOut();
            var valueOut = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_varying(ctx, handle, providedOut, valueOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var providedOut = MemorySegment.NULL;
            var valueOut = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_varying(ctx, handle, providedOut, valueOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var providedOut = manager.allocateBooleanOut();
            var valueOut = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_varying(ctx, handle, providedOut, valueOut);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    private void get_description_argError(TgFfiSqlColumn c) {
        var manager = getFfiObjectManager();
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_description(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = c.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_description(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }
}
