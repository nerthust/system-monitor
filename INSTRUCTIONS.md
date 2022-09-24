# System Monitor

## Operating system

This system monitor has been developed EXCLUSIVELY for LINUX distributions. It
has been tested on Ubuntu and Debian.

## Install dependencies

Execute:

```
./ first-time-install.sh
```

## Build binary

Execute:

```
make build
```

The binary's name is `rtop` and will be located at `<project-root>/dist`.


## Execute binary

```
./dist/rtop
```

By default rtop will show the CPU% based on the total CPU usage. However, if you provide the
optional flag `-u | --current_ussage` rtop will display the CPU% based on the current system CPU%.:

```
./dist/rtop --current_usage
```
