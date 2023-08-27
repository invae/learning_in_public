package main

import (
	"fmt"
	"os"
	"os/signal"
	"time"
)

func main() {
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	go func() {
		for sig := range c {
			fmt.Println(sig)
		}
	}()

	temp := 0
	for temp < 10 {
		time.Sleep(2 * time.Second)
		temp += 1
	}
}
