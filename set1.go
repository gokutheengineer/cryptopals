package cryptopals

import (
	"encoding/base64"
	"encoding/hex"
	"fmt"
	"log"
)

func HexToBase64(hs string) (string, error) {
	v, err := hex.DecodeString(hs)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%s\n", v)

	return base64.StdEncoding.EncodeToString(v), nil

}
