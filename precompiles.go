package precompiles

/*
#cgo LDFLAGS: -L./target/release/ -lprecompiles -lm

#include <stdbool.h>
#include <stdint.h>

uint64_t __precompile_anemoi_gas(const void* data_ptr, const uint32_t data_len);
uint8_t __precompile_anemoi(const void* data_ptr, const uint32_t data_len, void* ret_val);

uint64_t __precompile_ed_on_bn254_point_add_gas(const void* data_ptr, const uint32_t data_len);
uint8_t __precompile_ed_on_bn254_point_add(const void* data_ptr, const uint32_t data_len, const void* ret_val);
uint64_t __precompile_ed_on_bn254_scalar_mul_gas(const void* data_ptr, const uint32_t data_len);
uint8_t __precompile_ed_on_bn254_scalar_mul(const void* data_ptr, const uint32_t data_len, const void* ret_val);

*/
import "C"
import (
	"context"
	"errors"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/vm"
	"math/big"
	"unsafe"
)

func ErrHandle(code byte) error {
	if code == 1 {
		return errors.New("serialize error")
	} else if code == 2 {
		return errors.New("deserialzie error")
	}
	return nil
}

type Anemoi struct{}

func (a *Anemoi) Run(_ context.Context, _ vm.PrecompileEVM, input []byte, _ common.Address, _ *big.Int) ([]byte, error) {
	output := make([]byte, 64)
	cout := unsafe.Pointer(&output[0])

	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	res := C.__precompile_anemoi(cstr, len, cout)

	return output, ErrHandle(byte(res))
}

func (a *Anemoi) RequiredGas(input []byte) uint64 {
	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	gas := C.__precompile_anemoi_gas(cstr, len)

	return uint64(gas)
}

func (a *Anemoi) RegistryKey() common.Address {
	return common.BytesToAddress([]byte{20})
}

type EdOnBN254PointAdd struct{}

func (a *EdOnBN254PointAdd) RegistryKey() common.Address {
	return common.BytesToAddress([]byte{21})
}

func (a *EdOnBN254PointAdd) Run(_ context.Context, _ vm.PrecompileEVM, input []byte, _ common.Address, _ *big.Int) ([]byte, error) {
	output := make([]byte, 64)
	cout := unsafe.Pointer(&output[0])

	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	res := C.__precompile_ed_on_bn254_point_add(cstr, len, cout)

	return output, ErrHandle(byte(res))
}

func (a *EdOnBN254PointAdd) RequiredGas(input []byte) uint64 {
	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	gas := C.__precompile_ed_on_bn254_point_add_gas(cstr, len)

	return uint64(gas)
}

type EdOnBN254ScalarMul struct{}

func (a *EdOnBN254ScalarMul) RegistryKey() common.Address {
	return common.BytesToAddress([]byte{22})
}

func (a *EdOnBN254ScalarMul) Run(_ context.Context, _ vm.PrecompileEVM, input []byte, _ common.Address, _ *big.Int) ([]byte, error) {
	output := make([]byte, 64)
	cout := unsafe.Pointer(&output[0])

	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	res := C.__precompile_ed_on_bn254_scalar_mul(cstr, len, cout)

	return output, ErrHandle(byte(res))
}

func (a *EdOnBN254ScalarMul) RequiredGas(input []byte) uint64 {
	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	gas := C.__precompile_ed_on_bn254_scalar_mul_gas(cstr, len)

	return uint64(gas)
}
