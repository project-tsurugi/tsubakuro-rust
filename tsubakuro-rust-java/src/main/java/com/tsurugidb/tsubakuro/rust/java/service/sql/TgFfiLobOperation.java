package com.tsurugidb.tsubakuro.rust.java.service.sql;

public enum TgFfiLobOperation {

    /** upload_lob_file */
    UPLOAD_LOB_FILE(11),

    /** upload_lob */
    UPLOAD_LOB(12),

    /** create_lob_uploader */
    CREATE_LOB_UPLOADER(13),

    /** open_lob */
    OPEN_LOB(21),

    /** get_lob_cache */
    GET_LOB_CACHE(22),

    /** read_lob */
    READ_LOB(23),

    /** copy_lob_to */
    COPY_LOB_TO(24),

    /** create_lob_downloader */
    CREATE_LOB_DOWNLOADER(25),

    //
    ;

    private final int value;

    TgFfiLobOperation(int value) {
        this.value = value;
    }

    public int value() {
        return this.value;
    }
}
