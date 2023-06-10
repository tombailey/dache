# dache

## Introduction

This is a super simple and not very feature-complete durable cache (dache) written in Rust.

## Getting started

```dockerfile
FROM tombailey256/dache:0.0.0

ENV DURABILITY_ENGINE="memory"

ENV PORT = 8080
```

Note that dache does NOT currently support horizontal scaling. If you run dache on more than one node, even with the
same configuration for the durability engine, you will get unexpected results.

## Rest API

### Get entry
```text
$> curl -XGET localhost/dache/myKey
200 OK
{
  "value": "myValue"
}
```

### Set entry
```text
$> curl -XPOST localhost/dache/myKey -d { "value": "myValue" }
204 No content
```

`expiry` is an optional unix timestamp (UTC, seconds since 1970). If you don't give an expiry time, the entry will remain in the cache until a delete operation happens.

### Delete entry
```text
$> curl -XDELETE localhost/dache/myKey
204 No content
```

## Durability

Dache is able to survive crashes, node failure, etc by storing messages using a durability engine.

When dache starts, it creates an in-memory cache based on the state of a durability engine. After that, read requests do
not require interaction with a durability engine. However, set and delete operations still require interaction with a
durability engine in order to recreate the in-memory cache in the event of failure.

Note, if the durability engine becomes unavailable after dache has populated it's in-memory cache, read operations will
succeed but set and delete operations will fail.

The following durability engines are supported:

### In-memory

Stores entries in-memory. There is no durability if dache is restarted or crashes.

```sh
export DURABILITY_ENGINE="memory"
```

### Postgres

Stores entries using a postgres table.

```sh
export DURABILITY_ENGINE="postgres"
export POSTGRES_HOST="localhost"
export POSTGRES_PORT="5432"
export POSTGRES_DATABASE="database"
export POSTGRES_USER="user"
export POSTGRES_PASSWORD="password"
```

## Health check

Dache has a built-in health check endpoint (`/health`) to confirm that it is working correctly. At the moment, it does
NOT confirm that the durability engine is working correctly.

## Future work

1. Entry expiry
2. Disk durability engine
3. Better health checking
4. Metrics/observability?
