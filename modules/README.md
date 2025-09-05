# Acceptxmr-server nixos module.

This module is to ease the installation of acceptxmr-server
on the linux NixOs distribution.

## Run tests

To test the nixos module,
simply build the testing flake.

```sh
nixos-rebuild build \
      --flake "./modules/tests/acceptxmr#default" \
      --log-format multiline-with-logs \
      --show-trace
```
