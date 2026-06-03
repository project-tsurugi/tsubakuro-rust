package com.tsurugidb.tsubakuro.rust.java.type;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiTypeClobTest extends TgFfiTester {

    @BeforeAll
    static void createTable() throws Exception {
        String sql = """
                create table test (
                  pk int primary key,
                  value clob
                )
                """;
        dropAndCreateTable("test", sql);
    }

    @Test
    void tableMetadata() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var client = createSqlClient()) {
            try (var metadata = client.getTableMetadata(context, "test")) {
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
                    assertEquals(TgFfiAtomType.CLOB, c.getAtomType(context));
                    assertEquals("CLOB", c.getSqlTypeName(context));
                    assertEquals("CLOB", c.getSqlType(context));
                }
            }
        }
    }
}
