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

typedef struct TsurugiFfiConnectionOption TsurugiFfiConnectionOption;

typedef struct TsurugiFfiContext TsurugiFfiContext;

typedef struct TsurugiFfiEndpoint TsurugiFfiEndpoint;

typedef struct TsurugiFfiSession TsurugiFfiSession;

typedef uint32_t TsurugiFfiRc;

typedef struct TsurugiFfiContext *TsurugiFfiContextHandle;

typedef struct TsurugiFfiConnectionOption *TsurugiFfiConnectionOptionHandle;

typedef struct TsurugiFfiSession *TsurugiFfiSessionHandle;

typedef struct TsurugiFfiEndpoint *TsurugiFfiEndpointHandle;

#define TSURUGI_FFI_RC_OK 0

#define TSURUGI_FFI_RC_FFI_ARG0_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 0)

#define TSURUGI_FFI_RC_FFI_ARG1_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 1)

#define TSURUGI_FFI_RC_FFI_ARG2_ERROR (TSURUGI_FFI_RC_FFI_ARG_ERROR | 2)

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

void tsurugi_ffi_connection_option_dispose(TsurugiFfiConnectionOptionHandle connection_option);
