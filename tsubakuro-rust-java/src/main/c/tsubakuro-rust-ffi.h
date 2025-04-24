#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>







#define TSURUGI_FFI_RC_FFI_BASE (TSURUGI_FFI_RC_TYPE_FFI_ERROR << 30)

#define TSURUGI_FFI_RC_FFI_ARG_ERROR (TSURUGI_FFI_RC_FFI_BASE | (0 << 24))

#define TSURUGI_FFI_RC_FFI_JOB_ERROR (TSURUGI_FFI_RC_FFI_BASE | (1 << 24))

#define TSURUGI_FFI_RC_FFI_ERROR (TSURUGI_FFI_RC_FFI_BASE | (2 << 24))

#define TSURUGI_FFI_RC_CORE_CLIENT_ERROR (TSURUGI_FFI_RC_TYPE_CORE_CLIENT_ERROR << 30)

#define TSURUGI_FFI_RC_CORE_SERVER_ERROR (TSURUGI_FFI_RC_TYPE_CORE_SERVER_ERROR << 30)

/**
 * Atom type.
 */
enum TsurugiFfiAtomType {
  /**
   * unspecified type.
   */
  TSURUGI_FFI_ATOM_TYPE_TYPE_UNSPECIFIED = 0,
  /**
   * boolean type.
   */
  TSURUGI_FFI_ATOM_TYPE_BOOLEAN = 1,
  /**
   * 32-bit signed integer.
   */
  TSURUGI_FFI_ATOM_TYPE_INT4 = 4,
  /**
   * 64-bit signed integer.
   */
  TSURUGI_FFI_ATOM_TYPE_INT8 = 5,
  /**
   * 32-bit floating point number.
   */
  TSURUGI_FFI_ATOM_TYPE_FLOAT4 = 6,
  /**
   * 64-bit floating point number.
   */
  TSURUGI_FFI_ATOM_TYPE_FLOAT8 = 7,
  /**
   * multi precision decimal number.
   */
  TSURUGI_FFI_ATOM_TYPE_DECIMAL = 8,
  /**
   * character sequence.
   */
  TSURUGI_FFI_ATOM_TYPE_CHARACTER = 9,
  /**
   * octet sequence.
   */
  TSURUGI_FFI_ATOM_TYPE_OCTET = 11,
  /**
   * bit sequence.
   */
  TSURUGI_FFI_ATOM_TYPE_BIT = 13,
  /**
   * date.
   */
  TSURUGI_FFI_ATOM_TYPE_DATE = 15,
  /**
   * time of day.
   */
  TSURUGI_FFI_ATOM_TYPE_TIME_OF_DAY = 16,
  /**
   * time point.
   */
  TSURUGI_FFI_ATOM_TYPE_TIME_POINT = 17,
  /**
   * date-time interval.
   */
  TSURUGI_FFI_ATOM_TYPE_DATETIME_INTERVAL = 18,
  /**
   * time of day with time zone.
   */
  TSURUGI_FFI_ATOM_TYPE_TIME_OF_DAY_WITH_TIME_ZONE = 19,
  /**
   * time point with time zone.
   */
  TSURUGI_FFI_ATOM_TYPE_TIME_POINT_WITH_TIME_ZONE = 20,
  /**
   * character large objects.
   */
  TSURUGI_FFI_ATOM_TYPE_CLOB = 21,
  /**
   * binary large objects.
   */
  TSURUGI_FFI_ATOM_TYPE_BLOB = 22,
  /**
   * unknown type.
   */
  TSURUGI_FFI_ATOM_TYPE_UNKNOWN = 31,
  /**
   * unrecognized.
   */
  TSURUGI_FFI_ATOM_TYPE_UNRECOGNIZED = -1,
};
typedef int32_t TsurugiFfiAtomType;

/**
 * Commit type.
 */
enum TsurugiFfiCommitType {
  /**
   * the default commit status (rely on the database settings).
   */
  TSURUGI_FFI_COMMIT_TYPE_UNSPECIFIED = 0,
  /**
   * commit operation has accepted, and the transaction will never abort except system errors.
   */
  TSURUGI_FFI_COMMIT_TYPE_ACCEPTED = 10,
  /**
   * commit data has been visible for others.
   */
  TSURUGI_FFI_COMMIT_TYPE_AVAILABLE = 20,
  /**
   * commit data has been saved on the local disk.
   */
  TSURUGI_FFI_COMMIT_TYPE_STORED = 30,
  /**
   * commit data has been propagated to the all suitable nodes.
   */
  TSURUGI_FFI_COMMIT_TYPE_PROPAGATED = 40,
};
typedef int32_t TsurugiFfiCommitType;

/**
 * Type of return code.
 */
enum TsurugiFfiRcType {
  TSURUGI_FFI_RC_TYPE_OK = 0,
  TSURUGI_FFI_RC_TYPE_FFI_ERROR = 1,
  TSURUGI_FFI_RC_TYPE_CORE_CLIENT_ERROR = 2,
  TSURUGI_FFI_RC_TYPE_CORE_SERVER_ERROR = 3,
};
typedef uint32_t TsurugiFfiRcType;

/**
 * Shutdown type.
 */
enum TsurugiFfiShutdownType {
  /**
   * The default shutdown type.
   */
  TSURUGI_FFI_SHUTDOWN_TYPE_NOT_SET = 0,
  /**
   * Waits for the ongoing requests and safely shutdown the session.
   */
  TSURUGI_FFI_SHUTDOWN_TYPE_GRACEFUL = 1,
  /**
   * Cancelling the ongoing requests and safely shutdown the session.
   */
  TSURUGI_FFI_SHUTDOWN_TYPE_FORCEFUL = 2,
};
typedef int32_t TsurugiFfiShutdownType;

/**
 * Sql counter type.
 */
enum TsurugiFfiSqlCounterType {
  /**
   * the un-categorized counter type.
   */
  TSURUGI_FFI_SQL_COUNTER_TYPE_UNSPECIFIED = 0,
  /**
   * The number of rows inserted in the execution.
   */
  TSURUGI_FFI_SQL_COUNTER_TYPE_INSERTED_ROWS = 10,
  /**
   * The number of rows updated in the execution.
   */
  TSURUGI_FFI_SQL_COUNTER_TYPE_UPDATED_ROWS = 20,
  /**
   * The number of rows merged in the execution.
   */
  TSURUGI_FFI_SQL_COUNTER_TYPE_MERGED_ROWS = 30,
  /**
   * The number of rows deleted in the execution.
   */
  TSURUGI_FFI_SQL_COUNTER_TYPE_DELETED_ROWS = 40,
};
typedef int32_t TsurugiFfiSqlCounterType;

/**
 * Transaction priority.
 */
enum TsurugiFfiTransactionPriority {
  /**
   * use default transaction priority.
   */
  TSURUGI_FFI_TRANSACTION_PRIORITY_UNSPECIFIED = 0,
  /**
   * halts the running transactions immediately.
   */
  TSURUGI_FFI_TRANSACTION_PRIORITY_INTERRUPT = 1,
  /**
   * prevents new transactions and waits for the running transactions will end.
   */
  TSURUGI_FFI_TRANSACTION_PRIORITY_WAIT = 2,
  /**
   * halts the running transactions immediately, and keep lock-out until its end.
   */
  TSURUGI_FFI_TRANSACTION_PRIORITY_INTERRUPT_EXCLUDE = 3,
  /**
   * prevents new transactions and waits for the running transactions will end, and keep lock-out until its end.
   */
  TSURUGI_FFI_TRANSACTION_PRIORITY_WAIT_EXCLUDE = 4,
};
typedef int32_t TsurugiFfiTransactionPriority;

/**
 * Transaction type.
 */
enum TsurugiFfiTransactionType {
  /**
   * use default transaction type.
   */
  TSURUGI_FFI_TRANSACTION_TYPE_UNSPECIFIED = 0,
  /**
   * short transactions (optimistic concurrency control).
   */
  TSURUGI_FFI_TRANSACTION_TYPE_SHORT = 1,
  /**
   * long transactions (pessimistic concurrency control).
   */
  TSURUGI_FFI_TRANSACTION_TYPE_LONG = 2,
  /**
   * read only transactions (may be abort-free).
   */
  TSURUGI_FFI_TRANSACTION_TYPE_READ_ONLY = 3,
};
typedef int32_t TsurugiFfiTransactionType;

typedef struct TsurugiFfiBlobReference TsurugiFfiBlobReference;

typedef struct TsurugiFfiCancelJob TsurugiFfiCancelJob;

typedef struct TsurugiFfiClobReference TsurugiFfiClobReference;

typedef struct TsurugiFfiCommitOption TsurugiFfiCommitOption;

typedef struct TsurugiFfiConnectionOption TsurugiFfiConnectionOption;

typedef struct TsurugiFfiContext TsurugiFfiContext;

typedef struct TsurugiFfiEndpoint TsurugiFfiEndpoint;

typedef struct TsurugiFfiSession TsurugiFfiSession;

typedef struct TsurugiFfiSqlClient TsurugiFfiSqlClient;

typedef struct TsurugiFfiSqlColumn TsurugiFfiSqlColumn;

typedef struct TsurugiFfiSqlExecuteResult TsurugiFfiSqlExecuteResult;

typedef struct TsurugiFfiSqlExplainResult TsurugiFfiSqlExplainResult;

typedef struct TsurugiFfiSqlParameter TsurugiFfiSqlParameter;

typedef struct TsurugiFfiSqlPlaceholder TsurugiFfiSqlPlaceholder;

typedef struct TsurugiFfiSqlPreparedStatement TsurugiFfiSqlPreparedStatement;

typedef struct TsurugiFfiSqlQueryResult TsurugiFfiSqlQueryResult;

typedef struct TsurugiFfiSqlQueryResultMetadata TsurugiFfiSqlQueryResultMetadata;

typedef struct TsurugiFfiTableList TsurugiFfiTableList;

typedef struct TsurugiFfiTableMetadata TsurugiFfiTableMetadata;

typedef struct TsurugiFfiTransaction TsurugiFfiTransaction;

typedef struct TsurugiFfiTransactionErrorInfo TsurugiFfiTransactionErrorInfo;

typedef struct TsurugiFfiTransactionOption TsurugiFfiTransactionOption;

/**
 * Return code of tsubakuro-rust-ffi function.
 */
typedef uint32_t TsurugiFfiRc;

/**
 * Context object.
 *
 * Context object holds error information when an error occurs.
 */
typedef struct TsurugiFfiContext *TsurugiFfiContextHandle;

/**
 * String (UTF-8 with `nul` termination).
 */
typedef const char *TsurugiFfiStringHandle;

/**
 * Cancel job.
 */
typedef struct TsurugiFfiCancelJob *TsurugiFfiCancelJobHandle;

/**
 * Nanosecond.
 */
typedef uint64_t TsurugiFfiDuration;

/**
 * Job.
 */
typedef void *TsurugiFfiJobHandle;

/**
 * Sql column.
 */
typedef struct TsurugiFfiSqlColumn *TsurugiFfiSqlColumnHandle;

/**
 * Execute result of SQL statement.
 */
typedef struct TsurugiFfiSqlExecuteResult *TsurugiFfiSqlExecuteResultHandle;

/**
 * Explain result of SQL statement.
 */
typedef struct TsurugiFfiSqlExplainResult *TsurugiFfiSqlExplainResultHandle;

/**
 * Sql parameter.
 */
typedef struct TsurugiFfiSqlParameter *TsurugiFfiSqlParameterHandle;

/**
 * Byte array.
 */
typedef const uint8_t *TsurugiFfiByteArrayHandle;

/**
 * Sql placeholder.
 */
typedef struct TsurugiFfiSqlPlaceholder *TsurugiFfiSqlPlaceholderHandle;

/**
 * Sql prepared statement.
 */
typedef struct TsurugiFfiSqlPreparedStatement *TsurugiFfiSqlPreparedStatementHandle;

/**
 * Sql query result (sql result set).
 */
typedef struct TsurugiFfiSqlQueryResult *TsurugiFfiSqlQueryResultHandle;

/**
 * Sql query result metadata.
 */
typedef struct TsurugiFfiSqlQueryResultMetadata *TsurugiFfiSqlQueryResultMetadataHandle;

/**
 * Blob.
 */
typedef struct TsurugiFfiBlobReference *TsurugiFfiBlobReferenceHandle;

/**
 * Clob.
 */
typedef struct TsurugiFfiClobReference *TsurugiFfiClobReferenceHandle;

/**
 * Sql client.
 */
typedef struct TsurugiFfiSqlClient *TsurugiFfiSqlClientHandle;

/**
 * Table list.
 */
typedef struct TsurugiFfiTableList *TsurugiFfiTableListHandle;

/**
 * Table metadata.
 */
typedef struct TsurugiFfiTableMetadata *TsurugiFfiTableMetadataHandle;

/**
 * Transaction option.
 */
typedef struct TsurugiFfiTransactionOption *TsurugiFfiTransactionOptionHandle;

/**
 * Transaction.
 */
typedef struct TsurugiFfiTransaction *TsurugiFfiTransactionHandle;

/**
 * Transaction error information.
 */
typedef struct TsurugiFfiTransactionErrorInfo *TsurugiFfiTransactionErrorInfoHandle;

/**
 * Commit option.
 */
typedef struct TsurugiFfiCommitOption *TsurugiFfiCommitOptionHandle;

/**
 * String array.
 */
typedef const TsurugiFfiStringHandle *TsurugiFfiStringArrayHandle;

/**
 * Endpoint.
 */
typedef struct TsurugiFfiEndpoint *TsurugiFfiEndpointHandle;

/**
 * Connection option.
 */
typedef struct TsurugiFfiConnectionOption *TsurugiFfiConnectionOptionHandle;

/**
 * Session.
 */
typedef struct TsurugiFfiSession *TsurugiFfiSessionHandle;

/**
 * TsurugiFfiRc: Ok
 */
#define TSURUGI_FFI_RC_OK 0

/**
 * TsurugiFfiRc: FFI argument error (index=0)
 */
#define TSURUGI_FFI_RC_FFI_ARG0_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 0)

/**
 * TsurugiFfiRc: FFI argument error (index=1)
 */
#define TSURUGI_FFI_RC_FFI_ARG1_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 1)

/**
 * TsurugiFfiRc: FFI argument error (index=2)
 */
#define TSURUGI_FFI_RC_FFI_ARG2_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 2)

/**
 * TsurugiFfiRc: FFI argument error (index=3)
 */
#define TSURUGI_FFI_RC_FFI_ARG3_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 3)

/**
 * TsurugiFfiRc: FFI argument error (index=4)
 */
#define TSURUGI_FFI_RC_FFI_ARG4_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 4)

/**
 * TsurugiFfiRc: FFI argument error (index=5)
 */
#define TSURUGI_FFI_RC_FFI_ARG5_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 5)

/**
 * TsurugiFfiRc: FFI argument error (index=6)
 */
#define TSURUGI_FFI_RC_FFI_ARG6_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 6)

/**
 * TsurugiFfiRc: FFI argument error (index=7)
 */
#define TSURUGI_FFI_RC_FFI_ARG7_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 7)

/**
 * TsurugiFfiRc: FFI job already closed
 */
#define TSURUGI_FFI_RC_FFI_JOB_ALREADY_CLOSED (TSURUGI_FFI_RC_FFI_JOB_ERROR | 1)

/**
 * TsurugiFfiRc: FFI nul error
 */
#define TSURUGI_FFI_RC_FFI_NUL_ERROR (TSURUGI_FFI_RC_FFI_ERROR | 1)

/**
 * TsurugiFfiRc: FFI diagnostic code not found
 */
#define TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND (TSURUGI_FFI_RC_FFI_ERROR | 2)

#define TSURUGI_FFI_RC_CORE_CLIENT_CLIENT_ERROR (TSURUGI_FFI_RC_CORE_CLIENT_ERROR | (1 << 16))

#define TSURUGI_FFI_RC_CORE_CLIENT_TIMEOUT_ERROR (TSURUGI_FFI_RC_CORE_CLIENT_ERROR | (2 << 16))

#define TSURUGI_FFI_RC_CORE_CLIENT_IO_ERROR (TSURUGI_FFI_RC_CORE_CLIENT_ERROR | (3 << 16))

/**
 * Context: Creates a new instance.
 *
 * # Returns
 * - `context_out` - context object. To dispose, call [`tsurugi_ffi_context_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_context_create(TsurugiFfiContextHandle *context_out);

/**
 * Context: Get return code.
 *
 * # Receiver
 * - `context` - Context object.
 *
 * # Returns
 * - `rc_out` - return code.
 */
TsurugiFfiRc tsurugi_ffi_context_get_return_code(TsurugiFfiContextHandle context,
                                                 TsurugiFfiRc *rc_out);

/**
 * Context: Get error name.
 *
 * # Receiver
 * - `context` - Context object.
 *
 * # Returns
 * - `error_name_out` - error name. `null` if no error occurs.
 */
TsurugiFfiRc tsurugi_ffi_context_get_error_name(TsurugiFfiContextHandle context,
                                                TsurugiFfiStringHandle *error_name_out);

/**
 * Context: Get RcType.
 *
 * # Receiver
 * - `context` - Context object.
 *
 * # Returns
 * - `error_type_out` - type of return code.
 */
TsurugiFfiRc tsurugi_ffi_context_get_error_type(TsurugiFfiContextHandle context,
                                                TsurugiFfiRcType *error_type_out);

/**
 * Context: Get error message.
 *
 * # Receiver
 * - `context` - Context object.
 *
 * # Returns
 * - `error_message_out` - error message. `null` if no error occurs.
 */
TsurugiFfiRc tsurugi_ffi_context_get_error_message(TsurugiFfiContextHandle context,
                                                   TsurugiFfiStringHandle *error_message_out);

/**
 * Context: Get error category.
 *
 * Available only if a server error has occurred.
 *
 * # Receiver
 * - `context` - Context object.
 *
 * # Returns
 * - `category_number_out` - category number.
 */
TsurugiFfiRc tsurugi_ffi_context_get_server_error_category_number(TsurugiFfiContextHandle context,
                                                                  int32_t *category_number_out);

/**
 * Context: Get error category.
 *
 * Available only if a server error has occurred.
 *
 * # Receiver
 * - `context` - Context object.
 *
 * # Returns
 * - `category_str_out` - category name.
 */
TsurugiFfiRc tsurugi_ffi_context_get_server_error_category_str(TsurugiFfiContextHandle context,
                                                               TsurugiFfiStringHandle *category_str_out);

/**
 * Context: Get error code.
 *
 * Available only if a server error has occurred.
 *
 * # Receiver
 * - `context` - Context object.
 *
 * # Returns
 * - `code_number_out` - error code.
 */
TsurugiFfiRc tsurugi_ffi_context_get_server_error_code_number(TsurugiFfiContextHandle context,
                                                              int32_t *code_number_out);

/**
 * Context: Get structured error code.
 *
 * Available only if a server error has occurred.
 *
 * # Receiver
 * - `context` - Context object.
 *
 * # Returns
 * - `structured_code_out` - structured error code.
 */
TsurugiFfiRc tsurugi_ffi_context_get_server_error_structured_code(TsurugiFfiContextHandle context,
                                                                  TsurugiFfiStringHandle *structured_code_out);

/**
 * Context: Dispose.
 *
 * # Receiver
 * - `context` - Context object.
 */
void tsurugi_ffi_context_dispose(TsurugiFfiContextHandle context);

/**
 * CancelJob: Wait.
 *
 * See [`CancelJob::wait`].
 *
 * # Receiver
 * - `cancel_job` - canel job.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `done_out` - `true`: Response received / `false`: Timed out.
 */
TsurugiFfiRc tsurugi_ffi_cancel_job_wait(TsurugiFfiContextHandle context,
                                         TsurugiFfiCancelJobHandle cancel_job,
                                         TsurugiFfiDuration timeout,
                                         bool *done_out);

/**
 * CancelJob: Is done.
 *
 * See [`CancelJob::is_done`].
 *
 * # Receiver
 * - `cancel_job` - canel job.
 *
 * # Returns
 * - `done_out` - `true`: Response received / `false`: No response received.
 */
TsurugiFfiRc tsurugi_ffi_cancel_job_is_done(TsurugiFfiContextHandle context,
                                            TsurugiFfiCancelJobHandle cancel_job,
                                            bool *done_out);

/**
 * CancelJob: Dispose.
 *
 * # Receiver
 * - `cancel_job` - canel job.
 */
void tsurugi_ffi_cancel_job_dispose(TsurugiFfiCancelJobHandle cancel_job);

/**
 * Job: Get name.
 *
 * See [`Job::name`].
 *
 * # Receiver
 * - `job` - job.
 *
 * # Returns
 * - `name_out` - job name.
 */
TsurugiFfiRc tsurugi_ffi_job_get_name(TsurugiFfiContextHandle context,
                                      TsurugiFfiJobHandle job,
                                      TsurugiFfiStringHandle *name_out);

/**
 * Job: Wait.
 *
 * See [`Job::wait`].
 *
 * # Receiver
 * - `job` - job.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `done_out` - `true`: Response received / `false`: Timed out.
 */
TsurugiFfiRc tsurugi_ffi_job_wait(TsurugiFfiContextHandle context,
                                  TsurugiFfiJobHandle job,
                                  TsurugiFfiDuration timeout,
                                  bool *done_out);

/**
 * Job: Is done.
 *
 * See [`Job::is_done`].
 *
 * # Receiver
 * - `job` - job.
 *
 * # Returns
 * - `done_out` - `true`: Response received / `false`: No response received.
 */
TsurugiFfiRc tsurugi_ffi_job_is_done(TsurugiFfiContextHandle context,
                                     TsurugiFfiJobHandle job,
                                     bool *done_out);

/**
 * Job: Take result.
 *
 * See [`Job::take`].
 *
 * # Receiver
 * - `job` - job.
 *
 * # Returns
 * - `value_out` - result value (`null` if job type is `void`).
 *   The value must be cast to the appropriate type according to the function that generated this Job.
 *   And, call `tsurugi_ffi_XXX_dispose()` to dispose if value is not null.
 */
TsurugiFfiRc tsurugi_ffi_job_take(TsurugiFfiContextHandle context,
                                  TsurugiFfiJobHandle job,
                                  void **value_out);

/**
 * Job: Take result.
 *
 * See [`Job::take_for`].
 *
 * # Receiver
 * - `job` - job.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `value_out` - result value (`null` if job type is `void`).
 *   The value must be cast to the appropriate type according to the function that generated this Job.
 *   And, call `tsurugi_ffi_XXX_dispose()` to dispose if value is not null.
 */
TsurugiFfiRc tsurugi_ffi_job_take_for(TsurugiFfiContextHandle context,
                                      TsurugiFfiJobHandle job,
                                      TsurugiFfiDuration timeout,
                                      void **value_out);

/**
 * Job: Take result if ready.
 *
 * See [`Job::take_if_ready`].
 *
 * # Receiver
 * - `job` - job.
 *
 * # Returns
 * - `is_ready_out` - `true`: Response received / `false`: No response received.
 * - `value_out` - result value (`null` if no response received or job type is `void`).
 *   The value must be cast to the appropriate type according to the function that generated this Job.
 *   And, call `tsurugi_ffi_XXX_dispose()` to dispose if value is not null.
 */
TsurugiFfiRc tsurugi_ffi_job_take_if_ready(TsurugiFfiContextHandle context,
                                           TsurugiFfiJobHandle job,
                                           bool *is_ready_out,
                                           void **value_out);

/**
 * Job: Cancel.
 *
 * See [`Job::cancel`].
 *
 * # Receiver
 * - `job` - job.
 *
 * # Returns
 * - `cancell_done_out` - `true`: Response received / `false`: Timed out.
 */
TsurugiFfiRc tsurugi_ffi_job_cancel(TsurugiFfiContextHandle context,
                                    TsurugiFfiJobHandle job,
                                    bool *cancell_done_out);

/**
 * Job: Cancel.
 *
 * See [`Job::cancel_for`].
 *
 * # Receiver
 * - `job` - job.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `cancell_done_out` - `true`: Response received / `false`: Timed out.
 */
TsurugiFfiRc tsurugi_ffi_job_cancel_for(TsurugiFfiContextHandle context,
                                        TsurugiFfiJobHandle job,
                                        TsurugiFfiDuration timeout,
                                        bool *cancell_done_out);

/**
 * Job: Cancel.
 *
 * See [`Job::cancel_async`].
 *
 * # Receiver
 * - `job` - job.
 *
 * # Returns
 * - `cancel_job_out` - cancel job. To dispose, call [`tsurugi_ffi_cancel_job_dispose`](crate::job::cancel_job::tsurugi_ffi_cancel_job_dispose).
 */
TsurugiFfiRc tsurugi_ffi_job_cancel_async(TsurugiFfiContextHandle context,
                                          TsurugiFfiJobHandle job,
                                          TsurugiFfiCancelJobHandle *cancel_job_out);

/**
 * Job: Close.
 *
 * See [`Job::close`].
 *
 * Note: Close is called in [`tsurugi_ffi_job_dispose`].
 *
 * # Receiver
 * - `job` - job.
 */
TsurugiFfiRc tsurugi_ffi_job_close(TsurugiFfiContextHandle context, TsurugiFfiJobHandle job);

/**
 * Job: Dispose.
 *
 * # Receiver
 * - `job` - job.
 */
void tsurugi_ffi_job_dispose(TsurugiFfiJobHandle job);

/**
 * Initialize env_logger.
 */
TsurugiFfiRc tsurugi_ffi_env_logger_init(void);

/**
 * SqlColumn: Get name.
 *
 * See [`SqlColumn::name`].
 *
 * # Receiver
 * - `sql_column` - Sql column.
 *
 * # Returns
 * - `name_out` - column name.
 */
TsurugiFfiRc tsurugi_ffi_sql_column_get_name(TsurugiFfiContextHandle context,
                                             TsurugiFfiSqlColumnHandle sql_column,
                                             TsurugiFfiStringHandle *name_out);

/**
 * SqlColumn: Get AtomType.
 *
 * See [`SqlColumn::atom_type`].
 *
 * # Receiver
 * - `sql_column` - Sql column.
 *
 * # Returns
 * - `atom_type_out` - column type.
 */
TsurugiFfiRc tsurugi_ffi_sql_column_get_atom_type(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlColumnHandle sql_column,
                                                  TsurugiFfiAtomType *atom_type_out);

/**
 * SqlColumn: Get length for data types.
 *
 * See [`SqlColumn::length`].
 *
 * # Receiver
 * - `sql_column` - Sql column.
 *
 * # Returns
 * - `provided_out` - Whether length or arbitrary_length is provided.
 * - `length_out` - defined length. Valid when `arbitrary_length` is `false`.
 * - `arbitrary_length_out` - arbitrary length (*).
 */
TsurugiFfiRc tsurugi_ffi_sql_column_get_length(TsurugiFfiContextHandle context,
                                               TsurugiFfiSqlColumnHandle sql_column,
                                               bool *provided_out,
                                               uint32_t *length_out,
                                               bool *arbitrary_length_out);

/**
 * SqlColumn: Get precision for decimal types.
 *
 * See [`SqlColumn::precision`].
 *
 * # Receiver
 * - `sql_column` - Sql column.
 *
 * # Returns
 * - `provided_out` - Whether precision or arbitrary_precision is provided.
 * - `precision_out` - defined precision. Valid when `arbitrary_precision` is `false`.
 * - `arbitrary_precision_out` - arbitrary precision (*).
 */
TsurugiFfiRc tsurugi_ffi_sql_column_get_precision(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlColumnHandle sql_column,
                                                  bool *provided_out,
                                                  uint32_t *precision_out,
                                                  bool *arbitrary_precision_out);

/**
 * SqlColumn: Get scale for decimal types.
 *
 * See [`SqlColumn::scale`].
 *
 * # Receiver
 * - `sql_column` - Sql column.
 *
 * # Returns
 * - `provided_out` - Whether scale or arbitrary_scale is provided.
 * - `scale_out` - defined scale. Valid when `arbitrary_scale` is `false`.
 * - `arbitrary_scale_out` - arbitrary scale (*).
 */
TsurugiFfiRc tsurugi_ffi_sql_column_get_scale(TsurugiFfiContextHandle context,
                                              TsurugiFfiSqlColumnHandle sql_column,
                                              bool *provided_out,
                                              uint32_t *scale_out,
                                              bool *arbitrary_scale_out);

/**
 * SqlColumn: Whether the column type is nullable.
 *
 * See [`SqlColumn::nullable`].
 *
 * # Receiver
 * - `sql_column` - Sql column.
 *
 * # Returns
 * - `provided_out` - Whether nullable is provided.
 * - `nullable_out` - Whether the column is nullable.
 */
TsurugiFfiRc tsurugi_ffi_sql_column_get_nullable(TsurugiFfiContextHandle context,
                                                 TsurugiFfiSqlColumnHandle sql_column,
                                                 bool *provided_out,
                                                 bool *nullable_out);

/**
 * SqlColumn: Whether the column type is varying.
 *
 * See [`SqlColumn::varying`].
 *
 * # Receiver
 * - `sql_column` - Sql column.
 *
 * # Returns
 * - `provided_out` - Whether varying is provided.
 * - `varying_out` - Whether the column is varying.
 */
TsurugiFfiRc tsurugi_ffi_sql_column_get_varying(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlColumnHandle sql_column,
                                                bool *provided_out,
                                                bool *varying_out);

/**
 * SqlColumn: Get description.
 *
 * See [`SqlColumn::description`].
 *
 * # Receiver
 * - `sql_column` - Sql column.
 *
 * # Returns
 * - `description_out` - column description (nullable).
 */
TsurugiFfiRc tsurugi_ffi_sql_column_get_description(TsurugiFfiContextHandle context,
                                                    TsurugiFfiSqlColumnHandle sql_column,
                                                    TsurugiFfiStringHandle *description_out);

/**
 * SqlColumn: Dispose.
 *
 * # Receiver
 * - `sql_column` - Sql column.
 */
void tsurugi_ffi_sql_column_dispose(TsurugiFfiSqlColumnHandle sql_column);

/**
 * SqlExecuteResult: Get counters.
 *
 * See [`SqlExecuteResult::counters`].
 *
 * # Receiver
 * - `execute_result` - Execute result of SQL statement.
 *
 * # Returns
 * - `counters_keys_out` - counters key (int array).
 * - `counters_rows_out` - counters rows (long array).
 * - `counters_size_out` - `counters_keys_out`, `counters_rows_out` size \[number of counters\].
 */
TsurugiFfiRc tsurugi_ffi_sql_execute_result_get_counters(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlExecuteResultHandle execute_result,
                                                         const TsurugiFfiSqlCounterType **counters_keys_out,
                                                         const int64_t **counters_rows_out,
                                                         uint32_t *counters_size_out);

/**
 * SqlExecuteResult: Get inserted rows.
 *
 * See [`SqlExecuteResult::inserted_rows`].
 *
 * # Receiver
 * - `execute_result` - Execute result of SQL statement.
 *
 * # Returns
 * - `rows_out` - rows.
 */
TsurugiFfiRc tsurugi_ffi_sql_execute_result_get_inserted_rows(TsurugiFfiContextHandle context,
                                                              TsurugiFfiSqlExecuteResultHandle execute_result,
                                                              int64_t *rows_out);

/**
 * SqlExecuteResult: Get updated rows.
 *
 * See [`SqlExecuteResult::updated_rows`].
 *
 * # Receiver
 * - `execute_result` - Execute result of SQL statement.
 *
 * # Returns
 * - `rows_out` - rows.
 */
TsurugiFfiRc tsurugi_ffi_sql_execute_result_get_updated_rows(TsurugiFfiContextHandle context,
                                                             TsurugiFfiSqlExecuteResultHandle execute_result,
                                                             int64_t *rows_out);

/**
 * SqlExecuteResult: Get merged rows.
 *
 * See [`SqlExecuteResult::merged_rows`].
 *
 * # Receiver
 * - `execute_result` - Execute result of SQL statement.
 *
 * # Returns
 * - `rows_out` - rows.
 */
TsurugiFfiRc tsurugi_ffi_sql_execute_result_get_merged_rows(TsurugiFfiContextHandle context,
                                                            TsurugiFfiSqlExecuteResultHandle execute_result,
                                                            int64_t *rows_out);

/**
 * SqlExecuteResult: Get deleted rows.
 *
 * See [`SqlExecuteResult::deleted_rows`].
 *
 * # Receiver
 * - `execute_result` - Execute result of SQL statement.
 *
 * # Returns
 * - `rows_out` - rows.
 */
TsurugiFfiRc tsurugi_ffi_sql_execute_result_get_deleted_rows(TsurugiFfiContextHandle context,
                                                             TsurugiFfiSqlExecuteResultHandle execute_result,
                                                             int64_t *rows_out);

/**
 * SqlExecuteResult: Get total rows.
 *
 * See [`SqlExecuteResult::rows`].
 *
 * # Receiver
 * - `execute_result` - Execute result of SQL statement.
 *
 * # Returns
 * - `rows_out` - rows.
 */
TsurugiFfiRc tsurugi_ffi_sql_execute_result_get_rows(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlExecuteResultHandle execute_result,
                                                     int64_t *rows_out);

/**
 * SqlExecuteResult: Dispose.
 *
 * # Receiver
 * - `execute_result` - Execute result of SQL statement.
 */
void tsurugi_ffi_sql_execute_result_dispose(TsurugiFfiSqlExecuteResultHandle execute_result);

/**
 * SqlExplainResult: Get format id.
 *
 * See [`SqlExplainResult::format_id`].
 *
 * # Receiver
 * - `explain_result` - Explain result of SQL statement.
 *
 * # Returns
 * - `format_id_out` - format id.
 */
TsurugiFfiRc tsurugi_ffi_sql_explain_result_get_format_id(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlExplainResultHandle explain_result,
                                                          TsurugiFfiStringHandle *format_id_out);

/**
 * SqlExplainResult: Get format version.
 *
 * See [`SqlExplainResult::format_version`].
 *
 * # Receiver
 * - `explain_result` - Explain result of SQL statement.
 *
 * # Returns
 * - `format_version_out` - format version.
 */
TsurugiFfiRc tsurugi_ffi_sql_explain_result_get_format_version(TsurugiFfiContextHandle context,
                                                               TsurugiFfiSqlExplainResultHandle explain_result,
                                                               uint64_t *format_version_out);

/**
 * SqlExplainResult: Get contents.
 *
 * See [`SqlExplainResult::contents`].
 *
 * # Receiver
 * - `explain_result` - Explain result of SQL statement.
 *
 * # Returns
 * - `contents_out` - contents.
 */
TsurugiFfiRc tsurugi_ffi_sql_explain_result_get_contents(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlExplainResultHandle explain_result,
                                                         TsurugiFfiStringHandle *contents_out);

/**
 * SqlExplainResult: Get columns size.
 *
 * See [`SqlExplainResult::columns`].
 *
 * # Receiver
 * - `explain_result` - Explain result of SQL statement.
 *
 * # Returns
 * - `size_out` - number of columns \[number of columns\].
 */
TsurugiFfiRc tsurugi_ffi_explain_result_get_columns_size(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlExplainResultHandle explain_result,
                                                         uint32_t *size_out);

/**
 * SqlExplainResult: Get columns value.
 *
 * See [`SqlExplainResult::columns`].
 *
 * # Receiver
 * - `explain_result` - Explain result of SQL statement.
 *
 * # Parameters
 * - `index` - column index \[0..tsurugi_ffi_table_metadata_get_columns_size()-1\].
 *
 * # Returns
 * - `sql_column_out` - column. To dispose, call [`tsurugi_ffi_sql_column_dispose`](crate::service::sql::column::tsurugi_ffi_sql_column_dispose).
 */
TsurugiFfiRc tsurugi_ffi_explain_result_get_columns_value(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlExplainResultHandle explain_result,
                                                          uint32_t index,
                                                          TsurugiFfiSqlColumnHandle *sql_column_out);

/**
 * SqlExplainResult: Dispose.
 *
 * # Receiver
 * - `explain_result` - Explain result of SQL statement.
 */
void tsurugi_ffi_sql_explain_result_dispose(TsurugiFfiSqlExplainResultHandle explain_result);

/**
 * SqlParameter: Creates a null parameter.
 *
 * See [`SqlParameter::null`].
 *
 * # Parameters
 * - `name` - parameter name.
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_null(TsurugiFfiContextHandle context,
                                            TsurugiFfiStringHandle name,
                                            TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of boolean (boolean).
 *
 * See [`SqlParameter::of`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `value` - parameter value.
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_boolean(TsurugiFfiContextHandle context,
                                                  TsurugiFfiStringHandle name,
                                                  bool value,
                                                  TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of int4 (int).
 *
 * See [`SqlParameter::of`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `value` - parameter value.
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_int4(TsurugiFfiContextHandle context,
                                               TsurugiFfiStringHandle name,
                                               int32_t value,
                                               TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of int8 (bigint).
 *
 * See [`SqlParameter::of`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `value` - parameter value.
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_int8(TsurugiFfiContextHandle context,
                                               TsurugiFfiStringHandle name,
                                               int64_t value,
                                               TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of float4 (real).
 *
 * See [`SqlParameter::of`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `value` - parameter value.
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_float4(TsurugiFfiContextHandle context,
                                                 TsurugiFfiStringHandle name,
                                                 float value,
                                                 TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of float8 (double).
 *
 * See [`SqlParameter::of`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `value` - parameter value.
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_float8(TsurugiFfiContextHandle context,
                                                 TsurugiFfiStringHandle name,
                                                 double value,
                                                 TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of decimal.
 *
 * See [`SqlParameter::of`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `unscaled_value` - unscaled value of decimal.
 * - `unscaled_value_size` - `unscaled_value` size \[byte\].
 * - `exponent` - exponent of decimal.
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_decimal(TsurugiFfiContextHandle context,
                                                  TsurugiFfiStringHandle name,
                                                  TsurugiFfiByteArrayHandle unscaled_value,
                                                  uint32_t unscaled_value_size,
                                                  int32_t exponent,
                                                  TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of decimal.
 *
 * See [`SqlParameter::of`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `unscaled_value_high` - unscaled value of decimal (high 64 bit).
 * - `unscaled_value_low` - unscaled value of decimal (low 64 bit).
 * - `exponent` - exponent of decimal.
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_decimal_i128(TsurugiFfiContextHandle context,
                                                       TsurugiFfiStringHandle name,
                                                       int64_t unscaled_value_high,
                                                       uint64_t unscaled_value_low,
                                                       int32_t exponent,
                                                       TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of character (char/varchar).
 *
 * See [`SqlParameter::of`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `value` - parameter value.
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_character(TsurugiFfiContextHandle context,
                                                    TsurugiFfiStringHandle name,
                                                    TsurugiFfiStringHandle value,
                                                    TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of octet (binary/varbinary).
 *
 * See [`SqlParameter::of`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `value` - parameter value.
 * - `value_size` - `value` size \[byte\].
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_octet(TsurugiFfiContextHandle context,
                                                TsurugiFfiStringHandle name,
                                                TsurugiFfiByteArrayHandle value,
                                                uint64_t value_size,
                                                TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of date.
 *
 * See [`SqlParameter::of`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `epoch_days` - parameter value (number of days offset of epoch 1970-01-01).
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_date(TsurugiFfiContextHandle context,
                                               TsurugiFfiStringHandle name,
                                               int64_t epoch_days,
                                               TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of time of day (time).
 *
 * See [`SqlParameter::of`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `nanoseconds_of_day` - parameter value (nanoseconds since 00:00:00).
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_time_of_day(TsurugiFfiContextHandle context,
                                                      TsurugiFfiStringHandle name,
                                                      uint64_t nanoseconds_of_day,
                                                      TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of time point (timestamp).
 *
 * See [`SqlParameter::of`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `epoch_seconds` - parameter value (number of seconds offset of epoch 1970-01-01).
 * - `nanos` - parameter value (nanoseconds adjustment \[0, 10^9-1\]).
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_time_point(TsurugiFfiContextHandle context,
                                                     TsurugiFfiStringHandle name,
                                                     int64_t epoch_seconds,
                                                     uint32_t nanos,
                                                     TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of time of day with time zone (time with time zone).
 *
 * See [`SqlParameter::of`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `nanoseconds_of_day` - parameter value (nanoseconds since 00:00:00).
 * - `time_zone_offset` - parameter value (timezone offset in minute).
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_time_of_day_with_time_zone(TsurugiFfiContextHandle context,
                                                                     TsurugiFfiStringHandle name,
                                                                     uint64_t nanoseconds_of_day,
                                                                     int32_t time_zone_offset,
                                                                     TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of time point with time zone (timestamp with time zone).
 *
 * See [`SqlParameter::of`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `epoch_seconds` - parameter value (number of seconds offset of epoch 1970-01-01).
 * - `nanos` - parameter value (nanoseconds adjustment \[0, 10^9-1\]).
 * - `time_zone_offset` - parameter value (timezone offset in minute).
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_time_point_with_time_zone(TsurugiFfiContextHandle context,
                                                                    TsurugiFfiStringHandle name,
                                                                    int64_t epoch_seconds,
                                                                    uint32_t nanos,
                                                                    int32_t time_zone_offset,
                                                                    TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of blob.
 *
 * See [`TgBlob::new`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `path` - parameter value (path of file).
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_blob(TsurugiFfiContextHandle context,
                                               TsurugiFfiStringHandle name,
                                               TsurugiFfiStringHandle path,
                                               TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of blob.
 *
 * See [`TgBlob::from`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `value` - parameter value.
 * - `value_size` - `value` size \[byte\].
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_blob_contents(TsurugiFfiContextHandle context,
                                                        TsurugiFfiStringHandle name,
                                                        TsurugiFfiByteArrayHandle value,
                                                        uint64_t value_size,
                                                        TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of clob.
 *
 * See [`TgClob::new`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `path` - parameter value (path of file).
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_clob(TsurugiFfiContextHandle context,
                                               TsurugiFfiStringHandle name,
                                               TsurugiFfiStringHandle path,
                                               TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Creates a parameter of clob.
 *
 * See [`TgClob::from`].
 *
 * # Parameters
 * - `name` - parameter name.
 * - `value` - parameter value.
 *
 * # Returns
 * - `parameter_out` - parameter. To dispose, call [`tsurugi_ffi_sql_parameter_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_of_clob_contents(TsurugiFfiContextHandle context,
                                                        TsurugiFfiStringHandle name,
                                                        TsurugiFfiStringHandle value,
                                                        TsurugiFfiSqlParameterHandle *parameter_out);

/**
 * SqlParameter: Get name.
 *
 * See [`SqlParameter::name`].
 *
 * # Receiver
 * - `parameter` - Sql parameter.
 *
 * # Returns
 * - `name_out` - parameter name.
 */
TsurugiFfiRc tsurugi_ffi_sql_parameter_get_name(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlParameterHandle parameter,
                                                TsurugiFfiStringHandle *name_out);

/**
 * SqlParameter: Dispose.
 *
 * # Receiver
 * - `parameter` - Sql parameter.
 */
void tsurugi_ffi_sql_parameter_dispose(TsurugiFfiSqlParameterHandle parameter);

/**
 * SqlPlaceholder: Creates a placeholder.
 *
 * See [`SqlPlaceholder::of_atom_type`].
 *
 * # Parameters
 * - `name` - placeholder name.
 * - `atom_type` - parameter type.
 *
 * # Returns
 * - `placeholder_out` - placeholder. To dispose, call [`tsurugi_ffi_sql_placeholder_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_placeholder_of_atom_type(TsurugiFfiContextHandle context,
                                                      TsurugiFfiStringHandle name,
                                                      TsurugiFfiAtomType atom_type,
                                                      TsurugiFfiSqlPlaceholderHandle *placeholder_out);

/**
 * SqlPlaceholder: Get name.
 *
 * See [`SqlPlaceholder::name`].
 *
 * # Receiver
 * - `placeholder` - Sql placeholder.
 *
 * # Returns
 * - `name_out` - placeholder name.
 */
TsurugiFfiRc tsurugi_ffi_sql_placeholder_get_name(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlPlaceholderHandle placeholder,
                                                  TsurugiFfiStringHandle *name_out);

/**
 * SqlPlaceholder: Get AtomType.
 *
 * See [`SqlPlaceholder::atom_type`].
 *
 * # Receiver
 * - `placeholder` - Sql placeholder.
 *
 * # Returns
 * - `atom_type_out` - placeholder type.
 */
TsurugiFfiRc tsurugi_ffi_sql_placeholder_get_atom_type(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlPlaceholderHandle placeholder,
                                                       TsurugiFfiAtomType *atom_type_out);

/**
 * SqlPlaceholder: Dispose.
 *
 * # Receiver
 * - `placeholder` - Sql placeholder.
 */
void tsurugi_ffi_sql_placeholder_dispose(TsurugiFfiSqlPlaceholderHandle placeholder);

/**
 * SqlPreparedStatement: Has result records.
 *
 * See [`SqlPreparedStatement::has_result_records`].
 *
 * # Receiver
 * - `prepared_statement` - Sql prepared statement.
 *
 * # Returns
 * - `has_result_records_out` - `true`: Has result records (query) / `false`: No result records.
 */
TsurugiFfiRc tsurugi_ffi_sql_prepared_statement_has_result_records(TsurugiFfiContextHandle context,
                                                                   TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                                   bool *has_result_records_out);

/**
 * SqlPreparedStatement: Set close timeout.
 *
 * See [`SqlPreparedStatement::set_close_timeout`].
 *
 * # Receiver
 * - `prepared_statement` - Sql prepared statement.
 *
 * # Parameters
 * - `close_timeout` - timeout time \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_sql_prepared_statement_set_close_timeout(TsurugiFfiContextHandle context,
                                                                  TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                                  TsurugiFfiDuration close_timeout);

/**
 * SqlPreparedStatement: Get close timeout.
 *
 * See [`SqlPreparedStatement::close_timeout`].
 *
 * # Receiver
 * - `prepared_statement` - Sql prepared statement.
 *
 * # Returns
 * - `close_timeout_out` - timeout time \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_sql_prepared_statement_get_close_timeout(TsurugiFfiContextHandle context,
                                                                  TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                                  TsurugiFfiDuration *close_timeout_out);

/**
 * SqlPreparedStatement: Close.
 *
 * See [`SqlPreparedStatement::close`].
 *
 * Note: Close is called in [`tsurugi_ffi_sql_prepared_statement_dispose`].
 *
 * # Receiver
 * - `prepared_statement` - Sql prepared statement.
 */
TsurugiFfiRc tsurugi_ffi_sql_prepared_statement_close(TsurugiFfiContextHandle context,
                                                      TsurugiFfiSqlPreparedStatementHandle prepared_statement);

/**
 * SqlPreparedStatement: Close.
 *
 * See [`SqlPreparedStatement::close_for`].
 *
 * Note: Close is called in [`tsurugi_ffi_sql_prepared_statement_dispose`].
 *
 * # Receiver
 * - `prepared_statement` - Sql prepared statement.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_sql_prepared_statement_close_for(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                          TsurugiFfiDuration timeout);

/**
 * SqlPreparedStatement: Check if the session is closed.
 *
 * See [`SqlPreparedStatement::is_closed`].
 *
 * # Receiver
 * - `prepared_statement` - Sql prepared statement.
 *
 * # Returns
 * - `is_closed_out` - `true`: Already closed / `false`: Not closed.
 */
TsurugiFfiRc tsurugi_ffi_sql_prepared_statement_is_closed(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                          bool *is_closed_out);

/**
 * SqlPreparedStatement: Dispose.
 *
 * # Receiver
 * - `prepared_statement` - Sql prepared statement.
 */
void tsurugi_ffi_sql_prepared_statement_dispose(TsurugiFfiSqlPreparedStatementHandle prepared_statement);

/**
 * SqlQueryResult: Set default timeout.
 *
 * See [`SqlQueryResult::set_default_timeout`].
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_set_default_timeout(TsurugiFfiContextHandle context,
                                                              TsurugiFfiSqlQueryResultHandle query_result,
                                                              TsurugiFfiDuration timeout);

/**
 * SqlQueryResult: Get default timeout.
 *
 * See [`SqlQueryResult::default_timeout`].
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `default_timeout_out` - timeout time \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_get_default_timeout(TsurugiFfiContextHandle context,
                                                              TsurugiFfiSqlQueryResultHandle query_result,
                                                              TsurugiFfiDuration *default_timeout_out);

/**
 * SqlQueryResult: Get metadata.
 *
 * See [`SqlQueryResult::get_metadata`].
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `query_result_metadata_out` - metadata. To dispose, call [`tsurugi_ffi_sql_query_result_metadata_dispose`](crate::service::sql::query_result_metadata::tsurugi_ffi_sql_query_result_metadata_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_get_metadata(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlQueryResultHandle query_result,
                                                       TsurugiFfiSqlQueryResultMetadataHandle *query_result_metadata_out);

/**
 * SqlQueryResult: Next row.
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::next_row`].
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `has_row_out` - `true`: Has next row / `false`: No next row.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_next_row(TsurugiFfiContextHandle context,
                                                   TsurugiFfiSqlQueryResultHandle query_result,
                                                   bool *has_row_out);

/**
 * SqlQueryResult: Next row.
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::next_row_for`].
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `has_row_out` - `true`: Has next row / `false`: No next row.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_next_row_for(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlQueryResultHandle query_result,
                                                       TsurugiFfiDuration timeout,
                                                       bool *has_row_out);

/**
 * SqlQueryResult: Next column.
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::next_column`].
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `has_column_out` - `true`: Has next column / `false`: No next column.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_next_column(TsurugiFfiContextHandle context,
                                                      TsurugiFfiSqlQueryResultHandle query_result,
                                                      bool *has_column_out);

/**
 * SqlQueryResult: Next column.
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::next_column_for`].
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `has_column_out` - `true`: Has next column / `false`: No next column.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_next_column_for(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlQueryResultHandle query_result,
                                                          TsurugiFfiDuration timeout,
                                                          bool *has_column_out);

/**
 * SqlQueryResult: Whether the column on this cursor is `NULL` or not.
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::is_null`].
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `is_null_out` - `true`: Is null / `false`: Not null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_is_null(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlQueryResultHandle query_result,
                                                  bool *is_null_out);

/**
 * SqlQueryResult: fetch boolean (boolean).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `value_out` - value.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_boolean(TsurugiFfiContextHandle context,
                                                        TsurugiFfiSqlQueryResultHandle query_result,
                                                        bool *value_out);

/**
 * SqlQueryResult: fetch boolean (boolean).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `value_out` - value.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_boolean(TsurugiFfiContextHandle context,
                                                            TsurugiFfiSqlQueryResultHandle query_result,
                                                            TsurugiFfiDuration timeout,
                                                            bool *value_out);

/**
 * SqlQueryResult: fetch int4 (int).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `value_out` - value.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_int4(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlQueryResultHandle query_result,
                                                     int32_t *value_out);

/**
 * SqlQueryResult: fetch int4 (int).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `value_out` - value.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_int4(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlQueryResultHandle query_result,
                                                         TsurugiFfiDuration timeout,
                                                         int32_t *value_out);

/**
 * SqlQueryResult: fetch int8 (bigint).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `value_out` - value.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_int8(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlQueryResultHandle query_result,
                                                     int64_t *value_out);

/**
 * SqlQueryResult: fetch int8 (bigint).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `value_out` - value.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_int8(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlQueryResultHandle query_result,
                                                         TsurugiFfiDuration timeout,
                                                         int64_t *value_out);

/**
 * SqlQueryResult: fetch float4 (real).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `value_out` - value.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_float4(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlQueryResultHandle query_result,
                                                       float *value_out);

/**
 * SqlQueryResult: fetch float4 (real).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `value_out` - value.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_float4(TsurugiFfiContextHandle context,
                                                           TsurugiFfiSqlQueryResultHandle query_result,
                                                           TsurugiFfiDuration timeout,
                                                           float *value_out);

/**
 * SqlQueryResult: fetch float8 (double).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `value_out` - value.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_float8(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlQueryResultHandle query_result,
                                                       double *value_out);

/**
 * SqlQueryResult: fetch float8 (double).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `value_out` - value.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_float8(TsurugiFfiContextHandle context,
                                                           TsurugiFfiSqlQueryResultHandle query_result,
                                                           TsurugiFfiDuration timeout,
                                                           double *value_out);

/**
 * SqlQueryResult: fetch decimal.
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `unscaled_value_bytes_out` - unscaled value of decimal.
 * - `unscaled_value_bytes_size_out` - `unscaled_value_bytes_out` size \[byte\].
 * - `unscaled_value_out` - unscaled value of decimal if `unscaled_value_bytes_out` is null (`unscaled_value_bytes_size_out` = 0).
 * - `exponent_out` - exponent of decimal.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_decimal(TsurugiFfiContextHandle context,
                                                        TsurugiFfiSqlQueryResultHandle query_result,
                                                        TsurugiFfiByteArrayHandle *unscaled_value_bytes_out,
                                                        uint32_t *unscaled_value_bytes_size_out,
                                                        int64_t *unscaled_value_out,
                                                        int32_t *exponent_out);

/**
 * SqlQueryResult: fetch decimal.
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `unscaled_value_bytes_out` - unscaled value of decimal.
 * - `unscaled_value_bytes_size_out` - `unscaled_value_bytes_out` size \[byte\].
 * - `unscaled_value_out` - unscaled value of decimal if `unscaled_value_bytes_out` is null (`unscaled_value_bytes_size_out` = 0).
 * - `exponent_out` - exponent of decimal.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_decimal(TsurugiFfiContextHandle context,
                                                            TsurugiFfiSqlQueryResultHandle query_result,
                                                            TsurugiFfiDuration timeout,
                                                            TsurugiFfiByteArrayHandle *unscaled_value_bytes_out,
                                                            uint32_t *unscaled_value_bytes_size_out,
                                                            int64_t *unscaled_value_out,
                                                            int32_t *exponent_out);

/**
 * SqlQueryResult: fetch decimal.
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `unscaled_value_high_out` - unscaled value of decimal (high 64bit).
 * - `unscaled_value_low_out` - unscaled value of decimal (low 64bit).
 * - `exponent_out` - exponent of decimal.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_decimal_i128(TsurugiFfiContextHandle context,
                                                             TsurugiFfiSqlQueryResultHandle query_result,
                                                             int64_t *unscaled_value_high_out,
                                                             uint64_t *unscaled_value_low_out,
                                                             int32_t *exponent_out);

/**
 * SqlQueryResult: fetch decimal.
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `unscaled_value_high_out` - unscaled value of decimal (high 64bit).
 * - `unscaled_value_low_out` - unscaled value of decimal (low 64bit).
 * - `exponent_out` - exponent of decimal.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_decimal_i128(TsurugiFfiContextHandle context,
                                                                 TsurugiFfiSqlQueryResultHandle query_result,
                                                                 TsurugiFfiDuration timeout,
                                                                 int64_t *unscaled_value_high_out,
                                                                 uint64_t *unscaled_value_low_out,
                                                                 int32_t *exponent_out);

/**
 * SqlQueryResult: fetch character (char/varchar).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `value_out` - value.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_character(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlQueryResultHandle query_result,
                                                          TsurugiFfiStringHandle *value_out);

/**
 * SqlQueryResult: fetch character (char/varchar).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `value_out` - value.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_character(TsurugiFfiContextHandle context,
                                                              TsurugiFfiSqlQueryResultHandle query_result,
                                                              TsurugiFfiDuration timeout,
                                                              TsurugiFfiStringHandle *value_out);

/**
 * SqlQueryResult: fetch octet (binary/varbinary).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `value_out` - value.
 * - `size_out` - `value_out` size \[byte\].
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_octet(TsurugiFfiContextHandle context,
                                                      TsurugiFfiSqlQueryResultHandle query_result,
                                                      TsurugiFfiByteArrayHandle *value_out,
                                                      uint64_t *size_out);

/**
 * SqlQueryResult: fetch octet (binary/varbinary).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `value_out` - value.
 * - `size_out` - `value_out` size \[byte\].
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_octet(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlQueryResultHandle query_result,
                                                          TsurugiFfiDuration timeout,
                                                          TsurugiFfiByteArrayHandle *value_out,
                                                          uint64_t *size_out);

/**
 * SqlQueryResult: fetch date.
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `value_out` - number of days offset of epoch 1970-01-01.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_date(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlQueryResultHandle query_result,
                                                     int64_t *value_out);

/**
 * SqlQueryResult: fetch date.
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `value_out` - number of days offset of epoch 1970-01-01.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_date(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlQueryResultHandle query_result,
                                                         TsurugiFfiDuration timeout,
                                                         int64_t *value_out);

/**
 * SqlQueryResult: fetch time of day (time).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `value_out` - nanoseconds since 00:00:00.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_time_of_day(TsurugiFfiContextHandle context,
                                                            TsurugiFfiSqlQueryResultHandle query_result,
                                                            uint64_t *value_out);

/**
 * SqlQueryResult: fetch time of day (time).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `value_out` - nanoseconds since 00:00:00.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_time_of_day(TsurugiFfiContextHandle context,
                                                                TsurugiFfiSqlQueryResultHandle query_result,
                                                                TsurugiFfiDuration timeout,
                                                                uint64_t *value_out);

/**
 * SqlQueryResult: fetch time point (timestamp).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `value_out` - number of seconds offset of epoch 1970-01-01.
 * - `nanos_out` - nanoseconds adjustment \[0, 10^9-1\].
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_time_point(TsurugiFfiContextHandle context,
                                                           TsurugiFfiSqlQueryResultHandle query_result,
                                                           int64_t *value_out,
                                                           uint32_t *nanos_out);

/**
 * SqlQueryResult: fetch time point (timestamp).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `value_out` - number of seconds offset of epoch 1970-01-01.
 * - `nanos_out` - nanoseconds adjustment \[0, 10^9-1\].
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_time_point(TsurugiFfiContextHandle context,
                                                               TsurugiFfiSqlQueryResultHandle query_result,
                                                               TsurugiFfiDuration timeout,
                                                               int64_t *value_out,
                                                               uint32_t *nanos_out);

/**
 * SqlQueryResult: fetch time of day with time zone (time with time zone).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `value_out` - nanoseconds since 00:00:00.
 * - `time_zone_offset_out` - timezone offset in minute.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_time_of_day_with_time_zone(TsurugiFfiContextHandle context,
                                                                           TsurugiFfiSqlQueryResultHandle query_result,
                                                                           uint64_t *value_out,
                                                                           int32_t *time_zone_offset_out);

/**
 * SqlQueryResult: fetch time of day with time zone (time with time zone).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `value_out` - nanoseconds since 00:00:00.
 * - `time_zone_offset_out` - timezone offset in minute.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_time_of_day_with_time_zone(TsurugiFfiContextHandle context,
                                                                               TsurugiFfiSqlQueryResultHandle query_result,
                                                                               TsurugiFfiDuration timeout,
                                                                               uint64_t *value_out,
                                                                               int32_t *time_zone_offset_out);

/**
 * SqlQueryResult: fetch time point with time zone (timestamp with time zone).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `value_out` - number of seconds offset of epoch 1970-01-01.
 * - `nanos_out` - nanoseconds adjustment \[0, 10^9-1\].
 * - `time_zone_offset_out` - timezone offset in minute.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_time_point_with_time_zone(TsurugiFfiContextHandle context,
                                                                          TsurugiFfiSqlQueryResultHandle query_result,
                                                                          int64_t *value_out,
                                                                          uint32_t *nanos_out,
                                                                          int32_t *time_zone_offset_out);

/**
 * SqlQueryResult: fetch time point with time zone (timestamp with time zone).
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `value_out` - number of seconds offset of epoch 1970-01-01.
 * - `nanos_out` - nanoseconds adjustment \[0, 10^9-1\].
 * - `time_zone_offset_out` - timezone offset in minute.
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_time_point_with_time_zone(TsurugiFfiContextHandle context,
                                                                              TsurugiFfiSqlQueryResultHandle query_result,
                                                                              TsurugiFfiDuration timeout,
                                                                              int64_t *value_out,
                                                                              uint32_t *nanos_out,
                                                                              int32_t *time_zone_offset_out);

/**
 * SqlQueryResult: fetch blob.
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `blob_reference_out` - blob reference. To dispose, call [`tsurugi_ffi_blob_reference_dispose`](crate::service::sql::type::blob::tsurugi_ffi_blob_reference_dispose).
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 *
 * See [`tsurugi_ffi_sql_client_read_blob`](crate::service::sql::tsurugi_ffi_sql_client_read_blob),
 *     [`tsurugi_ffi_sql_client_copy_blob_to`](crate::service::sql::tsurugi_ffi_sql_client_copy_blob_to).
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_blob(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlQueryResultHandle query_result,
                                                     TsurugiFfiBlobReferenceHandle *blob_reference_out);

/**
 * SqlQueryResult: fetch blob.
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `blob_reference_out` - blob reference. To dispose, call [`tsurugi_ffi_blob_reference_dispose`](crate::service::sql::type::blob::tsurugi_ffi_blob_reference_dispose).
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 *
 * See [`tsurugi_ffi_sql_client_read_blob_for`](crate::service::sql::tsurugi_ffi_sql_client_read_blob_for),
 *     [`tsurugi_ffi_sql_client_copy_blob_to_for`](crate::service::sql::tsurugi_ffi_sql_client_copy_blob_to_for).
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_blob(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlQueryResultHandle query_result,
                                                         TsurugiFfiDuration timeout,
                                                         TsurugiFfiBlobReferenceHandle *blob_reference_out);

/**
 * SqlQueryResult: fetch clob.
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Returns
 * - `clob_reference_out` - clob reference. To dispose, call [`tsurugi_ffi_clob_reference_dispose`](crate::service::sql::type::clob::tsurugi_ffi_clob_reference_dispose).
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 *
 * See [`tsurugi_ffi_sql_client_read_clob`](crate::service::sql::tsurugi_ffi_sql_client_read_clob),
 *     [`tsurugi_ffi_sql_client_copy_clob_to`](crate::service::sql::tsurugi_ffi_sql_client_copy_clob_to).
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_clob(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlQueryResultHandle query_result,
                                                     TsurugiFfiClobReferenceHandle *clob_reference_out);

/**
 * SqlQueryResult: fetch clob.
 *
 * This method can only be used while the transaction is alive.
 *
 * See [`SqlQueryResult::fetch_for`].
 *
 * Retrieves a value on the column of the cursor position.
 *
 * Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
 * You can only take once to retrieve the value on the column.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `clob_reference_out` - clob reference. To dispose, call [`tsurugi_ffi_clob_reference_dispose`](crate::service::sql::type::clob::tsurugi_ffi_clob_reference_dispose).
 *
 * Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
 *
 * See [`tsurugi_ffi_sql_client_read_clob_for`](crate::service::sql::tsurugi_ffi_sql_client_read_clob_for),
 *     [`tsurugi_ffi_sql_client_copy_clob_to_for`](crate::service::sql::tsurugi_ffi_sql_client_copy_clob_to_for).
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_clob(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlQueryResultHandle query_result,
                                                         TsurugiFfiDuration timeout,
                                                         TsurugiFfiClobReferenceHandle *clob_reference_out);

/**
 * SqlQueryResult: Dispose.
 *
 * # Receiver
 * - `query_result` - Sql query result.
 */
void tsurugi_ffi_sql_query_result_dispose(TsurugiFfiSqlQueryResultHandle query_result);

/**
 * SqlQueryResultMetadata: get columns size.
 *
 * See [`SqlQueryResultMetadata::columns`].
 *
 * # Receiver
 * - `query_result_metadata` - Sql query result metadata.
 *
 * # Returns
 * - `size_out` - number of columns \[number of columns\].
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_metadata_get_columns_size(TsurugiFfiContextHandle context,
                                                                    TsurugiFfiSqlQueryResultMetadataHandle query_result_metadata,
                                                                    uint32_t *size_out);

/**
 * SqlQueryResultMetadata: get columns value.
 *
 * See [`SqlQueryResultMetadata::columns`].
 *
 * # Receiver
 * - `query_result_metadata` - Sql query result metadata.
 *
 * # Parameters
 * - `index` - column index \[0..tsurugi_ffi_table_metadata_get_columns_size()-1\].
 *
 * # Returns
 * - `sql_column_out` - column. To dispose, call [`tsurugi_ffi_sql_column_dispose`](crate::service::sql::column::tsurugi_ffi_sql_column_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_query_result_metadata_get_columns_value(TsurugiFfiContextHandle context,
                                                                     TsurugiFfiSqlQueryResultMetadataHandle query_result_metadata,
                                                                     uint32_t index,
                                                                     TsurugiFfiSqlColumnHandle *sql_column_out);

/**
 * SqlQueryResultMetadata: Dispose.
 *
 * # Receiver
 * - `query_result_metadata` - Sql query result metadata.
 */
void tsurugi_ffi_sql_query_result_metadata_dispose(TsurugiFfiSqlQueryResultMetadataHandle query_result_metadata);

/**
 * SqlClient: Get service message version.
 *
 * See [`SqlClient::service_message_version`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Returns
 * - `version_out` - service message version.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_get_service_message_version(TsurugiFfiContextHandle context,
                                                                TsurugiFfiSqlClientHandle sql_client,
                                                                TsurugiFfiStringHandle *version_out);

/**
 * SqlClient: List tables.
 *
 * See [`SqlClient::list_tables`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Returns
 * - `table_list_out` - table list. To dispose, call [`tsurugi_ffi_table_list_dispose`](crate::service::sql::table_list::tsurugi_ffi_table_list_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_list_tables(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlClientHandle sql_client,
                                                TsurugiFfiTableListHandle *table_list_out);

/**
 * SqlClient: List tables.
 *
 * See [`SqlClient::list_tables_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `table_list_out` - table list. To dispose, call [`tsurugi_ffi_table_list_dispose`](crate::service::sql::table_list::tsurugi_ffi_table_list_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_list_tables_for(TsurugiFfiContextHandle context,
                                                    TsurugiFfiSqlClientHandle sql_client,
                                                    TsurugiFfiDuration timeout,
                                                    TsurugiFfiTableListHandle *table_list_out);

/**
 * SqlClient: List tables.
 *
 * See [`SqlClient::list_tables_async`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Returns
 * - `table_list_job_out` - Job for `TsurugiFfiTableListHandle`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 *   Handle taken from Job casts to `TsurugiFfiTableListHandle` and call [`tsurugi_ffi_table_list_dispose`](crate::service::sql::table_list::tsurugi_ffi_table_list_dispose) to dispose.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_list_tables_async(TsurugiFfiContextHandle context,
                                                      TsurugiFfiSqlClientHandle sql_client,
                                                      TsurugiFfiJobHandle *table_list_job_out);

/**
 * SqlClient: Get table metadata.
 *
 * See [`SqlClient::get_table_metadata`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `table_name` - table name.
 *
 * # Returns
 * - `table_metadata_out` - table metadata. To dispose, call [`tsurugi_ffi_table_metadata_dispose`](crate::service::sql::table_metadata::tsurugi_ffi_table_metadata_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_get_table_metadata(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlClientHandle sql_client,
                                                       TsurugiFfiStringHandle table_name,
                                                       TsurugiFfiTableMetadataHandle *table_metadata_out);

/**
 * SqlClient: Get table metadata.
 *
 * See [`SqlClient::get_table_metadata_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `table_name` - table name.
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `table_metadata_out` - table metadata. To dispose, call [`tsurugi_ffi_table_metadata_dispose`](crate::service::sql::table_metadata::tsurugi_ffi_table_metadata_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_get_table_metadata_for(TsurugiFfiContextHandle context,
                                                           TsurugiFfiSqlClientHandle sql_client,
                                                           TsurugiFfiStringHandle table_name,
                                                           TsurugiFfiDuration timeout,
                                                           TsurugiFfiTableMetadataHandle *table_metadata_out);

/**
 * SqlClient: Get table metadata.
 *
 * See [`SqlClient::get_table_metadata_async`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `table_name` - table name.
 *
 * # Returns
 * - `table_metadata_job_out` - Job for `TsurugiFfiTableMetadataHandle`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 *   Handle taken from Job casts to `TsurugiFfiTableMetadataHandle` and call [`tsurugi_ffi_table_metadata_dispose`](crate::service::sql::table_metadata::tsurugi_ffi_table_metadata_dispose) to dispose.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_get_table_metadata_async(TsurugiFfiContextHandle context,
                                                             TsurugiFfiSqlClientHandle sql_client,
                                                             TsurugiFfiStringHandle table_name,
                                                             TsurugiFfiJobHandle *table_metadata_job_out);

/**
 * SqlClient: Create prepared statement.
 *
 * See [`SqlClient::prepare`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `sql` - SQL satement.
 * - `placeholders` - placeholders (TsurugiFfiSqlPlaceholderHandle array).
 * - `placeholders_size` - `placeholders` size \[number of placeholders\].
 *
 * # Returns
 * - `prepared_statement_out` - prepared statement. To dispose, call [`tsurugi_ffi_sql_prepared_statement_dispose`](crate::service::sql::prepare::prepared_statement::tsurugi_ffi_sql_prepared_statement_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_prepare(TsurugiFfiContextHandle context,
                                            TsurugiFfiSqlClientHandle sql_client,
                                            TsurugiFfiStringHandle sql,
                                            const TsurugiFfiSqlPlaceholderHandle *placeholders,
                                            uint32_t placeholders_size,
                                            TsurugiFfiSqlPreparedStatementHandle *prepared_statement_out);

/**
 * SqlClient: Create prepared statement.
 *
 * See [`SqlClient::prepare_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `sql` - SQL satement.
 * - `placeholders` - placeholders (TsurugiFfiSqlPlaceholderHandle array).
 * - `placeholders_size` - `placeholders` size \[number of placeholders\].
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `prepared_statement_out` - prepared statement. To dispose, call [`tsurugi_ffi_sql_prepared_statement_dispose`](crate::service::sql::prepare::prepared_statement::tsurugi_ffi_sql_prepared_statement_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_prepare_for(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlClientHandle sql_client,
                                                TsurugiFfiStringHandle sql,
                                                const TsurugiFfiSqlPlaceholderHandle *placeholders,
                                                uint32_t placeholders_size,
                                                TsurugiFfiDuration timeout,
                                                TsurugiFfiSqlPreparedStatementHandle *prepared_statement_out);

/**
 * SqlClient: Create prepared statement.
 *
 * See [`SqlClient::prepare_async`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `sql` - SQL satement.
 * - `placeholders` - placeholders (TsurugiFfiSqlPlaceholderHandle array).
 * - `placeholders_size` - `placeholders` size \[number of placeholders\].
 *
 * # Returns
 * - `prepared_statement_job_out` - Job for `TsurugiFfiSqlPreparedStatementHandle`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 *   Handle taken from Job casts to `TsurugiFfiSqlPreparedStatementHandle` and call [`tsurugi_ffi_sql_prepared_statement_dispose`](crate::service::sql::prepare::prepared_statement::tsurugi_ffi_sql_prepared_statement_dispose) to dispose.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_prepare_async(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlClientHandle sql_client,
                                                  TsurugiFfiStringHandle sql,
                                                  const TsurugiFfiSqlPlaceholderHandle *placeholders,
                                                  uint32_t placeholders_size,
                                                  TsurugiFfiJobHandle *prepared_statement_job_out);

/**
 * SqlClient: Retrieves execution plan of the statement.
 *
 * See [`SqlClient::explain`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `sql` - SQL satement.
 *
 * # Returns
 * - `explain_result_out` - explain result. To dispose, call [`tsurugi_ffi_sql_explain_result_dispose`](crate::service::sql::explain::tsurugi_ffi_sql_explain_result_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_explain(TsurugiFfiContextHandle context,
                                            TsurugiFfiSqlClientHandle sql_client,
                                            TsurugiFfiStringHandle sql,
                                            TsurugiFfiSqlExplainResultHandle *explain_result_out);

/**
 * SqlClient: Retrieves execution plan of the statement.
 *
 * See [`SqlClient::explain_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `sql` - SQL satement.
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `explain_result_out` - explain result. To dispose, call [`tsurugi_ffi]_sql_explain_result_dispose`](crate::service::sql::explain::tsurugi_ffi_sql_explain_result_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_explain_for(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlClientHandle sql_client,
                                                TsurugiFfiStringHandle sql,
                                                TsurugiFfiDuration timeout,
                                                TsurugiFfiSqlExplainResultHandle *explain_result_out);

/**
 * SqlClient: Retrieves execution plan of the statement.
 *
 * See [`SqlClient::explain_async`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `sql` - SQL satement.
 *
 * # Returns
 * - `explain_result_job_out` - Job for `TsurugiFfiSqlExplainResultHandle`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 *   Handle taken from Job casts to `TsurugiFfiSqlExplainResultHandle` and call [`tsurugi_ffi_sql_explain_result_dispose`](crate::service::sql::explain::tsurugi_ffi_sql_explain_result_dispose) to dispose.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_explain_async(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlClientHandle sql_client,
                                                  TsurugiFfiStringHandle sql,
                                                  TsurugiFfiJobHandle *explain_result_job_out);

/**
 * SqlClient: Retrieves execution plan of the statement.
 *
 * See [`SqlClient::prepared_explain`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `prepared_statement` - prepared satement.
 * - `parameters` - parameters (TsurugiFfiSqlParameterHandle array).
 * - `parameters_size` - `parameters` size \[number of parameters\].
 *
 * # Returns
 * - `explain_result_out` - explain result. To dispose, call [`tsurugi_ffi_sql_explain_result_dispose`](crate::service::sql::explain::tsurugi_ffi_sql_explain_result_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_prepared_explain(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlClientHandle sql_client,
                                                     TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                     const TsurugiFfiSqlParameterHandle *parameters,
                                                     uint32_t parameters_size,
                                                     TsurugiFfiSqlExplainResultHandle *explain_result_out);

/**
 * SqlClient: Retrieves execution plan of the statement.
 *
 * See [`SqlClient::prepared_explain_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `prepared_statement` - prepared satement.
 * - `parameters` - parameters (TsurugiFfiSqlParameterHandle array).
 * - `parameters_size` - `parameters` size \[number of parameters\].
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `explain_result_out` - explain result. To dispose, call [`tsurugi_ffi_sql_explain_result_dispose`](crate::service::sql::explain::tsurugi_ffi_sql_explain_result_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_prepared_explain_for(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlClientHandle sql_client,
                                                         TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                         const TsurugiFfiSqlParameterHandle *parameters,
                                                         uint32_t parameters_size,
                                                         TsurugiFfiDuration timeout,
                                                         TsurugiFfiSqlExplainResultHandle *explain_result_out);

/**
 * SqlClient: Retrieves execution plan of the statement.
 *
 * See [`SqlClient::prepared_explain_async`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `prepared_statement` - prepared satement.
 * - `parameters` - parameters (TsurugiFfiSqlParameterHandle array).
 * - `parameters_size` - `parameters` size \[number of parameters\].
 *
 * # Returns
 * - `explain_result_job_out` - Job for `TsurugiFfiSqlExplainResultHandle`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 *   Handle taken from Job casts to `TsurugiFfiSqlExplainResultHandle` and call [`tsurugi_ffi_sql_explain_result_dispose`](crate::service::sql::explain::tsurugi_ffi_sql_explain_result_dispose) to dispose.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_prepared_explain_async(TsurugiFfiContextHandle context,
                                                           TsurugiFfiSqlClientHandle sql_client,
                                                           TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                           const TsurugiFfiSqlParameterHandle *parameters,
                                                           uint32_t parameters_size,
                                                           TsurugiFfiJobHandle *explain_result_job_out);

/**
 * SqlClient: Start transaction.
 *
 * See [`SqlClient::start_transaction`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction_option` - transaction option.
 *
 * # Returns
 * - `transaction_out` - transaction. To dispose, call [`tsurugi_ffi_transaction_dispose`](crate::transaction::tsurugi_ffi_transaction_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_start_transaction(TsurugiFfiContextHandle context,
                                                      TsurugiFfiSqlClientHandle sql_client,
                                                      TsurugiFfiTransactionOptionHandle transaction_option,
                                                      TsurugiFfiTransactionHandle *transaction_out);

/**
 * SqlClient: Start transaction.
 *
 * See [`SqlClient::start_transaction_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction_option` - transaction option.
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `transaction_out` - transaction. To dispose, call [`tsurugi_ffi_transaction_dispose`](crate::transaction::tsurugi_ffi_transaction_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_start_transaction_for(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlClientHandle sql_client,
                                                          TsurugiFfiTransactionOptionHandle transaction_option,
                                                          TsurugiFfiDuration timeout,
                                                          TsurugiFfiTransactionHandle *transaction_out);

/**
 * SqlClient: Start transaction.
 *
 * See [`SqlClient::start_transaction_async`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction_option` - transaction option.
 *
 * # Returns
 * - `transaction_job_out` - Job for `TsurugiFfiTransactionHandle`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 *   Handle taken from Job casts to `TsurugiFfiTransactionHandle` and call [`tsurugi_ffi_transaction_dispose`](crate::transaction::tsurugi_ffi_transaction_dispose) to dispose.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_start_transaction_async(TsurugiFfiContextHandle context,
                                                            TsurugiFfiSqlClientHandle sql_client,
                                                            TsurugiFfiTransactionOptionHandle transaction_option,
                                                            TsurugiFfiJobHandle *transaction_job_out);

/**
 * SqlClient: Get transaction error information.
 *
 * See [`SqlClient::get_transaction_error_info`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 *
 * # Returns
 * - `transaction_error_info_out` - transaction error information. To dispose, call [`tsurugi_ffi_transaction_error_info_dispose`](crate::transaction::error_info::tsurugi_ffi_transaction_error_info_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_get_transaction_error_info(TsurugiFfiContextHandle context,
                                                               TsurugiFfiSqlClientHandle sql_client,
                                                               TsurugiFfiTransactionHandle transaction,
                                                               TsurugiFfiTransactionErrorInfoHandle *transaction_error_info_out);

/**
 * SqlClient: Get transaction error information.
 *
 * See [`SqlClient::get_transaction_error_info_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `transaction_error_info_out` - transaction error information. To dispose, call [`tsurugi_ffi_transaction_error_info_dispose`](crate::transaction::error_info::tsurugi_ffi_transaction_error_info_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_get_transaction_error_info_for(TsurugiFfiContextHandle context,
                                                                   TsurugiFfiSqlClientHandle sql_client,
                                                                   TsurugiFfiTransactionHandle transaction,
                                                                   TsurugiFfiDuration timeout,
                                                                   TsurugiFfiTransactionErrorInfoHandle *transaction_error_info_out);

/**
 * SqlClient: Get transaction error information.
 *
 * See [`SqlClient::get_transaction_error_info_async`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 *
 * # Returns
 * - `transaction_error_info_job_out` - Job for `TsurugiFfiTransactionErrorInfoHandle`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 *   Handle taken from Job casts to `TsurugiFfiTransactionErrorInfoHandle` and call [`tsurugi_ffi_transaction_error_info_dispose`](crate::transaction::error_info::tsurugi_ffi_transaction_error_info_dispose) to dispose.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_get_transaction_error_info_async(TsurugiFfiContextHandle context,
                                                                     TsurugiFfiSqlClientHandle sql_client,
                                                                     TsurugiFfiTransactionHandle transaction,
                                                                     TsurugiFfiJobHandle *transaction_error_info_job_out);

/**
 * SqlClient: Executes a SQL statement.
 *
 * See [`SqlClient::execute`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `sql` - SQL statement.
 *
 * # Returns
 * - `execute_result_out` - execute result. To dispose, call [`tsurugi_ffi_execute_result_dispose`](crate::service::sql::execute_result::tsurugi_ffi_sql_execute_result_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_execute(TsurugiFfiContextHandle context,
                                            TsurugiFfiSqlClientHandle sql_client,
                                            TsurugiFfiTransactionHandle transaction,
                                            TsurugiFfiStringHandle sql,
                                            TsurugiFfiSqlExecuteResultHandle *execute_result_out);

/**
 * SqlClient: Executes a SQL statement.
 *
 * See [`SqlClient::execute_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `sql` - SQL statement.
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `execute_result_out` - execute result. To dispose, call [`tsurugi_ffi_execute_result_dispose`](crate::service::sql::execute_result::tsurugi_ffi_sql_execute_result_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_execute_for(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlClientHandle sql_client,
                                                TsurugiFfiTransactionHandle transaction,
                                                TsurugiFfiStringHandle sql,
                                                TsurugiFfiDuration timeout,
                                                TsurugiFfiSqlExecuteResultHandle *execute_result_out);

/**
 * SqlClient: Executes a SQL statement.
 *
 * See [`SqlClient::execute_async`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `sql` - SQL statement.
 *
 * # Returns
 * - `execute_result_job_out` - Job for `TsurugiFfiSqlExecuteResultHandle`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 *   Handle taken from Job casts to `TsurugiFfiSqlExecuteResultHandle` and call [`tsurugi_ffi_sql_execute_result_dispose`](crate::service::sql::execute_result::tsurugi_ffi_sql_execute_result_dispose) to dispose.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_execute_async(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlClientHandle sql_client,
                                                  TsurugiFfiTransactionHandle transaction,
                                                  TsurugiFfiStringHandle sql,
                                                  TsurugiFfiJobHandle *execute_result_job_out);

/**
 * SqlClient: Executes a SQL statement.
 *
 * See [`SqlClient::prepared_execute`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `prepared_statement` - prepared satement.
 * - `parameters` - parameters (TsurugiFfiSqlParameterHandle array).
 * - `parameters_size` - `parameters` size \[number of parameters\].
 *
 * # Returns
 * - `execute_result_out` - execute result. To dispose, call [`tsurugi_ffi_execute_result_dispose`](crate::service::sql::execute_result::tsurugi_ffi_sql_execute_result_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_prepared_execute(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlClientHandle sql_client,
                                                     TsurugiFfiTransactionHandle transaction,
                                                     TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                     const TsurugiFfiSqlParameterHandle *parameters,
                                                     uint32_t parameters_size,
                                                     TsurugiFfiSqlExecuteResultHandle *execute_result_out);

/**
 * SqlClient: Executes a SQL statement.
 *
 * See [`SqlClient::prepared_execute_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `prepared_statement` - prepared satement.
 * - `parameters` - parameters (TsurugiFfiSqlParameterHandle array).
 * - `parameters_size` - `parameters` size \[number of parameters\].
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `execute_result_out` - execute result. To dispose, call [`tsurugi_ffi_execute_result_dispose`](crate::service::sql::execute_result::tsurugi_ffi_sql_execute_result_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_prepared_execute_for(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlClientHandle sql_client,
                                                         TsurugiFfiTransactionHandle transaction,
                                                         TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                         const TsurugiFfiSqlParameterHandle *parameters,
                                                         uint32_t parameters_size,
                                                         TsurugiFfiDuration timeout,
                                                         TsurugiFfiSqlExecuteResultHandle *execute_result_out);

/**
 * SqlClient: Executes a SQL statement.
 *
 * See [`SqlClient::prepared_execute_async`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `prepared_statement` - prepared satement.
 * - `parameters` - parameters (TsurugiFfiSqlParameterHandle array).
 * - `parameters_size` - `parameters` size \[number of parameters\].
 *
 * # Returns
 * - `execute_result_job_out` - Job for `TsurugiFfiSqlExecuteResultHandle`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 *   Handle taken from Job casts to `TsurugiFfiSqlExecuteResultHandle` and call [`tsurugi_ffi_sql_execute_result_dispose`](crate::service::sql::execute_result::tsurugi_ffi_sql_execute_result_dispose) to dispose.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_prepared_execute_async(TsurugiFfiContextHandle context,
                                                           TsurugiFfiSqlClientHandle sql_client,
                                                           TsurugiFfiTransactionHandle transaction,
                                                           TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                           const TsurugiFfiSqlParameterHandle *parameters,
                                                           uint32_t parameters_size,
                                                           TsurugiFfiJobHandle *execute_result_job_out);

/**
 * SqlClient: Executes a SQL statement and retrieve its result.
 *
 * See [`SqlClient::query`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `sql` - SQL satement.
 *
 * # Returns
 * - `query_result_out` - query result. To dispose, call [`tsurugi_ffi_query_result_dispose`](crate::service::sql::query_result::tsurugi_ffi_sql_query_result_dispose).
 *   A `SqlQueryResult` instance can only be used while the transaction is alive.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_query(TsurugiFfiContextHandle context,
                                          TsurugiFfiSqlClientHandle sql_client,
                                          TsurugiFfiTransactionHandle transaction,
                                          TsurugiFfiStringHandle sql,
                                          TsurugiFfiSqlQueryResultHandle *query_result_out);

/**
 * SqlClient: Executes a SQL statement and retrieve its result.
 *
 * See [`SqlClient::query_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `sql` - SQL satement.
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `query_result_out` - query result. To dispose, call [`tsurugi_ffi_query_result_dispose`](crate::service::sql::query_result::tsurugi_ffi_sql_query_result_dispose).
 *   A `SqlQueryResult` instance can only be used while the transaction is alive.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_query_for(TsurugiFfiContextHandle context,
                                              TsurugiFfiSqlClientHandle sql_client,
                                              TsurugiFfiTransactionHandle transaction,
                                              TsurugiFfiStringHandle sql,
                                              TsurugiFfiDuration timeout,
                                              TsurugiFfiSqlQueryResultHandle *query_result_out);

/**
 * SqlClient: Executes a SQL statement and retrieve its result.
 *
 * See [`SqlClient::query_async`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `sql` - SQL satement.
 *
 * # Returns
 * - `query_result_job_out` - Job for `TsurugiFfiSqlQueryResultHandle`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 *   Handle taken from Job casts to `TsurugiFfiSqlQueryResultHandle` and call [`tsurugi_ffi_sql_query_result_dispose`](crate::service::sql::query_result::tsurugi_ffi_sql_query_result_dispose) to dispose.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_query_async(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlClientHandle sql_client,
                                                TsurugiFfiTransactionHandle transaction,
                                                TsurugiFfiStringHandle sql,
                                                TsurugiFfiJobHandle *query_result_job_out);

/**
 * SqlClient: Executes a SQL statement and retrieve its result.
 *
 * See [`SqlClient::prepared_query`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `prepared_statement` - prepared satement.
 * - `parameters` - parameters (TsurugiFfiSqlParameterHandle array).
 * - `parameters_size` - `parameters` size \[number of parameters\].
 *
 * # Returns
 * - `query_result_out` - query result. To dispose, call [`tsurugi_ffi_query_result_dispose`](crate::service::sql::query_result::tsurugi_ffi_sql_query_result_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_prepared_query(TsurugiFfiContextHandle context,
                                                   TsurugiFfiSqlClientHandle sql_client,
                                                   TsurugiFfiTransactionHandle transaction,
                                                   TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                   const TsurugiFfiSqlParameterHandle *parameters,
                                                   uint32_t parameters_size,
                                                   TsurugiFfiSqlQueryResultHandle *query_result_out);

/**
 * SqlClient: Executes a SQL statement and retrieve its result.
 *
 * See [`SqlClient::prepared_query_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `prepared_statement` - prepared satement.
 * - `parameters` - parameters (TsurugiFfiSqlParameterHandle array).
 * - `parameters_size` - `parameters` size \[number of parameters\].
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `query_result_out` - query result. To dispose, call [`tsurugi_ffi_query_result_dispose`](crate::service::sql::query_result::tsurugi_ffi_sql_query_result_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_prepared_query_for(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlClientHandle sql_client,
                                                       TsurugiFfiTransactionHandle transaction,
                                                       TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                       const TsurugiFfiSqlParameterHandle *parameters,
                                                       uint32_t parameters_size,
                                                       TsurugiFfiDuration timeout,
                                                       TsurugiFfiSqlQueryResultHandle *query_result_out);

/**
 * SqlClient: Executes a SQL statement and retrieve its result.
 *
 * See [`SqlClient::prepared_query_async`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `prepared_statement` - prepared satement.
 * - `parameters` - parameters (TsurugiFfiSqlParameterHandle array).
 * - `parameters_size` - `parameters` size \[number of parameters\].
 *
 * # Returns
 * - `query_result_job_out` - Job for `TsurugiFfiSqlQueryResultHandle`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 *   Handle taken from Job casts to `TsurugiFfiSqlQueryResultHandle` and call [`tsurugi_ffi_sql_query_result_dispose`](crate::service::sql::query_result::tsurugi_ffi_sql_query_result_dispose) to dispose.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_prepared_query_async(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlClientHandle sql_client,
                                                         TsurugiFfiTransactionHandle transaction,
                                                         TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                         const TsurugiFfiSqlParameterHandle *parameters,
                                                         uint32_t parameters_size,
                                                         TsurugiFfiJobHandle *query_result_job_out);

/**
 * SqlClient: Read BLOB.
 *
 * See [`SqlClient::read_blob`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `blob` - BLOB.
 *
 * # Returns
 * - `value_out` - value.
 * - `size_out` - `value_out` size \[byte\].
 */
TsurugiFfiRc tsurugi_ffi_sql_client_read_blob(TsurugiFfiContextHandle context,
                                              TsurugiFfiSqlClientHandle sql_client,
                                              TsurugiFfiTransactionHandle transaction,
                                              TsurugiFfiBlobReferenceHandle blob,
                                              TsurugiFfiByteArrayHandle *value_out,
                                              uint64_t *size_out);

/**
 * SqlClient: Read BLOB.
 *
 * See [`SqlClient::read_blob_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `blob` - BLOB.
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `value_out` - value.
 * - `size_out` - `value_out` size \[byte\].
 */
TsurugiFfiRc tsurugi_ffi_sql_client_read_blob_for(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlClientHandle sql_client,
                                                  TsurugiFfiTransactionHandle transaction,
                                                  TsurugiFfiBlobReferenceHandle blob,
                                                  TsurugiFfiDuration timeout,
                                                  TsurugiFfiByteArrayHandle *value_out,
                                                  uint64_t *size_out);

/**
 * SqlClient: Read CLOB.
 *
 * See [`SqlClient::read_clob`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `clob` - CLOB.
 *
 * # Returns
 * - `value_out` - value.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_read_clob(TsurugiFfiContextHandle context,
                                              TsurugiFfiSqlClientHandle sql_client,
                                              TsurugiFfiTransactionHandle transaction,
                                              TsurugiFfiClobReferenceHandle clob,
                                              TsurugiFfiStringHandle *value_out);

/**
 * SqlClient: Read CLOB.
 *
 * See [`SqlClient::read_clob_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `clob` - CLOB.
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `value_out` - value.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_read_clob_for(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlClientHandle sql_client,
                                                  TsurugiFfiTransactionHandle transaction,
                                                  TsurugiFfiClobReferenceHandle clob,
                                                  TsurugiFfiDuration timeout,
                                                  TsurugiFfiStringHandle *value_out);

/**
 * SqlClient: Copy BLOB to local file.
 *
 * See [`SqlClient::copy_blob_to`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `blob` - BLOB.
 * - `destination` - the path of the destination file.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_copy_blob_to(TsurugiFfiContextHandle context,
                                                 TsurugiFfiSqlClientHandle sql_client,
                                                 TsurugiFfiTransactionHandle transaction,
                                                 TsurugiFfiBlobReferenceHandle blob,
                                                 TsurugiFfiStringHandle destination);

/**
 * SqlClient: Copy BLOB to local file.
 *
 * See [`SqlClient::copy_blob_to_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `blob` - BLOB.
 * - `destination` - the path of the destination file.
 * - `timeout` - timeout time \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_sql_client_copy_blob_to_for(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlClientHandle sql_client,
                                                     TsurugiFfiTransactionHandle transaction,
                                                     TsurugiFfiBlobReferenceHandle blob,
                                                     TsurugiFfiStringHandle destination,
                                                     TsurugiFfiDuration timeout);

/**
 * SqlClient: Copy BLOB to local file.
 *
 * See [`SqlClient::copy_blob_to_async`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `blob` - BLOB.
 * - `destination` - the path of the destination file.
 *
 * # Returns
 * - `copy_blob_to_job_out` - Job for `void`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_copy_blob_to_async(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlClientHandle sql_client,
                                                       TsurugiFfiTransactionHandle transaction,
                                                       TsurugiFfiBlobReferenceHandle blob,
                                                       TsurugiFfiStringHandle destination,
                                                       TsurugiFfiJobHandle *copy_blob_to_job_out);

/**
 * SqlClient: Copy CLOB to local file.
 *
 * See [`SqlClient::copy_clob_to`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `clob` - CLOB.
 * - `destination` - the path of the destination file.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_copy_clob_to(TsurugiFfiContextHandle context,
                                                 TsurugiFfiSqlClientHandle sql_client,
                                                 TsurugiFfiTransactionHandle transaction,
                                                 TsurugiFfiClobReferenceHandle clob,
                                                 TsurugiFfiStringHandle destination);

/**
 * SqlClient: Copy CLOB to local file.
 *
 * See [`SqlClient::copy_clob_to_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `clob` - CLOB.
 * - `destination` - the path of the destination file.
 * - `timeout` - timeout time \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_sql_client_copy_clob_to_for(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlClientHandle sql_client,
                                                     TsurugiFfiTransactionHandle transaction,
                                                     TsurugiFfiClobReferenceHandle clob,
                                                     TsurugiFfiStringHandle destination,
                                                     TsurugiFfiDuration timeout);

/**
 * SqlClient: Copy CLOB to local file.
 *
 * See [`SqlClient::copy_clob_to_async`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `clob` - CLOB.
 * - `destination` - the path of the destination file.
 *
 * # Returns
 * - `copy_clob_to_job_out` - Job for `void`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_copy_clob_to_async(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlClientHandle sql_client,
                                                       TsurugiFfiTransactionHandle transaction,
                                                       TsurugiFfiClobReferenceHandle clob,
                                                       TsurugiFfiStringHandle destination,
                                                       TsurugiFfiJobHandle *copy_clob_to_job_out);

/**
 * SqlClient: Request commit to the SQL service.
 *
 * See [`SqlClient::commit`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `commit_option` - commit option.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_commit(TsurugiFfiContextHandle context,
                                           TsurugiFfiSqlClientHandle sql_client,
                                           TsurugiFfiTransactionHandle transaction,
                                           TsurugiFfiCommitOptionHandle commit_option);

/**
 * SqlClient: Request commit to the SQL service.
 *
 * See [`SqlClient::commit_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `commit_option` - commit option.
 * - `timeout` - timeout time \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_sql_client_commit_for(TsurugiFfiContextHandle context,
                                               TsurugiFfiSqlClientHandle sql_client,
                                               TsurugiFfiTransactionHandle transaction,
                                               TsurugiFfiCommitOptionHandle commit_option,
                                               TsurugiFfiDuration timeout);

/**
 * SqlClient: Request commit to the SQL service.
 *
 * See [`SqlClient::commit_async`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `commit_option` - commit option.
 *
 * # Returns
 * - `commit_job_out` - Job for `void`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 */
TsurugiFfiRc tsurugi_ffi_sql_client_commit_async(TsurugiFfiContextHandle context,
                                                 TsurugiFfiSqlClientHandle sql_client,
                                                 TsurugiFfiTransactionHandle transaction,
                                                 TsurugiFfiCommitOptionHandle commit_option,
                                                 TsurugiFfiJobHandle *commit_job_out);

/**
 * SqlClient: Request rollback to the SQL service.
 *
 * See [`SqlClient::rollback`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 */
TsurugiFfiRc tsurugi_ffi_sql_client_rollback(TsurugiFfiContextHandle context,
                                             TsurugiFfiSqlClientHandle sql_client,
                                             TsurugiFfiTransactionHandle transaction);

/**
 * SqlClient: Request rollback to the SQL service.
 *
 * See [`SqlClient::rollback_for`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 * - `timeout` - timeout time \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_sql_client_rollback_for(TsurugiFfiContextHandle context,
                                                 TsurugiFfiSqlClientHandle sql_client,
                                                 TsurugiFfiTransactionHandle transaction,
                                                 TsurugiFfiDuration timeout);

/**
 * SqlClient: Request rollback to the SQL service.
 *
 * See [`SqlClient::rollback_async`].
 *
 * # Receiver
 * - `sql_client` - Sql client.
 *
 * # Parameters
 * - `transaction` - transaction.
 *
 * # Returns
 * - `rollback_job_out` - Job for `void`. To dispose, call [`tsurugi_ffi]_job_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_sql_client_rollback_async(TsurugiFfiContextHandle context,
                                                   TsurugiFfiSqlClientHandle sql_client,
                                                   TsurugiFfiTransactionHandle transaction,
                                                   TsurugiFfiJobHandle *rollback_job_out);

/**
 * SqlClient: Dispose.
 *
 * # Receiver
 * - `sql_client` - Sql client.
 */
void tsurugi_ffi_sql_client_dispose(TsurugiFfiSqlClientHandle sql_client);

/**
 * TableList: Get table names.
 *
 * See [`TableList::table_names`].
 *
 * # Receiver
 * - `table_list` - Table list.
 *
 * # Returns
 * - `table_names_out` - table names (string array).
 * - `table_names_size_out` - `table_names_out` size (number of tables).
 */
TsurugiFfiRc tsurugi_ffi_table_list_get_table_names(TsurugiFfiContextHandle context,
                                                    TsurugiFfiTableListHandle table_list,
                                                    TsurugiFfiStringArrayHandle *table_names_out,
                                                    uint32_t *table_names_size_out);

/**
 * TableList: Dispose.
 *
 * # Receiver
 * - `table_list` - Table list.
 */
void tsurugi_ffi_table_list_dispose(TsurugiFfiTableListHandle table_list);

/**
 * TableMetadata: Get database name.
 *
 * See [`TableMetadata::database_name`].
 *
 * # Receiver
 * - `table_metadata` - Table metadata.
 *
 * # Returns
 * - `database_name_out` - database name (nullable).
 */
TsurugiFfiRc tsurugi_ffi_table_metadata_get_database_name(TsurugiFfiContextHandle context,
                                                          TsurugiFfiTableMetadataHandle table_metadata,
                                                          TsurugiFfiStringHandle *database_name_out);

/**
 * TableMetadata: Get schema name.
 *
 * See [`TableMetadata::schema_name`].
 *
 * # Receiver
 * - `table_metadata` - Table metadata.
 *
 * # Returns
 * - `schema_name_out` - schema name (nullable).
 */
TsurugiFfiRc tsurugi_ffi_table_metadata_get_schema_name(TsurugiFfiContextHandle context,
                                                        TsurugiFfiTableMetadataHandle table_metadata,
                                                        TsurugiFfiStringHandle *schema_name_out);

/**
 * TableMetadata: Get table name.
 *
 * See [`TableMetadata::table_name`].
 *
 * # Receiver
 * - `table_metadata` - Table metadata.
 *
 * # Returns
 * - `table_name_out` - table name.
 */
TsurugiFfiRc tsurugi_ffi_table_metadata_get_table_name(TsurugiFfiContextHandle context,
                                                       TsurugiFfiTableMetadataHandle table_metadata,
                                                       TsurugiFfiStringHandle *table_name_out);

/**
 * TableMetadata: Get description.
 *
 * See [`TableMetadata::description`].
 *
 * # Receiver
 * - `table_metadata` - Table metadata.
 *
 * # Returns
 * - `description_out` - description (nullable).
 */
TsurugiFfiRc tsurugi_ffi_table_metadata_get_description(TsurugiFfiContextHandle context,
                                                        TsurugiFfiTableMetadataHandle table_metadata,
                                                        TsurugiFfiStringHandle *description_out);

/**
 * TableMetadata: Get columns size.
 *
 * See [`TableMetadata::columns`].
 *
 * # Receiver
 * - `table_metadata` - Table metadata.
 *
 * # Returns
 * - `size_out` - number of columns \[number of columns\].
 */
TsurugiFfiRc tsurugi_ffi_table_metadata_get_columns_size(TsurugiFfiContextHandle context,
                                                         TsurugiFfiTableMetadataHandle table_metadata,
                                                         uint32_t *size_out);

/**
 * TableMetadata: Get columns value.
 *
 * See [`TableMetadata::columns`].
 *
 * # Receiver
 * - `table_metadata` - Table metadata.
 *
 * # Parameters
 * - `index` - column index \[0..tsurugi_ffi_table_metadata_get_columns_size()-1\].
 *
 * # Returns
 * - `sql_column_out` - column. To dispose, call [`tsurugi_ffi_sql_column_dispose`](crate::service::sql::column::tsurugi_ffi_sql_column_dispose).
 */
TsurugiFfiRc tsurugi_ffi_table_metadata_get_columns_value(TsurugiFfiContextHandle context,
                                                          TsurugiFfiTableMetadataHandle table_metadata,
                                                          uint32_t index,
                                                          TsurugiFfiSqlColumnHandle *sql_column_out);

/**
 * TableMetadata: Dispose.
 *
 * # Receiver
 * - `table_metadata` - Table metadata.
 */
void tsurugi_ffi_table_metadata_dispose(TsurugiFfiTableMetadataHandle table_metadata);

/**
 * BlobReference: Dispose.
 *
 * # Receiver
 * - `blob_reference` - blob reference.
 */
void tsurugi_ffi_blob_reference_dispose(TsurugiFfiBlobReferenceHandle blob_reference);

/**
 * ClobReference: Dispose.
 *
 * # Receiver
 * - `clob_reference` - clob reference.
 */
void tsurugi_ffi_clob_reference_dispose(TsurugiFfiClobReferenceHandle clob_reference);

/**
 * Endpoint: Creates a new instance.
 *
 * See [`Endpoint::parse`].
 *
 * # Parameters
 * - `endpoint` - endpoint url. (e.g. `tcp://localhost:12345`)
 *
 * # Returns
 * - `endpoint_out` - endpoint. To dispose, call [`tsurugi_ffi_endpoint_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_endpoint_parse(TsurugiFfiContextHandle context,
                                        TsurugiFfiStringHandle endpoint,
                                        TsurugiFfiEndpointHandle *endpoint_out);

/**
 * Endpoint: Dispose.
 *
 * # Receiver
 * - `endpoint` - endpoint.
 */
void tsurugi_ffi_endpoint_dispose(TsurugiFfiEndpointHandle endpoint);

/**
 * ConnectionOption: Creates a new instance.
 *
 * See [`ConnectionOption::new`].
 *
 * # Returns
 * - `connection_option_out` - connection option. To dispose, call [`tsurugi_ffi_connection_option_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_connection_option_create(TsurugiFfiContextHandle context,
                                                  TsurugiFfiConnectionOptionHandle *connection_option_out);

/**
 * ConnectionOption: Set endpoint.
 *
 * See [`ConnectionOption::set_endpoint`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Parameters
 * - `endpoint` - endpoint.
 */
TsurugiFfiRc tsurugi_ffi_connection_option_set_endpoint(TsurugiFfiContextHandle context,
                                                        TsurugiFfiConnectionOptionHandle connection_option,
                                                        TsurugiFfiEndpointHandle endpoint);

/**
 * ConnectionOption: Set endpoint.
 *
 * See [`ConnectionOption::set_endpoint_url`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Parameters
 * - `endpoint` - endpoint url. (e.g. `tcp://localhost:12345`)
 */
TsurugiFfiRc tsurugi_ffi_connection_option_set_endpoint_url(TsurugiFfiContextHandle context,
                                                            TsurugiFfiConnectionOptionHandle connection_option,
                                                            TsurugiFfiStringHandle endpoint_url);

/**
 * ConnectionOption: Get endpoint.
 *
 * See [`ConnectionOption::endpoint`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Returns
 * - `endpoint_url_out` - endpoint url.
 */
TsurugiFfiRc tsurugi_ffi_connection_option_get_endpoint_url(TsurugiFfiContextHandle context,
                                                            TsurugiFfiConnectionOptionHandle connection_option,
                                                            TsurugiFfiStringHandle *endpoint_url_out);

/**
 * ConnectionOption: Set application name.
 *
 * See [`ConnectionOption::set_application_name`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Parameters
 * - `application_name` - application name.
 */
TsurugiFfiRc tsurugi_ffi_connection_option_set_application_name(TsurugiFfiContextHandle context,
                                                                TsurugiFfiConnectionOptionHandle connection_option,
                                                                TsurugiFfiStringHandle application_name);

/**
 * ConnectionOption: Get application name.
 *
 * See [`ConnectionOption::application_name`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Returns
 * - `application_name_out` - application name.
 */
TsurugiFfiRc tsurugi_ffi_connection_option_get_application_name(TsurugiFfiContextHandle context,
                                                                TsurugiFfiConnectionOptionHandle connection_option,
                                                                TsurugiFfiStringHandle *application_name_out);

/**
 * ConnectionOption: Set session label.
 *
 * See [`ConnectionOption::set_session_label`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Parameters
 * - `session_label` - session label.
 */
TsurugiFfiRc tsurugi_ffi_connection_option_set_session_label(TsurugiFfiContextHandle context,
                                                             TsurugiFfiConnectionOptionHandle connection_option,
                                                             TsurugiFfiStringHandle session_label);

/**
 * ConnectionOption: Get session label.
 *
 * See [`ConnectionOption::session_label`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Returns
 * - `session_label_out` - session label.
 */
TsurugiFfiRc tsurugi_ffi_connection_option_get_session_label(TsurugiFfiContextHandle context,
                                                             TsurugiFfiConnectionOptionHandle connection_option,
                                                             TsurugiFfiStringHandle *session_label_out);

/**
 * ConnectionOption: Set keep alive interval.
 *
 * See [`ConnectionOption::set_keep_alive`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Parameters
 * - `keep_alive` - keep alive interval \[nanosecond\].
 *   Do not keep alive when `keep_alive` is 0.
 */
TsurugiFfiRc tsurugi_ffi_connection_option_set_keep_alive(TsurugiFfiContextHandle context,
                                                          TsurugiFfiConnectionOptionHandle connection_option,
                                                          TsurugiFfiDuration keep_alive);

/**
 * ConnectionOption: Get keep alive interval.
 *
 * See [`ConnectionOption::keep_alive`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Returns
 * - `keep_alive_out` - keep alive interval \[nanosecond\].
 */
TsurugiFfiRc tsurugi_ffi_connection_option_get_keep_alive(TsurugiFfiContextHandle context,
                                                          TsurugiFfiConnectionOptionHandle connection_option,
                                                          TsurugiFfiDuration *keep_alive_out);

/**
 * ConnectionOption: Adds a path mapping entry for both sending and receiving BLOB/CLOB.
 *
 * See [`ConnectionOption::add_large_object_path_mapping`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Parameters
 * - `client_path` - the client path, must be a directory
 * - `server_path` - the server path, must be a directory
 */
TsurugiFfiRc tsurugi_ffi_connection_option_add_large_object_path_mapping(TsurugiFfiContextHandle context,
                                                                         TsurugiFfiConnectionOptionHandle connection_option,
                                                                         TsurugiFfiStringHandle client_path,
                                                                         TsurugiFfiStringHandle server_path);

/**
 * ConnectionOption: Adds a path mapping entry for sending BLOB/CLOB.
 *
 * See [`ConnectionOption::add_large_object_path_mapping_on_send`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Parameters
 * - `client_path` - the client path to be transformed, must be a directory
 * - `server_path` - the server path, must be a directory
 */
TsurugiFfiRc tsurugi_ffi_connection_option_add_large_object_path_mapping_on_send(TsurugiFfiContextHandle context,
                                                                                 TsurugiFfiConnectionOptionHandle connection_option,
                                                                                 TsurugiFfiStringHandle client_path,
                                                                                 TsurugiFfiStringHandle server_path);

/**
 * ConnectionOption: Adds a path mapping entry for receiving BLOB/CLOB.
 *
 * See [`ConnectionOption::add_large_object_path_mapping_on_recv`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Parameters
 * - `server_path` - the target server path to be transformed, must be a directory
 * - `client_path` - the target client path, must be a directory
 */
TsurugiFfiRc tsurugi_ffi_connection_option_add_large_object_path_mapping_on_recv(TsurugiFfiContextHandle context,
                                                                                 TsurugiFfiConnectionOptionHandle connection_option,
                                                                                 TsurugiFfiStringHandle server_path,
                                                                                 TsurugiFfiStringHandle client_path);

/**
 * ConnectionOption: Set default timeout.
 *
 * See [`ConnectionOption::set_default_timeout`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Parameters
 * - `default_timeout` - default timeout \[nanosecond\].
 */
TsurugiFfiRc tsurugi_ffi_connection_option_set_default_timeout(TsurugiFfiContextHandle context,
                                                               TsurugiFfiConnectionOptionHandle connection_option,
                                                               TsurugiFfiDuration default_timeout);

/**
 * ConnectionOption: Get default timeout.
 *
 * See [`ConnectionOption::default_timeout`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Returns
 * - `default_timeout_out` - default timeout \[nanosecond\].
 */
TsurugiFfiRc tsurugi_ffi_connection_option_get_default_timeout(TsurugiFfiContextHandle context,
                                                               TsurugiFfiConnectionOptionHandle connection_option,
                                                               TsurugiFfiDuration *default_timeout_out);

/**
 * ConnectionOption: Set communication send timeout.
 *
 * See [`ConnectionOption::set_send_timeout`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Parameters
 * - `send_timeout` - send timeout \[nanosecond\].
 */
TsurugiFfiRc tsurugi_ffi_connection_option_set_send_timeout(TsurugiFfiContextHandle context,
                                                            TsurugiFfiConnectionOptionHandle connection_option,
                                                            TsurugiFfiDuration send_timeout);

/**
 * ConnectionOption: Get communication send timeout.
 *
 * See [`ConnectionOption::send_timeout`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Returns
 * - `send_timeout_out` - send timeout \[nanosecond\].
 */
TsurugiFfiRc tsurugi_ffi_connection_option_get_send_timeout(TsurugiFfiContextHandle context,
                                                            TsurugiFfiConnectionOptionHandle connection_option,
                                                            TsurugiFfiDuration *send_timeout_out);

/**
 * ConnectionOption: Set communication recv timeout.
 *
 * See [`ConnectionOption::set_recv_timeout`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Parameters
 * - `recv_timeout` - recv timeout \[nanosecond\].
 */
TsurugiFfiRc tsurugi_ffi_connection_option_set_recv_timeout(TsurugiFfiContextHandle context,
                                                            TsurugiFfiConnectionOptionHandle connection_option,
                                                            TsurugiFfiDuration recv_timeout);

/**
 * ConnectionOption: Get communication recv timeout.
 *
 * See [`ConnectionOption::recv_timeout`].
 *
 * # Receiver
 * - `connection_option` - Connection option.
 *
 * # Returns
 * - `recv_timeout_out` - recv timeout \[nanosecond\].
 */
TsurugiFfiRc tsurugi_ffi_connection_option_get_recv_timeout(TsurugiFfiContextHandle context,
                                                            TsurugiFfiConnectionOptionHandle connection_option,
                                                            TsurugiFfiDuration *recv_timeout_out);

/**
 * ConnectionOption: Dispose.
 *
 * # Receiver
 * - `connection_option` - Connection option.
 */
void tsurugi_ffi_connection_option_dispose(TsurugiFfiConnectionOptionHandle connection_option);

/**
 * Establishes a connection to the Tsurugi server.
 *
 * See [`Session::connect`].
 *
 * # Parameters
 * - `connection_option` - connection option.
 *
 * # Returns
 * - `session_out` - session. To dispose, call [`tsurugi_ffi_session_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_session_connect(TsurugiFfiContextHandle context,
                                         TsurugiFfiConnectionOptionHandle connection_option,
                                         TsurugiFfiSessionHandle *session_out);

/**
 * Establishes a connection to the Tsurugi server.
 *
 * See [`Session::connect_for`].
 *
 * # Parameters
 * - `connection_option` - connection option.
 * - `timeout` - timeout time \[nanoseconds\].
 *
 * # Returns
 * - `session_out` - session. To dispose, call [`tsurugi_ffi_session_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_session_connect_for(TsurugiFfiContextHandle context,
                                             TsurugiFfiConnectionOptionHandle connection_option,
                                             TsurugiFfiDuration timeout,
                                             TsurugiFfiSessionHandle *session_out);

/**
 * Establishes a connection to the Tsurugi server.
 *
 * See [`Session::connect_async`].
 *
 * # Parameters
 * - `connection_option` - connection option.
 *
 * # Returns
 * - `session_job_out` - Job for `TsurugiFfiSessionHandle`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 *   Handle taken from Job casts to `TsurugiFfiSessionHandle` and call [`tsurugi_ffi_session_dispose`] to dispose.
 */
TsurugiFfiRc tsurugi_ffi_session_connect_async(TsurugiFfiContextHandle context,
                                               TsurugiFfiConnectionOptionHandle connection_option,
                                               TsurugiFfiJobHandle *session_job_out);

/**
 * Session: Set default timeout.
 *
 * See [`Session::set_default_timeout`].
 *
 * # Receiver
 * - `session` - Session.
 *
 * # Parameters
 * - `default_timeout` - default timeout \[nanosecond\].
 */
TsurugiFfiRc tsurugi_ffi_session_set_default_timeout(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSessionHandle session,
                                                     TsurugiFfiDuration default_timeout);

/**
 * Session: Get default timeout.
 *
 * See [`Session::default_timeout`].
 *
 * # Receiver
 * - `session` - Session.
 *
 * # Returns
 * - `default_timeout_out` - default timeout \[nanosecond\].
 */
TsurugiFfiRc tsurugi_ffi_session_get_default_timeout(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSessionHandle session,
                                                     TsurugiFfiDuration *default_timeout_out);

/**
 * Session: Make SqlClient.
 *
 * # Receiver
 * - `session` - Session.
 *
 * See [`Session::make_client`].
 *
 * # Returns
 * - `sql_client_out` - SqlClient. To dispose, call [`tsurugi_ffi_sql_client_dispose`](crate::service::sql::tsurugi_ffi_sql_client_dispose).
 */
TsurugiFfiRc tsurugi_ffi_session_make_sql_client(TsurugiFfiContextHandle context,
                                                 TsurugiFfiSessionHandle session,
                                                 TsurugiFfiSqlClientHandle *sql_client_out);

/**
 * Session: Update expiration time.
 *
 * See [`Session::update_expiration_time`].
 *
 * # Receiver
 * - `session` - Session.
 *
 * # Parameters
 * - `expiration_time_exists` - `true`: Using `expiration_time` / `false`: the server's default value is used.
 * - `expiration_time` - expiration time \[nanosecond\].
 */
TsurugiFfiRc tsurugi_ffi_session_update_expiration_time(TsurugiFfiContextHandle context,
                                                        TsurugiFfiSessionHandle session,
                                                        bool expiration_time_exists,
                                                        TsurugiFfiDuration expiration_time);

/**
 * Session: Update expiration time.
 *
 * See [`Session::update_expiration_time_for`].
 *
 * # Receiver
 * - `session` - Session.
 *
 * # Parameters
 * - `expiration_time_exists` - `true`: Using `expiration_time` / `false`: the server's default value is used.
 * - `expiration_time` - expiration time \[nanosecond\].
 * - `timeout` - timeout time \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_session_update_expiration_time_for(TsurugiFfiContextHandle context,
                                                            TsurugiFfiSessionHandle session,
                                                            bool expiration_time_exists,
                                                            TsurugiFfiDuration expiration_time,
                                                            TsurugiFfiDuration timeout);

/**
 * Session: Update expiration time.
 *
 * See [`Session::update_expiration_time_async`].
 *
 * # Receiver
 * - `session` - Session.
 *
 * # Parameters
 * - `expiration_time_exists` - `true`: Using `expiration_time` / `false`: the server's default value is used.
 * - `expiration_time` - expiration time \[nanosecond\].
 *
 * # Returns
 * - `update_expiration_time_job_out` - Job for `void`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 */
TsurugiFfiRc tsurugi_ffi_session_update_expiration_time_async(TsurugiFfiContextHandle context,
                                                              TsurugiFfiSessionHandle session,
                                                              bool expiration_time_exists,
                                                              TsurugiFfiDuration expiration_time,
                                                              TsurugiFfiJobHandle *update_expiration_time_job_out);

/**
 * Session: Shutdown.
 *
 * See [`Session::shutdown`].
 *
 * # Receiver
 * - `session` - Session.
 *
 * # Parameters
 * - `shutdown_type` - shutdown type.
 */
TsurugiFfiRc tsurugi_ffi_session_shutdown(TsurugiFfiContextHandle context,
                                          TsurugiFfiSessionHandle session,
                                          TsurugiFfiShutdownType shutdown_type);

/**
 * Session: Shutdown.
 *
 * See [`Session::shutdown_for`].
 *
 * # Receiver
 * - `session` - Session.
 *
 * # Parameters
 * - `shutdown_type` - shutdown type.
 * - `timeout` - timeout time \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_session_shutdown_for(TsurugiFfiContextHandle context,
                                              TsurugiFfiSessionHandle session,
                                              TsurugiFfiShutdownType shutdown_type,
                                              TsurugiFfiDuration timeout);

/**
 * Session: Shutdown.
 *
 * See [`Session::shutdown_async`].
 *
 * # Receiver
 * - `session` - Session.
 *
 * # Parameters
 * - `shutdown_type` - shutdown type.
 *
 * # Returns
 * - `shutdown_job_out` - Job for `void`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
 */
TsurugiFfiRc tsurugi_ffi_session_shutdown_async(TsurugiFfiContextHandle context,
                                                TsurugiFfiSessionHandle session,
                                                TsurugiFfiShutdownType shutdown_type,
                                                TsurugiFfiJobHandle *shutdown_job_out);

/**
 * Session: Check if the session is shut down.
 *
 * See [`Session::is_shutdowned`].
 *
 * # Receiver
 * - `session` - Session.
 *
 * # Returns
 * - `is_shutdowned_out` - `true`: Already shutdowned / `false`: Not shutdowned.
 */
TsurugiFfiRc tsurugi_ffi_session_is_shutdowned(TsurugiFfiContextHandle context,
                                               TsurugiFfiSessionHandle session,
                                               bool *is_shutdowned_out);

/**
 * Session: Close.
 *
 * See [`Session::close`].
 *
 * # Receiver
 * - `session` - Session.
 */
TsurugiFfiRc tsurugi_ffi_session_close(TsurugiFfiContextHandle context,
                                       TsurugiFfiSessionHandle session);

/**
 * Session: Check if the session is closed.
 *
 * See [`Session::is_closed`].
 *
 * # Receiver
 * - `session` - Session.
 *
 * # Returns
 * - `is_closed_out` - `true`: Already closed / `false`: Not closed.
 */
TsurugiFfiRc tsurugi_ffi_session_is_closed(TsurugiFfiContextHandle context,
                                           TsurugiFfiSessionHandle session,
                                           bool *is_closed_out);

/**
 * Session: Dispose.
 *
 * # Receiver
 * - `session` - Session.
 */
void tsurugi_ffi_session_dispose(TsurugiFfiSessionHandle session);

/**
 * CommitOption: Creates a new instance.
 *
 * See [`CommitOption::new`].
 *
 * # Returns
 * - `commit_option_out` - commit option. To dispose, call [`tsurugi_ffi_commit_option_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_commit_option_create(TsurugiFfiContextHandle context,
                                              TsurugiFfiCommitOptionHandle *commit_option_out);

/**
 * CommitOption: Set commit type.
 *
 * See [`CommitOption::set_commit_type`].
 *
 * # Receiver
 * - `commit_option` - Commit option.
 *
 * # Parameters
 * - `commit_type` - commit type.
 */
TsurugiFfiRc tsurugi_ffi_commit_option_set_commit_type(TsurugiFfiContextHandle context,
                                                       TsurugiFfiCommitOptionHandle commit_option,
                                                       TsurugiFfiCommitType commit_type);

/**
 * CommitOption: Get commit type.
 *
 * See [`CommitOption::commit_type`].
 *
 * # Receiver
 * - `commit_option` - Commit option.
 *
 * # Returns
 * - `commit_type_out` - commit type.
 */
TsurugiFfiRc tsurugi_ffi_commit_option_get_commit_type(TsurugiFfiContextHandle context,
                                                       TsurugiFfiCommitOptionHandle commit_option,
                                                       TsurugiFfiCommitType *commit_type_out);

/**
 * CommitOption: Set auto dispose.
 *
 * See [`CommitOption::set_auto_dispose`].
 *
 * # Receiver
 * - `commit_option` - Commit option.
 *
 * # Parameters
 * - `auto_dispose` - auto dispose.
 */
TsurugiFfiRc tsurugi_ffi_commit_option_set_auto_dispose(TsurugiFfiContextHandle context,
                                                        TsurugiFfiCommitOptionHandle commit_option,
                                                        bool auto_dispose);

/**
 * CommitOption: Get auto dispose.
 *
 * See [`CommitOption::auto_dispose`].
 *
 * # Receiver
 * - `commit_option` - Commit option.
 *
 * # Returns
 * - `auto_dispose_out` - auto dispose.
 */
TsurugiFfiRc tsurugi_ffi_commit_option_get_auto_dispose(TsurugiFfiContextHandle context,
                                                        TsurugiFfiCommitOptionHandle commit_option,
                                                        bool *auto_dispose_out);

/**
 * CommitOption: Dispose.
 *
 * # Receiver
 * - `commit_option` - Commit option.
 */
void tsurugi_ffi_commit_option_dispose(TsurugiFfiCommitOptionHandle commit_option);

/**
 * TransactionErrorInfo: Whether the status is normal.
 *
 * See [`TransactionErrorInfo::is_normal`].
 *
 * # Receiver
 * - `transaction_error_info` - Transaction error information.
 *
 * # Returns
 * - `is_normal_out` - `true`: No error / `false`: Error occurred in transaction.
 */
TsurugiFfiRc tsurugi_ffi_transaction_error_info_is_normal(TsurugiFfiContextHandle context,
                                                          TsurugiFfiTransactionErrorInfoHandle transaction_error_info,
                                                          bool *is_normal_out);

/**
 * TransactionErrorInfo: Whether the status is error.
 *
 * See [`TransactionErrorInfo::is_error`].
 *
 * # Receiver
 * - `transaction_error_info` - Transaction error information.
 *
 * # Returns
 * - `is_error_out` - `true`: Error occurred in transaction / `false`: No error.
 */
TsurugiFfiRc tsurugi_ffi_transaction_error_info_is_error(TsurugiFfiContextHandle context,
                                                         TsurugiFfiTransactionErrorInfoHandle transaction_error_info,
                                                         bool *is_error_out);

/**
 * TransactionErrorInfo: Get server error name.
 *
 * See [`TransactionErrorInfo::server_error`].
 *
 * # Receiver
 * - `transaction_error_info` - Transaction error information.
 *
 * # Returns
 * - `error_name_out` - error name (`null` if no error).
 */
TsurugiFfiRc tsurugi_ffi_transaction_error_info_get_server_error_name(TsurugiFfiContextHandle context,
                                                                      TsurugiFfiTransactionErrorInfoHandle transaction_error_info,
                                                                      TsurugiFfiStringHandle *error_name_out);

/**
 * TransactionErrorInfo: Get server error message.
 *
 * See [`TransactionErrorInfo::server_error`].
 *
 * # Receiver
 * - `transaction_error_info` - Transaction error information.
 *
 * # Returns
 * - `error_message_out` - error message (`null` if no error).
 */
TsurugiFfiRc tsurugi_ffi_transaction_error_info_get_server_error_message(TsurugiFfiContextHandle context,
                                                                         TsurugiFfiTransactionErrorInfoHandle transaction_error_info,
                                                                         TsurugiFfiStringHandle *error_message_out);

/**
 * TransactionErrorInfo: Get server error category.
 *
 * See [`TransactionErrorInfo::server_error`].
 *
 * # Receiver
 * - `transaction_error_info` - Transaction error information.
 *
 * # Returns
 * - `category_number_out` - error category (0 if no error).
 */
TsurugiFfiRc tsurugi_ffi_transaction_error_info_get_server_error_category_number(TsurugiFfiContextHandle context,
                                                                                 TsurugiFfiTransactionErrorInfoHandle transaction_error_info,
                                                                                 int32_t *category_number_out);

/**
 * TransactionErrorInfo: Get server error category.
 *
 * See [`TransactionErrorInfo::server_error`].
 *
 * # Receiver
 * - `transaction_error_info` - Transaction error information.
 *
 * # Returns
 * - `category_str_out` - error category (`null` if no error).
 */
TsurugiFfiRc tsurugi_ffi_transaction_error_info_get_server_error_category_str(TsurugiFfiContextHandle context,
                                                                              TsurugiFfiTransactionErrorInfoHandle transaction_error_info,
                                                                              TsurugiFfiStringHandle *category_str_out);

/**
 * TransactionErrorInfo: Get server error code.
 *
 * See [`TransactionErrorInfo::server_error`].
 *
 * # Receiver
 * - `transaction_error_info` - Transaction error information.
 *
 * # Returns
 * - `code_number_out` - error code (0 if no error).
 */
TsurugiFfiRc tsurugi_ffi_transaction_error_info_get_server_error_code_number(TsurugiFfiContextHandle context,
                                                                             TsurugiFfiTransactionErrorInfoHandle transaction_error_info,
                                                                             int32_t *code_number_out);

/**
 * TransactionErrorInfo: Get server error structured code.
 *
 * See [`TransactionErrorInfo::server_error`].
 *
 * # Receiver
 * - `transaction_error_info` - Transaction error information.
 *
 * # Returns
 * - `structured_code_out` - structured error code (`null` if no error).
 */
TsurugiFfiRc tsurugi_ffi_transaction_error_info_get_server_error_structured_code(TsurugiFfiContextHandle context,
                                                                                 TsurugiFfiTransactionErrorInfoHandle transaction_error_info,
                                                                                 TsurugiFfiStringHandle *structured_code_out);

/**
 * TransactionErrorInfo: Dispose.
 *
 * # Receiver
 * - `transaction_error_info` - Transaction error information.
 */
void tsurugi_ffi_transaction_error_info_dispose(TsurugiFfiTransactionErrorInfoHandle transaction_error_info);

/**
 * TransactionOption: Creates a new instance.
 *
 * See [`TransactionOption::new`].
 *
 * # Returns
 * - `transaction_option_out` - transaction option. To dispose, call [`tsurugi_ffi_transaction_option_dispose`].
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_create(TsurugiFfiContextHandle context,
                                                   TsurugiFfiTransactionOptionHandle *transaction_option_out);

/**
 * TransactionOption: Set transaction type.
 *
 * See [`TransactionOption::set_transaction_type`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Parameters
 * - `transaction_type` - transaction type.
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_set_transaction_type(TsurugiFfiContextHandle context,
                                                                 TsurugiFfiTransactionOptionHandle transaction_option,
                                                                 TsurugiFfiTransactionType transaction_type);

/**
 * TransactionOption: Get transaction type.
 *
 * See [`TransactionOption::transaction_type`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Returns
 * - `transaction_type_out` - transaction type.
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_get_transaction_type(TsurugiFfiContextHandle context,
                                                                 TsurugiFfiTransactionOptionHandle transaction_option,
                                                                 TsurugiFfiTransactionType *transaction_type_out);

/**
 * TransactionOption: Set transaction label.
 *
 * See [`TransactionOption::set_transaction_label`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Parameters
 * - `transaction_label` - transaction label.
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_set_transaction_label(TsurugiFfiContextHandle context,
                                                                  TsurugiFfiTransactionOptionHandle transaction_option,
                                                                  TsurugiFfiStringHandle transaction_label);

/**
 * TransactionOption: Get transaction label.
 *
 * See [`TransactionOption::transaction_label`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Returns
 * - `transaction_label_out` - transaction label.
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_get_transaction_label(TsurugiFfiContextHandle context,
                                                                  TsurugiFfiTransactionOptionHandle transaction_option,
                                                                  TsurugiFfiStringHandle *transaction_label_out);

/**
 * TransactionOption: Set modifies definitions.
 *
 * See [`TransactionOption::set_modifies_definitions`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Parameters
 * - `modifies_definitions` - modifies definitions.
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_set_modifies_definitions(TsurugiFfiContextHandle context,
                                                                     TsurugiFfiTransactionOptionHandle transaction_option,
                                                                     bool modifies_definitions);

/**
 * TransactionOption: Get modifies definitions.
 *
 * See [`TransactionOption::modifies_definitions`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Returns
 * - `modifies_definitions_out` - modifies definitions.
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_get_modifies_definitions(TsurugiFfiContextHandle context,
                                                                     TsurugiFfiTransactionOptionHandle transaction_option,
                                                                     bool *modifies_definitions_out);

/**
 * TransactionOption: Set write preserve.
 *
 * See [`TransactionOption::set_write_preserve`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Parameters
 * - `table_names` - table names (String array).
 * - `table_names_size` - `table_names` size \[number of tables\].
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_set_write_preserve(TsurugiFfiContextHandle context,
                                                               TsurugiFfiTransactionOptionHandle transaction_option,
                                                               const TsurugiFfiStringHandle *table_names,
                                                               uint32_t table_names_size);

/**
 * TransactionOption: Get write preserve.
 *
 * See [`TransactionOption::write_preserve`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Returns
 * - `table_names_out` - table names (String array).
 * - `table_names_size_out` - `table_names_out` size \[number of tables\].
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_get_write_preserve(TsurugiFfiContextHandle context,
                                                               TsurugiFfiTransactionOptionHandle transaction_option,
                                                               TsurugiFfiStringArrayHandle *table_names_out,
                                                               uint32_t *table_names_size_out);

/**
 * TransactionOption: Set inclusive read area.
 *
 * See [`TransactionOption::set_inclusive_read_area`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Parameters
 * - `table_names` - table names (String array).
 * - `table_names_size` - `table_names` size \[number of tables\].
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_set_inclusive_read_area(TsurugiFfiContextHandle context,
                                                                    TsurugiFfiTransactionOptionHandle transaction_option,
                                                                    const TsurugiFfiStringHandle *table_names,
                                                                    uint32_t table_names_size);

/**
 * TransactionOption: Get inclusive read area.
 *
 * See [`TransactionOption::inclusive_read_area`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Returns
 * - `table_names_out` - table names (String array).
 * - `table_names_size_out` - `table_names_out` size \[number of tables\].
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_get_inclusive_read_area(TsurugiFfiContextHandle context,
                                                                    TsurugiFfiTransactionOptionHandle transaction_option,
                                                                    TsurugiFfiStringArrayHandle *table_names_out,
                                                                    uint32_t *table_names_size_out);

/**
 * TransactionOption: Set exclusive read area.
 *
 * See [`TransactionOption::set_exclusive_read_area`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Parameters
 * - `table_names` - table names (String array).
 * - `table_names_size` - `table_names` size \[number of tables\].
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_set_exclusive_read_area(TsurugiFfiContextHandle context,
                                                                    TsurugiFfiTransactionOptionHandle transaction_option,
                                                                    const TsurugiFfiStringHandle *table_names,
                                                                    uint32_t table_names_size);

/**
 * TransactionOption: Get exclusive read area.
 *
 * See [`TransactionOption::exclusive_read_area`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Returns
 * - `table_names_out` - table names (String array).
 * - `table_names_size_out` - `table_names_out` size \[number of tables\].
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_get_exclusive_read_area(TsurugiFfiContextHandle context,
                                                                    TsurugiFfiTransactionOptionHandle transaction_option,
                                                                    TsurugiFfiStringArrayHandle *table_names_out,
                                                                    uint32_t *table_names_size_out);

/**
 * TransactionOption: Set scan parallel.
 *
 * See [`TransactionOption::set_scan_parallel`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Parameters
 * - `scan_parallel` - scan parallel.
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_set_scan_parallel(TsurugiFfiContextHandle context,
                                                              TsurugiFfiTransactionOptionHandle transaction_option,
                                                              int32_t scan_parallel);

/**
 * TransactionOption: Get scan parallel.
 *
 * See [`TransactionOption::scan_parallel`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Returns
 * - `scan_parallel_exists_out` - `true`: scan parallel exists.
 * - `scan_parallel_out` - scan parallel.
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_get_scan_parallel(TsurugiFfiContextHandle context,
                                                              TsurugiFfiTransactionOptionHandle transaction_option,
                                                              bool *scan_parallel_exists_out,
                                                              int32_t *scan_parallel_out);

/**
 * TransactionOption: Set priority.
 *
 * See [`TransactionOption::set_priority`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Parameters
 * - `priority` - priority.
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_set_priority(TsurugiFfiContextHandle context,
                                                         TsurugiFfiTransactionOptionHandle transaction_option,
                                                         TsurugiFfiTransactionPriority priority);

/**
 * TransactionOption: Get priority.
 *
 * See [`TransactionOption::priority`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Returns
 * - `priority_out` - priority.
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_get_priority(TsurugiFfiContextHandle context,
                                                         TsurugiFfiTransactionOptionHandle transaction_option,
                                                         TsurugiFfiTransactionPriority *priority_out);

/**
 * TransactionOption: Set close timeout.
 *
 * See [`TransactionOption::set_close_timeout`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Parameters
 * - `close_timeout` - close timeout \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_set_close_timeout(TsurugiFfiContextHandle context,
                                                              TsurugiFfiTransactionOptionHandle transaction_option,
                                                              TsurugiFfiDuration close_timeout);

/**
 * TransactionOption: Get close timeout.
 *
 * See [`TransactionOption::close_timeout`].
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 *
 * # Returns
 * - `close_timeout_exists_out` - `true`: close timeout exists.
 * - `close_timeout_out` - close timeout \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_transaction_option_get_close_timeout(TsurugiFfiContextHandle context,
                                                              TsurugiFfiTransactionOptionHandle transaction_option,
                                                              bool *close_timeout_exists_out,
                                                              TsurugiFfiDuration *close_timeout_out);

/**
 * TransactionOption: Dispose.
 *
 * # Receiver
 * - `transaction_option` - Transaction option.
 */
void tsurugi_ffi_transaction_option_dispose(TsurugiFfiTransactionOptionHandle transaction_option);

/**
 * Transaction: Get transaction id.
 *
 * See [`Transaction::transaction_id`].
 *
 * # Receiver
 * - `transaction` - Transaction.
 *
 * # Returns
 * - `transaction_id_out` - transaction id.
 */
TsurugiFfiRc tsurugi_ffi_transaction_get_transaction_id(TsurugiFfiContextHandle context,
                                                        TsurugiFfiTransactionHandle transaction,
                                                        TsurugiFfiStringHandle *transaction_id_out);

/**
 * Transaction: Set close timeout.
 *
 * See [`Transaction::set_close_timeout`].
 *
 * # Receiver
 * - `transaction` - Transaction.
 *
 * # Parameters
 * - `close_timeout` - timeout time \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_transaction_set_close_timeout(TsurugiFfiContextHandle context,
                                                       TsurugiFfiTransactionHandle transaction,
                                                       TsurugiFfiDuration close_timeout);

/**
 * Transaction: Get close timeout.
 *
 * See [`Transaction::close_timeout`].
 *
 * # Receiver
 * - `transaction` - Transaction.
 *
 * # Returns
 * - `close_timeout_out` - timeout time \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_transaction_get_close_timeout(TsurugiFfiContextHandle context,
                                                       TsurugiFfiTransactionHandle transaction,
                                                       TsurugiFfiDuration *close_timeout_out);

/**
 * Transaction: Close.
 *
 * See [`Transaction::close`].
 *
 * Note: Close is called in [`tsurugi_ffi_transaction_dispose`].
 *
 * # Receiver
 * - `transaction` - Transaction.
 */
TsurugiFfiRc tsurugi_ffi_transaction_close(TsurugiFfiContextHandle context,
                                           TsurugiFfiTransactionHandle transaction);

/**
 * Transaction: Close.
 *
 * See [`Transaction::close_for`].
 *
 * Note: Close is called in [`tsurugi_ffi_transaction_dispose`].
 *
 * # Receiver
 * - `transaction` - Transaction.
 *
 * # Parameters
 * - `timeout` - timeout time \[nanoseconds\].
 */
TsurugiFfiRc tsurugi_ffi_transaction_close_for(TsurugiFfiContextHandle context,
                                               TsurugiFfiTransactionHandle transaction,
                                               TsurugiFfiDuration timeout);

/**
 * Transaction: Check if the session is closed.
 *
 * See [`Transaction::is_closed`].
 *
 * # Receiver
 * - `transaction` - Transaction.
 *
 * # Returns
 * - `is_closed_out` - `true`: Already closed / `false`: Not closed.
 */
TsurugiFfiRc tsurugi_ffi_transaction_is_closed(TsurugiFfiContextHandle context,
                                               TsurugiFfiTransactionHandle transaction,
                                               bool *is_closed_out);

/**
 * Transaction: Dispose.
 *
 * # Receiver
 * - `transaction` - Transaction.
 */
void tsurugi_ffi_transaction_dispose(TsurugiFfiTransactionHandle transaction);
