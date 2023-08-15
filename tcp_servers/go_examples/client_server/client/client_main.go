package client

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
	fmt.Println("[INFO]  client entry called")

	conn, err := net.Dial("tcp", HOST+":"+PORT)
	if err != nil {
		fmt.Println("[ERROR] failed to connect")
		os.Exit(1)
	}
	conn.Write([]byte("hello server\n"))
	
	buffer := make([]byte, 1024)
	conn.Read(buffer)
	if err != nil {
		fmt.Println("failed to read the client connection")
	}
	fmt.Print(string(buffer))
	
	
	conn.Close()
}