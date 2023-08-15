package server

import (
	"fmt"
	"os"
	"net"
)

const (
	HOST = "localhost"
	PORT = "8000"
	TYPE = "tcp"
)

func Entry() {
	fmt.Println("[INFO]  called server entry")
	fmt.Println("[INFO]  listening on: " + HOST+":"+PORT)

	listen, err := net.Listen( TYPE, HOST + ":" +PORT )

	if err != nil {
		fmt.Println("cannot bind to")
		os.Exit(1)
	}

	for {
		conn, err := listen.Accept()
		if err != nil {
			fmt.Println("failed to accept connection")
			os.Exit(1)
		}
		fmt.Println("[INFO]  client connected")
		go handleClient( conn )
	}

}

func handleClient(conn net.Conn) {
	
	for {
		buffer := make([]byte, 1024)
		_, err := conn.Read(buffer)
		if err != nil {
			fmt.Println("failed to read the client connection")
			return
		}
		fmt.Print(string(buffer))
		conn.Write([]byte("you connected to server\n"))
	}

	conn.Close()
}

