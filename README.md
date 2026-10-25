# `wg-gen`

Generate WireGuard config files.
Each peer is assigned an IPv6 ULA prefix.
This means no risk of CIDR overlapping.
Useful for private roaming interconnected networks.

**Android**:
When connected to an IPv4-only network, Android supports DNS queries sent over an IPv6-only
[split VPN](https://en.wikipedia.org/wiki/Split_tunneling).
*However*,
any other traffic sent over the tunnel, such as HTTP requests, do not work.
This would require the device be connected to a dual stack IPv4/IPv6 network.
This appears to be a limitation with the Android network stack (more research required).

## Build

```sh
git clone https://git.camj.me/wireguard
cd wireguard

cargo build --release
install -m 755 target/release/wg-gen /usr/local/bin/
```

## Usage

```sh
pb --help
```

Generate peers:

```sh
wg-gen server phone laptop computer backup
```

Set additional options in [`wireguard.json`](doc/wireguard.json) and export:

```
wg-gen -e
```

[`wireguard.conf`](doc/wireguard/wireguard.conf) is the "server" peer, remainder are "client" peers.

[`unbound.conf`](doc/wireguard/unbound.conf) is for Unbound.

## Help

```sh
wg show
cat /etc/resolv.conf
dig +short server AAAA
ssh server
rsync -a <dir> server@server:
```

## Related

See `man 8 wg wg-quick` for more config options.

* [IPv6](https://en.wikipedia.org/wiki/IPv6_address)
* [Unique Local Address](https://en.wikipedia.org/wiki/Unique_local_address)
* [Unbound](https://www.nlnetlabs.nl/projects/unbound/about/)
* [WireGuard](https://www.wireguard.com/)
* [WireGuard Cross-platform Userspace Implementation](https://www.wireguard.com/xplatform/)
* [`wg-easy`](https://github.com/wg-easy/wg-easy)
* [`wg-ez`](https://github.com/wg-ez/wg-ez)

## LICENSE

MIT
