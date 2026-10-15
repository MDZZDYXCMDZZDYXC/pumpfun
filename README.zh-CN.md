# pumpfun

<img src="assets/icon-128.png" width="120" height="120" alt="pumpfun icon">

**简体中文** · [English](README.md)

面向 [Pumpkin](https://pumpkinmc.org) 服务器的「粘液科技」插件——把
[Slimefun 4](https://github.com/Slimefun/Slimefun4) 的**核心玩法**用 Rust 重写为
Pumpkin 的 WebAssembly 插件。

工程结构与写法参考了同目录下的 `two048`（一个可运行的 Pumpkin 插件范例）：
单 crate、`cdylib`、编译到 `wasm32-wasip2` 组件，文案按玩家客户端语言在服务端渲染。

- 包名 / 插件显示名：`pumpfun` v0.1.0（Rust edition 2024）
- 产物：`target/wasm32-wasip2/release/pumpfun.wasm`
- 依赖：`pumpkin-plugin-api = "0.2.0+26.3-26.51"`、`tracing`

## 这是什么

Slimefun4 是一个 774 个 Java 文件、500+ 物品的巨型 Bukkit 插件。Pumpkin 的插件
API 与 Bukkit 差距很大（**不能注册自定义方块 / 方块实体**，**没有按方块的 tick 回调**，
**没有自定义容器类型**），因此本项目是它的**核心 MVP 移植**，而不是逐文件搬运：

| 已实现 | 说明 |
| --- | --- |
| **物品注册表** | 56 件物品。用「原版物品 + 名称 + 描述 + `pumpfun:id` NBT 标记」表示；NBT 标记是身份的唯一来源，也是跨语言识别依据 |
| **物品分类** | 7 个 `ItemGroup`：武器 / 工具 / 资源 / 基础机器 / 技术组件 / 魔法物品 / 食物 |
| **指南书 GUI** | `generic_9x6` 容器：主菜单 → 分类（分页）→ 物品 → 配方页。首次进入游戏自动发放指南书，右键打开 |
| **多方块机器** | 7 台机器，结构与 Slimefun 原版一致（3×3 竖直图案），右键中心方块触发 |
| **配方** | 21 份形状配方（强化工作台）+ 53 条机器配方，数据迁移自 Slimefun 源码 |
| **研究系统** | 7 项研究，消耗**原版经验等级**解锁（与原版一致，Slimefun 没有独立的 XP 货币） |
| **双语** | 简体中文 / English，按客户端 locale 前缀选择（`zh*` → 中文，其余 → 英文） |
| **持久化** | 玩家已解锁研究写入插件数据目录的 `players.tsv` |

未实现（有意留在 MVP 之外）：电力网络、电动机器运转、货运网络、机器人、GPS、
背包、灵魂资源、护甲特效、自动更新与统计、季节分类、Addon API 兼容。

## 与 Slimefun 原版的关键差异

Pumpkin API 的限制决定了下面这些「必须换一种做法」的地方：

1. **机器不是自定义方块**，而是原版方块搭出的结构。结构与 Slimefun 完全一致，但
   原版只把「材料」纳入匹配（与原版 `MultiBlock.compareMaterials` 一致，不校验朝向）。
2. **机器不靠把物品丢进发射器**。Pumpkin 无法读取容器方块的物品栏，因此改为：
   **手持原料右键机器中心方块即加工**；空手右键则打开配方列表界面。
3. **强化工作台是自定义 GUI 合成站**。由于原版配方只能按物品类型匹配、无法区分
   同样基础物品的不同粘液科技物品，形状配方（含粘液科技材料的那些）在强化工作台
   GUI 里点击合成；而**只用原版材料的配方**会额外注册为原版工作台配方，
   以解决「必须先有强化工作台才能合成强化工作台」的起步问题。
4. **物品贴图**：Slimefun 大量使用玩家头颅贴图，本项目一律改用普通原版物品 +
   名称/描述区分，不追求贴图还原。
5. **没有 GUI 实例 id**：指南与机器界面都用 `generic_9x6`，点击通过全局
   `inventory-click` 事件 + 「每位玩家当前界面状态」来处理。
6. **NBT 命名空间用本插件自己的名字**：物品身份标记是 `pumpfun:id`，而不是沿用
   Slimefun 的命名空间，这样与原版 Slimefun 及其附件的物品数据互不干扰。
   Slimefun 仅在**文档层面**作为来源与参考说明保留（见文首与文末）。

## 来源与参考

本项目是对 [Slimefun 4](https://github.com/Slimefun/Slimefun4)（TheBusyBiscuit 等，
GNU GPLv3）的**参考式 Rust 重写**，用于 Pumpkin 服务器的学习与实验：

- **玩法与机制**参考 Slimefun（物品分类、多方块机器结构、配方、研究解锁）。
- **物品 id 与文案、配方数据、多方块结构图案**迁移自 Slimefun 源码，属其表达范畴。
- **所有 Rust 代码**为本项目自行编写，与 Slimefun 的 Java 实现无逐行对应关系。
- 因此分发时建议按 GNU GPLv3 对待本项目（见文末「许可」）；若仅自用开服则无分发义务。

## 命令

根命令 `/slimefun`，别名 `/sf`，权限节点 `pumpfun.command`（默认所有玩家可用）。

| 命令 | 说明 | 权限 |
| --- | --- | --- |
| `/sf`、`/sf help` | 帮助 | 所有人 |
| `/sf guide`、`/sf open` | 打开指南书主菜单 | 所有人 |
| `/sf give <物品id> [数量]` | 给自己一份粘液科技物品 | 权限等级 ≥ 2（OP） |
| `/sf research` | 列出全部研究及其花费与解锁状态 | 所有人 |
| `/sf research unlock <key>` | 消耗经验等级解锁研究 | 所有人 |
| `/sf stats` | 查看个人统计 | 所有人 |
| `/sf reload` | 重载配置 | 权限等级 ≥ 2（OP） |

物品 id 与 Slimefun 一致（大写），例如 `/sf give STEEL_INGOT 8`。

## 多方块机器

结构与 Slimefun 原版相同（3×3 竖直图案，中排中间的方块即「中心」，玩家右键它）。
`D` 为漏斗（原版要求朝上）：

| 机器 | 结构 |
| --- | --- |
| 强化工作台 Enhanced Crafting Table | 中心 工作台，下方 漏斗 |
| 研磨机 Grind Stone | 中心 任意木质栅栏，下方 漏斗 |
| 打粉机 Ore Crusher | 中心 下界砖栅栏，下排 铁栅栏·漏斗·铁栅栏 |
| 洗矿机 Ore Washer | 上 漏斗，中心 任意木质栅栏，下 炼药锅 |
| 压缩机 Compressor | 中心 下界砖栅栏，下排 活塞·漏斗·活塞 |
| 冶炼厂 Smeltery | 中心 漏斗（右键其上方方块），上 下界砖栅栏，中排 下界砖·漏斗·下界砖，下 打火石 |
| 盔甲锻造台 Armor Forge | 中心 铁砧，下方 漏斗 |

**用法**：搭好结构 → 空手右键中心方块查看配方 → 手持原料右键中心方块直接加工
（消耗 1 份或配方要求的数量，产物进入背包）。

## 构建与安装

需要 Rust 工具链与 `wasm32-wasip2` 目标：

```sh
rustup target add wasm32-wasip2
cargo build --release
```

`.cargo/config.toml` 已把 `wasm32-wasip2` 设为默认目标，因此一条 `cargo build --release`
即可产出插件；`[profile.release]` 开启了 LTO 以缩小体积。随后把产物拷进服务器插件目录：

```sh
cp target/wasm32-wasip2/release/pumpfun.wasm /path/to/server/plugins/
```

进入游戏后用 `/sf guide` 打开指南，或 `/sf give GRIND_STONE` 拿一台研磨机试玩。

## 工程结构

```
src/
  lib.rs        插件入口（Plugin 实现、命令/事件注册、右键与点击分发）
  lang.rs       双语（Lang 枚举 + 每条消息一个方法）
  item.rs       物品注册表与物品构造（名称/描述/NBT id）
  groups.rs     物品分类 ItemGroup
  recipes.rs    形状配方数据 + 材料匹配/扣除
  machines.rs   多方块结构定义与匹配、机器配方数据
  research.rs   研究定义
  storage.rs    玩家数据持久化（players.tsv）
  guide.rs      指南与机器界面（渲染 + 点击处理）
  command.rs    /slimefun 命令树与处理
  text_util.rs  文本组件构造辅助
assets/
  make_icon.py  图标生成脚本（输出 icon.svg；装有 Pillow 时一并渲染各尺寸 PNG）
  icon.svg      矢量图标源文件
  icon-64/128/256/512.png
                各尺寸位图图标，供 README 与仓库展示使用
```

## 说明与已知限制

- `cargo test` 不适用于本 crate（`crate-type = "cdylib"` 且默认目标是 wasm）。逻辑通过
  `cargo clippy` 与 `cargo build --release` 零告警验证。
- `Cargo.toml` 中的 `pumpkin-plugin-api = "0.2.0+26.3-26.51"` 带有 `+` 后的构建元数据，
  Cargo 会忽略它并给出提示；这是刻意的版本锁定，避免 `cargo update` 把插件挪到
  服务器未实现的 API 版本上。该提示无害（与 `two048` 相同）。
- 配方数据是从 Slimefun 源码**迁移的节选**，并非全部 500+ 物品与配方；新增物品/配方
  只需在 `item.rs` / `recipes.rs` / `machines.rs` 的常量表中追加条目。
- `assets/` 下的图标由 `assets/make_icon.py` 生成。写出 `icon.svg` 无任何依赖；
  渲染 PNG 需要 Pillow（`pip install pillow`）。已提交的 PNG 可直接使用，
  仅在你想改设计时才需要重新生成。

## 许可

参考的 Slimefun4 以 GNU GPLv3 授权；本移植项目为独立重写，供学习与实验用途。
