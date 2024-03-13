package precompiles

import (
	"fmt"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
)

func TestAnemoi(t *testing.T) {
	a := &Anemoi{}
	t.Run("eval_jive4", func(t *testing.T) {
		inputData := "73808263b6b714840000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003644ea1f2fc768cf2749997d154f34d16beb50249a19fab3386705997c594629"
		inputBytes := common.Hex2Bytes(inputData)
		res, err := a.Run(inputBytes)
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
		res, err := a.Run(inputBytes)
		assert.Nil(t, err)
		fmt.Println(common.Bytes2Hex(res))

		/// 8d1a8129419fa9b4353c88ed8d5b17c910fd1fc0b80bc9f74f67792eb9234c2c
		/// 8d1a8129419fa9b4353c88ed8d5b17c910fd1fc0b80bc9f74f67792eb9234c2c
	})

	t.Run("eval_jive4 2", func(t *testing.T) {
		inputData := "738082630000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003644ea1f2fc768cf2749997d154f34d16beb50249a19fab3386705997c594629"
		inputBytes := common.Hex2Bytes(inputData)
		res, err := a.Run(inputBytes)
		assert.Nil(t, err)
		fmt.Println(common.Bytes2Hex(res))

	})
	t.Run("anemoi_variable_length_hash", func(t *testing.T) {
		inputData := "47f3b0980000000000000000000000000000000000000000000000000000000137be295817412d25ba70f29bc3496dfdf0ff1a44754c307ff5cb5f749f6bc6b6a0c39f09"
		inputBytes := common.Hex2Bytes(inputData)
		res, err := a.Run(inputBytes)
		assert.Nil(t, err)
		fmt.Println(common.Bytes2Hex(res))
	})
	// 0x47f3b09800000000000000000000000000000000000000000000000000000062a32b155172b9cd2de6972234f8f8311127dda07164617f4a1e442478ddc2c0472450e514
	t.Run("anemoi_variable_length_hash 2", func(t *testing.T) {
		inputData := "47f3b09800000000000000000000000000000000000000000000000000000062a32b155172b9cd2de6972234f8f8311127dda07164617f4a1e442478ddc2c0472450e514"
		inputBytes := common.Hex2Bytes(inputData)
		res, err := a.Run(inputBytes)
		assert.Nil(t, err)
		fmt.Println(common.Bytes2Hex(res))
	})
}
