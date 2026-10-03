# [Thorium](https://github.com/Alex313031/thorium) Updater (Windows)

Work in progress, proof of concept.

Based on [`template-rs`](https://github.com/just-shadyumbrella/template-rs#prerequisites).

## Usage:
- `/help`, `/?`: Show this
- `/silent`: Do not show console output.
- `/reinstall`: Reinstall Thorium without uninstalling current install.
- `/force`: Force uninstallation of existing Thorium install if any.
  - `/clearuserdata`: Clear Thorium user profile and data as well.
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
