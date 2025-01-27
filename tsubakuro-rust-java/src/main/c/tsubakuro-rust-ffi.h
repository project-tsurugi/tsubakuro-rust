#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define TSURUGI_FFI_RC_TYPE_OK 0

#define TSURUGI_FFI_RC_TYPE_FFI_ERROR 1

#define TSURUGI_FFI_RC_TYPE_CORE_CLIENT_ERROR 2

#define TSURUGI_FFI_RC_TYPE_CORE_SERVER_ERROR 3

#define TSURUGI_FFI_RC_FFI_BASE (TSURUGI_FFI_RC_TYPE_FFI_ERROR << 30)

#define TSURUGI_FFI_RC_FFI_ARG_ERROR (TSURUGI_FFI_RC_FFI_BASE | (0 << 24))

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

typedef struct TsurugiFfiConnectionOption TsurugiFfiConnectionOption;

typedef struct TsurugiFfiContext TsurugiFfiContext;

typedef struct TsurugiFfiEndpoint TsurugiFfiEndpoint;

typedef struct TsurugiFfiSession TsurugiFfiSession;

typedef struct TsurugiFfiSqlClient TsurugiFfiSqlClient;

typedef struct TsurugiFfiSqlColumn TsurugiFfiSqlColumn;

typedef struct TsurugiFfiTableList TsurugiFfiTableList;

typedef struct TsurugiFfiTableMetadata TsurugiFfiTableMetadata;

typedef struct TsurugiFfiTransactionOption TsurugiFfiTransactionOption;

typedef uint32_t TsurugiFfiRc;

typedef struct TsurugiFfiContext *TsurugiFfiContextHandle;

typedef struct TsurugiFfiSession *TsurugiFfiSessionHandle;

typedef struct TsurugiFfiSqlClient *TsurugiFfiSqlClientHandle;

typedef struct TsurugiFfiTableList *TsurugiFfiTableListHandle;

typedef struct TsurugiFfiTableMetadata *TsurugiFfiTableMetadataHandle;

typedef struct TsurugiFfiSqlColumn *TsurugiFfiSqlColumnHandle;

typedef struct TsurugiFfiConnectionOption *TsurugiFfiConnectionOptionHandle;

typedef struct TsurugiFfiEndpoint *TsurugiFfiEndpointHandle;

typedef struct TsurugiFfiTransactionOption *TsurugiFfiTransactionOptionHandle;

#define TSURUGI_FFI_RC_OK 0

#define TSURUGI_FFI_RC_FFI_ARG0_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 0)

#define TSURUGI_FFI_RC_FFI_ARG1_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 1)

#define TSURUGI_FFI_RC_FFI_ARG2_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 2)

#define TSURUGI_FFI_RC_FFI_ARG3_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 3)

#define TSURUGI_FFI_RC_FFI_NUL_ERROR (TSURUGI_FFI_RC_FFI_ERROR | 1)

#define TSURUGI_FFI_RC_CORE_CLIENT_CLIENT_ERROR (TSURUGI_FFI_RC_CORE_CLIENT_ERROR | (1 << 16))

#define TSURUGI_FFI_RC_CORE_CLIENT_TIMEOUT_ERROR (TSURUGI_FFI_RC_CORE_CLIENT_ERROR | (2 << 16))

#define TSURUGI_FFI_RC_CORE_CLIENT_IO_ERROR (TSURUGI_FFI_RC_CORE_CLIENT_ERROR | (3 << 16))

TsurugiFfiRc tsurugi_ffi_context_create(TsurugiFfiContextHandle *context_out);

TsurugiFfiRc tsurugi_ffi_context_get_return_code(TsurugiFfiContextHandle context,
                                                 TsurugiFfiRc *rc_out);

TsurugiFfiRc tsurugi_ffi_context_get_error_type(TsurugiFfiContextHandle context,
                                                TsurugiFfiRc *error_type_out);

TsurugiFfiRc tsurugi_ffi_context_get_error_message(TsurugiFfiContextHandle context,
                                                   char **error_message_out);

void tsurugi_ffi_context_dispose(TsurugiFfiContextHandle context);

TsurugiFfiRc tsurugi_ffi_env_logger_init(void);

TsurugiFfiRc tsurugi_ffi_session_make_sql_client(TsurugiFfiContextHandle context,
                                                 TsurugiFfiSessionHandle session,
                                                 TsurugiFfiSqlClientHandle *sql_client_out);

TsurugiFfiRc tsurugi_ffi_sql_client_list_tables(TsurugiFfiContextHandle context,
                                                TsurugiFfiSqlClientHandle sql_client,
                                                TsurugiFfiTableListHandle *table_list_out);

TsurugiFfiRc tsurugi_ffi_sql_client_get_table_metadata(TsurugiFfiContextHandle context,
                                                       TsurugiFfiSqlClientHandle sql_client,
                                                       const char *table_name,
                                                       TsurugiFfiTableMetadataHandle *table_metadata_out);

void tsurugi_ffi_sql_client_dispose(TsurugiFfiSqlClientHandle sql_client);

TsurugiFfiRc tsurugi_ffi_sql_column_get_name(TsurugiFfiContextHandle context,
                                             TsurugiFfiSqlColumnHandle sql_column,
                                             char **name_out);

TsurugiFfiRc tsurugi_ffi_sql_column_get_atom_type(TsurugiFfiContextHandle context,
                                                  TsurugiFfiSqlColumnHandle sql_column,
                                                  TsurugiFfiAtomType *atom_type_out);

void tsurugi_ffi_sql_column_dispose(TsurugiFfiSqlColumnHandle sql_column);

TsurugiFfiRc tsurugi_ffi_table_list_get_table_names_size(TsurugiFfiContextHandle context,
                                                         TsurugiFfiTableListHandle table_list,
                                                         uint32_t *size_out);

TsurugiFfiRc tsurugi_ffi_table_list_get_table_names_element(TsurugiFfiContextHandle context,
                                                            TsurugiFfiTableListHandle table_list,
                                                            uint32_t index,
                                                            char **element_out);

void tsurugi_ffi_table_list_dispose(TsurugiFfiTableListHandle table_list);

TsurugiFfiRc tsurugi_ffi_table_metadata_get_table_name(TsurugiFfiContextHandle context,
                                                       TsurugiFfiTableMetadataHandle table_metadata,
                                                       char **table_name_out);

TsurugiFfiRc tsurugi_ffi_table_metadata_get_columns_size(TsurugiFfiContextHandle context,
                                                         TsurugiFfiTableMetadataHandle table_metadata,
                                                         uint32_t *size_out);

TsurugiFfiRc tsurugi_ffi_table_metadata_get_columns_element(TsurugiFfiContextHandle context,
                                                            TsurugiFfiTableMetadataHandle table_metadata,
                                                            uint32_t index,
                                                            TsurugiFfiSqlColumnHandle *element_out);

void tsurugi_ffi_table_metadata_dispose(TsurugiFfiTableMetadataHandle table_metadata);

TsurugiFfiRc tsurugi_ffi_session_connect(TsurugiFfiContextHandle context,
                                         TsurugiFfiConnectionOptionHandle connection_option,
                                         TsurugiFfiSessionHandle *session_out);

void tsurugi_ffi_session_dispose(TsurugiFfiSessionHandle session);

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

TsurugiFfiRc tsurugi_ffi_connection_option_set_label(TsurugiFfiContextHandle context,
                                                     TsurugiFfiConnectionOptionHandle connection_option,
                                                     const char *label);

TsurugiFfiRc tsurugi_ffi_connection_option_get_label(TsurugiFfiContextHandle context,
                                                     TsurugiFfiConnectionOptionHandle connection_option,
                                                     char **label_out);

void tsurugi_ffi_connection_option_dispose(TsurugiFfiConnectionOptionHandle connection_option);

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
