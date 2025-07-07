package com.tsurugidb.tsubakuro.rust.odbc.api;

public enum CDataType {
    /** CHAR, VARCHAR */
    SQL_C_CHAR(1),
    /** WCHAR (wide char) */
    SQL_C_WCHAR(-8),
    /** 32-bit signed integer */
    SQL_C_LONG(4),
    /** 16-bit signed integer */
    SQL_C_SHORT(5),
    /** 32-bit floating point */
    SQL_C_FLOAT(7),
    /** 64-bit floating point */
    SQL_C_DOUBLE(8),
    /** Numeric type */
    SQL_C_NUMERIC(2),
    /** Default C type */
    SQL_C_DEFAULT(99),

    /** SQL DATE type */
    SQL_C_DATE(9),
    /** SQL TIME type */
    SQL_C_TIME(10),
    /** SQL TIMESTAMP type */
    SQL_C_TIMESTAMP(11),

    /** SQL TYPE_DATE (ODBC 3.x) */
    SQL_C_TYPE_DATE(91),
    /** SQL TYPE_TIME (ODBC 3.x) */
    SQL_C_TYPE_TIME(92),
    /** SQL TYPE_TIMESTAMP (ODBC 3.x) */
    SQL_C_TYPE_TIMESTAMP(93),
    /** SQL INTERVAL YEAR */
    SQL_C_INTERVAL_YEAR(-80),
    /** SQL INTERVAL MONTH */
    SQL_C_INTERVAL_MONTH(-81),
    /** SQL INTERVAL DAY */
    SQL_C_INTERVAL_DAY(-83),
    /** SQL INTERVAL HOUR */
    SQL_C_INTERVAL_HOUR(-84),
    /** SQL INTERVAL MINUTE */
    SQL_C_INTERVAL_MINUTE(-85),
    /** SQL INTERVAL SECOND */
    SQL_C_INTERVAL_SECOND(-86),
    /** SQL INTERVAL YEAR TO MONTH */
    SQL_C_INTERVAL_YEAR_TO_MONTH(-82),
    /** SQL INTERVAL DAY TO HOUR */
    SQL_C_INTERVAL_DAY_TO_HOUR(-87),
    /** SQL INTERVAL DAY TO MINUTE */
    SQL_C_INTERVAL_DAY_TO_MINUTE(-88),
    /** SQL INTERVAL DAY TO SECOND */
    SQL_C_INTERVAL_DAY_TO_SECOND(-89),
    /** SQL INTERVAL HOUR TO MINUTE */
    SQL_C_INTERVAL_HOUR_TO_MINUTE(-90),
    /** SQL INTERVAL HOUR TO SECOND */
    SQL_C_INTERVAL_HOUR_TO_SECOND(-91),
    /** SQL INTERVAL MINUTE TO SECOND */
    SQL_C_INTERVAL_MINUTE_TO_SECOND(-92),

    /** Binary data */
    SQL_C_BINARY(-2),
    /** Bit (boolean) data */
    SQL_C_BIT(-7),

    /** Signed 64-bit integer */
    SQL_C_SBIGINT(-25),
    /** Unsigned 64-bit integer */
    SQL_C_UBIGINT(-27),

    /** 8-bit signed integer */
    SQL_C_TINYINT(-6),
    /** Signed 32-bit integer */
    SQL_C_SLONG(-16),
    /** Signed 16-bit integer */
    SQL_C_SSHORT(-15),
    /** Signed 8-bit integer */
    SQL_C_STINYINT(-26),
    /** Unsigned 32-bit integer */
    SQL_C_ULONG(-18),
    /** Unsigned 16-bit integer */
    SQL_C_USHORT(-17),
    /** Unsigned 8-bit integer */
    SQL_C_UTINYINT(-28),

    ;

    private final short value;

    CDataType(int value) {
        this.value = (short) value;
    }

    public short value() {
        return value;
    }
}
