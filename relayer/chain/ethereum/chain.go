// Copyright 2020 Snowfork
// SPDX-License-Identifier: LGPL-3.0-only

package ethereum

import (
	"context"
	"fmt"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/snowfork/polkadot-ethereum/relayer/chain"
	"github.com/snowfork/polkadot-ethereum/relayer/contracts/inbound"
	"github.com/snowfork/polkadot-ethereum/relayer/store"
	"github.com/snowfork/polkadot-ethereum/relayer/substrate"
	"golang.org/x/sync/errgroup"

	"github.com/sirupsen/logrus"
	"github.com/snowfork/polkadot-ethereum/relayer/crypto/secp256k1"
)

// Chain streams the Ethereum blockchain and routes tx data packets
type Chain struct {
	config   *Config
	db       *store.Database
	listener *Listener
	writer   *Writer
	conn     *Connection
	log      *logrus.Entry
}

const Name = "Ethereum"

// NewChain initializes a new instance of EthChain
func NewChain(config *Config, db *store.Database) (*Chain, error) {
	log := logrus.WithField("chain", Name)

	kp, err := secp256k1.NewKeypairFromString(config.PrivateKey)
	if err != nil {
		return nil, err
	}

	return &Chain{
		config:   config,
		db:       db,
		listener: nil,
		writer:   nil,
		conn:     NewConnection(config.Endpoint, kp, log),
		log:      log,
	}, nil
}

func (ch *Chain) SetReceiver(subMessages <-chan []chain.Message, _ <-chan chain.Header,
	dbMessages chan<- store.DatabaseCmd, beefyMessages <-chan store.BeefyRelayInfo) error {
	contracts := make(map[substrate.ChannelID]*inbound.Contract)

	writer, err := NewWriter(ch.config, ch.conn, ch.db, subMessages, dbMessages, beefyMessages, contracts, ch.log)
	if err != nil {
		return err
	}
	ch.writer = writer

	return nil
}

func (ch *Chain) SetSender(ethMessages chan<- []chain.Message, ethHeaders chan<- chain.Header,
	dbMessages chan<- store.DatabaseCmd, beefyMessages chan<- store.BeefyRelayInfo) error {
	listener, err := NewListener(ch.config, ch.conn, ch.db, ethMessages,
		beefyMessages, dbMessages, ethHeaders, ch.log)
	if err != nil {
		return err
	}
	ch.listener = listener

	return nil
}

func (ch *Chain) Start(ctx context.Context, eg *errgroup.Group, subInit chan<- chain.Init, ethInit <-chan chain.Init) error {
	if ch.listener == nil && ch.writer == nil {
		return fmt.Errorf("Sender and/or receiver need to be set before starting chain")
	}

	err := ch.conn.Connect(ctx)
	if err != nil {
		return err
	}

	err = ch.sendInitParams(ctx, subInit)
	if err != nil {
		return err
	}

	headerID, err := ch.receiveInitParams(ethInit)
	if err != nil {
		return err
	}

	if ch.listener != nil {
		err = ch.listener.Start(ctx, eg, uint64(headerID.Number), uint64(ch.config.DescendantsUntilFinal))
		if err != nil {
			return err
		}
	}

	if ch.writer != nil {
		err := ch.writer.Start(ctx, eg)
		if err != nil {
			return err
		}
	}

	return nil
}

// Send init params to Substrate chain
func (ch *Chain) sendInitParams(ctx context.Context, subInit chan<- chain.Init) error {
	startingBlocks, err := ch.queryStartingBlocks(ctx)
	if err != nil {
		return err
	}

	subInit <- startingBlocks
	close(subInit)
	return nil
}

func (ch *Chain) queryStartingBlocks(ctx context.Context) (*[2]uint32, error) {

	options := bind.CallOpts{
		Pending: false,
		From:    ch.conn.kp.CommonAddress(),
		Context: ctx,
	}

	// Starting block for basic channel
	contract, err := inbound.NewContract(common.HexToAddress(ch.config.Channels.Basic.Inbound), ch.conn.client)
	if err != nil {
		return nil, err
	}

	basicChannelStartBlock, err := contract.BlockNumber(&options)
	if err != nil {
		return nil, err
	}

	// Starting block for incentivized channel
	contract, err = inbound.NewContract(common.HexToAddress(ch.config.Channels.Incentivized.Inbound), ch.conn.client)
	if err != nil {
		return nil, err
	}

	incentivizedChannelStartBlock, err := contract.BlockNumber(&options)
	if err != nil {
		return nil, err
	}

	numbers := [2]uint32{basicChannelStartBlock, incentivizedChannelStartBlock}

	return &numbers, nil
}

// Receive init params from Substrate chain
func (ch *Chain) receiveInitParams(ethInit <-chan chain.Init) (*HeaderID, error) {
	// Receive init params from Ethereum chain
	headerID, ok := (<-ethInit).(*HeaderID)
	if !ok {
		return nil, fmt.Errorf("invalid init params")
	}
	ch.log.WithFields(logrus.Fields{
		"blockNumber": headerID.Number,
		"blockHash":   headerID.Hash.Hex(),
	}).Debug("Received init params for Ethereum from Substrate")

	return headerID, nil
}

func (ch *Chain) Stop() {
	if ch.conn != nil {
		ch.conn.Close()
	}
}

func (ch *Chain) Name() string {
	return Name
}
