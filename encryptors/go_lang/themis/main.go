package main

import (
	"encoding/base64"
	"fmt"
	"os"

	"github.com/cossacklabs/themis/gothemis/keys"
	"github.com/cossacklabs/themis/gothemis/message"
)

func main() {
	alice_keyPair := gen_keys()
	fmt.Println("printing alice keys:")
	print_keys(alice_keyPair)

	bob_keyPair := gen_keys()
	fmt.Println("printing bob keys:")
	print_keys(bob_keyPair)

	// https://docs.cossacklabs.com/themis/languages/go/features/#secure-message
	clear_text := []byte("The quick brown fox jumps over the lazy dog")
	fmt.Println("clear_text: " + string(clear_text))
	cipher_text := encryptor(clear_text, alice_keyPair.Private, bob_keyPair.Public)
	
	decrypted := decryptor(cipher_text, bob_keyPair.Private, alice_keyPair.Public)
	fmt.Print("decrypted: ")
	fmt.Println(string(decrypted))
	
	return
}

func gen_keys() ( *keys.Keypair) {
	alice_keyPair, err := keys.New(keys.TypeEC)
	if nil != err {
		fmt.Println("Keypair generating error")
		os.Exit(1)
	}
	return alice_keyPair
}
func print_keys(key_pair *keys.Keypair) {
	fmt.Print("Private: ")
	fmt.Println(base64.StdEncoding.EncodeToString(key_pair.Private.Value))
	fmt.Print("Public: ")
	fmt.Println(base64.StdEncoding.EncodeToString(key_pair.Public.Value))
	fmt.Println()
}
func encryptor(clear_text []byte, my_priv_key *keys.PrivateKey, peer_publ_key *keys.PublicKey) ([]byte) {
	aliceToBob := message.New(my_priv_key, peer_publ_key)
	cipher_text, err := aliceToBob.Wrap(clear_text)
	if err != nil {
		fmt.Println("message cannot be empty")
	}
	return cipher_text
}
func decryptor(cipher_text []byte, my_priv_key *keys.PrivateKey, peer_publ_key *keys.PublicKey) ([]byte) {
	// we do not need the priv key to dec, the SecureMessage API is just misleading
	bobToAlice := message.New(my_priv_key, peer_publ_key)
	decrypted, err := bobToAlice.Unwrap(cipher_text)
	if err != nil {
		fmt.Println("decryption failure")
	}
	return decrypted
}
