package precompiles

/*
#cgo LDFLAGS: -L../target/release/ -lprecompiles -lm

#include <stdbool.h>
#include <stdint.h>

uint64_t __precompile_anemoi_gas(const void* data_ptr, const uint32_t data_len);
uint8_t __precompile_anemoi(const void* data_ptr, const uint32_t data_len, void* ret_val);

uint64_t __precompile_ed_on_bn254_point_add_gas(const void* data_ptr, const uint32_t data_len);
uint8_t __precompile_ed_on_bn254_point_add(const void* data_ptr, const uint32_t data_len);
uint64_t __precompile_ed_on_bn254_scalar_mul_gas(const void* data_ptr, const uint32_t data_len);
uint8_t __precompile_ed_on_bn254_scalar_mul(const void* data_ptr, const uint32_t data_len);

*/
import "C"
import (
	"unsafe"
)

type Anemoi struct{}

func (a *Anemoi) RequiredGas(input []byte) uint64 {
	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	gas := C.__precompile_anemoi_gas(cstr, len)

	return uint64(gas)
}

func (a *Anemoi) Run(input []byte) ([]byte, error) {
	output := make([]byte, 64)
	cout := unsafe.Pointer(&output[0])

	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	res := C.__precompile_anemoi(cstr, len, cout)

	output[63] = byte(res)

	return output, nil
}

type EdOnBN254PointAdd struct{}

func (a *EdOnBN254PointAdd) RequiredGas(input []byte) uint64 {
	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	gas := C.__precompile_ed_on_bn254_point_add_gas(cstr, len)

	return uint64(gas)
}

func (a *EdOnBN254PointAdd) Run(input []byte) ([]byte, error) {
	output := make([]byte, 64)
	cout := unsafe.Pointer(&output[0])

	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	res := C.__precompile_ed_on_bn254_point_add(cstr, len, cout)

	output[63] = byte(res)

	return output, nil
}

type EdOnBN254ScalarMul struct{}

func (a *EdOnBN254ScalarMul) RequiredGas(input []byte) uint64 {
	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	gas := C.__precompile_ed_on_bn254_scalar_mul_gas(cstr, len)

	return uint64(gas)
}

func (a *EdOnBN254ScalarMul) Run(input []byte) ([]byte, error) {
	output := make([]byte, 64)
	cout := unsafe.Pointer(&output[0])

	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	res := C.__precompile_ed_on_bn254_scalar_mul(cstr, len, cout)

	output[63] = byte(res)

	return output, nil
}
