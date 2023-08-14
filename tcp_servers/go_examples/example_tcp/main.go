package main

import (
	// "fmt"
	"os"
	"net"
	"log"
	"time"
)

const (
	HOST = "localhost"
	PORT = "8000"
	TYPE = "tcp"
)

func main() {
	listen, err := net.Listen(TYPE, HOST + ":" + PORT)
	if err != nil {
		log.Fatal(err)
		os.Exit(1)
	}

	for {
		conn, err := listen.Accept()
		if err != nil {
			log.Fatal(err)
			os.Exit(1)
		}
		go handleIncomingRequests( conn )
	}

	// listen.close()
}

func handleIncomingRequests(conn net.Conn) {
	buffer := make([]byte, 1024)
	_, err := conn.Read(buffer)
	if err != nil {
		log.Fatal(err)
	}
	time := time.Now().Format("Monday, 02-Jan-06 15:04:05 MST")
    conn.Write([]byte("Hi back!"))
    conn.Write([]byte(time))
	handleIncomingRequests(conn)
    // close conn
    // conn.Close()
}
