:: https://github.com/protocolbuffers/protobuf/releases
:: go install google.golang.org/protobuf/cmd/protoc-gen-go@latest

protoc.exe --proto_path=../pkg/proto/message/ --go_out=../pkg/proto/ message.proto