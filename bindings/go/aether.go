// Package aether provides Go bindings for the Aether DSL language.
//
// Aether is a lightweight, embeddable domain-specific language designed for
// configuration management, business rules, and scripting.
//
// Basic Usage:
//
//	engine := aether.New()
//	defer engine.Close()
//
//	result, err := engine.Eval(`
//	    Set X 10
//	    Set Y 20
//	    (X + Y)
//	`)
//	if err != nil {
//	    log.Fatal(err)
//	}
//	fmt.Println(result) // "30"
package aether

/*
#cgo LDFLAGS: -L${SRCDIR}/../../target/release -laether -ldl -lm -lpthread
#cgo darwin LDFLAGS: -framework Security -framework CoreFoundation
#include <stdlib.h>

typedef struct AetherHandle AetherHandle;

typedef enum AetherErrorCode {
    Success = 0,
    ParseError = 1,
    RuntimeError = 2,
    NullPointer = 3,
    Panic = 4,
} AetherErrorCode;

AetherHandle* aether_new();
AetherHandle* aether_new_with_permissions();
int aether_eval(AetherHandle* handle, const char* code, char** result, char** error);
const char* aether_version();
void aether_free(AetherHandle* handle);
void aether_free_string(char* s);
*/
import "C"
import (
	"errors"
	"fmt"
	"runtime"
	"unsafe"
)

// Aether represents an instance of the Aether language engine.
type Aether struct {
	handle *C.AetherHandle
}

// New creates a new Aether engine instance with default (restricted) IO permissions.
//
// For security, IO operations are disabled by default when using Aether as an
// embedded DSL. Use NewWithPermissions() if you need to enable IO operations.
func New() *Aether {
	a := &Aether{
		handle: C.aether_new(),
	}
	runtime.SetFinalizer(a, (*Aether).Close)
	return a
}

// NewWithPermissions creates a new Aether engine with all IO permissions enabled.
//
// Warning: Only use this when you trust the scripts being executed, as it allows
// filesystem and network operations.
func NewWithPermissions() *Aether {
	a := &Aether{
		handle: C.aether_new_with_permissions(),
	}
	runtime.SetFinalizer(a, (*Aether).Close)
	return a
}

// Eval evaluates the given Aether code and returns the result as a string.
//
// Returns an error if the code fails to parse or encounters a runtime error.
func (a *Aether) Eval(code string) (string, error) {
	if a.handle == nil {
		return "", errors.New("aether: engine closed")
	}

	cCode := C.CString(code)
	defer C.free(unsafe.Pointer(cCode))

	var result *C.char
	var errorMsg *C.char

	status := C.aether_eval(a.handle, cCode, &result, &errorMsg)

	if status != C.Success {
		if errorMsg != nil {
			defer C.aether_free_string(errorMsg)
			errStr := C.GoString(errorMsg)
			return "", fmt.Errorf("aether: %s", errStr)
		}
		return "", errors.New("aether: unknown error")
	}

	if result != nil {
		defer C.aether_free_string(result)
		return C.GoString(result), nil
	}

	return "", nil
}

// Version returns the version string of the Aether engine.
func Version() string {
	return C.GoString(C.aether_version())
}

// Close frees the resources associated with the Aether engine.
//
// After calling Close(), the engine cannot be used anymore.
// It's safe to call Close() multiple times.
func (a *Aether) Close() {
	if a.handle != nil {
		C.aether_free(a.handle)
		a.handle = nil
	}
}
