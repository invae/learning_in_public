package main

import (
	"io"
	"log"
	"net/http"
)

func main() {
	helloHandler := func( w http.ResponseWriter, req *http.Request) {
		io.WriteString(w , "Hello World\n")
	}

	http.HandleFunc( "/hello", helloHandler)

	log.Println("Listening on localhost:8000")
	log.Fatal( http.ListenAndServe(":8000" , nil) )
}
