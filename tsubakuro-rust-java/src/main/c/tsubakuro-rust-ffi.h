#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define TSURUGI_FFI_RC_OK 0

#define TSURUGI_FFI_RC_NG_FFI_HEADER 3221225472

#define TSURUGI_FFI_RC_NG_FFI_ARG0 (TSURUGI_FFI_RC_NG_FFI_HEADER | 0)

typedef struct TsurugiFfiContext TsurugiFfiContext;

typedef uint32_t TsurugiFfiRc;

typedef struct TsurugiFfiContext *TsurugiFfiContextHandle;

TsurugiFfiRc tsurugi_ffi_context_create(TsurugiFfiContextHandle *context_out);

void tsurugi_ffi_context_dispose(TsurugiFfiContextHandle context);

TsurugiFfiRc tsurugi_ffi_env_logger_init(void);
