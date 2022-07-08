package config

import (
	"os"
	"time"

	"github.com/pelletier/go-toml"
)

type config struct {
	Gateway gatewayConfig
}

type gatewayConfig struct {
	Addr         string
	Tick         bool
	TickInterval time.Duration `toml:"tick_interval"`
	TickMax      int           `toml:"tick_max"`
}

var Config *config

func init() {
	Config = new(config)
	path := os.Getenv("CONFIG_PATH")
	if len(path) == 0 {
		path = "config/config.toml"
	}
	c, err := toml.LoadFile(path)
	if err != nil {
		panic(err)
	}
	err = c.Unmarshal(Config)
	if err != nil {
		panic(err)
	}
}
