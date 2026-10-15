# pumpfun

[简体中文](README.zh-CN.md) · **English**

A "Slimefun" plugin for the [Pumpkin](https://pumpkinmc.org) Minecraft server — a Rust
rewrite of the **core gameplay** of
[Slimefun 4](https://github.com/Slimefun/Slimefun4), compiled to a Pumpkin WebAssembly
plugin.

The project layout and coding style follow `two048` (a working Pumpkin plugin example
shipped alongside this one): a single crate, `cdylib`, compiled to a `wasm32-wasip2`
component, with all in-game text rendered server-side per the player's client locale.

- Package / plugin name: `pumpfun` v0.1.0 (Rust edition 2024)
- Artifact: `target/wasm32-wasip2/release/pumpfun.wasm`
- Dependencies: `pumpkin-plugin-api = "0.2.0+26.3-26.51"`, `tracing`

## What this is

Slimefun4 is a 774-file Java Bukkit plugin with 500+ items. Pumpkin's plugin API differs
from Bukkit in ways that matter — a plugin **cannot register custom blocks or block
entities**, there is **no per-block tick callback**, and there are **no custom container
types** — so this project is a **core MVP port**, not a file-by-file conversion:

| Implemented | Notes |
| --- | --- |
| **Item registry** | 56 items, represented as "vanilla item + name + description + a `pumpfun:id` NBT tag". The NBT tag is the single source of identity and the basis for cross-language recognition |
| **Item categories** | 7 `ItemGroup`s: Weapons / Tools / Resources / Basic Machines / Technical Components / Magical Items / Food |
| **Guide GUI** | A `generic_9x6` container: main menu → category (paginated) → item → recipe page. The guide book is granted on first join; right-click to open |
| **Multiblock machines** | 7 machines, with structures identical to upstream Slimefun (3×3 vertical pattern); right-click the center block to trigger |
| **Recipes** | 21 shaped recipes (Enhanced Crafting Table) + 53 machine recipes, migrated from the Slimefun sources |
| **Research** | 7 researches, unlocked by spending **vanilla experience levels** (as upstream — Slimefun has no separate XP currency) |
| **Bilingual** | Simplified Chinese / English, chosen by the client locale prefix (`zh*` → Chinese, everything else → English) |
| **Persistence** | Unlocked researches are stored in `players.tsv` inside the plugin data folder |

Not implemented (deliberately out of scope for the MVP): the energy network, running
electric machines, the cargo network, androids, GPS, backpacks, soul (GEO) resources,
armor effects, the auto-updater and metrics, seasonal categories, and the addon API.

## Key differences from upstream Slimefun

Pumpkin API constraints force the following "do it differently" decisions:

1. **Machines are not custom blocks** but structures built from vanilla blocks. The
   structures match Slimefun exactly, and only the *materials* are matched — the same as
   upstream `MultiBlock.compareMaterials`, which ignores facing.
2. **Machines do not work by dropping items into a dispenser.** Pumpkin cannot read a
   container block's inventory, so instead: **right-click the machine's center block while
   holding an ingredient to process it**; right-click empty-handed to open the recipe list.
3. **The Enhanced Crafting Table is a custom GUI crafting station.** Because vanilla
   recipes match by item type only and cannot tell apart different Slimefun items that
   share a base item, shaped recipes (those involving Slimefun materials) are crafted by
   clicking them in the Enhanced Crafting Table GUI. Recipes that use **vanilla materials
   only** are *additionally* registered as vanilla workbench recipes, which resolves the
   "you need an Enhanced Crafting Table to craft an Enhanced Crafting Table" bootstrap
   problem.
4. **Item textures**: Slimefun relies heavily on player-head textures; this project uses
   plain vanilla items distinguished by name/description and does not attempt to reproduce
   the textures.
5. **No GUI instance ids**: the guide and machine UIs both use `generic_9x6`, and clicks
   are handled through the global `inventory-click` event plus a per-player "current UI
   state".
6. **The NBT namespace uses this plugin's own name**: the item identity tag is
   `pumpfun:id` rather than Slimefun's namespace, so item data does not collide with real
   Slimefun or its addons. Slimefun is retained only as a **documentation-level** source
   and reference (see the top of this file and the license section).

## Source and references

This project is a **reference-based Rust rewrite** of
[Slimefun 4](https://github.com/Slimefun/Slimefun4) (by TheBusyBiscuit and contributors,
GNU GPLv3), intended for learning and experimentation on Pumpkin servers:

- **Gameplay and mechanics** reference Slimefun (item categories, multiblock structures,
  recipes, research unlocking).
- **Item ids and texts, recipe data, and multiblock structure patterns** were migrated
  from the Slimefun sources and fall within its scope of expression.
- **All Rust code** was written for this project and has no line-by-line correspondence
  with Slimefun's Java implementation.
- For distribution, treating this project as GNU GPLv3 is therefore recommended (see the
  license section below); running it on your own server involves no distribution duty.

## Commands

Root command `/slimefun`, alias `/sf`, permission node `pumpfun.command` (allow by
default for all players).

| Command | Description | Permission |
| --- | --- | --- |
| `/sf`, `/sf help` | Help | Everyone |
| `/sf guide`, `/sf open` | Open the guide book main menu | Everyone |
| `/sf give <item-id> [amount]` | Give yourself a Slimefun item | Permission level ≥ 2 (OP) |
| `/sf research` | List all researches with their cost and unlock state | Everyone |
| `/sf research unlock <key>` | Unlock a research by spending experience levels | Everyone |
| `/sf stats` | Show your personal stats | Everyone |
| `/sf reload` | Reload configuration | Permission level ≥ 2 (OP) |

Item ids match Slimefun (uppercase), e.g. `/sf give STEEL_INGOT 8`.

## Multiblock machines

The structures are identical to upstream Slimefun (a 3×3 vertical pattern; the block in
the middle of the center row is the "center", which the player right-clicks). `D` is a
dispenser (upstream requires it facing up):

| Machine | Structure |
| --- | --- |
| Enhanced Crafting Table | center crafting table, dispenser below |
| Grind Stone | center any wooden fence, dispenser below |
| Ore Crusher | center nether brick fence, bottom row iron bars · dispenser · iron bars |
| Ore Washer | dispenser above, center any wooden fence, cauldron below |
| Compressor | center nether brick fence, bottom row piston · dispenser · piston |
| Smeltery | center dispenser (right-click the block above it), nether brick fence above, center row nether bricks · dispenser · nether bricks, flint and steel below |
| Armor Forge | center anvil, dispenser below |

**Usage**: build the structure → right-click the center block empty-handed to view the
recipes → right-click the center block while holding an ingredient to process it (it
consumes 1 or the amount the recipe calls for, and the output goes into your inventory).

## Building and installing

You need a Rust toolchain with the `wasm32-wasip2` target:

```sh
rustup target add wasm32-wasip2
cargo build --release
```

`.cargo/config.toml` already sets `wasm32-wasip2` as the default target, so a plain
`cargo build --release` produces the plugin; the `[profile.release]` block enables LTO to
shrink the binary. Then copy the artifact into the server's plugin directory:

```sh
cp target/wasm32-wasip2/release/pumpfun.wasm /path/to/server/plugins/
```

In game, run `/sf guide` to open the guide, or `/sf give GRIND_STONE` to get a Grind Stone
and try it out.

## Project layout

```
src/
  lib.rs        Plugin entry point (Plugin impl, command/event registration, interact + click dispatch)
  lang.rs       Bilingual text (Lang enum + one method per message)
  item.rs       Item registry and item construction (name/description/NBT id)
  groups.rs     Item categories (ItemGroup)
  recipes.rs    Shaped recipe data + ingredient matching/consumption
  machines.rs   Multiblock structure definitions and matching, machine recipe data
  research.rs   Research definitions
  storage.rs    Player data persistence (players.tsv)
  guide.rs      Guide and machine UIs (rendering + click handling)
  command.rs    The /slimefun command tree and handlers
  text_util.rs  Text component construction helpers
```

## Notes and known limitations

- `cargo test` does not work for this crate (`crate-type = "cdylib"` and the default
  target is wasm). Logic is verified with a warning-free `cargo clippy` and
  `cargo build --release`.
- `pumpkin-plugin-api = "0.2.0+26.3-26.51"` in `Cargo.toml` carries build metadata after
  the `+`, which Cargo ignores and warns about. This is a deliberate version pin so that
  `cargo update` cannot silently move the plugin onto an API track your server does not
  implement. The warning is harmless (the same as in `two048`).
- Recipe data is a **curated subset** migrated from the Slimefun sources, not all 500+
  items and recipes. To add items or recipes, simply append entries to the constant tables
  in `item.rs` / `recipes.rs` / `machines.rs`.

## License

The referenced Slimefun4 is licensed under GNU GPLv3; this port is an independent rewrite,
intended for learning and experimentation.
