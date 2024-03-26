package precompiles

import (
	"context"
	"fmt"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
	//"github.com/zytron/zytron-op-percompiles/precompiles"
)

func Test2(t *testing.T) {
	addr1 := common.BytesToAddress([]byte{20})
	fmt.Printf("1. %v \n", addr1)

	addr2 := common.BytesToAddress([]byte{21})
	fmt.Printf("2. %v \n", addr2)

	addr3 := common.BytesToAddress([]byte{22})
	fmt.Printf("3. %v \n", addr3)
}

func TestAnemoi(t *testing.T) {
	a := &Anemoi{}
	t.Run("eval_jive4", func(t *testing.T) {
		inputData := "73808263b6b714840000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003644ea1f2fc768cf2749997d154f34d16beb50249a19fab3386705997c594629"
		inputBytes := common.Hex2Bytes(inputData)
		res, err := a.Run(context.Background(), nil, inputBytes, common.Address{}, nil)
		assert.Nil(t, err)
		fmt.Println(common.Bytes2Hex(res))

	})

	t.Run("temp", func(t *testing.T) {
		// first := "bc4f54ce0000000000000000000000000000000000000000000000000000000000000000"
		// firstBytes := common.Hex2Bytes(first)
		// res, err := a.Run(firstBytes)
		// assert.Nil(t, err)
		// fmt.Println(common.Bytes2Hex(res))
		// 3644ea1f2fc768cf2749997d154f34d16beb50249a19fab3386705997c594629
		inputData := "738082636a53d9a7051342adb81e637883005bc399c40c32c5d9f5e12cf0112a4066f22cc6e831f5f3d6eecddfa763199a633c9ab49b851c7112df7b59b17b87473a7217dcefbd4f25049ddf3ff781f89d638c98ea657bd7f616b04c54122b4cac03550d3644ea1f2fc768cf2749997d154f34d16beb50249a19fab3386705997c594629"
		inputBytes := common.Hex2Bytes(inputData)
		res, err := a.Run(context.Background(), nil, inputBytes, common.Address{}, nil)
		assert.Nil(t, err)
		fmt.Println(common.Bytes2Hex(res))

		/// 8d1a8129419fa9b4353c88ed8d5b17c910fd1fc0b80bc9f74f67792eb9234c2c
		/// 8d1a8129419fa9b4353c88ed8d5b17c910fd1fc0b80bc9f74f67792eb9234c2c
	})

	t.Run("eval_jive4 2", func(t *testing.T) {
		inputData := "738082630000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003644ea1f2fc768cf2749997d154f34d16beb50249a19fab3386705997c594629"
		inputBytes := common.Hex2Bytes(inputData)
		res, err := a.Run(context.Background(), nil, inputBytes, common.Address{}, nil)
		assert.Nil(t, err)
		fmt.Println(common.Bytes2Hex(res))

	})
	t.Run("anemoi_variable_length_hash", func(t *testing.T) {
		inputData := "47f3b0980000000000000000000000000000000000000000000000000000000137be295817412d25ba70f29bc3496dfdf0ff1a44754c307ff5cb5f749f6bc6b6a0c39f09"
		inputBytes := common.Hex2Bytes(inputData)
		res, err := a.Run(context.Background(), nil, inputBytes, common.Address{}, nil)
		assert.Nil(t, err)
		fmt.Println(common.Bytes2Hex(res))
	})
	// 0x47f3b09800000000000000000000000000000000000000000000000000000062a32b155172b9cd2de6972234f8f8311127dda07164617f4a1e442478ddc2c0472450e514
	t.Run("anemoi_variable_length_hash 2", func(t *testing.T) {
		inputData := "47f3b09800000000000000000000000000000000000000000000000000000062a32b155172b9cd2de6972234f8f8311127dda07164617f4a1e442478ddc2c0472450e514"
		inputBytes := common.Hex2Bytes(inputData)
		res, err := a.Run(context.Background(), nil, inputBytes, common.Address{}, nil)
		assert.Nil(t, err)
		fmt.Println(common.Bytes2Hex(res))
	})
}

func TestBn254Add(t *testing.T) {
	a := EdOnBN254PointAdd{}
	inputData := "0d52c3aa573af39845660735de0d3d9efb481a112cf00623ab22546122d4e16a0e7e20b3cb30785b64cd6972e2ddf919db64d03d6cf01456243c5ef2fb766a65242cbada3ae8d6e90056e73e4941eeccee72cb99945a194f754205b3678bd7692d7690deeaa77c9d89b0ceb3c25f7bb09c44f40b4b8cf5d6fcb512c7be8fcba9"
	inputBytes := common.Hex2Bytes(inputData)
	res, err := a.Run(context.Background(), nil, inputBytes, common.Address{}, nil)
	assert.Nil(t, err)
	fmt.Println(common.Bytes2Hex(res))
}

func TestBn245Mul(t *testing.T) {
	a := EdOnBN254ScalarMul{}
	inputData := "008d7a42a4dde1d8f8bcacddcae9bc78b1480eb547d4a490d9cfa5c268a076c71738fd301654d891e32235d03a64b7ebe0c3f37df67db0b798f2664783b1bac922a689a1c0aebf70ceee76fe7891729002e072ceb7ba94a32b1fce79f8c009d9"
	inputBytes := common.Hex2Bytes(inputData)
	res, err := a.Run(context.Background(), nil, inputBytes, common.Address{}, nil)
	assert.Nil(t, err)
	fmt.Println(common.Bytes2Hex(res))
}
