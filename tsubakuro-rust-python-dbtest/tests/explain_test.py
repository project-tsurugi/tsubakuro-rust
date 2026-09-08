import tsurugi_dbapi as tsurugi


def test_explain(connection, endpoint):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test (foo int primary key, bar bigint, zzz varchar(10))"
        )
        connection.commit()

        sql = "insert into tsubakuro_rust_python_test values (?, ?, ?)"
        parameters = (1, 100, "abc")
        explain_result = cursor.explain(sql, parameters)
        assert explain_result.format_id == "jogasaki-statement.json"
        assert explain_result.format_version == 1
        assert explain_result.contents != ""
        assert explain_result.columns == []

        cursor.execute(sql, parameters)
        connection.commit()

        sql = "insert into tsubakuro_rust_python_test values (:foo, :bar, :zzz)"
        parameters = {"foo": 2, "bar": 200, "zzz": "def"}
        explain_result = cursor.explain(sql, parameters)
        assert explain_result.format_id == "jogasaki-statement.json"
        assert explain_result.format_version == 1
        assert explain_result.contents != ""
        assert explain_result.columns == []

        cursor.execute(sql, parameters)
        connection.commit()

        sql = "select * from tsubakuro_rust_python_test order by foo"
        explain_result = cursor.explain(sql)
        assert explain_result.format_id == "jogasaki-statement.json"
        assert explain_result.format_version == 1
        assert explain_result.contents != ""
        assert len(explain_result.columns) == 3
        assert explain_result.columns[0].name == "foo"
        assert explain_result.columns[0].type_code == "Int32"
        assert explain_result.columns[1].name == "bar"
        assert explain_result.columns[1].type_code == "Int64"
        assert explain_result.columns[2].name == "zzz"
        assert explain_result.columns[2].type_code == "Str"

        cursor.execute(sql)
        rows = cursor.fetchall()
        assert rows == [(1, 100, "abc"), (2, 200, "def")]


def test_explain_with_prepare(connection, endpoint):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test (foo int primary key, bar bigint, zzz varchar(10))"
        )
        connection.commit()

        sql = "insert into tsubakuro_rust_python_test values (?, ?, ?)"
        placeholders = (tsurugi.type_code.Int32, tsurugi.type_code.Int64, tsurugi.type_code.Str)
        cursor.prepare(sql, placeholders)

        parameters = (1, 100, "abc")
        explain_result = cursor.explain(sql, parameters)
        assert explain_result.format_id == "jogasaki-statement.json"
        assert explain_result.format_version == 1
        assert explain_result.contents != ""
        assert explain_result.columns == []

        cursor.execute(sql, parameters)
        connection.commit()

        sql = "insert into tsubakuro_rust_python_test values (:foo, :bar, :zzz)"
        placeholders = {"foo": tsurugi.type_code.Int32, "bar": tsurugi.type_code.Int64, "zzz": tsurugi.type_code.Str}
        cursor.prepare(sql, placeholders)

        parameters = {"foo": 2, "bar": 200, "zzz": "def"}
        explain_result = cursor.explain(sql, parameters)
        assert explain_result.format_id == "jogasaki-statement.json"
        assert explain_result.format_version == 1
        assert explain_result.contents != ""
        assert explain_result.columns == []

        cursor.execute(sql, parameters)
        connection.commit()

        sql = "select * from tsubakuro_rust_python_test order by foo"
        placeholders = ()
        cursor.prepare(sql, placeholders)

        explain_result = cursor.explain(sql)
        assert explain_result.format_id == "jogasaki-statement.json"
        assert explain_result.format_version == 1
        assert explain_result.contents != ""
        assert len(explain_result.columns) == 3
        assert explain_result.columns[0].name == "foo"
        assert explain_result.columns[0].type_code == "Int32"
        assert explain_result.columns[1].name == "bar"
        assert explain_result.columns[1].type_code == "Int64"
        assert explain_result.columns[2].name == "zzz"
        assert explain_result.columns[2].type_code == "Str"

        cursor.execute(sql)
        rows = cursor.fetchall()
        assert rows == [(1, 100, "abc"), (2, 200, "def")]
