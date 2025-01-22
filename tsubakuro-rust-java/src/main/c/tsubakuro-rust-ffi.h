#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define TSURUGI_FFI_RC_OK 0

typedef uint32_t TsurugiFfiRc;

TsurugiFfiRc tsurugi_ffi_env_logger_init(void);
