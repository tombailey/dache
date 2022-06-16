# dache

## Introduction

This is a super simple and not very feature-complete durable cache (dache). Right now, postgres is the only supported durability engine.

## Getting started

```dockerfile
FROM tombailey256/dache:0.1.1

ENV DURABILITY_ENGINE="postgres"
ENV POSTGRES_HOST="postgres"
ENV POSTGRES_PORT=5432
ENV POSTGRES_USER="user"
ENV POSTGRES_PASSWORD="password"
ENV POSTGRES_DATABASE="database"

ENV PORT = 8080
```

Note that currently, dache does NOT support horizontal scaling. If you run dache on more than one node, even with the same durability engine, you will get unexpected results.

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
$> curl -XPOST localhost/dache/myKey -d { "value": "myValue", "expiry": 1735689600 }
204 No content
```

`expiry` is an optional unix timestamp (UTC, seconds since 1970). If you don't give an expiry time, the entry will remain in the cache until a delete operation happens.

### Delete entry
```text
$> curl -XDELETE localhost/dache/myKey
204 No content
```


## Durability

Dache is able to survive crashes, node failure, etc by storing entries using a durability engine.

When dache starts, it creates an in-memory cache so it can skip the durability engine and reduce latency for read options. However, set and delete operations still require interaction with the durability engine in order to support durability.

Right now, postgres is the only supported durability engine.

Note, if the durability engine is unavailable after dache has populated it's in-memory cache, read operations will succeed but set and delete operations will fail.

## Health check

Dache has a built-in health check endpoint (`/health`) to confirm that it is working correctly. At the moment, it does NOT confirm that the durability engine is working correctly.

## Future work

1. Better health checking
2. Metrics/observability?
