# Review of the Diom API 2026-02-09

Note: I reviewed the docs (localhost:8050/docs), so maybe some things are correct but just not documented correctly?

IMPORTANT: I only commented on the things that need to change, not all the great things there. So sorry that it's a long list of "let's change this" and not a lot of "this is awesome", but it's mostly awesome, just comments for API consistency and other improvement ideas.

## General
- [ ] Docs viewer:
  - [ ] Should remove all the "POST" on the sidebar, as everything is "POST" given how we do RPC.
  - [ ] Figure out if there's a way to de-emphasize the HTTP nature of it (maybe we need a different viewer) - as we don't care about 200, almost everything should be 200, but we want to show the outputs.
  - [ ] Want to show that body can be msgpack too (but same kind of data).
  - [ ] Start with the response schema open?
- [ ] Need to do the same `enum` label fixes we did for Svix.
- [ ] I haven't reviewed the `v1.cache.set` "operation naming" - need to do that.
- [ ] `key` should also support `/` and tbh probably `#`, `|` and potentially other common delimiters.
- [ ] Default values should be better for properties (in the docs).
- [ ] SDKs: do we generate enums for enums? We should if not.
- [ ] Additional thoughts:
  - [ ] How will schemas work later? (e.g. if we want to enforce schemas on keys in kv, etc) Just adding a schema ID to kv create and the likes?
- [ ] Need to support request timeouts, and having a consistent way of reporting it from commands.
  - [ ] One area where it matters (and we maybe want to override it on calls) is `limit` of rate-limiter. E.g. we may want to have a low timeout there and just give up quickly as to not block requests with that?
- [ ] Should we return entities upon creation? E.g. with `create stream` below? I think probably no?
- [ ] We use `snake_case`, not `camelCase`.
- [ ] I noticed a lot of APIs don't have error conditions. E.g. what does `stream ack` return when there's no such message id on the stream? Or a missing consumer group? Or the likes?

## Cache
- [ ] Cache Set:
  - [ ] `expire_in` is confusing - `expiry` would be better, though `ttl` feels much clearer.
  - [ ] Currently doesn't return an object, should at least return {}
- [ ] Cache get:
  - [ ] `expires_at` is a bit verbose and unnatural (we don't want to add at/in/etc. everywhere). `expiry` or `expires` would probably be better?
- [ ] Cache delete:
  - [ ] returns `deleted: true`. I'm thinking out loud, but maybe a more generic (and consistent everywhere) `status: success` or `status: ok` or even `success: true` (maybe not the last one) would be better?
    - [ ] Or is this to indicate that there was an item? If so, maybe a count of deleted items would be better? Because that will also fit wit doing things like `DELETE item/*` and the likes?
- [ ] Fix cache get/return bytes (atm it's string).

## Idempotency
- [ ] Idempotency Start:
  - [ ] `ttl_seconds` -> `lock_ttl`. This is a bit confusing calling it just ttl. I think my suggestion is terrible btw, but we need something that indicates what it is (I don't think it's a timeout either). Has to have a different name than the other ttl in complete.
  - [ ] `status: completed` - I know "completed" is because idempotency previously completed. I wonder if that's the right name though, or if there's a clearer name like `found`?
  - [ ] We are missing a `status`? There should be one for "this is already locked", one for "we got the lock", and one for "we found it!". We are missing one of the locked ones.
- [ ] Idempotency Complete:
  - [ ] `complete` -> `finish` to match `start`? Or potentially rename `start` to `init`? Or some other pair?
  - [ ] `ttl_seconds` -> `ttl` - we don't put units in names.
- [ ] Idempotency Abandon:
  - [ ] `abandon` -> `abort`?
  - [ ] What happens if there's no key already? (never called start) Do we want to return an error here in the body?

## KV
- [ ] KV Set:
  - [ ] All the comments from cache.
  - [ ] `behavior`: I think it's good the way it is, just curious if we had other ideas we were considering for passing this?
  - [ ] How do I update just the TTL? Do we even want that?
  - [ ] How does failure look like? E.g. if I use `insert` and there's already a key, how is the error reported?
- [ ] KV Get:
  - [ ] All the comments from cache.
- [ ] Cache delete:
  - [ ] All the comments from cache.
- [ ] Add a "scan" operation to list keys. I think we should probably support * and maybe in the future ** like mentioned in the ACL RFC. Though for now, maybe let's start with just prefix, so just foo/bar/* or foo/* (so always needs to have a /* at the end, and that's the only place a * is allowed for now.

## Rate-limiter
- [ ] I mentioned it somewhere (don't remember where and what came out of it): Token bucket is superior to fixed window and is a superset of it - you can implement fixed window with token bucket when refill time is the same as window size.
  - [ ] This means that having both is needlessly confusing (let's remove fixed window).
  - [ ] Though I'd also say: do we even want multiple strategies, or just one? E.g. maybe we want leaky bucket? Though maybe that can be a future thing and we shouldn't soil the API yet? (as long as we can support it later with `type` added optionally and changes the config).
  - [ ] Either way, `method` feels like an add name for the type.
  - [ ] NOTE: I'm not commenting on fixed window because of it.
- [ ] Rate limiter limit:
  - [ ] `refill_interval_seconds` -> `refill_interval`.
  - [ ] `refill_interval` should have 1 second as the default.
  - [ ] Should `refill_interval` be in seconds or milliseconds? I feel like milliseconds?
  - [ ] Should we allow optionally taking multiple tokens rather than just 1? OK, I just see it's not in the docs, but yes in the example and is called `units`. Let's call it `tokens` to match what we call it elsewhere?
  - [ ] Returning data:
    - [ ] `retry_after`: isn't that just `refill_interval` always? Unless we support multiple `token` fetches and then it's a bit more complicated potentially. Should this be in seconds or ms if we change refill interval too?
    - [ ] Here we called it `result` is idempotency we called it `status`. In idempotency value is all lowercase (`completed`) here it's upper case (`BLOCK`). Need to be consistent.
    - [ ] `remaining` -> `available`? Not sure, just bouncing ideas. Maybe it's fine.
- [ ] Get remaining:
  - [ ] Not sure: but do we want `retry_after` here? Given that it's not really a try? It also lets us not pass the config which I thin kis a big win for this function in terms of simplicity?
- [ ] How do you envision "multi rate-limit" looking like in the future? So letting us limit on a few in parallel?
- [ ] How do we pass the config around for the rate-limiter? Do we want to init something before, or just pass it live?
  - [ ] I'm just concerned it may be (1) an implementation detail, and (2) confusing.
- [ ] Should we add a command to reset the amount of tokens to a specific number? "Set remaining" or something?

## Stream
- [ ] Create stream:
  - [ ] `name` - let's talk about that later. If we change this to be `namespace` maybe it should be name, though need to decide.
  - [ ] Retention:
    - [ ] `maxByteSize` and `retentionPeriodSeconds` should both sit under a nested `retention` and be called `size` and `time` or `size` and `age`.
    - [ ] `time/age` should be in milliseconds (not seconds). `size` should remain in bytes.
  - [ ] Also mentioned in General above: returning entities upon creation.
  - [ ] `created_at` and `updated_at` - I mentioned it in kv, let's remove the `_at`.
  - [ ] Should there be a stream-level default visibility timeout? Can override on receive, but at least have a default? Feels easier, especially with multi-language codebases.
- [ ] Append to Stream:
  - [ ] Should rename it to `publish` or whatever else is common. Append is unusual for a stream product (kafka, pulsar, rabbitmq, all don't use it).
  - [ ] `payload` -> `value` to be more consistent with the rest of the API (and Kafka and the likes).
  - [ ] I don't know how it'll look like in the SDK, maybe ask Jonas, but I'd love it to be able to send `{"msgs": ["msg1", "msg2"]}` instead of the full decorated record every time (want to support both).
    - [ ] In rust it's easy with `.into()` but I wonder about other languages.
  - [ ] I'm more of a `msgs` guy myself (so not sure I even want to make this comment), but I wonder if we should call it `messages` as we don't use shortened versions elsewhere?
  - [ ] Return info:
    - [ ] Should change `msgIds` to `offset` and return just an integer (last offset iirc, see what Kafka does). The way stream appending works means that the messages are always going to go in together, so always sequential IDs. No need to send them separately (that's also what Kafka does).
- [ ] Fetch from stream:
  - [ ] `fetch` -> `receive` or maybe `consume`? Fetch feels weird to me.
  - [ ] Where does it start fetching from? Is this something we can control? How do we control it? Should we have an optional property to allow for that initial seek?
  - [ ] Again snake_case over camelCase.
  - [ ] `consumerGroup` -> `consumer`? I don't think we lose any information, and it makes it more concise and potentially even clearer?
  - [ ] `visibilityTimeoutSeconds` -> `visibilityTimeout`: no need for the word seconds. Also, maybe let's make it milliseconds since that's what we use elsewhere in the API?
  - [ ] visibilityTimeout: can make it optional if we set it on the stream level itself.
  - [ ] In addition to `batchSize` need `batchWait`, which is the amount of time to wait/block before returning if you don't have `batchSize` messages available?
- [ ] Locking fetch from stream:
  - [ ] I think this should be a `behavior` or `consumer_type` or something on the consumer group, not a separate API (so the same call as `fetch from stream`).
    - [ ] This should also fail if there is already a consumer group with a different behavior assigned to it.
- [ ] Ack range:
  - [ ] This should accept `offset` instead of `min/maxMsgId`. If you're processing a stream, then you want to commit an offset. If you're processing messages individually then to use it you'll need to review and verify they are sequential which is not fun (if we want batch ack, see next section).
  - [ ] This API should only work with behavior: stream.
  - [ ] This should have a different name. In kafka it's called "commit offset" iirc.
- [ ] Ack
  - [ ] This API should only work with behavior: queue. Maybe we should name it accordingly? E.g. prefix all the queue semantics one to have queue in the name?
  - [ ] Allow passing a list of msg ids for batch ack?
- [ ] Dlq
  - [ ] I think we should have `nack` instead, and then a configuration on the stream for what to do with nack (e.g. after X failures, move to DLQ, before then, follow this visibility/retry schedule).
  - [ ] We should probably have a flag on `nack` saying something like `force DLQ now` that will skip the retries and just send to DLQ. I think that's useful for terminal errors.
  - [ ] Unlike ack, nack can work in stream too, though it immediately DLQs. Or potentially a separate API.
- [ ] Redrive
  - [ ] I think having a built-in redrive is great.
- [ ] Need to think about DLQ, how it works (for both streams and queues), how you define which is the DLQ for a stream/queue, how you define where to redrive from, etc. This should probably be a configuration on the stream itself, so need to be mindful of this. Should also default to have a sane default, e.g. `/dlq` or `#dlq` as a naming convention for the DLQ.
- [ ] We use `snake_case` not `camelCase` (e.g. updatedAt and retentinoPeriodSeconds should change)
