package cryptopals

import (
	"encoding/base64"
	"encoding/hex"
	"log"
)

func HexToBase64(hs string) (string, error) {
	v, err := hex.DecodeString(hs)
	if err != nil {
		log.Fatal(err)
	}
	//fmt.Printf("%s\n", v)

	return base64.StdEncoding.EncodeToString(v), nil

}

func Xor(a, b []byte) []byte {
	if len(a) != len(b) {
		panic("Xor lengths are not equal")
	}

	res := make([]byte, len(a))

	for ind, val := range a {
		res[ind] = val ^ b[ind]
	}

	return res
}
