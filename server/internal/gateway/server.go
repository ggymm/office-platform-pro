package gateway

import (
	"fmt"
	"time"

	"server/pkg/common/config"
	"server/pkg/common/constant"
	"server/pkg/common/logger"
	"server/pkg/proto/message"

	"github.com/panjf2000/gnet"
	"github.com/panjf2000/gnet/pkg/pool/goroutine"
	"google.golang.org/protobuf/proto"
)

type MessageServer struct {
	*gnet.EventServer

	addr       string
	async      bool
	multicore  bool
	workerPool *goroutine.Pool
}

// NewMessageServer 初始化服务
func NewMessageServer() *MessageServer {
	return &MessageServer{
		addr:       config.Config.Gateway.Addr,
		async:      true,
		multicore:  true,
		workerPool: goroutine.Default(),
	}
}

// OnInitComplete 服务端初始化
func (ms *MessageServer) OnInitComplete(s gnet.Server) (action gnet.Action) {
	logger.Info("[Gateway 消息服务] 启动成功, 绑定端口: %s (是否多核: %t, 线程数: %d)", s.Addr.String(), s.Multicore, s.NumEventLoop)
	return gnet.None
}

// OnOpened 客户端已连接
func (ms *MessageServer) OnOpened(c gnet.Conn) (out []byte, action gnet.Action) {
	logger.Info("[Gateway 消息服务] 客户端已链接 %s", c.RemoteAddr())

	// 发送连接成功消息
	tickMsg := &message.Message{
		From: constant.ServerId,
		Cmd:  message.CommandType_HEARTBEAT,
	}
	tickMsgData, _ := proto.Marshal(tickMsg)
	if err := c.AsyncWrite(tickMsgData); err != nil {
		fmt.Println(err.Error())
	}
	return
}

// OnClosed 客户端已断开
func (ms *MessageServer) OnClosed(c gnet.Conn, _ error) (action gnet.Action) {
	logger.Info("[Gateway 消息服务] 客户端已关闭 %s", c.RemoteAddr())
	return
}

func (ms *MessageServer) React(recv []byte, c gnet.Conn) (out []byte, action gnet.Action) {
	_ = ms.workerPool.Submit(func() {
		recvMsg := &message.Message{}
		if err := proto.Unmarshal(recv, recvMsg); err != nil {
			fmt.Println(err.Error())
			return
		}

		fmt.Println(recvMsg)

		data, _ := proto.Marshal(recvMsg)
		_ = c.AsyncWrite(data)
	})
	return
}

func (ms *MessageServer) Start() {
	codec := &MessageProtocol{}
	if err := gnet.Serve(
		ms,
		ms.addr,
		gnet.WithMulticore(ms.multicore),
		gnet.WithTCPKeepAlive(time.Minute*5),
		gnet.WithCodec(codec)); err != nil {
		panic(err)
	}

}
