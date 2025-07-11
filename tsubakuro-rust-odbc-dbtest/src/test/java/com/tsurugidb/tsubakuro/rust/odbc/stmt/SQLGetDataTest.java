package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.odbc.api.SqlReturn;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLGetDataTest extends TgOdbcTester {

    @Test
    void binary_length() {
        dropAndCreateTable("test", "create table test (value varbinary(10))");
        try (var stmt = createStmt()) {
            stmt.execDirect("insert into test values(X'12345678')", false);
        }

        for (int bufferLength = 4; bufferLength <= 5; bufferLength++) {
            try (var stmt = createStmt()) {
                var manager = stmt.manager();

                stmt.execDirect("select * from test", false);
                assertTrue(stmt.fetch());

                var arg = TgOdbcGetDataArgument.ofBinary(manager, bufferLength);
                short rc = stmt.getData0(1, arg);
                assertEquals(SqlReturn.SQL_SUCCESS, rc);
                assertEquals(4, arg.lengthOrInd());
                assertArrayEquals(new byte[] { 0x12, 0x34, 0x56, 0x78 }, arg.getData());
            }
        }
        try (var stmt = createStmt()) {
            var manager = stmt.manager();

            stmt.execDirect("select * from test", false);
            assertTrue(stmt.fetch());

            var arg = TgOdbcGetDataArgument.ofBinary(manager, 3);
            short rc = stmt.getData0(1, arg);
            assertEquals(SqlReturn.SQL_SUCCESS_WITH_INFO, rc);
            assertEquals(4, arg.lengthOrInd());
            assertArrayEquals(new byte[] { 0x12, 0x34, 0x56 }, arg.getData());

            var rec = stmt.getDiagRec(1);
            assertEquals("01004", rec.sqlState());
        }
    }

    @Test
    void char_length() {
        boolean wideChar = false;

        dropAndCreateTable("test", "create table test (value varchar(10))");
        try (var stmt = createStmt()) {
            stmt.execDirect("insert into test values('abc')", wideChar);
        }

        for (int bufferLength = 4; bufferLength <= 5; bufferLength++) {
            try (var stmt = createStmt()) {
                var manager = stmt.manager();

                stmt.execDirect("select * from test", wideChar);
                assertTrue(stmt.fetch());

                var arg = TgOdbcGetDataArgument.ofString(manager, bufferLength, wideChar);
                short rc = stmt.getData0(1, arg);
                assertEquals(SqlReturn.SQL_SUCCESS, rc);
                assertEquals(3, arg.lengthOrInd());
                assertEquals("abc", arg.getData());
            }
        }
        try (var stmt = createStmt()) {
            var manager = stmt.manager();

            stmt.execDirect("select * from test", wideChar);
            assertTrue(stmt.fetch());

            var arg = TgOdbcGetDataArgument.ofString(manager, 3, wideChar);
            short rc = stmt.getData0(1, arg);
            assertEquals(SqlReturn.SQL_SUCCESS_WITH_INFO, rc);
            assertEquals(3, arg.lengthOrInd());
//          assertEquals("ab", arg.getData());

            var rec = stmt.getDiagRec(1);
            assertEquals("01004", rec.sqlState());
        }
    }

    @Test
    @Disabled // 文字化けする模様
    void char_utf8_length() {
        boolean wideChar = false;

        dropAndCreateTable("test", "create table test (value varchar(20))");
        try (var stmt = createStmt()) {
            stmt.execDirect("insert into test values('あいう')", wideChar);
        }

        for (int bufferLength = 10; bufferLength <= 11; bufferLength++) {
            try (var stmt = createStmt()) {
                var manager = stmt.manager();

                stmt.execDirect("select * from test", false);
                assertTrue(stmt.fetch());

                var arg = TgOdbcGetDataArgument.ofString(manager, bufferLength, wideChar);
                short rc = stmt.getData0(1, arg);
                assertEquals(SqlReturn.SQL_SUCCESS, rc);
                assertEquals(9, arg.lengthOrInd());
                assertEquals("あいう", arg.getData());
            }
        }
        try (var stmt = createStmt()) {
            var manager = stmt.manager();

            stmt.execDirect("select * from test", wideChar);
            assertTrue(stmt.fetch());

            var arg = TgOdbcGetDataArgument.ofString(manager, 9, wideChar);
            short rc = stmt.getData0(1, arg);
            assertEquals(SqlReturn.SQL_SUCCESS_WITH_INFO, rc);
            assertEquals(9, arg.lengthOrInd());
//          assertEquals("あいう", arg.getData());

            var rec = stmt.getDiagRec(1);
            assertEquals("01004", rec.sqlState());
        }
    }

    @Test
    void wchar_length() {
        boolean wideChar = true;

        dropAndCreateTable("test", "create table test (value varchar(10))");
        try (var stmt = createStmt()) {
            stmt.execDirect("insert into test values('abc')", wideChar);
        }

        for (int bufferLength = 8; bufferLength <= 10; bufferLength += 2) {
            try (var stmt = createStmt()) {
                var manager = stmt.manager();

                stmt.execDirect("select * from test", wideChar);
                assertTrue(stmt.fetch());

                var arg = TgOdbcGetDataArgument.ofString(manager, bufferLength, wideChar);
                short rc = stmt.getData0(1, arg);
                assertEquals(SqlReturn.SQL_SUCCESS, rc);
                assertEquals(6, arg.lengthOrInd());
                assertEquals("abc", arg.getData());
            }
        }
        try (var stmt = createStmt()) {
            var manager = stmt.manager();

            stmt.execDirect("select * from test", wideChar);
            assertTrue(stmt.fetch());

            var arg = TgOdbcGetDataArgument.ofString(manager, 6, wideChar);
            short rc = stmt.getData0(1, arg);
            assertEquals(SqlReturn.SQL_SUCCESS_WITH_INFO, rc);
            assertEquals(6, arg.lengthOrInd());
//          assertEquals("ab", arg.getData());

            var rec = stmt.getDiagRec(1);
            assertEquals("01004", rec.sqlState());
        }
    }

    @Test
    void wchar_utf16_length() {
        boolean wideChar = true;

        dropAndCreateTable("test", "create table test (value varchar(20))");
        try (var stmt = createStmt()) {
            stmt.execDirect("insert into test values('あいう')", wideChar);
        }

        for (int bufferLength = 8; bufferLength <= 10; bufferLength += 2) {
            try (var stmt = createStmt()) {
                var manager = stmt.manager();

                stmt.execDirect("select * from test", wideChar);
                assertTrue(stmt.fetch());

                var arg = TgOdbcGetDataArgument.ofString(manager, bufferLength, wideChar);
                short rc = stmt.getData0(1, arg);
                assertEquals(SqlReturn.SQL_SUCCESS, rc);
                assertEquals(6, arg.lengthOrInd());
                assertEquals("あいう", arg.getData());
            }
        }
        try (var stmt = createStmt()) {
            var manager = stmt.manager();

            stmt.execDirect("select * from test", wideChar);
            assertTrue(stmt.fetch());

            var arg = TgOdbcGetDataArgument.ofString(manager, 6, wideChar);
            short rc = stmt.getData0(1, arg);
            assertEquals(SqlReturn.SQL_SUCCESS_WITH_INFO, rc);
            assertEquals(6, arg.lengthOrInd());

            var rec = stmt.getDiagRec(1);
            assertEquals("01004", rec.sqlState());
        }
    }
}