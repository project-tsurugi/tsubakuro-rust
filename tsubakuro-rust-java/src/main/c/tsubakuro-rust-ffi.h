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

enum TsurugiFfiRcType {
  TSURUGI_FFI_RC_TYPE_OK = 0,
  TSURUGI_FFI_RC_TYPE_FFI_ERROR = 1,
  TSURUGI_FFI_RC_TYPE_CORE_CLIENT_ERROR = 2,
  TSURUGI_FFI_RC_TYPE_CORE_SERVER_ERROR = 3,
};
typedef uint32_t TsurugiFfiRcType;

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

typedef struct TsurugiFfiCancelJob TsurugiFfiCancelJob;

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

typedef struct TsurugiFfiTgBlobReference TsurugiFfiTgBlobReference;

typedef struct TsurugiFfiTgClobReference TsurugiFfiTgClobReference;

typedef struct TsurugiFfiTransaction TsurugiFfiTransaction;

typedef struct TsurugiFfiTransactionOption TsurugiFfiTransactionOption;

typedef struct TsurugiFfiTransactionStatus TsurugiFfiTransactionStatus;

typedef uint32_t TsurugiFfiRc;

typedef struct TsurugiFfiContext *TsurugiFfiContextHandle;

typedef const char *TsurugiFfiStringHandle;

typedef struct TsurugiFfiCancelJob *TsurugiFfiCancelJobHandle;

/**
 * nanosecond
 */
typedef uint64_t TsurugiFfiDuration;

typedef void *TsurugiFfiJobHandle;

typedef struct TsurugiFfiSqlColumn *TsurugiFfiSqlColumnHandle;

typedef struct TsurugiFfiSqlExecuteResult *TsurugiFfiSqlExecuteResultHandle;

typedef struct TsurugiFfiSqlExplainResult *TsurugiFfiSqlExplainResultHandle;

typedef struct TsurugiFfiSqlParameter *TsurugiFfiSqlParameterHandle;

typedef const uint8_t *TsurugiFfiByteArrayHandle;

typedef struct TsurugiFfiSqlPlaceholder *TsurugiFfiSqlPlaceholderHandle;

typedef struct TsurugiFfiSqlPreparedStatement *TsurugiFfiSqlPreparedStatementHandle;

typedef struct TsurugiFfiSqlQueryResult *TsurugiFfiSqlQueryResultHandle;

typedef struct TsurugiFfiSqlQueryResultMetadata *TsurugiFfiSqlQueryResultMetadataHandle;

typedef struct TsurugiFfiTgBlobReference *TsurugiFfiBlobReferenceHandle;

typedef struct TsurugiFfiTgClobReference *TsurugiFfiClobReferenceHandle;

typedef struct TsurugiFfiSqlClient *TsurugiFfiSqlClientHandle;

typedef struct TsurugiFfiTableList *TsurugiFfiTableListHandle;

typedef struct TsurugiFfiTableMetadata *TsurugiFfiTableMetadataHandle;

typedef struct TsurugiFfiTransactionOption *TsurugiFfiTransactionOptionHandle;

typedef struct TsurugiFfiTransaction *TsurugiFfiTransactionHandle;

typedef struct TsurugiFfiTransactionStatus *TsurugiFfiTransactionStatusHandle;

typedef struct TsurugiFfiCommitOption *TsurugiFfiCommitOptionHandle;

typedef const TsurugiFfiStringHandle *TsurugiFfiStringArrayHandle;

typedef struct TsurugiFfiEndpoint *TsurugiFfiEndpointHandle;

typedef struct TsurugiFfiConnectionOption *TsurugiFfiConnectionOptionHandle;

typedef struct TsurugiFfiSession *TsurugiFfiSessionHandle;

#define TSURUGI_FFI_RC_OK 0

#define TSURUGI_FFI_RC_FFI_ARG0_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 0)

#define TSURUGI_FFI_RC_FFI_ARG1_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 1)

#define TSURUGI_FFI_RC_FFI_ARG2_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 2)

#define TSURUGI_FFI_RC_FFI_ARG3_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 3)

#define TSURUGI_FFI_RC_FFI_ARG4_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 4)

#define TSURUGI_FFI_RC_FFI_ARG5_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 5)

#define TSURUGI_FFI_RC_FFI_ARG6_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 6)

#define TSURUGI_FFI_RC_FFI_ARG7_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 7)

#define TSURUGI_FFI_RC_FFI_JOB_ALREADY_CLOSED (TSURUGI_FFI_RC_FFI_JOB_ERROR | 1)

#define TSURUGI_FFI_RC_FFI_NUL_ERROR (TSURUGI_FFI_RC_FFI_ERROR | 1)

#define TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND (TSURUGI_FFI_RC_FFI_ERROR | 2)

#define TSURUGI_FFI_RC_CORE_CLIENT_CLIENT_ERROR (TSURUGI_FFI_RC_CORE_CLIENT_ERROR | (1 << 16))

#define TSURUGI_FFI_RC_CORE_CLIENT_TIMEOUT_ERROR (TSURUGI_FFI_RC_CORE_CLIENT_ERROR | (2 << 16))

#define TSURUGI_FFI_RC_CORE_CLIENT_IO_ERROR (TSURUGI_FFI_RC_CORE_CLIENT_ERROR | (3 << 16))

TsurugiFfiRc tsurugi_ffi_context_create(TsurugiFfiContextHandle *context_out);

TsurugiFfiRc tsurugi_ffi_context_get_return_code(TsurugiFfiContextHandle context,
                                                 TsurugiFfiRc *rc_out);

TsurugiFfiRc tsurugi_ffi_context_get_error_name(TsurugiFfiContextHandle context,
                                                TsurugiFfiStringHandle *error_name_out);

TsurugiFfiRc tsurugi_ffi_context_get_error_type(TsurugiFfiContextHandle context,
                                                TsurugiFfiRcType *error_type_out);

TsurugiFfiRc tsurugi_ffi_context_get_error_message(TsurugiFfiContextHandle context,
                                                   TsurugiFfiStringHandle *error_message_out);

TsurugiFfiRc tsurugi_ffi_context_get_server_error_category_number(TsurugiFfiContextHandle context,
                                                                  int32_t *category_number_out);

TsurugiFfiRc tsurugi_ffi_context_get_server_error_category_str(TsurugiFfiContextHandle context,
                                                               TsurugiFfiStringHandle *category_str_out);

TsurugiFfiRc tsurugi_ffi_context_get_server_error_code_number(TsurugiFfiContextHandle context,
                                                              int32_t *code_number_out);

TsurugiFfiRc tsurugi_ffi_context_get_server_error_structured_code(TsurugiFfiContextHandle context,
                                                                  TsurugiFfiStringHandle *structured_code_out);

void tsurugi_ffi_context_dispose(TsurugiFfiContextHandle context);

TsurugiFfiRc tsurugi_ffi_cancel_job_wait(TsurugiFfiContextHandle context,
                                         TsurugiFfiCancelJobHandle cancel_job,
                                         TsurugiFfiDuration timeout,
                                         bool *done_out);

TsurugiFfiRc tsurugi_ffi_cancel_job_is_done(TsurugiFfiContextHandle context,
                                            TsurugiFfiCancelJobHandle cancel_job,
                                            bool *done_out);

void tsurugi_ffi_cancel_job_dispose(TsurugiFfiCancelJobHandle cancel_job);

TsurugiFfiRc tsurugi_ffi_job_get_name(TsurugiFfiContextHandle context,
                                      TsurugiFfiJobHandle job,
                                      TsurugiFfiStringHandle *name_out);

TsurugiFfiRc tsurugi_ffi_job_wait(TsurugiFfiContextHandle context,
                                  TsurugiFfiJobHandle job,
                                  TsurugiFfiDuration timeout,
                                  bool *done_out);

TsurugiFfiRc tsurugi_ffi_job_is_done(TsurugiFfiContextHandle context,
                                     TsurugiFfiJobHandle job,
                                     bool *done_out);

TsurugiFfiRc tsurugi_ffi_job_take(TsurugiFfiContextHandle context,
                                  TsurugiFfiJobHandle job,
                                  void **value_out);

TsurugiFfiRc tsurugi_ffi_job_take_for(TsurugiFfiContextHandle context,
                                      TsurugiFfiJobHandle job,
                                      TsurugiFfiDuration timeout,
                                      void **value_out);

TsurugiFfiRc tsurugi_ffi_job_take_if_ready(TsurugiFfiContextHandle context,
                                           TsurugiFfiJobHandle job,
                                           bool *is_ready_out,
                                           void **value_out);

TsurugiFfiRc tsurugi_ffi_job_cancel(TsurugiFfiContextHandle context,
                                    TsurugiFfiJobHandle job,
                                    bool *cancell_done_out);

TsurugiFfiRc tsurugi_ffi_job_cancel_for(TsurugiFfiContextHandle context,
                                        TsurugiFfiJobHandle job,
                                        TsurugiFfiDuration timeout,
                                        bool *cancell_done_out);

TsurugiFfiRc tsurugi_ffi_job_cancel_async(TsurugiFfiContextHandle context,
                                          TsurugiFfiJobHandle job,
                                          TsurugiFfiCancelJobHandle *cancel_job_out);

TsurugiFfiRc tsurugi_ffi_job_close(TsurugiFfiContextHandle context, TsurugiFfiJobHandle job);

void tsurugi_ffi_job_dispose(TsurugiFfiJobHandle job);

TsurugiFfiRc tsurugi_ffi_env_logger_init(void);

TsurugiFfiRc tsurugi_ffi_sql_column_get_name(TsurugiFfiContextHandle context,
                                             TsurugiFfiSqlColumnHandle sql_column,
                                             TsurugiFfiStringHandle *name_out);

TsurugiFfiRc tsurugi_ffi_sql_column_get_atom_type(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlColumnHandle sql_column,
                                                  TsurugiFfiAtomType *atom_type_out);

void tsurugi_ffi_sql_column_dispose(TsurugiFfiSqlColumnHandle sql_column);

TsurugiFfiRc tsurugi_ffi_sql_execute_result_get_counters(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlExecuteResultHandle execute_result,
                                                         const TsurugiFfiSqlCounterType **counters_keys_out,
                                                         const int64_t **counters_rows_out,
                                                         uint32_t *counters_size_out);

TsurugiFfiRc tsurugi_ffi_sql_execute_result_get_inserted_rows(TsurugiFfiContextHandle context,
                                                              TsurugiFfiSqlExecuteResultHandle execute_result,
                                                              int64_t *rows_out);

TsurugiFfiRc tsurugi_ffi_sql_execute_result_get_updated_rows(TsurugiFfiContextHandle context,
                                                             TsurugiFfiSqlExecuteResultHandle execute_result,
                                                             int64_t *rows_out);

TsurugiFfiRc tsurugi_ffi_sql_execute_result_get_merged_rows(TsurugiFfiContextHandle context,
                                                            TsurugiFfiSqlExecuteResultHandle execute_result,
                                                            int64_t *rows_out);

TsurugiFfiRc tsurugi_ffi_sql_execute_result_get_deleted_rows(TsurugiFfiContextHandle context,
                                                             TsurugiFfiSqlExecuteResultHandle execute_result,
                                                             int64_t *rows_out);

TsurugiFfiRc tsurugi_ffi_sql_execute_result_get_rows(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlExecuteResultHandle execute_result,
                                                     int64_t *rows_out);

void tsurugi_ffi_sql_execute_result_dispose(TsurugiFfiSqlExecuteResultHandle execute_result);

TsurugiFfiRc tsurugi_ffi_sql_explain_result_get_format_id(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlExplainResultHandle explain_result,
                                                          TsurugiFfiStringHandle *format_id_out);

TsurugiFfiRc tsurugi_ffi_sql_explain_result_get_format_version(TsurugiFfiContextHandle context,
                                                               TsurugiFfiSqlExplainResultHandle explain_result,
                                                               uint64_t *format_version_out);

TsurugiFfiRc tsurugi_ffi_sql_explain_result_get_contents(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlExplainResultHandle explain_result,
                                                         TsurugiFfiStringHandle *contents_out);

TsurugiFfiRc tsurugi_ffi_explain_result_get_columns_size(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlExplainResultHandle explain_result,
                                                         uint32_t *size_out);

TsurugiFfiRc tsurugi_ffi_explain_result_get_columns_value(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlExplainResultHandle explain_result,
                                                          uint32_t index,
                                                          TsurugiFfiSqlColumnHandle *sql_column_out);

void tsurugi_ffi_sql_explain_result_dispose(TsurugiFfiSqlExplainResultHandle explain_result);

TsurugiFfiRc tsurugi_ffi_sql_parameter_null(TsurugiFfiContextHandle context,
                                            TsurugiFfiStringHandle name,
                                            TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_int4(TsurugiFfiContextHandle context,
                                               TsurugiFfiStringHandle name,
                                               int32_t value,
                                               TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_int8(TsurugiFfiContextHandle context,
                                               TsurugiFfiStringHandle name,
                                               int64_t value,
                                               TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_float4(TsurugiFfiContextHandle context,
                                                 TsurugiFfiStringHandle name,
                                                 float value,
                                                 TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_float8(TsurugiFfiContextHandle context,
                                                 TsurugiFfiStringHandle name,
                                                 double value,
                                                 TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_decimal(TsurugiFfiContextHandle context,
                                                  TsurugiFfiStringHandle name,
                                                  TsurugiFfiByteArrayHandle unscaled_value,
                                                  uint32_t unscaled_value_size,
                                                  int32_t exponent,
                                                  TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_decimal_i128(TsurugiFfiContextHandle context,
                                                       TsurugiFfiStringHandle name,
                                                       int64_t unscaled_value_high,
                                                       uint64_t unscaled_value_low,
                                                       int32_t exponent,
                                                       TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_character(TsurugiFfiContextHandle context,
                                                    TsurugiFfiStringHandle name,
                                                    TsurugiFfiStringHandle value,
                                                    TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_octet(TsurugiFfiContextHandle context,
                                                TsurugiFfiStringHandle name,
                                                TsurugiFfiByteArrayHandle value,
                                                uint64_t value_size,
                                                TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_date(TsurugiFfiContextHandle context,
                                               TsurugiFfiStringHandle name,
                                               int64_t epoch_days,
                                               TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_time_of_day(TsurugiFfiContextHandle context,
                                                      TsurugiFfiStringHandle name,
                                                      uint64_t nanoseconds_of_day,
                                                      TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_time_point(TsurugiFfiContextHandle context,
                                                     TsurugiFfiStringHandle name,
                                                     int64_t epoch_seconds,
                                                     uint32_t nanos,
                                                     TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_time_of_day_with_time_zone(TsurugiFfiContextHandle context,
                                                                     TsurugiFfiStringHandle name,
                                                                     uint64_t nanoseconds_of_day,
                                                                     int32_t time_zone_offset,
                                                                     TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_time_point_with_time_zone(TsurugiFfiContextHandle context,
                                                                    TsurugiFfiStringHandle name,
                                                                    int64_t epoch_seconds,
                                                                    uint32_t nanos,
                                                                    int32_t time_zone_offset,
                                                                    TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_blob(TsurugiFfiContextHandle context,
                                               TsurugiFfiStringHandle name,
                                               TsurugiFfiStringHandle path,
                                               TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_blob_contents(TsurugiFfiContextHandle context,
                                                        TsurugiFfiStringHandle name,
                                                        TsurugiFfiByteArrayHandle value,
                                                        uint64_t value_size,
                                                        TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_clob(TsurugiFfiContextHandle context,
                                               TsurugiFfiStringHandle name,
                                               TsurugiFfiStringHandle path,
                                               TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_clob_contents(TsurugiFfiContextHandle context,
                                                        TsurugiFfiStringHandle name,
                                                        TsurugiFfiStringHandle value,
                                                        TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_get_name(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlParameterHandle parameter,
                                                TsurugiFfiStringHandle *name_out);

void tsurugi_ffi_sql_parameter_dispose(TsurugiFfiSqlParameterHandle parameter);

TsurugiFfiRc tsurugi_ffi_sql_placeholder_of_atom_type(TsurugiFfiContextHandle context,
                                                      TsurugiFfiStringHandle name,
                                                      TsurugiFfiAtomType atom_type,
                                                      TsurugiFfiSqlPlaceholderHandle *placeholder_out);

TsurugiFfiRc tsurugi_ffi_sql_placeholder_get_name(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlPlaceholderHandle placeholder,
                                                  TsurugiFfiStringHandle *name_out);

TsurugiFfiRc tsurugi_ffi_sql_placeholder_get_atom_type(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlPlaceholderHandle placeholder,
                                                       TsurugiFfiAtomType *atom_type_out);

void tsurugi_ffi_sql_placeholder_dispose(TsurugiFfiSqlPlaceholderHandle placeholder);

TsurugiFfiRc tsurugi_ffi_sql_prepared_statement_has_result_records(TsurugiFfiContextHandle context,
                                                                   TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                                   bool *has_result_records_out);

TsurugiFfiRc tsurugi_ffi_sql_prepared_statement_set_close_timeout(TsurugiFfiContextHandle context,
                                                                  TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                                  TsurugiFfiDuration timeout);

TsurugiFfiRc tsurugi_ffi_sql_prepared_statement_get_close_timeout(TsurugiFfiContextHandle context,
                                                                  TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                                  TsurugiFfiDuration *close_timeout_out);

TsurugiFfiRc tsurugi_ffi_sql_prepared_statement_close(TsurugiFfiContextHandle context,
                                                      TsurugiFfiSqlPreparedStatementHandle prepared_statement);

TsurugiFfiRc tsurugi_ffi_sql_prepared_statement_close_for(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                          TsurugiFfiDuration timeout);

TsurugiFfiRc tsurugi_ffi_sql_prepared_statement_is_closed(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                          bool *is_closed_out);

void tsurugi_ffi_sql_prepared_statement_dispose(TsurugiFfiSqlPreparedStatementHandle prepared_statement);

TsurugiFfiRc tsurugi_ffi_sql_query_result_set_default_timeout(TsurugiFfiContextHandle context,
                                                              TsurugiFfiSqlQueryResultHandle query_result,
                                                              TsurugiFfiDuration timeout);

TsurugiFfiRc tsurugi_ffi_sql_query_result_get_default_timeout(TsurugiFfiContextHandle context,
                                                              TsurugiFfiSqlQueryResultHandle query_result,
                                                              TsurugiFfiDuration *default_timeout_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_get_metadata(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlQueryResultHandle query_result,
                                                       TsurugiFfiSqlQueryResultMetadataHandle *query_result_metadata_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_next_row(TsurugiFfiContextHandle context,
                                                   TsurugiFfiSqlQueryResultHandle query_result,
                                                   bool *has_row_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_next_row_for(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlQueryResultHandle query_result,
                                                       TsurugiFfiDuration timeout,
                                                       bool *has_row_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_next_column(TsurugiFfiContextHandle context,
                                                      TsurugiFfiSqlQueryResultHandle query_result,
                                                      bool *has_column_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_next_column_for(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlQueryResultHandle query_result,
                                                          TsurugiFfiDuration timeout,
                                                          bool *has_column_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_is_null(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlQueryResultHandle query_result,
                                                  bool *is_null_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_int4(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlQueryResultHandle query_result,
                                                     int32_t *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_int4(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlQueryResultHandle query_result,
                                                         TsurugiFfiDuration timeout,
                                                         int32_t *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_int8(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlQueryResultHandle query_result,
                                                     int64_t *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_int8(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlQueryResultHandle query_result,
                                                         TsurugiFfiDuration timeout,
                                                         int64_t *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_float4(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlQueryResultHandle query_result,
                                                       float *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_float4(TsurugiFfiContextHandle context,
                                                           TsurugiFfiSqlQueryResultHandle query_result,
                                                           TsurugiFfiDuration timeout,
                                                           float *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_float8(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlQueryResultHandle query_result,
                                                       double *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_float8(TsurugiFfiContextHandle context,
                                                           TsurugiFfiSqlQueryResultHandle query_result,
                                                           TsurugiFfiDuration timeout,
                                                           double *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_decimal(TsurugiFfiContextHandle context,
                                                        TsurugiFfiSqlQueryResultHandle query_result,
                                                        TsurugiFfiByteArrayHandle *unscaled_value_bytes_out,
                                                        uint32_t *unscaled_value_bytes_size_out,
                                                        int64_t *unscaled_value_out,
                                                        int32_t *exponent_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_decimal(TsurugiFfiContextHandle context,
                                                            TsurugiFfiSqlQueryResultHandle query_result,
                                                            TsurugiFfiDuration timeout,
                                                            TsurugiFfiByteArrayHandle *unscaled_value_bytes_out,
                                                            uint32_t *unscaled_value_bytes_size_out,
                                                            int64_t *unscaled_value_out,
                                                            int32_t *exponent_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_decimal_i128(TsurugiFfiContextHandle context,
                                                             TsurugiFfiSqlQueryResultHandle query_result,
                                                             int64_t *unscaled_value_high_out,
                                                             uint64_t *unscaled_value_low_out,
                                                             int32_t *exponent_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_decimal_i128(TsurugiFfiContextHandle context,
                                                                 TsurugiFfiSqlQueryResultHandle query_result,
                                                                 TsurugiFfiDuration timeout,
                                                                 int64_t *unscaled_value_high_out,
                                                                 uint64_t *unscaled_value_low_out,
                                                                 int32_t *exponent_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_character(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlQueryResultHandle query_result,
                                                          TsurugiFfiStringHandle *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_character(TsurugiFfiContextHandle context,
                                                              TsurugiFfiSqlQueryResultHandle query_result,
                                                              TsurugiFfiDuration timeout,
                                                              TsurugiFfiStringHandle *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_octet(TsurugiFfiContextHandle context,
                                                      TsurugiFfiSqlQueryResultHandle query_result,
                                                      TsurugiFfiByteArrayHandle *value_out,
                                                      uint64_t *size_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_octet(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlQueryResultHandle query_result,
                                                          TsurugiFfiDuration timeout,
                                                          TsurugiFfiByteArrayHandle *value_out,
                                                          uint64_t *size_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_date(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlQueryResultHandle query_result,
                                                     int64_t *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_date(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlQueryResultHandle query_result,
                                                         TsurugiFfiDuration timeout,
                                                         int64_t *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_time_of_day(TsurugiFfiContextHandle context,
                                                            TsurugiFfiSqlQueryResultHandle query_result,
                                                            uint64_t *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_time_of_day(TsurugiFfiContextHandle context,
                                                                TsurugiFfiSqlQueryResultHandle query_result,
                                                                TsurugiFfiDuration timeout,
                                                                uint64_t *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_time_point(TsurugiFfiContextHandle context,
                                                           TsurugiFfiSqlQueryResultHandle query_result,
                                                           int64_t *value_out,
                                                           uint32_t *nanos_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_time_point(TsurugiFfiContextHandle context,
                                                               TsurugiFfiSqlQueryResultHandle query_result,
                                                               TsurugiFfiDuration timeout,
                                                               int64_t *value_out,
                                                               uint32_t *nanos_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_time_of_day_with_time_zone(TsurugiFfiContextHandle context,
                                                                           TsurugiFfiSqlQueryResultHandle query_result,
                                                                           uint64_t *value_out,
                                                                           int32_t *time_zone_offset_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_time_of_day_with_time_zone(TsurugiFfiContextHandle context,
                                                                               TsurugiFfiSqlQueryResultHandle query_result,
                                                                               TsurugiFfiDuration timeout,
                                                                               uint64_t *value_out,
                                                                               int32_t *time_zone_offset_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_time_point_with_time_zone(TsurugiFfiContextHandle context,
                                                                          TsurugiFfiSqlQueryResultHandle query_result,
                                                                          int64_t *value_out,
                                                                          uint32_t *nanos_out,
                                                                          int32_t *time_zone_offset_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_time_point_with_time_zone(TsurugiFfiContextHandle context,
                                                                              TsurugiFfiSqlQueryResultHandle query_result,
                                                                              TsurugiFfiDuration timeout,
                                                                              int64_t *value_out,
                                                                              uint32_t *nanos_out,
                                                                              int32_t *time_zone_offset_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_blob(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlQueryResultHandle query_result,
                                                     TsurugiFfiBlobReferenceHandle *blob_reference_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_blob(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlQueryResultHandle query_result,
                                                         TsurugiFfiDuration timeout,
                                                         TsurugiFfiBlobReferenceHandle *blob_reference_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_clob(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlQueryResultHandle query_result,
                                                     TsurugiFfiClobReferenceHandle *clob_reference_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_for_clob(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlQueryResultHandle query_result,
                                                         TsurugiFfiDuration timeout,
                                                         TsurugiFfiClobReferenceHandle *clob_reference_out);

void tsurugi_ffi_sql_query_result_dispose(TsurugiFfiSqlQueryResultHandle query_result);

TsurugiFfiRc tsurugi_ffi_sql_query_result_metadata_get_columns_size(TsurugiFfiContextHandle context,
                                                                    TsurugiFfiSqlQueryResultMetadataHandle query_result_metadata,
                                                                    uint32_t *size_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_metadata_get_columns_value(TsurugiFfiContextHandle context,
                                                                     TsurugiFfiSqlQueryResultMetadataHandle query_result_metadata,
                                                                     uint32_t index,
                                                                     TsurugiFfiSqlColumnHandle *sql_column_out);

void tsurugi_ffi_sql_query_result_metadata_dispose(TsurugiFfiSqlQueryResultMetadataHandle query_result_metadata);

TsurugiFfiRc tsurugi_ffi_sql_client_get_service_message_version(TsurugiFfiContextHandle context,
                                                                TsurugiFfiSqlClientHandle sql_client,
                                                                TsurugiFfiStringHandle *version_out);

TsurugiFfiRc tsurugi_ffi_sql_client_list_tables(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlClientHandle sql_client,
                                                TsurugiFfiTableListHandle *table_list_out);

TsurugiFfiRc tsurugi_ffi_sql_client_list_tables_for(TsurugiFfiContextHandle context,
                                                    TsurugiFfiSqlClientHandle sql_client,
                                                    TsurugiFfiDuration timeout,
                                                    TsurugiFfiTableListHandle *table_list_out);

TsurugiFfiRc tsurugi_ffi_sql_client_list_tables_async(TsurugiFfiContextHandle context,
                                                      TsurugiFfiSqlClientHandle sql_client,
                                                      TsurugiFfiJobHandle *table_list_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_get_table_metadata(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlClientHandle sql_client,
                                                       TsurugiFfiStringHandle table_name,
                                                       TsurugiFfiTableMetadataHandle *table_metadata_out);

TsurugiFfiRc tsurugi_ffi_sql_client_get_table_metadata_for(TsurugiFfiContextHandle context,
                                                           TsurugiFfiSqlClientHandle sql_client,
                                                           TsurugiFfiStringHandle table_name,
                                                           TsurugiFfiDuration timeout,
                                                           TsurugiFfiTableMetadataHandle *table_metadata_out);

TsurugiFfiRc tsurugi_ffi_sql_client_get_table_metadata_async(TsurugiFfiContextHandle context,
                                                             TsurugiFfiSqlClientHandle sql_client,
                                                             TsurugiFfiStringHandle table_name,
                                                             TsurugiFfiJobHandle *table_metadata_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepare(TsurugiFfiContextHandle context,
                                            TsurugiFfiSqlClientHandle sql_client,
                                            TsurugiFfiStringHandle sql,
                                            const TsurugiFfiSqlPlaceholderHandle *placeholders,
                                            uint32_t placeholders_size,
                                            TsurugiFfiSqlPreparedStatementHandle *prepared_statement_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepare_for(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlClientHandle sql_client,
                                                TsurugiFfiStringHandle sql,
                                                const TsurugiFfiSqlPlaceholderHandle *placeholders,
                                                uint32_t placeholders_size,
                                                TsurugiFfiDuration timeout,
                                                TsurugiFfiSqlPreparedStatementHandle *prepared_statement_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepare_async(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlClientHandle sql_client,
                                                  TsurugiFfiStringHandle sql,
                                                  const TsurugiFfiSqlPlaceholderHandle *placeholders,
                                                  uint32_t placeholders_size,
                                                  TsurugiFfiJobHandle *prepared_statement_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_explain(TsurugiFfiContextHandle context,
                                            TsurugiFfiSqlClientHandle sql_client,
                                            TsurugiFfiStringHandle sql,
                                            TsurugiFfiSqlExplainResultHandle *explain_result_out);

TsurugiFfiRc tsurugi_ffi_sql_client_explain_for(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlClientHandle sql_client,
                                                TsurugiFfiStringHandle sql,
                                                TsurugiFfiDuration timeout,
                                                TsurugiFfiSqlExplainResultHandle *explain_result_out);

TsurugiFfiRc tsurugi_ffi_sql_client_explain_async(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlClientHandle sql_client,
                                                  TsurugiFfiStringHandle sql,
                                                  TsurugiFfiJobHandle *explain_result_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepared_explain(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlClientHandle sql_client,
                                                     TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                     const TsurugiFfiSqlParameterHandle *parameters,
                                                     uint32_t parameter_size,
                                                     TsurugiFfiSqlExplainResultHandle *explain_result_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepared_explain_for(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlClientHandle sql_client,
                                                         TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                         const TsurugiFfiSqlParameterHandle *parameters,
                                                         uint32_t parameter_size,
                                                         TsurugiFfiDuration timeout,
                                                         TsurugiFfiSqlExplainResultHandle *explain_result_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepared_explain_async(TsurugiFfiContextHandle context,
                                                           TsurugiFfiSqlClientHandle sql_client,
                                                           TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                           const TsurugiFfiSqlParameterHandle *parameters,
                                                           uint32_t parameter_size,
                                                           TsurugiFfiJobHandle *explain_result_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_start_transaction(TsurugiFfiContextHandle context,
                                                      TsurugiFfiSqlClientHandle sql_client,
                                                      TsurugiFfiTransactionOptionHandle transaction_option,
                                                      TsurugiFfiTransactionHandle *transaction_out);

TsurugiFfiRc tsurugi_ffi_sql_client_start_transaction_for(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlClientHandle sql_client,
                                                          TsurugiFfiTransactionOptionHandle transaction_option,
                                                          TsurugiFfiDuration timeout,
                                                          TsurugiFfiTransactionHandle *transaction_out);

TsurugiFfiRc tsurugi_ffi_sql_client_start_transaction_async(TsurugiFfiContextHandle context,
                                                            TsurugiFfiSqlClientHandle sql_client,
                                                            TsurugiFfiTransactionOptionHandle transaction_option,
                                                            TsurugiFfiJobHandle *transaction_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_get_transaction_status(TsurugiFfiContextHandle context,
                                                           TsurugiFfiSqlClientHandle sql_client,
                                                           TsurugiFfiTransactionHandle transaction,
                                                           TsurugiFfiTransactionStatusHandle *transaction_status_out);

TsurugiFfiRc tsurugi_ffi_sql_client_get_transaction_status_for(TsurugiFfiContextHandle context,
                                                               TsurugiFfiSqlClientHandle sql_client,
                                                               TsurugiFfiTransactionHandle transaction,
                                                               TsurugiFfiDuration timeout,
                                                               TsurugiFfiTransactionStatusHandle *transaction_status_out);

TsurugiFfiRc tsurugi_ffi_sql_client_get_transaction_status_async(TsurugiFfiContextHandle context,
                                                                 TsurugiFfiSqlClientHandle sql_client,
                                                                 TsurugiFfiTransactionHandle transaction,
                                                                 TsurugiFfiJobHandle *transaction_status_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_execute(TsurugiFfiContextHandle context,
                                            TsurugiFfiSqlClientHandle sql_client,
                                            TsurugiFfiTransactionHandle transaction,
                                            TsurugiFfiStringHandle sql,
                                            TsurugiFfiSqlExecuteResultHandle *execute_result_out);

TsurugiFfiRc tsurugi_ffi_sql_client_execute_for(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlClientHandle sql_client,
                                                TsurugiFfiTransactionHandle transaction,
                                                TsurugiFfiStringHandle sql,
                                                TsurugiFfiDuration timeout,
                                                TsurugiFfiSqlExecuteResultHandle *execute_result_out);

TsurugiFfiRc tsurugi_ffi_sql_client_execute_async(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlClientHandle sql_client,
                                                  TsurugiFfiTransactionHandle transaction,
                                                  TsurugiFfiStringHandle sql,
                                                  TsurugiFfiJobHandle *execute_result_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepared_execute(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlClientHandle sql_client,
                                                     TsurugiFfiTransactionHandle transaction,
                                                     TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                     const TsurugiFfiSqlParameterHandle *parameters,
                                                     uint32_t parameter_size,
                                                     TsurugiFfiSqlExecuteResultHandle *execute_result_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepared_execute_for(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlClientHandle sql_client,
                                                         TsurugiFfiTransactionHandle transaction,
                                                         TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                         const TsurugiFfiSqlParameterHandle *parameters,
                                                         uint32_t parameter_size,
                                                         TsurugiFfiDuration timeout,
                                                         TsurugiFfiSqlExecuteResultHandle *execute_result_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepared_execute_async(TsurugiFfiContextHandle context,
                                                           TsurugiFfiSqlClientHandle sql_client,
                                                           TsurugiFfiTransactionHandle transaction,
                                                           TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                           const TsurugiFfiSqlParameterHandle *parameters,
                                                           uint32_t parameter_size,
                                                           TsurugiFfiJobHandle *execute_result_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_query(TsurugiFfiContextHandle context,
                                          TsurugiFfiSqlClientHandle sql_client,
                                          TsurugiFfiTransactionHandle transaction,
                                          TsurugiFfiStringHandle sql,
                                          TsurugiFfiSqlQueryResultHandle *query_result_out);

TsurugiFfiRc tsurugi_ffi_sql_client_query_for(TsurugiFfiContextHandle context,
                                              TsurugiFfiSqlClientHandle sql_client,
                                              TsurugiFfiTransactionHandle transaction,
                                              TsurugiFfiStringHandle sql,
                                              TsurugiFfiDuration timeout,
                                              TsurugiFfiSqlQueryResultHandle *query_result_out);

TsurugiFfiRc tsurugi_ffi_sql_client_query_async(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlClientHandle sql_client,
                                                TsurugiFfiTransactionHandle transaction,
                                                TsurugiFfiStringHandle sql,
                                                TsurugiFfiJobHandle *query_result_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepared_query(TsurugiFfiContextHandle context,
                                                   TsurugiFfiSqlClientHandle sql_client,
                                                   TsurugiFfiTransactionHandle transaction,
                                                   TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                   const TsurugiFfiSqlParameterHandle *parameters,
                                                   uint32_t parameter_size,
                                                   TsurugiFfiSqlQueryResultHandle *query_result_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepared_query_for(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlClientHandle sql_client,
                                                       TsurugiFfiTransactionHandle transaction,
                                                       TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                       const TsurugiFfiSqlParameterHandle *parameters,
                                                       uint32_t parameter_size,
                                                       TsurugiFfiDuration timeout,
                                                       TsurugiFfiSqlQueryResultHandle *query_result_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepared_query_async(TsurugiFfiContextHandle context,
                                                         TsurugiFfiSqlClientHandle sql_client,
                                                         TsurugiFfiTransactionHandle transaction,
                                                         TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                         const TsurugiFfiSqlParameterHandle *parameters,
                                                         uint32_t parameter_size,
                                                         TsurugiFfiJobHandle *query_result_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_commit(TsurugiFfiContextHandle context,
                                           TsurugiFfiSqlClientHandle sql_client,
                                           TsurugiFfiTransactionHandle transaction,
                                           TsurugiFfiCommitOptionHandle commit_option);

TsurugiFfiRc tsurugi_ffi_sql_client_commit_for(TsurugiFfiContextHandle context,
                                               TsurugiFfiSqlClientHandle sql_client,
                                               TsurugiFfiTransactionHandle transaction,
                                               TsurugiFfiCommitOptionHandle commit_option,
                                               TsurugiFfiDuration timeout);

TsurugiFfiRc tsurugi_ffi_sql_client_commit_async(TsurugiFfiContextHandle context,
                                                 TsurugiFfiSqlClientHandle sql_client,
                                                 TsurugiFfiTransactionHandle transaction,
                                                 TsurugiFfiCommitOptionHandle commit_option,
                                                 TsurugiFfiJobHandle *commit_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_rollback(TsurugiFfiContextHandle context,
                                             TsurugiFfiSqlClientHandle sql_client,
                                             TsurugiFfiTransactionHandle transaction);

TsurugiFfiRc tsurugi_ffi_sql_client_rollback_for(TsurugiFfiContextHandle context,
                                                 TsurugiFfiSqlClientHandle sql_client,
                                                 TsurugiFfiTransactionHandle transaction,
                                                 TsurugiFfiDuration timeout);

TsurugiFfiRc tsurugi_ffi_sql_client_rollback_async(TsurugiFfiContextHandle context,
                                                   TsurugiFfiSqlClientHandle sql_client,
                                                   TsurugiFfiTransactionHandle transaction,
                                                   TsurugiFfiJobHandle *rollback_job_out);

void tsurugi_ffi_sql_client_dispose(TsurugiFfiSqlClientHandle sql_client);

TsurugiFfiRc tsurugi_ffi_table_list_get_table_names(TsurugiFfiContextHandle context,
                                                    TsurugiFfiTableListHandle table_list,
                                                    TsurugiFfiStringArrayHandle *table_names_out,
                                                    uint32_t *table_names_size_out);

void tsurugi_ffi_table_list_dispose(TsurugiFfiTableListHandle table_list);

TsurugiFfiRc tsurugi_ffi_table_metadata_get_database_name(TsurugiFfiContextHandle context,
                                                          TsurugiFfiTableMetadataHandle table_metadata,
                                                          TsurugiFfiStringHandle *database_name_out);

TsurugiFfiRc tsurugi_ffi_table_metadata_get_schema_name(TsurugiFfiContextHandle context,
                                                        TsurugiFfiTableMetadataHandle table_metadata,
                                                        TsurugiFfiStringHandle *schema_name_out);

TsurugiFfiRc tsurugi_ffi_table_metadata_get_table_name(TsurugiFfiContextHandle context,
                                                       TsurugiFfiTableMetadataHandle table_metadata,
                                                       TsurugiFfiStringHandle *table_name_out);

TsurugiFfiRc tsurugi_ffi_table_metadata_get_columns_size(TsurugiFfiContextHandle context,
                                                         TsurugiFfiTableMetadataHandle table_metadata,
                                                         uint32_t *size_out);

TsurugiFfiRc tsurugi_ffi_table_metadata_get_columns_value(TsurugiFfiContextHandle context,
                                                          TsurugiFfiTableMetadataHandle table_metadata,
                                                          uint32_t index,
                                                          TsurugiFfiSqlColumnHandle *sql_column_out);

void tsurugi_ffi_table_metadata_dispose(TsurugiFfiTableMetadataHandle table_metadata);

void tsurugi_ffi_blob_reference_dispose(TsurugiFfiBlobReferenceHandle blob_reference);

void tsurugi_ffi_clob_reference_dispose(TsurugiFfiClobReferenceHandle clob_reference);

TsurugiFfiRc tsurugi_ffi_endpoint_parse(TsurugiFfiContextHandle context,
                                        TsurugiFfiStringHandle endpoint,
                                        TsurugiFfiEndpointHandle *endpoint_out);

void tsurugi_ffi_endpoint_dispose(TsurugiFfiEndpointHandle endpoint);

TsurugiFfiRc tsurugi_ffi_connection_option_create(TsurugiFfiContextHandle context,
                                                  TsurugiFfiConnectionOptionHandle *connection_option_out);

TsurugiFfiRc tsurugi_ffi_connection_option_set_endpoint(TsurugiFfiContextHandle context,
                                                        TsurugiFfiConnectionOptionHandle connection_option,
                                                        TsurugiFfiEndpointHandle endpoint);

TsurugiFfiRc tsurugi_ffi_connection_option_set_endpoint_url(TsurugiFfiContextHandle context,
                                                            TsurugiFfiConnectionOptionHandle connection_option,
                                                            TsurugiFfiStringHandle endpoint);

TsurugiFfiRc tsurugi_ffi_connection_option_get_endpoint(TsurugiFfiContextHandle context,
                                                        TsurugiFfiConnectionOptionHandle connection_option,
                                                        TsurugiFfiStringHandle *endpoint_out);

TsurugiFfiRc tsurugi_ffi_connection_option_set_application_name(TsurugiFfiContextHandle context,
                                                                TsurugiFfiConnectionOptionHandle connection_option,
                                                                TsurugiFfiStringHandle application_name);

TsurugiFfiRc tsurugi_ffi_connection_option_get_application_name(TsurugiFfiContextHandle context,
                                                                TsurugiFfiConnectionOptionHandle connection_option,
                                                                TsurugiFfiStringHandle *application_name_out);

TsurugiFfiRc tsurugi_ffi_connection_option_set_session_label(TsurugiFfiContextHandle context,
                                                             TsurugiFfiConnectionOptionHandle connection_option,
                                                             TsurugiFfiStringHandle session_label);

TsurugiFfiRc tsurugi_ffi_connection_option_get_session_label(TsurugiFfiContextHandle context,
                                                             TsurugiFfiConnectionOptionHandle connection_option,
                                                             TsurugiFfiStringHandle *session_label_out);

TsurugiFfiRc tsurugi_ffi_connection_option_set_keep_alive(TsurugiFfiContextHandle context,
                                                          TsurugiFfiConnectionOptionHandle connection_option,
                                                          TsurugiFfiDuration keep_alive);

TsurugiFfiRc tsurugi_ffi_connection_option_get_keep_alive(TsurugiFfiContextHandle context,
                                                          TsurugiFfiConnectionOptionHandle connection_option,
                                                          TsurugiFfiDuration *keep_alive_out);

TsurugiFfiRc tsurugi_ffi_connection_option_set_default_timeout(TsurugiFfiContextHandle context,
                                                               TsurugiFfiConnectionOptionHandle connection_option,
                                                               TsurugiFfiDuration default_timeout);

TsurugiFfiRc tsurugi_ffi_connection_option_get_default_timeout(TsurugiFfiContextHandle context,
                                                               TsurugiFfiConnectionOptionHandle connection_option,
                                                               TsurugiFfiDuration *default_timeout_out);

TsurugiFfiRc tsurugi_ffi_connection_option_set_send_timeout(TsurugiFfiContextHandle context,
                                                            TsurugiFfiConnectionOptionHandle connection_option,
                                                            TsurugiFfiDuration send_timeout);

TsurugiFfiRc tsurugi_ffi_connection_option_get_send_timeout(TsurugiFfiContextHandle context,
                                                            TsurugiFfiConnectionOptionHandle connection_option,
                                                            TsurugiFfiDuration *send_timeout_out);

TsurugiFfiRc tsurugi_ffi_connection_option_set_recv_timeout(TsurugiFfiContextHandle context,
                                                            TsurugiFfiConnectionOptionHandle connection_option,
                                                            TsurugiFfiDuration recv_timeout);

TsurugiFfiRc tsurugi_ffi_connection_option_get_recv_timeout(TsurugiFfiContextHandle context,
                                                            TsurugiFfiConnectionOptionHandle connection_option,
                                                            TsurugiFfiDuration *recv_timeout_out);

void tsurugi_ffi_connection_option_dispose(TsurugiFfiConnectionOptionHandle connection_option);

TsurugiFfiRc tsurugi_ffi_session_connect(TsurugiFfiContextHandle context,
                                         TsurugiFfiConnectionOptionHandle connection_option,
                                         TsurugiFfiSessionHandle *session_out);

TsurugiFfiRc tsurugi_ffi_session_connect_for(TsurugiFfiContextHandle context,
                                             TsurugiFfiConnectionOptionHandle connection_option,
                                             TsurugiFfiDuration timeout,
                                             TsurugiFfiSessionHandle *session_out);

TsurugiFfiRc tsurugi_ffi_session_connect_async(TsurugiFfiContextHandle context,
                                               TsurugiFfiConnectionOptionHandle connection_option,
                                               TsurugiFfiJobHandle *session_job_out);

TsurugiFfiRc tsurugi_ffi_session_set_default_timeout(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSessionHandle session,
                                                     TsurugiFfiDuration default_timeout);

TsurugiFfiRc tsurugi_ffi_session_get_default_timeout(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSessionHandle session,
                                                     TsurugiFfiDuration *default_timeout_out);

TsurugiFfiRc tsurugi_ffi_session_make_sql_client(TsurugiFfiContextHandle context,
                                                 TsurugiFfiSessionHandle session,
                                                 TsurugiFfiSqlClientHandle *sql_client_out);

TsurugiFfiRc tsurugi_ffi_session_update_expiration_time(TsurugiFfiContextHandle context,
                                                        TsurugiFfiSessionHandle session,
                                                        bool expiration_time_exists,
                                                        TsurugiFfiDuration expiration_time);

TsurugiFfiRc tsurugi_ffi_session_update_expiration_time_for(TsurugiFfiContextHandle context,
                                                            TsurugiFfiSessionHandle session,
                                                            bool expiration_time_exists,
                                                            TsurugiFfiDuration expiration_time,
                                                            TsurugiFfiDuration timeout);

TsurugiFfiRc tsurugi_ffi_session_update_expiration_time_async(TsurugiFfiContextHandle context,
                                                              TsurugiFfiSessionHandle session,
                                                              bool expiration_time_exists,
                                                              TsurugiFfiDuration expiration_time,
                                                              TsurugiFfiJobHandle *update_expiration_time_job_out);

TsurugiFfiRc tsurugi_ffi_session_shutdown(TsurugiFfiContextHandle context,
                                          TsurugiFfiSessionHandle session,
                                          TsurugiFfiShutdownType shutdown_type);

TsurugiFfiRc tsurugi_ffi_session_shutdown_for(TsurugiFfiContextHandle context,
                                              TsurugiFfiSessionHandle session,
                                              TsurugiFfiShutdownType shutdown_type,
                                              TsurugiFfiDuration timeout);

TsurugiFfiRc tsurugi_ffi_session_shutdown_async(TsurugiFfiContextHandle context,
                                                TsurugiFfiSessionHandle session,
                                                TsurugiFfiShutdownType shutdown_type,
                                                TsurugiFfiJobHandle *shutdown_job_out);

TsurugiFfiRc tsurugi_ffi_session_is_shutdowned(TsurugiFfiContextHandle context,
                                               TsurugiFfiSessionHandle session,
                                               bool *is_shutdowned_out);

TsurugiFfiRc tsurugi_ffi_session_close(TsurugiFfiContextHandle context,
                                       TsurugiFfiSessionHandle session);

TsurugiFfiRc tsurugi_ffi_session_is_closed(TsurugiFfiContextHandle context,
                                           TsurugiFfiSessionHandle session,
                                           bool *is_closed_out);

void tsurugi_ffi_session_dispose(TsurugiFfiSessionHandle session);

TsurugiFfiRc tsurugi_ffi_commit_option_create(TsurugiFfiContextHandle context,
                                              TsurugiFfiCommitOptionHandle *commit_option_out);

TsurugiFfiRc tsurugi_ffi_commit_option_set_commit_type(TsurugiFfiContextHandle context,
                                                       TsurugiFfiCommitOptionHandle commit_option,
                                                       TsurugiFfiCommitType commit_type);

TsurugiFfiRc tsurugi_ffi_commit_option_get_commit_type(TsurugiFfiContextHandle context,
                                                       TsurugiFfiCommitOptionHandle commit_option,
                                                       TsurugiFfiCommitType *commit_type_out);

TsurugiFfiRc tsurugi_ffi_commit_option_set_auto_dispose(TsurugiFfiContextHandle context,
                                                        TsurugiFfiCommitOptionHandle commit_option,
                                                        bool auto_dispose);

TsurugiFfiRc tsurugi_ffi_commit_option_get_auto_dispose(TsurugiFfiContextHandle context,
                                                        TsurugiFfiCommitOptionHandle commit_option,
                                                        bool *auto_dispose_out);

void tsurugi_ffi_commit_option_dispose(TsurugiFfiCommitOptionHandle commit_option);

TsurugiFfiRc tsurugi_ffi_transaction_option_create(TsurugiFfiContextHandle context,
                                                   TsurugiFfiTransactionOptionHandle *transaction_option_out);

TsurugiFfiRc tsurugi_ffi_transaction_option_set_transaction_type(TsurugiFfiContextHandle context,
                                                                 TsurugiFfiTransactionOptionHandle transaction_option,
                                                                 TsurugiFfiTransactionType transaction_type);

TsurugiFfiRc tsurugi_ffi_transaction_option_get_transaction_type(TsurugiFfiContextHandle context,
                                                                 TsurugiFfiTransactionOptionHandle transaction_option,
                                                                 TsurugiFfiTransactionType *transaction_type_out);

TsurugiFfiRc tsurugi_ffi_transaction_option_set_transaction_label(TsurugiFfiContextHandle context,
                                                                  TsurugiFfiTransactionOptionHandle transaction_option,
                                                                  TsurugiFfiStringHandle transaction_label);

TsurugiFfiRc tsurugi_ffi_transaction_option_get_transaction_label(TsurugiFfiContextHandle context,
                                                                  TsurugiFfiTransactionOptionHandle transaction_option,
                                                                  TsurugiFfiStringHandle *transaction_label_out);

TsurugiFfiRc tsurugi_ffi_transaction_option_set_modifies_definitions(TsurugiFfiContextHandle context,
                                                                     TsurugiFfiTransactionOptionHandle transaction_option,
                                                                     bool modifies_definitions);

TsurugiFfiRc tsurugi_ffi_transaction_option_get_modifies_definitions(TsurugiFfiContextHandle context,
                                                                     TsurugiFfiTransactionOptionHandle transaction_option,
                                                                     bool *modifies_definitions_out);

TsurugiFfiRc tsurugi_ffi_transaction_option_set_write_preserve(TsurugiFfiContextHandle context,
                                                               TsurugiFfiTransactionOptionHandle transaction_option,
                                                               const TsurugiFfiStringHandle *table_names,
                                                               uint32_t table_names_size);

TsurugiFfiRc tsurugi_ffi_transaction_option_get_write_preserve(TsurugiFfiContextHandle context,
                                                               TsurugiFfiTransactionOptionHandle transaction_option,
                                                               TsurugiFfiStringArrayHandle *table_names_out,
                                                               uint32_t *table_names_size_out);

TsurugiFfiRc tsurugi_ffi_transaction_option_set_inclusive_read_area(TsurugiFfiContextHandle context,
                                                                    TsurugiFfiTransactionOptionHandle transaction_option,
                                                                    const TsurugiFfiStringHandle *table_names,
                                                                    uint32_t table_names_size);

TsurugiFfiRc tsurugi_ffi_transaction_option_get_inclusive_read_area(TsurugiFfiContextHandle context,
                                                                    TsurugiFfiTransactionOptionHandle transaction_option,
                                                                    TsurugiFfiStringArrayHandle *table_names_out,
                                                                    uint32_t *table_names_size_out);

TsurugiFfiRc tsurugi_ffi_transaction_option_set_exclusive_read_area(TsurugiFfiContextHandle context,
                                                                    TsurugiFfiTransactionOptionHandle transaction_option,
                                                                    const TsurugiFfiStringHandle *table_names,
                                                                    uint32_t table_names_size);

TsurugiFfiRc tsurugi_ffi_transaction_option_get_exclusive_read_area(TsurugiFfiContextHandle context,
                                                                    TsurugiFfiTransactionOptionHandle transaction_option,
                                                                    TsurugiFfiStringArrayHandle *table_names_out,
                                                                    uint32_t *table_names_size_out);

TsurugiFfiRc tsurugi_ffi_transaction_option_set_priority(TsurugiFfiContextHandle context,
                                                         TsurugiFfiTransactionOptionHandle transaction_option,
                                                         TsurugiFfiTransactionPriority priority);

TsurugiFfiRc tsurugi_ffi_transaction_option_get_priority(TsurugiFfiContextHandle context,
                                                         TsurugiFfiTransactionOptionHandle transaction_option,
                                                         TsurugiFfiTransactionPriority *priority_out);

TsurugiFfiRc tsurugi_ffi_transaction_option_set_close_timeout(TsurugiFfiContextHandle context,
                                                              TsurugiFfiTransactionOptionHandle transaction_option,
                                                              TsurugiFfiDuration close_timeout);

TsurugiFfiRc tsurugi_ffi_transaction_option_get_close_timeout(TsurugiFfiContextHandle context,
                                                              TsurugiFfiTransactionOptionHandle transaction_option,
                                                              bool *close_timeout_exists_out,
                                                              TsurugiFfiDuration *close_timeout_out);

void tsurugi_ffi_transaction_option_dispose(TsurugiFfiTransactionOptionHandle transaction_option);

TsurugiFfiRc tsurugi_ffi_transaction_status_is_normal(TsurugiFfiContextHandle context,
                                                      TsurugiFfiTransactionStatusHandle transaction_status,
                                                      bool *is_normal_out);

TsurugiFfiRc tsurugi_ffi_transaction_status_is_error(TsurugiFfiContextHandle context,
                                                     TsurugiFfiTransactionStatusHandle transaction_status,
                                                     bool *is_error_out);

TsurugiFfiRc tsurugi_ffi_transaction_status_get_server_error_name(TsurugiFfiContextHandle context,
                                                                  TsurugiFfiTransactionStatusHandle transaction_status,
                                                                  TsurugiFfiStringHandle *error_name_out);

TsurugiFfiRc tsurugi_ffi_transaction_status_get_server_error_message(TsurugiFfiContextHandle context,
                                                                     TsurugiFfiTransactionStatusHandle transaction_status,
                                                                     TsurugiFfiStringHandle *error_message_out);

TsurugiFfiRc tsurugi_ffi_transaction_status_get_server_error_category_number(TsurugiFfiContextHandle context,
                                                                             TsurugiFfiTransactionStatusHandle transaction_status,
                                                                             int32_t *category_number_out);

TsurugiFfiRc tsurugi_ffi_transaction_status_get_server_error_category_str(TsurugiFfiContextHandle context,
                                                                          TsurugiFfiTransactionStatusHandle transaction_status,
                                                                          TsurugiFfiStringHandle *category_str_out);

TsurugiFfiRc tsurugi_ffi_transaction_status_get_server_error_code_number(TsurugiFfiContextHandle context,
                                                                         TsurugiFfiTransactionStatusHandle transaction_status,
                                                                         int32_t *code_number_out);

TsurugiFfiRc tsurugi_ffi_transaction_status_get_server_error_structured_code(TsurugiFfiContextHandle context,
                                                                             TsurugiFfiTransactionStatusHandle transaction_status,
                                                                             TsurugiFfiStringHandle *structured_code_out);

void tsurugi_ffi_transaction_status_dispose(TsurugiFfiTransactionStatusHandle transaction_status);

TsurugiFfiRc tsurugi_ffi_transaction_get_transaction_id(TsurugiFfiContextHandle context,
                                                        TsurugiFfiTransactionHandle transaction,
                                                        TsurugiFfiStringHandle *transaction_id_out);

TsurugiFfiRc tsurugi_ffi_transaction_set_close_timeout(TsurugiFfiContextHandle context,
                                                       TsurugiFfiTransactionHandle transaction,
                                                       TsurugiFfiDuration closetimeout);

TsurugiFfiRc tsurugi_ffi_transaction_get_close_timeout(TsurugiFfiContextHandle context,
                                                       TsurugiFfiTransactionHandle transaction,
                                                       TsurugiFfiDuration *close_timeout_out);

TsurugiFfiRc tsurugi_ffi_transaction_close(TsurugiFfiContextHandle context,
                                           TsurugiFfiTransactionHandle transaction);

TsurugiFfiRc tsurugi_ffi_transaction_close_for(TsurugiFfiContextHandle context,
                                               TsurugiFfiTransactionHandle transaction,
                                               TsurugiFfiDuration timeout);

TsurugiFfiRc tsurugi_ffi_transaction_is_closed(TsurugiFfiContextHandle context,
                                               TsurugiFfiTransactionHandle transaction,
                                               bool *is_closed_out);

void tsurugi_ffi_transaction_dispose(TsurugiFfiTransactionHandle transaction);
