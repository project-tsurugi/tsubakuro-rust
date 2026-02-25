Module tsubakuro_rust_python
============================
Python library for Tsurugi.

Examples:

    ```python
    import tsubakuro_rust_python as tsurugi

    config = tsurugi.Config()
    config.endpoint = "tcp://localhost:12345"
    config.user = "tsurugi"
    config.password = "password"
    config.default_timeout = 30  # seconds
    with tsurugi.connect(config) as connection:
        with connection.cursor() as cursor:
            cursor.execute("insert into example values (1, 100, 'abc')")
            print("insert rowcount:", cursor.rowcount)
            connection.commit()

            cursor.execute("select * from example")
            for row in cursor:
                print("row:", row)
            connection.commit()
    ```

Note:
    See `Config`, `connect()`, `Connection`, and `Cursor` for more details.

Sub-modules
-----------
* tsubakuro_rust_python.error
* tsubakuro_rust_python.type_code

Functions
---------

`connect(*args, **kwargs)`
:   Constructor for creating a connection to the Tsurugi.
    
    Args:
        *args (Config, optional): configuration object.
        **kwargs (dict, optional): e.g. `endpoint="tcp://localhost:12345"`, `user="tsurugi"`
    
    Returns:
        Connection: Connection object.
    
    Examples:
        ```python
        import tsubakuro_rust_python as tsurugi
    
        config = tsurugi.Config()
        config.endpoint = "tcp://localhost:12345"
        config.user = "tsurugi"
        config.password = "password"
        config.default_timeout = 30  # seconds
        with tsurugi.connect(config) as connection:
            pass
        ```
    
        ```python
        import tsubakuro_rust_python as tsurugi
    
        with tsurugi.connect(
            endpoint="tcp://localhost:12345",
            user="tsurugi",
            password="password",
            default_timeout=30,  # seconds
        ) as connection:
            pass
        ```

`env_logger_init(filters='tsubakuro_rust_python=info', file_path=None)`
:   Initialize env_logger.
    
    Args:
        filters (str, optional): filter string. If ommitted, `"tsubakuro_rust_python=info"` is used.
        file_path (str, optional): log file path. If None, logs to stderr.
    
    Examples:
        ```python
        import tsubakuro_rust_python as tsurugi
    
        tsurugi.env_logger_init("tsubakuro_rust_python=trace")
        ```
    
    Note:
        Calls to `env_logger_init` other than the first one are ignored.

Classes
-------

`Column()`
:   Column metadata.
    
    Attributes:
        name (str): Column name. (read only)
        description (Optional[str]): Column description. (read only)
        type_code (str): Type code. (read only)
        atom_type_code (int): AtomType code. -1 if unknown. (read only)
        sql_type (str): SQL type. (read only)
        sql_type_name (Optional[str]): SQL type name. (read only)
        length (Optional[int]): Length for string types. (read only)
        precision (Optional[int]): Precision for numeric types. (read only)
        scale (Optional[int]): Scale for numeric types. (read only)
        nullable (Optional[bool]): Nullable flag. (read only)

    ### Instance variables

    `atom_type_code`
    :   AtomType code.

    `description`
    :   Column description.

    `length`
    :   Length.

    `name`
    :   Column name.

    `nullable`
    :   Nullable.

    `precision`
    :   Precision.

    `scale`
    :   Scale.

    `sql_type`
    :   SQL type.

    `sql_type_name`
    :   SQL type name.

    `type_code`
    :   type_code.

`CommitOption(commit_type=Ellipsis, auto_dispose=False, timeout=None)`
:   Commit option for transaction.
    
    Attributes:
        commit_type (CommitType): Commit type. Default is `CommitType.DEFAULT`.
        auto_dispose (bool): Auto dispose flag. Default is `False`.
        commit_timeout (int): Commit timeout in seconds.
    
    Examples:
        ```python
        import tsubakuro_rust_python as tsurugi
    
        commit_option = tsurugi.CommitOption(tsurugi.CommitType.DEFAULT, False, 60)
        ```

    ### Instance variables

    `auto_dispose`
    :   Auto dispose flag.

    `commit_timeout`
    :   Commit timeout in seconds.

    `commit_type`
    :   Commit type.

`CommitType()`
:   Commit type for transaction.
    
    Attributes:
        DEFAULT: the default commit type (rely on the database settings).
        ACCEPTED: commit operation has accepted, and the transaction will never abort except system errors.
        AVAILABLE: commit data has been visible for others.
        STORED: commit data has been saved on the local disk.
        PROPAGATED: commit data has been propagated to the all suitable nodes.

    ### Class variables

    `ACCEPTED`
    :

    `AVAILABLE`
    :

    `DEFAULT`
    :

    `PROPAGATED`
    :

    `STORED`
    :

`Config(*args, **kwargs)`
:   Configuration options for connecting to Tsurugi.
    
    Attributes:
        application_name (str): Application name.
        endpoint (str): Endpoint URL of the Tsurugi server.
        user (str): Username for authentication.
        password (str): Password for authentication.
        auth_token (str): Authentication token.
        credentials (str): Path to credentials file.
        session_label (str): Session label for the connection.
        transaction_option (TransactionOption): Transaction option.
        commit_option (CommitOption): Commit option.
        shutdown_option (ShutdownOption): Shutdown option.
        default_timeout (int): Default timeout in seconds.
    
    Examples:
        ```python
        import tsubakuro_rust_python as tsurugi
    
        config = tsurugi.Config()
        config.application_name = "tsubakuro-rust-python example"
        config.endpoint = "tcp://localhost:12345"
        config.user = "tsurugi"
        config.password = "password"
        config.session_label = "tsubakuro-rust-python session"
        config.default_timeout = 30  # seconds
        ```
    
        ```python
        import tsubakuro_rust_python as tsurugi
    
        config = tsurugi.Config(
            application_name="tsubakuro-rust-python example",
            endpoint="tcp://localhost:12345",
            user="tsurugi",
            password="password",
            session_label="tsubakuro-rust-python session",
            default_timeout=30,  # seconds
        )
        ```

    ### Instance variables

    `application_name`
    :   Application name.

    `auth_token`
    :   Authentication token.

    `commit_option`
    :   Commit option.

    `credentials`
    :   Path to credentials file.

    `default_timeout`
    :   Default timeout in seconds.

    `endpoint`
    :   Endpoint URL of the Tsurugi server.

    `password`
    :   Password for authentication.

    `session_label`
    :   Session label for the connection.

    `shutdown_option`
    :   Shutdown option.

    `transaction_option`
    :   Transaction option.

    `user`
    :   Username for authentication.

    ### Methods

    `merge(self, /, other)`
    :   Merge another `Config` into this one.
        
        Args:
            other (Config): other configuration object.

    `set(self, /, *args, **kwargs)`
    :   Set configuration options.
        
        Args:
            *args (Config | TransactionOption | CommitOption | ShutdownOption | str, optional): other configuration object.
            **kwargs (dict, optional): e.g. `endpoint="tcp://localhost:12345"`, `user="tsurugi"`

`Connection()`
:   Connection to Tsurugi.
    
    Attributes:
        transaction_option (TransactionOption): Transaction option. (write only)
        commit_option (CommitOption): Commit option. (write only)
        shutdown_option (ShutdownOption): Shutdown option. (write only)
        closed (bool): Whether the connection is closed. (read only)

    ### Instance variables

    `closed`
    :   Whether the connection is closed.

    `commit_option`
    :   Commit option.

    `shutdown_option`
    :   Shutdown option.

    `transaction_option`
    :   Transaction option.

    ### Methods

    `close(self, /)`
    :   Close the connection.

    `commit(self, /, option=None)`
    :   Commit the current transaction.
        
        Args:
            option (CommitOption, optional): CommitOption object.
        
        Examples:
            ```python
            connection.commit()
            ```

    `cursor(self, /)`
    :   Create a new cursor object using the connection.
        
        Returns:
            Cursor: Cursor object.
        
        Examples:
            ```python
            with connection.cursor() as cursor:
               pass
            ```

    `find_table_metadata(self, /, table_name)`
    :   Find table metadata.
        
        Args:
            table_name (str): Table name.
        
        Returns:
            Optional[TableMetadata]: Table metadata, or None if the table does not exist.
        
        Examples:
            ```python
            metadata = connection.find_table_metadata("my_table")
            ```

    `get_table_metadata(self, /, table_name)`
    :   Get table metadata.
        
        Args:
            table_name (str): Table name.
        
        Returns:
           TableMetadata: Table metadata.
        
        Raises:
            TargetNotFoundException: If the table does not exist.
        
        Examples:
            ```python
            import tsubakuro_rust_python as tsurugi
        
            try:
                metadata = connection.get_table_metadata("my_table")
            except tsurugi.error.TargetNotFoundException:
                pass
            ```

    `list_tables(self, /)`
    :   List table names.
        
        Returns:
            List[str]: List of table names.
        
        Examples:
            ```python
            table_names = connection.list_tables()
            ```

    `rollback(self, /)`
    :   Rollback the current transaction.
        
        Examples:
            ```python
            connection.rollback()
            ```

`Cursor()`
:   Cursor object for executing SQL statements and fetching results.
    
    Attributes:
        connection (Connection): Connection object associated with the cursor. (read only)
        description (Optional[Sequence[Tuple[str, str, None, Optional[int], Optional[int], Optional[int], Optional[bool]]]]): Description of the query result set.
            `(name, type_code, display_size, internal_size, precision, scale, null_ok)`.  (read only)
        arraysize (int): Number of rows to fetch at a time with `Cursor.fetchmany()`. Default is 1.
        rownumber (int): Current row number (0-based). (read only)
        rowcount (int): Number of rows affected by the last `Cursor.execute*()` method. -1 if not applicable. (read only)
        closed (bool): Whether the cursor is closed. (read only)

    ### Instance variables

    `arraysize`
    :   Number of rows to fetch at a time with `Cursor.fetchmany()`.

    `closed`
    :   Whether the cursor is closed.

    `connection`
    :   Connection object associated with the cursor. (read only)

    `description`
    :   Description of the query result set.

    `executemany_async`
    :   Whether to execute `Cursor.executemany()` asynchronously. Default is `True`.

    `rowcount`
    :   Number of rows affected by the last `Cursor.execute*()` method.

    `rownumber`
    :   Current row number (0-based).

    ### Methods

    `callproc(self, /, _procname)`
    :   Not supported in this implementation.

    `clear(self, /)`
    :   Clear the current query result and prepared statements.

    `close(self, /)`
    :   Close the cursor.

    `execute(self, /, operation, parameters=None)`
    :   Execute a SQL statement.
        
        Args:
            operation (str): SQL statement to be executed.
            parameters (Tuple[Any, ...] | dict[str, Any], optional): Parameters for the SQL statement.
        
        Examples:
            ```python
            cursor.execute("insert into example values (1, 'Hello')")
            connection.commit()
            ```
        
            ```python
            cursor.execute("insert into example values (?, ?)", (1, "Hello"))
            connection.commit()
            ```
        
            ```python
            cursor.execute("insert into example values (:id, :name)", {"id": 1, "name": "Hello"})
            connection.commit()
            ```

    `executemany(self, /, operation, seq_of_parameters)`
    :   Execute a prepared SQL statement multiple times.
        
        Args:
            operation (str): SQL statement to be executed.
            seq_of_parameters (Sequence[Tuple[Any, ...] | dict[str, Any]]): Sequence of parameters for the SQL statement.
        
        Examples:
            ```python
            cursor.executemany("insert into example values (?, ?)", [(1, "Hello"), (2, "World")])
            connection.commit()
            ```
        
            ```python
            cursor.executemany("insert into example values (:id, :name)", [{"id": 1, "name": "Hello"}, {"id": 2, "name": "World"}])
            connection.commit()
            ```

    `fetchall(self, /)`
    :   Fetch all (remaining) rows of a query result set.
        
        Returns:
             List[Tuple[Any, ...]]: A list of sequences, each representing a row of the result set.
        
        Examples:
            ```python
            cursor.execute("select * from example")
            rows = cursor.fetchall()
            connection.commit()
            ```

    `fetchmany(self, /, size=None)`
    :   Fetch the next set of rows of a query result set.
        
        Args:
            size (int, optional) - Number of rows to fetch. If not specified, use the cursor's `arraysize` attribute.
        
        Returns:
             List[Tuple[Any, ...]]: A list of sequences, each representing a row of the result set.
        
        Examples:
            ```python
            cursor.execute("select * from example")
            rows = cursor.fetchmany(10)
            connection.commit()
            ```
        
        Note:
            See also `Cursor.arraysize` for setting the default number of rows to fetch with `fetchmany()`.

    `fetchone(self, /)`
    :   Fetch the next row of a query result set.
        
        Returns:
              Optional[Tuple[Any, ...]]: A single sequence representing the next row of the result set, or `None` if no more data is available.
        
        Examples:
            ```python
            cursor.execute("select * from example where id = 1")
            row = cursor.fetchone()
            connection.commit()
            ```

    `next(self, /)`
    :   Fetch the next row of a query result set.
        
        Returns:
             Tuple[Any, ...]: A single sequence representing the next row of the result set.
        
        Raises:
            StopIteration: When no more data is available.

    `nextset(self, /)`
    :   Not supported in this implementation.

    `prepare(self, /, operation, parameters)`
    :   Prepare a SQL statement for execution.
        
        Args:
            operation (str): SQL statement to be prepared.
            parameters (Tuple[Any, ...] | dict[str, Any]): Parameters for the SQL statement.
        
        Examples:
            ```python
            import tsubakuro_rust_python as tsurugi
        
            sql = "insert into example values (?, ?)"
            cursor.prepare(sql, (tsurugi.type_code.Int64, tsurugi.type_code.Str))
            cursor.execute(sql, (1, "Hello"))
            connection.commit()
            ```
        
            ```python
            import tsubakuro_rust_python as tsurugi
        
            sql = "insert into example values (:id, :name)"
            cursor.prepare(sql, {"id": tsurugi.type_code.Int64, "name": tsurugi.type_code.Str})
            cursor.execute(sql, {"id": 1, "name": "Hello"})
            connection.commit()
            ```

    `setinputsizes(self, /, _sizes)`
    :   This method is a no-op in this implementation.

    `setoutputsize(self, /, _size)`
    :   This method is a no-op in this implementation.

`ShutdownOption(shutdown_type=Ellipsis, timeout=None)`
:   Shutdown option for connection.
    
    Attributes:
        shutdown_type (ShutdownType): Shutdown type. Default is `ShutdownType.GRACEFUL`.
        shutdown_timeout (int): Shutdown timeout in seconds.
    
    Examples:
        ```python
        import tsubakuro_rust_python as tsurugi
    
        shutdown_option = tsurugi.ShutdownOption(tsurugi.ShutdownType.GRACEFUL, 30)
        ```

    ### Instance variables

    `shutdown_timeout`
    :   Shutdown timeout in seconds.

    `shutdown_type`
    :   Shutdown type.

`ShutdownType()`
:   Shutdown type for connection.
    
    Attributes:
        NOTHING: Do nothing special during shutdown.
        GRACEFUL: Waits for the ongoing requests and safely shutdown the session.
        FORCEFUL: Cancelling the ongoing requests and safely shutdown the session.

    ### Class variables

    `FORCEFUL`
    :

    `GRACEFUL`
    :

    `NOTHING`
    :

`TableMetadata()`
:   Table metadata.
    
    Attributes:
        database_name (str): Database name. (read only)
        schema_name (str): Schema name. (read only)
        table_name (str): Table name. (read only)
        table_description (Optional[str]): Table description. (read only)
        columns (List[Column]): Columns metadata. (read only)
        description (Sequence[Tuple[str, str, None, Optional[int], Optional[int], Optional[int], Optional[bool]]]): Columns description.
          `(name, type_code, display_size, internal_size, precision, scale, null_ok)`. (read only)
        primary_keys (List[str]): Primary keys. (read only)

    ### Instance variables

    `columns`
    :   Columns metadata.

    `database_name`
    :   Database name.

    `description`
    :   Columns description.

    `primary_keys`
    :   Primary keys.

    `schema_name`
    :   Schema name.

    `table_description`
    :   Table description.

    `table_name`
    :   Table name.

`TransactionOption(type=Ellipsis)`
:   Transaction option.
    
    Attributes:
        transaction_type (TransactionType): Transaction type. Default is `TransactionType.OCC`.
        label (str): Transaction label.
        include_ddl (bool): Whether the transaction modifies definitions (DDL). Default is `False`. Only applicable for `TransactionType.LTX`.
        write_preserve (List[str]): List of table names to preserve for write operations. Only applicable for `TransactionType.LTX`.
        inclusive_read_area (List[str]): List of table names to include in the read area. Only applicable for `TransactionType.LTX`.
        exclusive_read_area (List[str]): List of table names to exclude from the read area. Only applicable for `TransactionType.LTX`.
        scan_parallel (int): Degree of parallelism for scanning. Only applicable for `TransactionType.RTX`.
        begin_timeout (int): Begin transaction timeout in seconds
    
    Examples:
        ```python
        import tsubakuro_rust_python as tsurugi
    
        tx_option = tsurugi.TransactionOption(tsurugi.TransactionType.OCC)
        tx_option.label = "tsubakuro-rust-python OCC example"
        ```
    
        ```python
        import tsubakuro_rust_python as tsurugi
    
        tx_option = tsurugi.TransactionOption(tsurugi.TransactionType.LTX)
        tx_option.label = "tsubakuro-rust-python LTX example"
        tx_option.write_preserve = ["table1", "table2"]
        ```
    
        ```python
        import tsubakuro_rust_python as tsurugi
    
        tx_option = tsurugi.TransactionOption(tsurugi.TransactionType.RTX)
        tx_option.label = "tsubakuro-rust-python RTX example"
        tx_option.scan_parallel = 4
        ```

    ### Instance variables

    `begin_timeout`
    :   Begin transaction timeout in seconds.

    `exclusive_read_area`
    :   Exclusive read area.

    `include_ddl`
    :   Include DDL flag.

    `inclusive_read_area`
    :   Inclusive read area.

    `label`
    :   Transaction label.

    `scan_parallel`
    :   Scan parallel.

    `transaction_type`
    :   Transaction type.

    `write_preserve`
    :   Write preserve.

    ### Methods

    `ddl(label=None)`
    :   Create a new `TransactionOption` for LTX transaction for DDL.
        
        Args:
            label (str, optional): Transaction label.
        
        Returns:
            TransactionOption: A new `TransactionOption` instance for LTX transaction for DDL.
        
        Examples:
            ```python
            import tsubakuro_rust_python as tsurugi
        
            tx_option = tsurugi.TransactionOption.ddl(label="LTX transaction for DDL")
            ```

    `ltx(label=None, write_preserve=None, inclusive_read_area=None, exclusive_read_area=None)`
    :   Create a new `TransactionOption` for LTX transaction.
        
        Args:
            label (str, optional): Transaction label.
            write_preserve (List[str], optional): List of table names to preserve for write operations.
            inclusive_read_area (List[str], optional): List of table names to include in the read area.
            exclusive_read_area (List[str], optional): List of table names to exclude from the read area.
        
        Returns:
            TransactionOption: A new `TransactionOption` instance for LTX transaction.
        
        Examples:
            ```python
            import tsubakuro_rust_python as tsurugi
        
            tx_option = tsurugi.TransactionOption.ltx(
                label="LTX transaction",
                write_preserve=["table1", "table2"],
            )
            ```

    `occ(label=None)`
    :   Create a new `TransactionOption` for OCC transaction.
        
        Args:
            label (str, optional): Transaction label.
        
        Returns:
            TransactionOption: A new `TransactionOption` instance for OCC transaction.
        
        Examples:
            ```python
            import tsubakuro_rust_python as tsurugi
        
            tx_option = tsurugi.TransactionOption.occ(label="OCC transaction")
            ```

    `rtx(label=None, scan_parallel=None)`
    :   Create a new `TransactionOption` for RTX transaction.
        
        Args:
            label (str, optional): Transaction label.
            scan_parallel (int, optional): Degree of parallelism for scanning.
        
        Returns:
            TransactionOption: A new `TransactionOption` instance for RTX transaction.
        
        Examples:
            ```python
            import tsubakuro_rust_python as tsurugi
        
            tx_option = tsurugi.TransactionOption.rtx(
                label="RTX transaction",
                scan_parallel=4,
            )
            ```

`TransactionType()`
:   Transaction type.
    
    Attributes:
        OCC: Optimistic concurrency control (OCC) transaction.
        LTX: Long transaction (LTX).
        RTX: Read-only transaction (RTX).

    ### Class variables

    `LTX`
    :

    `OCC`
    :

    `RTX`
    :