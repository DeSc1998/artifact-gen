# artifact-gen
Simulated Artifact generation based on the artifacts in the game [Genshin Impact](https://genshin.hoyoverse.com/en/home)

## Building
Simply run:

```
$ cargo build --release
```

## Usage
The binary can be found in `target/release/`.

To see all available options:
```
$ artifacts -h
```


Example (without colored output):
```
$ artifacts 2
type: Flower +0 | *****
  main stat: Health(717.0)
  sub stats:
    Crit. Damage(7.8)
    Elemantal Mastery(18.6)
    Energy Recharge(5.8)
    Defence(23.1)

type: Circlet +0 | *****
  main stat: Health %(7.0)
  sub stats:
    Energy Recharge(4.5)
    Elemantal Mastery(18.6)
    Defence(18.5)
```

