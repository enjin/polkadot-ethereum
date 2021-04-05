package ethereum

type Config struct {
	Endpoint                 string         `mapstructure:"endpoint"`
	PrivateKey               string         `mapstructure:"private-key"`
	DescendantsUntilFinal    byte           `mapstructure:"descendants-until-final"`
	Channels                 ChannelsConfig `mapstructure:"channels"`
	PolkadotRelayChainBridge string         `mapstructure:"polkadot_relay_chain_bridge"`
	BeefyBlockDelay          uint64         `mapstructure:"beefy_block_delay"`
}

type ChannelsConfig struct {
	Basic        ChannelConfig `mapstructure:"basic"`
	Incentivized ChannelConfig `mapstructure:"incentivized"`
}

type ChannelConfig struct {
	Inbound  string `mapstructure:"inbound"`
	Outbound string `mapstructure:"outbound"`
}
