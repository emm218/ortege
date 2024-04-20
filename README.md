# [Örtege](https://en.wikipedia.org/wiki/Yam_(route))

experimenting with building an end to end encrypted messenger using rust and
grpc.

still very much a work in progress

## Todo

- [x] basic public key verification on registration
- [ ] persist registered users to database
- [ ] set up inboxes and message fetching
- [ ] implement [sesame](https://www.signal.org/docs/specifications/sesame/) and
[X3DH](https://www.signal.org/docs/specifications/x3dh/) for managing sessions
and key exchange
- [ ] implement double ratchet encryption

### Bonus Goals

- [ ] federated zero knowledge groups
- [ ] multi-device support
- [ ] nice gui built with iced
- [ ] invite codes instead of open registration
  - [ ] web admin panel to generate invite codes and view error logs

## License

Örtege is licensed under the AGPL 3.0.
