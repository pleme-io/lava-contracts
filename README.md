# lava-contracts

Typed result contracts for lava architectures — `NetworkResult`,
`IamResult`, `ClusterResult`, and friends.

An architecture returns a typed contract rather than a loose bag of outputs,
so cross-architecture composition validates **at compile time, not apply
time**: wiring a cluster to something that never produced a network is a
build error.

The lava analog of Pangea's `Pangea::Contracts::*Result`.

## Install

```toml
[dependencies]
lava-contracts = "0.1"
```

## The suite

```
lava-core ──► lava-contracts ──► lava-architectures
```

Depends only on [`lava-core`](https://github.com/pleme-io/lava-core).

## License

MIT
