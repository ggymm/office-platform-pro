package gateway

import (
	"bytes"
	"encoding/binary"
	"errors"

	"server/pkg/common/logger"

	"github.com/panjf2000/gnet"
)

type MessageProtocol struct {
	Id     uint16
	Length uint32
	Data   []byte
}

const (
	DefaultHeadLength = 6

	ProtocolId uint16 = 0x0001 // v1 protocol version
)

// Encode 编码
func (mp *MessageProtocol) Encode(_ gnet.Conn, buf []byte) ([]byte, error) {
	result := make([]byte, 0)
	buffer := bytes.NewBuffer(result)
	if err := binary.Write(buffer, binary.LittleEndian, ProtocolId); err != nil {
		logger.Error("协议ID编码错误, %v", err)
		return nil, err
	}
	dataLen := uint32(len(buf))
	if err := binary.Write(buffer, binary.LittleEndian, dataLen); err != nil {
		logger.Error("数据长度编码错误, %v", err)
		return nil, err
	}
	if dataLen > 0 {
		if err := binary.Write(buffer, binary.LittleEndian, buf); err != nil {
			logger.Error("数据编码错误, %v", err)
			return nil, err
		}
	}
	return buffer.Bytes(), nil
}

// Decode 解码
func (mp *MessageProtocol) Decode(c gnet.Conn) ([]byte, error) {
	headerLen := DefaultHeadLength // uint16+uint32
	if size, header := c.ReadN(headerLen); size == headerLen {
		byteBuffer := bytes.NewBuffer(header)
		var (
			id         uint16
			dataLength uint32
		)
		_ = binary.Read(byteBuffer, binary.LittleEndian, &id)
		_ = binary.Read(byteBuffer, binary.LittleEndian, &dataLength)
		if id != ProtocolId {
			c.ResetBuffer()
			logger.Error("协议ID错误: %d, 正确协议ID: %d", id, ProtocolId)
			return nil, errors.New("协议ID错误")
		}
		dataLen := int(dataLength)
		protocolLen := headerLen + dataLen
		if dataSize, data := c.ReadN(protocolLen); dataSize == protocolLen {
			c.ShiftN(protocolLen)
			return data[headerLen:], nil
		}
		return nil, errors.New("数据长度错误")
	}
	return nil, errors.New("数据长度不足")
}
