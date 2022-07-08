package gateway

import (
	"fmt"
	"google.golang.org/protobuf/proto"
	"server/pkg/common/config"
	"server/pkg/common/logger"
	"server/pkg/proto/message"
	"time"

	"github.com/panjf2000/gnet"
	"github.com/panjf2000/gnet/pkg/pool/goroutine"
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
	logger.Info("[Gateway消息服务]启动成功, 绑定端口: %s (是否多核: %t, 线程数: %d)", s.Addr.String(), s.Multicore, s.NumEventLoop)
	return gnet.None
}

func (ms *MessageServer) React(recv []byte, c gnet.Conn) (out []byte, action gnet.Action) {
	_ = ms.workerPool.Submit(func() {
		recvMsg := &message.Message{}
		_ = proto.Unmarshal(recv, recvMsg)

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
