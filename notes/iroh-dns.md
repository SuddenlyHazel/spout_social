## Regarding `discovery_n0` and `Tickets`

Use.. 

```
`AddrInfoOptions::RelayAndAddresses` for "well known" nodes with static ips, and permissible firewalls.
`AddrInfoOptions::Id` for nodes which are likely to be roaming
`AddrInfoOptions::Relay` should be the default for most likely for roaming nodes. Having the relay never hurts
`AddrInfoOptions::Addresses` To enable that devices on local networks can leverage direct connections before attempting to connect via a rely
```