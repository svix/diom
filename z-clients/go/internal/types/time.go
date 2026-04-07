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
	s := time.Time(*t).Format(time.RFC3339)
	return enc.EncodeString(s)
}

func (t *Timestamp) DecodeMsgpack(dec *msgpack.Decoder) error {
	s, err := dec.DecodeString()
	if err != nil {
		return err
	}

	time, err := time.Parse(time.RFC3339, s)
	if err != nil {
		return err
	}

	timestamp := Timestamp(time)
	t = &timestamp
	return nil
}
