# [Thorium](https://github.com/Alex313031/thorium) Updater (Windows)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fjust-shadyumbrella%2FThoriumUpdate_rs.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fjust-shadyumbrella%2FThoriumUpdate_rs?ref=badge_shield)


Based on [`template-rs`](https://github.com/just-shadyumbrella/template-rs#prerequisites).

## Usage:
- `/help`, `/?`: Show this
- `/silent`: Do not show console output.
- `/repair`: Reinstall Thorium without uninstalling current install.
  - `/force`: Force uninstall existing Thorium install if any.
    - `/clearuserdata`: Clear Thorium user profile and data as well.
- `/cache`: Do not delete downloaded installer after use.
- `/repo`: Override GitHub repository of Thorium update source. (Default: current active maintainer).
  > ```batch
  > /repo=Alex313031/thorium
  > ```
> [!WARNING]
> This may not future-proof, expect breaking changes in the future.

- `/simd`: Force select specific CPU instruction set build. (Default: automatically detect your maximum CPU capability)
  > ```batch
  > /simd=AVX512
  >
  > :: Show list
  > /simd=show
  > ```
> [!NOTE]
> You can append Chromium installer arguments to the end of the arguments.

## See for more:
- [Bugs](../../issues?q=is%3Aissue+label%3Abug)
- [Feature requests](../../issues?q=is%3Aissue+label%3Aenhancement)


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fjust-shadyumbrella%2FThoriumUpdate_rs.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fjust-shadyumbrella%2FThoriumUpdate_rs?ref=badge_large)