#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define TSURUGI_FFI_RC_NG_FFI_HEADER 3221225472

typedef struct TsurugiFfiContext TsurugiFfiContext;

typedef struct TsurugiFfiEndpoint TsurugiFfiEndpoint;

typedef uint32_t TsurugiFfiRc;

typedef struct TsurugiFfiContext *TsurugiFfiContextHandle;

typedef struct TsurugiFfiEndpoint *TsurugiFfiEndpointHandle;

#define TSURUGI_FFI_RC_OK 0

#define TSURUGI_FFI_RC_NG_FFI_ARG0 (TSURUGI_FFI_RC_NG_FFI_HEADER | 0)

TsurugiFfiRc tsurugi_ffi_context_create(TsurugiFfiContextHandle *context_out);

TsurugiFfiRc tsurugi_ffi_context_get_return_code(TsurugiFfiContextHandle context,
                                                 TsurugiFfiRc *rc_out);

TsurugiFfiRc tsurugi_ffi_context_get_error_message(TsurugiFfiContextHandle context,
                                                   char **error_message_out);

void tsurugi_ffi_context_dispose(TsurugiFfiContextHandle context);

TsurugiFfiRc tsurugi_ffi_env_logger_init(void);

TsurugiFfiRc tsurugi_ffi_endpoint_parse(TsurugiFfiContextHandle context,
                                        const char *endpoint,
                                        TsurugiFfiEndpointHandle *endpoint_out);

void tsurugi_ffi_endpoint_dispose(TsurugiFfiEndpointHandle endpoint);
