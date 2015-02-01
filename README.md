# idunn-dht

## Plan

- DHT for discovery *only*
- Nodes are identities
- Multiple locations per node
- No data entries in DHT – only nodes
- TODO: cryptographic protocol for peer joining:
    - Support verification
    - Signing subkeys
    - Revocation of subkeys

## Design notes

- Routing table, per bit:
    - Primary contact bucket
    - “Replacement cache” of all nodes
- Collections are multimaps
    - Node IDs to 1+ contacts/locations
- One message for DHT: `Lookup(node)`
- On receipt of `Lookup(node)`:
    - Build and reply with set of `k` nodes
      - If we have `node` in replacement cache, include it
      - Fill remaining entries with closest nodes routing table
- To perform a node lookup for `node`:
    1. From routing table, build set of `k` closest nodes
    2. Send `Lookup(node)` messages to the `α` closest we have not yet queried.
    3. As responses arrive, update set of `k` closest nodes as necessary.
    4. Repeat at (2) until either `node` is found or all nodes in `k`-set have
       been queried.
