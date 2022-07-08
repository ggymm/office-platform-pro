package main

import "server/internal/gateway"

func main() {
	messageServer := gateway.NewMessageServer()
	messageServer.Start()
}
