package precompiles

/*
#cgo LDFLAGS: -L../target/release -lprecompiles -lm

#include <stdbool.h>
#include <stdint.h>

uint64_t __precompile_anemoi_gas(const void* data_ptr, const uint32_t data_len);
uint8_t __precompile_anemoi(const void* data_ptr, const uint32_t data_len, void* ret_val);

uint64_t __precompile_ed_on_bn254_point_add_gas(const void* data_ptr, const uint32_t data_len);
uint8_t __precompile_ed_on_bn254_point_add(const void* data_ptr, const uint32_t data_len, const void* ret_val);
uint64_t __precompile_ed_on_bn254_scalar_mul_gas(const void* data_ptr, const uint32_t data_len);
uint8_t __precompile_ed_on_bn254_scalar_mul(const void* data_ptr, const uint32_t data_len, const void* ret_val);

uint8_t __precompile_verify_matchmaking(const void* data_ptr, const uint32_t data_len);
uint8_t __precompile_verify_shuffle(const void* data_ptr, const uint32_t data_len);
uint64_t __precompile_plonk_verify_gas(const void* data_ptr, const uint32_t data_len);
*/
import "C"
import (
	"errors"
	"unsafe"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
)

func ErrHandle(code byte) error {
	if code == 1 {
		return errors.New("serialize error")
	} else if code == 2 {
		return errors.New("deserialize error")
	} else if code == 3 {
		return errors.New("VerifyFail error")
	} else if code == 4 {
		return errors.New("input error")
	}
	return nil
}

type Anemoi struct{}

func (a *Anemoi) Run(input []byte) ([]byte, error) {
	if len(input) < 0 {
		return nil, ErrHandle(byte(4))
	}
	output := make([]byte, 32)
	cout := unsafe.Pointer(&output[0])

	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	res := C.__precompile_anemoi(cstr, len, cout)

	return output, ErrHandle(byte(res))
}

func (a *Anemoi) RequiredGas(input []byte) uint64 {
	if len(input) < 0 {
		return 0
	}
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

func (a *EdOnBN254PointAdd) Run(input []byte) ([]byte, error) {
	if len(input) < 0 {
		return nil, ErrHandle(byte(4))
	}
	output := make([]byte, 64)
	cout := unsafe.Pointer(&output[0])

	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	res := C.__precompile_ed_on_bn254_point_add(cstr, len, cout)

	return output, ErrHandle(byte(res))
}

func (a *EdOnBN254PointAdd) RequiredGas(input []byte) uint64 {
	if len(input) < 0 {
		return 0
	}
	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	gas := C.__precompile_ed_on_bn254_point_add_gas(cstr, len)

	return uint64(gas)
}

type EdOnBN254ScalarMul struct{}

func (a *EdOnBN254ScalarMul) RegistryKey() common.Address {
	return common.BytesToAddress([]byte{22})
}

func (a *EdOnBN254ScalarMul) Run(input []byte) ([]byte, error) {
	if len(input) < 0 {
		return nil, ErrHandle(byte(4))
	}

	output := make([]byte, 64)
	cout := unsafe.Pointer(&output[0])

	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	res := C.__precompile_ed_on_bn254_scalar_mul(cstr, len, cout)

	return output, ErrHandle(byte(res))
}

func (a *EdOnBN254ScalarMul) RequiredGas(input []byte) uint64 {
	if len(input) < 0 {
		return 0
	}
	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	gas := C.__precompile_ed_on_bn254_scalar_mul_gas(cstr, len)

	return uint64(gas)
}

type VerifyMatchmaking struct{}

func (m *VerifyMatchmaking) RegistryKey() common.Address {
	return common.BytesToAddress([]byte{23})
}

func (m *VerifyMatchmaking) RequiredGas(input []byte) uint64 {
	if len(input) < 0 {
		return 0
	}
	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	gas := C.__precompile_plonk_verify_gas(cstr, len)

	return uint64(gas)
}

func (m *VerifyMatchmaking) Run(input []byte) ([]byte, error) {
	if len(input) < 0 {
		return nil, ErrHandle(byte(4))
	}

	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	res := C.__precompile_verify_matchmaking(cstr, len)

	boolType, _ := abi.NewType("bool", "", nil)

	arguments := abi.Arguments{
		{
			Type: boolType,
		},
	}

	data := true
	if res != 0 {
		data = false
	}

	encodedData, _ := arguments.Pack(data)

	return encodedData, ErrHandle(byte(res))
}

type VerifyShuffle struct{}

func (s *VerifyShuffle) RegistryKey() common.Address {
	return common.BytesToAddress([]byte{24})
}

func (s *VerifyShuffle) RequiredGas(input []byte) uint64 {
	if len(input) < 0 {
		return 0
	}
	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	gas := C.__precompile_plonk_verify_gas(cstr, len)

	return uint64(gas)
}

func (s *VerifyShuffle) Run(input []byte) ([]byte, error) {
	if len(input) < 0 {
		return nil, ErrHandle(byte(4))
	}

	cstr := unsafe.Pointer(&input[0])
	len := C.uint(len(input))

	res := C.__precompile_verify_shuffle(cstr, len)

	boolType, _ := abi.NewType("bool", "", nil)

	arguments := abi.Arguments{
		{
			Type: boolType,
		},
	}

	data := true
	if res != 0 {
		data = false
	}

	encodedData, _ := arguments.Pack(data)

	return encodedData, ErrHandle(byte(res))
}
