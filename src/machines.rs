//! 多方块机器。
//!
//! Pumpkin 无法注册自定义方块或方块实体，因此机器用「原版方块搭出的 3×3 竖直结构」
//! 来模拟：玩家右键结构的**中心方块**即触发。
//!
//! 结构与 Slimefun 原插件一致——图案是一个竖直的 3×3 墙，
//! 索引 0/1/2 为上排、3/4/5 为中排、6/7/8 为下排，
//! 中心索引 4 即玩家右键的那个方块（`trigger_y` 为 0 时）。
//!
//! 触发逻辑：
//! * **手持匹配配方原料** → 消耗原料，产出结果；
//! * **空手**（或强化工作台）→ 打开该机器的配方界面。

use pumpkin_plugin_api::common::BlockPos;
use pumpkin_plugin_api::world::{Block, World};

/// 构造一个方块坐标。
#[must_use]
pub fn pos(x: i32, y: i32, z: i32) -> BlockPos {
    BlockPos { x, y, z }
}

/// 一件机器配方：单输入 → 输出。
pub struct MRecipe {
    /// 输入 token（`sf:` 前缀表示粘液科技物品）。
    pub input: &'static str,
    /// 输入数量。
    pub in_count: u8,
    /// 输出 token。
    pub out: &'static str,
    /// 输出数量。
    pub out_count: u8,
}

/// 机器类型。
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// 强化工作台：界面里进行 3×3 形状合成。
    CraftingTable,
    /// 单输入加工机器：界面展示配方，手持原料可直接加工。
    Machine,
}

/// 一台机器。
pub struct Machine {
    /// 对应物品 id（也是机器方块在物品表里的 id）。
    pub id: &'static str,
    /// 中心相对被点击方块的 Y 偏移（0 = 直接点击中心，-1 = 中心在被点击方块下方）。
    pub trigger_y: i32,
    /// 3×3 结构图案，`""` 表示该格不限。
    pub pattern: [&'static str; 9],
    /// 配方表（`CraftingTable` 为空，改用形状配方）。
    pub recipes: &'static [MRecipe],
    /// 机器类型。
    pub kind: Kind,
}

/// 全部机器。
pub const MACHINES: &[Machine] = &[
    Machine {
        id: "ENHANCED_CRAFTING_TABLE",
        trigger_y: 0,
        pattern: [
            "", "", "",
            "", "minecraft:crafting_table", "",
            "", "minecraft:dispenser", "",
        ],
        recipes: &[],
        kind: Kind::CraftingTable,
    },
    Machine {
        id: "GRIND_STONE",
        trigger_y: 0,
        pattern: [
            "", "", "",
            "", "#wooden_fence", "",
            "", "minecraft:dispenser", "",
        ],
        recipes: &[
            MRecipe { input: "minecraft:blaze_rod", in_count: 1, out: "minecraft:blaze_powder", out_count: 4 },
            MRecipe { input: "minecraft:bone", in_count: 1, out: "minecraft:bone_meal", out_count: 4 },
            MRecipe { input: "minecraft:bone_block", in_count: 1, out: "minecraft:bone_meal", out_count: 9 },
            MRecipe { input: "minecraft:cobblestone", in_count: 1, out: "minecraft:gravel", out_count: 1 },
            MRecipe { input: "minecraft:andesite", in_count: 1, out: "minecraft:gravel", out_count: 1 },
            MRecipe { input: "minecraft:blackstone", in_count: 1, out: "minecraft:gravel", out_count: 1 },
            MRecipe { input: "minecraft:diorite", in_count: 1, out: "minecraft:gravel", out_count: 1 },
            MRecipe { input: "minecraft:granite", in_count: 1, out: "minecraft:gravel", out_count: 1 },
            MRecipe { input: "minecraft:cobbled_deepslate", in_count: 1, out: "minecraft:gravel", out_count: 1 },
            MRecipe { input: "minecraft:dirt", in_count: 1, out: "sf:STONE_CHUNK", out_count: 1 },
            MRecipe { input: "minecraft:sandstone", in_count: 1, out: "minecraft:sand", out_count: 4 },
            MRecipe { input: "minecraft:red_sandstone", in_count: 1, out: "minecraft:red_sand", out_count: 4 },
            MRecipe { input: "minecraft:prismarine_bricks", in_count: 1, out: "minecraft:prismarine", out_count: 2 },
            MRecipe { input: "minecraft:prismarine", in_count: 1, out: "minecraft:prismarine_shard", out_count: 4 },
            MRecipe { input: "minecraft:nether_wart_block", in_count: 1, out: "minecraft:nether_wart", out_count: 9 },
            MRecipe { input: "minecraft:quartz_block", in_count: 1, out: "minecraft:quartz", out_count: 4 },
            MRecipe { input: "minecraft:basalt", in_count: 2, out: "minecraft:blackstone", out_count: 1 },
            MRecipe { input: "minecraft:amethyst_block", in_count: 1, out: "minecraft:amethyst_shard", out_count: 4 },
            MRecipe { input: "sf:MAGIC_LUMP_3", in_count: 1, out: "sf:MAGIC_LUMP_2", out_count: 4 },
            MRecipe { input: "sf:MAGIC_LUMP_2", in_count: 1, out: "sf:MAGIC_LUMP_1", out_count: 4 },
            MRecipe { input: "sf:ENDER_LUMP_3", in_count: 1, out: "sf:ENDER_LUMP_2", out_count: 4 },
            MRecipe { input: "sf:ENDER_LUMP_2", in_count: 1, out: "sf:ENDER_LUMP_1", out_count: 4 },
            MRecipe { input: "minecraft:diamond", in_count: 1, out: "sf:CARBON", out_count: 4 },
        ],
        kind: Kind::Machine,
    },
    Machine {
        id: "ORE_CRUSHER",
        trigger_y: 0,
        pattern: [
            "", "", "",
            "", "minecraft:nether_brick_fence", "",
            "minecraft:iron_bars", "minecraft:dispenser", "minecraft:iron_bars",
        ],
        recipes: &[
            MRecipe { input: "minecraft:blackstone", in_count: 8, out: "minecraft:red_sand", out_count: 1 },
            MRecipe { input: "minecraft:cobblestone", in_count: 8, out: "minecraft:sand", out_count: 1 },
            MRecipe { input: "minecraft:cobbled_deepslate", in_count: 8, out: "minecraft:sand", out_count: 1 },
            MRecipe { input: "minecraft:gravel", in_count: 1, out: "minecraft:sand", out_count: 1 },
            MRecipe { input: "minecraft:magma_block", in_count: 1, out: "sf:SULFATE", out_count: 1 },
            MRecipe { input: "sf:CARBON", in_count: 1, out: "minecraft:coal", out_count: 8 },
            MRecipe { input: "sf:COMPRESSED_CARBON", in_count: 1, out: "sf:CARBON", out_count: 4 },
            MRecipe { input: "sf:GOLD_4K", in_count: 1, out: "sf:GOLD_DUST", out_count: 1 },
            MRecipe { input: "sf:GOLD_6K", in_count: 1, out: "sf:GOLD_DUST", out_count: 2 },
            MRecipe { input: "sf:GOLD_8K", in_count: 1, out: "sf:GOLD_DUST", out_count: 2 },
            MRecipe { input: "sf:GOLD_10K", in_count: 1, out: "sf:GOLD_DUST", out_count: 3 },
            MRecipe { input: "sf:GOLD_12K", in_count: 1, out: "sf:GOLD_DUST", out_count: 3 },
            MRecipe { input: "sf:GOLD_14K", in_count: 1, out: "sf:GOLD_DUST", out_count: 4 },
            MRecipe { input: "sf:GOLD_16K", in_count: 1, out: "sf:GOLD_DUST", out_count: 4 },
            MRecipe { input: "sf:GOLD_18K", in_count: 1, out: "sf:GOLD_DUST", out_count: 5 },
            MRecipe { input: "sf:GOLD_20K", in_count: 1, out: "sf:GOLD_DUST", out_count: 5 },
            MRecipe { input: "sf:GOLD_22K", in_count: 1, out: "sf:GOLD_DUST", out_count: 6 },
            MRecipe { input: "sf:GOLD_24K", in_count: 1, out: "sf:GOLD_DUST", out_count: 6 },
        ],
        kind: Kind::Machine,
    },
    Machine {
        id: "ORE_WASHER",
        trigger_y: 0,
        pattern: [
            "", "minecraft:dispenser", "",
            "", "#wooden_fence", "",
            "", "minecraft:cauldron", "",
        ],
        recipes: &[
            MRecipe { input: "sf:SIFTED_ORE", in_count: 1, out: "sf:IRON_DUST", out_count: 1 },
            MRecipe { input: "minecraft:sand", in_count: 1, out: "sf:SALT", out_count: 1 },
        ],
        kind: Kind::Machine,
    },
    Machine {
        id: "COMPRESSOR",
        trigger_y: 0,
        pattern: [
            "", "", "",
            "", "minecraft:nether_brick_fence", "",
            "minecraft:piston", "minecraft:dispenser", "minecraft:piston",
        ],
        recipes: &[
            MRecipe { input: "sf:STONE_CHUNK", in_count: 4, out: "minecraft:cobblestone", out_count: 1 },
            MRecipe { input: "minecraft:flint", in_count: 8, out: "minecraft:cobblestone", out_count: 1 },
            MRecipe { input: "minecraft:coal_block", in_count: 8, out: "sf:CARBON", out_count: 9 },
            MRecipe { input: "minecraft:charcoal", in_count: 4, out: "minecraft:coal", out_count: 1 },
        ],
        kind: Kind::Machine,
    },
    Machine {
        id: "SMELTERY",
        trigger_y: -1,
        pattern: [
            "", "minecraft:nether_brick_fence", "",
            "minecraft:nether_bricks", "minecraft:dispenser", "minecraft:nether_bricks",
            "", "minecraft:flint_and_steel", "",
        ],
        recipes: &[
            MRecipe { input: "sf:IRON_DUST", in_count: 1, out: "minecraft:iron_ingot", out_count: 1 },
            MRecipe { input: "sf:GOLD_DUST", in_count: 1, out: "minecraft:gold_ingot", out_count: 1 },
            MRecipe { input: "sf:COPPER_DUST", in_count: 1, out: "minecraft:copper_ingot", out_count: 1 },
            MRecipe { input: "sf:ALUMINUM_DUST", in_count: 1, out: "sf:ALUMINUM_INGOT", out_count: 1 },
            MRecipe { input: "sf:MAGNESIUM_DUST", in_count: 1, out: "sf:MAGNESIUM_INGOT", out_count: 1 },
        ],
        kind: Kind::Machine,
    },
    Machine {
        id: "ARMOR_FORGE",
        trigger_y: 0,
        pattern: [
            "", "", "",
            "", "minecraft:anvil", "",
            "", "minecraft:dispenser", "",
        ],
        recipes: &[],
        kind: Kind::Machine,
    },
];

/// 按 id 查找机器。
#[must_use]
pub fn find(id: &str) -> Option<&'static Machine> {
    MACHINES.iter().find(|m| m.id == id)
}

/// 被点击方块所属的机器（若结构成立）。
#[must_use]
pub fn machine_at(world: &World, clicked: BlockPos) -> Option<&'static Machine> {
    for m in MACHINES {
        let c = pos(clicked.x, clicked.y + m.trigger_y, clicked.z);
        if matches_pattern(world, c, &m.pattern) {
            return Some(m);
        }
    }
    None
}

/// 判断以 `center` 为中心的 3×3 竖直结构是否符合图案。
fn matches_pattern(world: &World, center: BlockPos, p: &[&str; 9]) -> bool {
    if !column_ok(world, center, p[1], p[4], p[7]) {
        return false;
    }
    // 四个水平方向都试一遍：对称结构任一方向都能匹配，非对称结构只有一个方向匹配。
    const DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    for (dx, dz) in DIRS {
        let a = pos(center.x + dx, center.y, center.z + dz);
        let b = pos(center.x - dx, center.y, center.z - dz);
        if column_ok(world, a, p[0], p[3], p[6]) && column_ok(world, b, p[2], p[5], p[8]) {
            return true;
        }
    }
    false
}

/// 检查一列（上/中/下）是否匹配。
fn column_ok(world: &World, base: BlockPos, top: &str, mid: &str, bottom: &str) -> bool {
    block_is(world, pos(base.x, base.y + 1, base.z), top)
        && block_is(world, base, mid)
        && block_is(world, pos(base.x, base.y - 1, base.z), bottom)
}

/// 单个方块是否匹配 token（`""` 表示不限）。
fn block_is(world: &World, p: BlockPos, token: &str) -> bool {
    if token.is_empty() {
        return true;
    }
    let block: Block = world.get_block(p);
    block_matches(&block.name, token)
}

/// 方块名与 token 的匹配，支持少量分组 token。
#[must_use]
pub fn block_matches(actual: &str, token: &str) -> bool {
    let actual_clean = actual.strip_prefix("minecraft:").unwrap_or(actual);
    match token {
        "#wooden_fence" => {
            actual_clean.ends_with("_fence") && !actual_clean.starts_with("nether")
        }
        "#log" => actual_clean.ends_with("_log") || actual_clean.ends_with("_wood"),
        _ => {
            let token_clean = token.strip_prefix("minecraft:").unwrap_or(token);
            actual_clean == token_clean
        }
    }
}
