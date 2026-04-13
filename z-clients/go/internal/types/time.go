package diom_types

import (
	"time"

	"github.com/vmihailenco/msgpack/v5"
)

// Custom timestamp type for encoding control.
//
// Convert to / from time.Time to use.
type Timestamp time.Time

func (t *Timestamp) EncodeMsgpack(enc *msgpack.Encoder) error {
	i := time.Time(*t).UnixMilli()
	return enc.EncodeInt64(i)
}

func (t *Timestamp) DecodeMsgpack(dec *msgpack.Decoder) error {
	i, err := dec.DecodeInt64()
	if err != nil {
		return err
	}

	timestamp := Timestamp(time.UnixMilli(i))
	t = &timestamp
	return nil
}

// Duration in milliseconds.
type DurationMs uint64

func (d DurationMs) Milliseconds() uint64 {
	return uint64(d)
}

func (d *DurationMs) EncodeMsgpack(enc *msgpack.Encoder) error {
	return enc.EncodeUint64(uint64(*d))
}

func (d *DurationMs) DecodeMsgpack(dec *msgpack.Decoder) error {
	i, err := dec.DecodeUint64()
	if err != nil {
		return err
	}

	duration := DurationMs(i)
	d = &duration
	return nil
}
