package cryptopals

import (
	"bytes"
	"encoding/hex"
	"testing"
)

func TestSet1Problem1(t *testing.T) {
	res, err := HexToBase64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")
	if err != nil {
		t.Error(err)
	}
	if res != "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t" {
		t.Error("wrong result: ", res)
	}
}

func TestSet1Problem2(t *testing.T) {
	first := HexDecode(t, "1c0111001f010100061a024b53535009181c")
	second := HexDecode(t, "686974207468652062756c6c277320657965")

	res := Xor(first, second)

	if !bytes.Equal(res, HexDecode(t, "746865206b696420646f6e277420706c6179")) {
		t.Errorf("Wrong result: %x", res)
	}

}

func HexDecode(t *testing.T, hs string) []byte {
	v, err := hex.DecodeString(hs)
	if err != nil {
		t.Fatal("Failed to decode hex: ", err)
	}

	return v
}
