#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>







#define TSURUGI_FFI_RC_FFI_BASE (TSURUGI_FFI_RC_TYPE_FFI_ERROR << 30)

#define TSURUGI_FFI_RC_FFI_ARG_ERROR (TSURUGI_FFI_RC_FFI_BASE | (0 << 24))

#define TSURUGI_FFI_RC_FFI_JOB_ERROR (TSURUGI_FFI_RC_FFI_BASE | (1 << 24))

#define TSURUGI_FFI_RC_FFI_JOB_ALREADY_CLOSED (TSURUGI_FFI_RC_FFI_JOB_ERROR | 1)

#define TSURUGI_FFI_RC_FFI_ERROR (TSURUGI_FFI_RC_FFI_BASE | (1 << 24))

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
   *
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

typedef struct TsurugiFfiSqlParameter TsurugiFfiSqlParameter;

typedef struct TsurugiFfiSqlPlaceholder TsurugiFfiSqlPlaceholder;

typedef struct TsurugiFfiSqlPreparedStatement TsurugiFfiSqlPreparedStatement;

typedef struct TsurugiFfiSqlQueryResult TsurugiFfiSqlQueryResult;

typedef struct TsurugiFfiSqlQueryResultMetadata TsurugiFfiSqlQueryResultMetadata;

typedef struct TsurugiFfiTableList TsurugiFfiTableList;

typedef struct TsurugiFfiTableMetadata TsurugiFfiTableMetadata;

typedef struct TsurugiFfiTransaction TsurugiFfiTransaction;

typedef struct TsurugiFfiTransactionOption TsurugiFfiTransactionOption;

typedef uint32_t TsurugiFfiRc;

typedef struct TsurugiFfiContext *TsurugiFfiContextHandle;

typedef struct TsurugiFfiCancelJob *TsurugiFfiCancelJobHandle;

/**
 * nanosecond
 */
typedef uint64_t TsurugiFfiDuration;

typedef void *TsurugiFfiJobHandle;

typedef struct TsurugiFfiSqlColumn *TsurugiFfiSqlColumnHandle;

typedef struct TsurugiFfiSqlExecuteResult *TsurugiFfiSqlExecuteResultHandle;

typedef struct TsurugiFfiSqlParameter *TsurugiFfiSqlParameterHandle;

typedef struct TsurugiFfiSqlPlaceholder *TsurugiFfiSqlPlaceholderHandle;

typedef struct TsurugiFfiSqlPreparedStatement *TsurugiFfiSqlPreparedStatementHandle;

typedef struct TsurugiFfiSqlQueryResult *TsurugiFfiSqlQueryResultHandle;

typedef struct TsurugiFfiSqlQueryResultMetadata *TsurugiFfiSqlQueryResultMetadataHandle;

typedef struct TsurugiFfiSqlClient *TsurugiFfiSqlClientHandle;

typedef struct TsurugiFfiTableList *TsurugiFfiTableListHandle;

typedef struct TsurugiFfiTableMetadata *TsurugiFfiTableMetadataHandle;

typedef struct TsurugiFfiTransactionOption *TsurugiFfiTransactionOptionHandle;

typedef struct TsurugiFfiTransaction *TsurugiFfiTransactionHandle;

typedef struct TsurugiFfiCommitOption *TsurugiFfiCommitOptionHandle;

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

#define TSURUGI_FFI_RC_FFI_NUL_ERROR (TSURUGI_FFI_RC_FFI_ERROR | 1)

#define TSURUGI_FFI_RC_CORE_CLIENT_CLIENT_ERROR (TSURUGI_FFI_RC_CORE_CLIENT_ERROR | (1 << 16))

#define TSURUGI_FFI_RC_CORE_CLIENT_TIMEOUT_ERROR (TSURUGI_FFI_RC_CORE_CLIENT_ERROR | (2 << 16))

#define TSURUGI_FFI_RC_CORE_CLIENT_IO_ERROR (TSURUGI_FFI_RC_CORE_CLIENT_ERROR | (3 << 16))

TsurugiFfiRc tsurugi_ffi_context_create(TsurugiFfiContextHandle *context_out);

TsurugiFfiRc tsurugi_ffi_context_get_return_code(TsurugiFfiContextHandle context,
                                                 TsurugiFfiRc *rc_out);

TsurugiFfiRc tsurugi_ffi_context_get_error_type(TsurugiFfiContextHandle context,
                                                TsurugiFfiRcType *error_type_out);

TsurugiFfiRc tsurugi_ffi_context_get_error_message(TsurugiFfiContextHandle context,
                                                   char **error_message_out);

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
                                      char **name_out);

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
                                           void **value_out);

TsurugiFfiRc tsurugi_ffi_job_cancel(TsurugiFfiContextHandle context,
                                    TsurugiFfiJobHandle job,
                                    bool *cancell_done_out);

TsurugiFfiRc tsurugi_ffi_job_cancel_async(TsurugiFfiContextHandle context,
                                          TsurugiFfiJobHandle job,
                                          TsurugiFfiCancelJobHandle *cancel_job_out);

TsurugiFfiRc tsurugi_ffi_job_close(TsurugiFfiContextHandle context, TsurugiFfiJobHandle job);

void tsurugi_ffi_job_dispose(TsurugiFfiJobHandle job);

TsurugiFfiRc tsurugi_ffi_env_logger_init(void);

TsurugiFfiRc tsurugi_ffi_sql_column_get_name(TsurugiFfiContextHandle context,
                                             TsurugiFfiSqlColumnHandle sql_column,
                                             char **name_out);

TsurugiFfiRc tsurugi_ffi_sql_column_get_atom_type(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlColumnHandle sql_column,
                                                  TsurugiFfiAtomType *atom_type_out);

void tsurugi_ffi_sql_column_dispose(TsurugiFfiSqlColumnHandle sql_column);

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

TsurugiFfiRc tsurugi_ffi_sql_parameter_null(TsurugiFfiContextHandle context,
                                            const char *name,
                                            TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_int4(TsurugiFfiContextHandle context,
                                               const char *name,
                                               int32_t value,
                                               TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_int8(TsurugiFfiContextHandle context,
                                               const char *name,
                                               int64_t value,
                                               TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_float4(TsurugiFfiContextHandle context,
                                                 const char *name,
                                                 float value,
                                                 TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_float8(TsurugiFfiContextHandle context,
                                                 const char *name,
                                                 double value,
                                                 TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_of_character(TsurugiFfiContextHandle context,
                                                    const char *name,
                                                    const char *value,
                                                    TsurugiFfiSqlParameterHandle *parameter_out);

TsurugiFfiRc tsurugi_ffi_sql_parameter_get_name(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlParameterHandle parameter,
                                                char **name_out);

void tsurugi_ffi_sql_parameter_dispose(TsurugiFfiSqlParameterHandle parameter);

TsurugiFfiRc tsurugi_ffi_sql_placeholder_of_atom_type(TsurugiFfiContextHandle context,
                                                      const char *name,
                                                      TsurugiFfiAtomType atom_type,
                                                      TsurugiFfiSqlPlaceholderHandle *placeholder_out);

TsurugiFfiRc tsurugi_ffi_sql_placeholder_get_name(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlPlaceholderHandle placeholder,
                                                  char **name_out);

TsurugiFfiRc tsurugi_ffi_sql_placeholder_get_atom_type(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlPlaceholderHandle placeholder,
                                                       TsurugiFfiAtomType *atom_type_out);

void tsurugi_ffi_sql_placeholder_dispose(TsurugiFfiSqlPlaceholderHandle placeholder);

TsurugiFfiRc tsurugi_ffi_sql_prepared_statement_close(TsurugiFfiContextHandle context,
                                                      TsurugiFfiSqlPreparedStatementHandle prepared_statement);

void tsurugi_ffi_sql_prepared_statement_dispose(TsurugiFfiSqlPreparedStatementHandle prepared_statement);

TsurugiFfiRc tsurugi_ffi_sql_query_result_get_metadata(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlQueryResultHandle query_result,
                                                       TsurugiFfiSqlQueryResultMetadataHandle *metadata_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_next_row(TsurugiFfiContextHandle context,
                                                   TsurugiFfiSqlQueryResultHandle query_result,
                                                   bool *has_row_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_next_column(TsurugiFfiContextHandle context,
                                                      TsurugiFfiSqlQueryResultHandle query_result,
                                                      bool *has_column_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_is_null(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlQueryResultHandle query_result,
                                                  bool *is_null_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_int4(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlQueryResultHandle query_result,
                                                     int32_t *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_int8(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlQueryResultHandle query_result,
                                                     int64_t *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_float4(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlQueryResultHandle query_result,
                                                       float *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_float8(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlQueryResultHandle query_result,
                                                       double *value_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_fetch_character(TsurugiFfiContextHandle context,
                                                          TsurugiFfiSqlQueryResultHandle query_result,
                                                          char **value_out);

void tsurugi_ffi_sql_query_result_dispose(TsurugiFfiSqlQueryResultHandle query_result);

TsurugiFfiRc tsurugi_ffi_sql_query_result_metadata_get_columns_size(TsurugiFfiContextHandle context,
                                                                    TsurugiFfiSqlQueryResultMetadataHandle query_result_metadata,
                                                                    uint32_t *size_out);

TsurugiFfiRc tsurugi_ffi_sql_query_result_metadata_get_columns_value(TsurugiFfiContextHandle context,
                                                                     TsurugiFfiSqlQueryResultMetadataHandle query_result_metadata,
                                                                     uint32_t index,
                                                                     TsurugiFfiSqlColumnHandle *sql_column_out);

void tsurugi_ffi_sql_query_result_metadata_dispose(TsurugiFfiSqlQueryResultMetadataHandle query_result_metadata);

TsurugiFfiRc tsurugi_ffi_sql_client_list_tables(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlClientHandle sql_client,
                                                TsurugiFfiTableListHandle *table_list_out);

TsurugiFfiRc tsurugi_ffi_sql_client_list_tables_async(TsurugiFfiContextHandle context,
                                                      TsurugiFfiSqlClientHandle sql_client,
                                                      TsurugiFfiJobHandle *table_list_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_get_table_metadata(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlClientHandle sql_client,
                                                       const char *table_name,
                                                       TsurugiFfiTableMetadataHandle *table_metadata_out);

TsurugiFfiRc tsurugi_ffi_sql_client_get_table_metadata_async(TsurugiFfiContextHandle context,
                                                             TsurugiFfiSqlClientHandle sql_client,
                                                             const char *table_name,
                                                             TsurugiFfiJobHandle *table_metadata_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepare(TsurugiFfiContextHandle context,
                                            TsurugiFfiSqlClientHandle sql_client,
                                            const char *sql,
                                            const TsurugiFfiSqlPlaceholderHandle *placeholders,
                                            uint32_t placeholder_size,
                                            TsurugiFfiSqlPreparedStatementHandle *prepared_statement_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepare_async(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlClientHandle sql_client,
                                                  const char *sql,
                                                  const TsurugiFfiSqlPlaceholderHandle *placeholders,
                                                  uint32_t placeholder_size,
                                                  TsurugiFfiJobHandle *prepared_statement_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_start_transaction(TsurugiFfiContextHandle context,
                                                      TsurugiFfiSqlClientHandle sql_client,
                                                      TsurugiFfiTransactionOptionHandle transaction_option,
                                                      TsurugiFfiTransactionHandle *transaction_out);

TsurugiFfiRc tsurugi_ffi_sql_client_start_transaction_async(TsurugiFfiContextHandle context,
                                                            TsurugiFfiSqlClientHandle sql_client,
                                                            TsurugiFfiTransactionOptionHandle transaction_option,
                                                            TsurugiFfiJobHandle *transaction_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_execute(TsurugiFfiContextHandle context,
                                            TsurugiFfiSqlClientHandle sql_client,
                                            TsurugiFfiTransactionHandle transaction,
                                            const char *sql,
                                            TsurugiFfiSqlExecuteResultHandle *execute_result_out);

TsurugiFfiRc tsurugi_ffi_sql_client_execute_async(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlClientHandle sql_client,
                                                  TsurugiFfiTransactionHandle transaction,
                                                  const char *sql,
                                                  TsurugiFfiJobHandle *execute_result_job_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepared_execute(TsurugiFfiContextHandle context,
                                                     TsurugiFfiSqlClientHandle sql_client,
                                                     TsurugiFfiTransactionHandle transaction,
                                                     TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                     const TsurugiFfiSqlParameterHandle *parameters,
                                                     uint32_t parameter_size,
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
                                          const char *sql,
                                          TsurugiFfiSqlQueryResultHandle *query_result_out);

TsurugiFfiRc tsurugi_ffi_sql_client_prepared_query(TsurugiFfiContextHandle context,
                                                   TsurugiFfiSqlClientHandle sql_client,
                                                   TsurugiFfiTransactionHandle transaction,
                                                   TsurugiFfiSqlPreparedStatementHandle prepared_statement,
                                                   const TsurugiFfiSqlParameterHandle *parameters,
                                                   uint32_t parameter_size,
                                                   TsurugiFfiSqlQueryResultHandle *query_result_out);

TsurugiFfiRc tsurugi_ffi_sql_client_commit(TsurugiFfiContextHandle context,
                                           TsurugiFfiSqlClientHandle sql_client,
                                           TsurugiFfiTransactionHandle transaction,
                                           TsurugiFfiCommitOptionHandle commit_option);

TsurugiFfiRc tsurugi_ffi_sql_client_rollback(TsurugiFfiContextHandle context,
                                             TsurugiFfiSqlClientHandle sql_client,
                                             TsurugiFfiTransactionHandle transaction);

void tsurugi_ffi_sql_client_dispose(TsurugiFfiSqlClientHandle sql_client);

TsurugiFfiRc tsurugi_ffi_table_list_get_table_names_size(TsurugiFfiContextHandle context,
                                                         TsurugiFfiTableListHandle table_list,
                                                         uint32_t *size_out);

TsurugiFfiRc tsurugi_ffi_table_list_get_table_names_value(TsurugiFfiContextHandle context,
                                                          TsurugiFfiTableListHandle table_list,
                                                          uint32_t index,
                                                          char **table_name_out);

void tsurugi_ffi_table_list_dispose(TsurugiFfiTableListHandle table_list);

TsurugiFfiRc tsurugi_ffi_table_metadata_get_table_name(TsurugiFfiContextHandle context,
                                                       TsurugiFfiTableMetadataHandle table_metadata,
                                                       char **table_name_out);

TsurugiFfiRc tsurugi_ffi_table_metadata_get_columns_size(TsurugiFfiContextHandle context,
                                                         TsurugiFfiTableMetadataHandle table_metadata,
                                                         uint32_t *size_out);

TsurugiFfiRc tsurugi_ffi_table_metadata_get_columns_value(TsurugiFfiContextHandle context,
                                                          TsurugiFfiTableMetadataHandle table_metadata,
                                                          uint32_t index,
                                                          TsurugiFfiSqlColumnHandle *sql_column_out);

void tsurugi_ffi_table_metadata_dispose(TsurugiFfiTableMetadataHandle table_metadata);

TsurugiFfiRc tsurugi_ffi_endpoint_parse(TsurugiFfiContextHandle context,
                                        const char *endpoint,
                                        TsurugiFfiEndpointHandle *endpoint_out);

void tsurugi_ffi_endpoint_dispose(TsurugiFfiEndpointHandle endpoint);

TsurugiFfiRc tsurugi_ffi_connection_option_create(TsurugiFfiContextHandle context,
                                                  TsurugiFfiConnectionOptionHandle *connection_option_out);

TsurugiFfiRc tsurugi_ffi_connection_option_set_endpoint(TsurugiFfiContextHandle context,
                                                        TsurugiFfiConnectionOptionHandle connection_option,
                                                        TsurugiFfiEndpointHandle endpoint);

TsurugiFfiRc tsurugi_ffi_connection_option_set_endpoint_url(TsurugiFfiContextHandle context,
                                                            TsurugiFfiConnectionOptionHandle connection_option,
                                                            const char *endpoint);

TsurugiFfiRc tsurugi_ffi_connection_option_get_endpoint(TsurugiFfiContextHandle context,
                                                        TsurugiFfiConnectionOptionHandle connection_option,
                                                        char **endpoint_out);

TsurugiFfiRc tsurugi_ffi_connection_option_set_application_name(TsurugiFfiContextHandle context,
                                                                TsurugiFfiConnectionOptionHandle connection_option,
                                                                const char *application_name);

TsurugiFfiRc tsurugi_ffi_connection_option_get_application_name(TsurugiFfiContextHandle context,
                                                                TsurugiFfiConnectionOptionHandle connection_option,
                                                                char **application_name_out);

TsurugiFfiRc tsurugi_ffi_connection_option_set_session_label(TsurugiFfiContextHandle context,
                                                             TsurugiFfiConnectionOptionHandle connection_option,
                                                             const char *label);

TsurugiFfiRc tsurugi_ffi_connection_option_get_session_label(TsurugiFfiContextHandle context,
                                                             TsurugiFfiConnectionOptionHandle connection_option,
                                                             char **label_out);

void tsurugi_ffi_connection_option_dispose(TsurugiFfiConnectionOptionHandle connection_option);

TsurugiFfiRc tsurugi_ffi_session_connect(TsurugiFfiContextHandle context,
                                         TsurugiFfiConnectionOptionHandle connection_option,
                                         TsurugiFfiSessionHandle *session_out);

TsurugiFfiRc tsurugi_ffi_session_connect_async(TsurugiFfiContextHandle context,
                                               TsurugiFfiConnectionOptionHandle connection_option,
                                               TsurugiFfiJobHandle *session_job_out);

TsurugiFfiRc tsurugi_ffi_session_make_sql_client(TsurugiFfiContextHandle context,
                                                 TsurugiFfiSessionHandle session,
                                                 TsurugiFfiSqlClientHandle *sql_client_out);

void tsurugi_ffi_session_dispose(TsurugiFfiSessionHandle session);

TsurugiFfiRc tsurugi_ffi_commit_option_create(TsurugiFfiContextHandle context,
                                              TsurugiFfiCommitOptionHandle *commit_option_out);

TsurugiFfiRc tsurugi_ffi_commit_option_set_commit_type(TsurugiFfiContextHandle context,
                                                       TsurugiFfiCommitOptionHandle commit_option,
                                                       TsurugiFfiCommitType commit_type);

TsurugiFfiRc tsurugi_ffi_commit_option_get_commit_type(TsurugiFfiContextHandle context,
                                                       TsurugiFfiCommitOptionHandle commit_option,
                                                       TsurugiFfiCommitType *commit_type_out);

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
                                                                  const char *label);

TsurugiFfiRc tsurugi_ffi_transaction_option_get_transaction_label(TsurugiFfiContextHandle context,
                                                                  TsurugiFfiTransactionOptionHandle transaction_option,
                                                                  char **label_out);

void tsurugi_ffi_transaction_option_dispose(TsurugiFfiTransactionOptionHandle transaction_option);

TsurugiFfiRc tsurugi_ffi_transaction_get_transaction_id(TsurugiFfiContextHandle context,
                                                        TsurugiFfiTransactionHandle transaction,
                                                        char **transaction_id_out);

TsurugiFfiRc tsurugi_ffi_transaction_close(TsurugiFfiContextHandle context,
                                           TsurugiFfiTransactionHandle transaction);

void tsurugi_ffi_transaction_dispose(TsurugiFfiTransactionHandle transaction);
