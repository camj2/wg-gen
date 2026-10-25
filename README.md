# `wg-gen`

// https://github.com/wg-easy/wg-easy
// https://github.com/wg-ez/wg-ez

// todo: support utilizing all last 8 octets, instead of just the last 2 octets (crazy) u64
// https://en.wikipedia.org/wiki/IPv6_address
// https://www.wireguard.com/xplatform/
// https://en.wikipedia.org/wiki/IPv6_address#Unicast_and_anycast_address_format

// man 8 wg
// man 8 wg-quick

// Each peer is assigned an IPv6 unique local address (ULA). This means no risk of CIDR overlapping.

Generate IPv6-only WireGuard peers.
Each peer is assigned a [ULA](https://en.wikipedia.org/wiki/Unique_local_address).
This means no risk of CIDR overlapping.
Useful for private roaming interconnected networks.

**Note**:
When connected to an IPv4-only network, Android supports DNS queries sent over an IPv6-only
[split VPN](https://en.wikipedia.org/wiki/Split_tunneling).
*However*,
any other traffic sent over the tunnel, such as HTTP requests, do not work.
This would require the device be connected to a dual stack IPv4/IPv6 network.
This appears to be a limitation with the Android network stack (more research required).

## Generator flags

`-o <dir>`: output directory (default: `./wireguard`)

`-e <host>:<port>`: server peer endpoint (required)

`-p <port>`: listening port (default: `51820`)

`-P <prefix>`: [64-bit prefix](https://www.unique-local-ipv6.com/) (default: random)

`-g`: generate random prefix and exit (debug)

## Peer flags

`-e <host>:<port>`: server endpoint (useful for static devices on the same network)

`-s <host>:<port>`: peer endpoint (useful for static devices on the same network)

`-p <port>`: listening port (used with `-s`, default: `51820`)

`-k <int>`: keepalive (recommended value: `25`)

## Usage

Generate peers with human-readable names:

```sh
wg-gen -e wg.example.com:51820 \
  phone \
  laptop \
    -k 25 \
  computer \
    -e 10.0.0.2:51820 \
    -s 10.0.0.4:51820 \
  backup \
    -e 10.0.0.2:51820 \
    -s 10.0.0.3:51820
```

Generate many peers for large scale deployment:

```sh
wg-gen -e wg.example.com:51820 $(seq 1000)
```

### Unbound

The generator creates an unbound config with a map of all the peers:

```
local-data: "server   AAAA fd87:9b28:1e2f:b635::1"
local-data: "phone    AAAA fd87:9b28:1e2f:b635::2"
local-data: "laptop   AAAA fd87:9b28:1e2f:b635::3"
local-data: "computer AAAA fd87:9b28:1e2f:b635::4"
local-data: "backup   AAAA fd87:9b28:1e2f:b635::5"
```

Useful commands:

```sh
wg show
cat /etc/resolv.conf
dig +short server AAAA
ssh server
rsync -a <dir> server@server:
```

## LICENSE

MIT
