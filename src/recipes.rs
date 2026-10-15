//! 配方数据与材料匹配。
//!
//! ## 两种配方
//!
//! * **形状配方（强化工作台）**——3×3 图案，由强化工作台 GUI 完成合成。
//!   若某配方的全部材料都是原版物品，则会*额外*注册成原版工作台配方，
//!   这样玩家不必先造出强化工作台就能起步（解决「先有鸡还是先有蛋」的引导问题）。
//! * **机器配方**——单输入 → 输出，见 `machines` 模块。
//!
//! 材料 token 约定：`sf:ITEM_ID` 表示粘液科技物品，其余表示原版物品键
//! （如 `minecraft:iron_ingot`）。

use pumpkin_plugin_api::inventory::Inventory;
use pumpkin_plugin_api::ItemStackExt;

use crate::item;
use crate::lang::Lang;

/// 解析一个材料 token 是否为粘液科技物品；是则返回其 id。
#[must_use]
pub fn sf_id(tok: &str) -> Option<&str> {
    tok.strip_prefix("sf:")
}

/// 构造一份材料/产物物品。
#[must_use]
pub fn build_token(tok: &str, lang: Lang, count: u8) -> Option<pumpkin_plugin_api::ItemStack> {
    if let Some(id) = sf_id(tok) {
        item::build(id, lang, count)
    } else {
        Some(pumpkin_plugin_api::ItemStack::of(tok, count))
    }
}

/// 一件形状配方的材料需求（token，数量）。
#[must_use]
pub fn shaped_requirements(r: &Shaped) -> Vec<(&'static str, u32)> {
    let mut req: Vec<(&'static str, u32)> = Vec::new();
    for row in r.rows {
        for ch in row.chars() {
            if ch == ' ' {
                continue;
            }
            if let Some((_, tok)) = r.keys.iter().find(|(c, _)| *c == ch) {
                if let Some(entry) = req.iter_mut().find(|(t, _)| t == tok) {
                    entry.1 += 1;
                } else {
                    req.push((tok, 1));
                }
            }
        }
    }
    req
}

/// 一件形状配方。
pub struct Shaped {
    /// 用于原版配方注册的唯一键（`pumpfun:<key>`）。
    pub key: &'static str,
    /// 三行图案，字符对应 `keys`，空格表示留空。
    pub rows: [&'static str; 3],
    /// 字符 → 材料 token。
    pub keys: &'static [(char, &'static str)],
    /// 产物 token。
    pub out: &'static str,
    /// 产物数量。
    pub out_count: u8,
    /// 所属研究（`None` 表示无需研究）。
    pub research: Option<&'static str>,
}

impl Shaped {
    /// 该配方是否只用到原版材料（可用于额外注册为原版工作台配方）。
    #[must_use]
    pub fn is_vanilla_only(&self) -> bool {
        !self.keys.is_empty() && self.keys.iter().all(|(_, tok)| !tok.starts_with("sf:"))
    }

    /// 判断整套材料是否都在背包里。
    #[must_use]
    pub fn can_craft(&self, inv: &Inventory) -> bool {
        shaped_requirements(self)
            .iter()
            .all(|(tok, need)| count_in_inventory(inv, tok) >= *need)
    }

    /// 扣除材料。
    pub fn consume(&self, inv: &Inventory) {
        for (tok, need) in shaped_requirements(self) {
            consume_from_inventory(inv, tok, need);
        }
    }
}

/// 统计背包中匹配某 token 的物品总数。
#[must_use]
pub fn count_in_inventory(inv: &Inventory, tok: &str) -> u32 {
    let mut total = 0u32;
    for slot in 0..inv.get_size() {
        if let Some(stack) = inv.get_item(slot)
            && matches_token(&stack, tok)
        {
            total += u32::from(stack.get_count());
        }
    }
    total
}

/// 判断一件物品是否匹配 token。
#[must_use]
pub fn matches_token(stack: &pumpkin_plugin_api::ItemStack, tok: &str) -> bool {
    if let Some(id) = sf_id(tok) {
        item::id_of(stack).as_deref() == Some(id)
    } else {
        stack.matches_item(tok)
    }
}

/// 从背包扣除若干匹配 token 的物品；不足则扣除全部可扣的。
pub fn consume_from_inventory(inv: &Inventory, tok: &str, mut need: u32) {
    for slot in 0..inv.get_size() {
        if need == 0 {
            break;
        }
        let Some(stack) = inv.get_item(slot) else {
            continue;
        };
        if !matches_token(&stack, tok) {
            continue;
        }
        let have = u32::from(stack.get_count());
        if have <= need {
            need -= have;
            inv.set_item(slot, None);
        } else {
            stack.set_count(u8::try_from(have - need).unwrap_or(0));
            inv.set_item(slot, Some(stack));
            need = 0;
        }
    }
}

/// 产出一份物品到背包，成功返回 `true`；背包满则返回 `false`。
#[must_use]
pub fn give_to_inventory(inv: &Inventory, stack: pumpkin_plugin_api::ItemStack) -> bool {
    // 先尝试堆叠到同类物品上。
    for slot in 0..inv.get_size() {
        if let Some(existing) = inv.get_item(slot)
            && same_kind(&existing, &stack)
        {
            let max = u32::from(existing.get_max_count());
            let cur = u32::from(existing.get_count());
            let add = u32::from(stack.get_count());
            if cur + add <= max {
                existing.set_count(u8::try_from(cur + add).unwrap_or(u8::MAX));
                inv.set_item(slot, Some(existing));
                return true;
            }
        }
    }
    // 否则找一个空位。
    for slot in 0..inv.get_size() {
        if inv.get_item(slot).is_none() {
            inv.set_item(slot, Some(stack));
            return true;
        }
    }
    false
}

/// 判断两件物品是否「同一种」（粘液科技物品按 id，原版按注册键）。
#[must_use]
pub fn same_kind(a: &pumpkin_plugin_api::ItemStack, b: &pumpkin_plugin_api::ItemStack) -> bool {
    match (item::id_of(a), item::id_of(b)) {
        (Some(ia), Some(ib)) => ia == ib,
        (None, None) => a.get_registry_key() == b.get_registry_key(),
        _ => false,
    }
}

/// 全部形状配方（由强化工作台完成合成）。
pub const SHAPED: &[Shaped] = &[
    // ---- 基础机器（纯原版材料，同时注册为原版工作台配方）----
    Shaped {
        key: "enhanced_crafting_table",
        rows: ["PPP", "PCP", "PPP"],
        keys: &[('P', "minecraft:oak_planks"), ('C', "minecraft:crafting_table")],
        out: "sf:ENHANCED_CRAFTING_TABLE",
        out_count: 1,
        research: Some("basic"),
    },
    Shaped {
        key: "grind_stone",
        rows: [" S ", "SDS", " S "],
        keys: &[('S', "minecraft:stick"), ('D', "minecraft:dispenser")],
        out: "sf:GRIND_STONE",
        out_count: 1,
        research: Some("basic"),
    },
    Shaped {
        key: "ore_crusher",
        rows: ["I I", " D ", "I I"],
        keys: &[('I', "minecraft:iron_bars"), ('D', "minecraft:dispenser")],
        out: "sf:ORE_CRUSHER",
        out_count: 1,
        research: Some("basic"),
    },
    Shaped {
        key: "ore_washer",
        rows: ["F F", " C ", "F F"],
        keys: &[('F', "minecraft:oak_fence"), ('C', "minecraft:cauldron")],
        out: "sf:ORE_WASHER",
        out_count: 1,
        research: Some("basic"),
    },
    Shaped {
        key: "compressor",
        rows: ["P P", " D ", "P P"],
        keys: &[('P', "minecraft:piston"), ('D', "minecraft:dispenser")],
        out: "sf:COMPRESSOR",
        out_count: 1,
        research: Some("basic"),
    },
    Shaped {
        key: "smeltery",
        rows: ["N N", " D ", "F F"],
        keys: &[
            ('N', "minecraft:nether_bricks"),
            ('D', "minecraft:dispenser"),
            ('F', "minecraft:flint_and_steel"),
        ],
        out: "sf:SMELTERY",
        out_count: 1,
        research: Some("basic"),
    },
    Shaped {
        key: "armor_forge",
        rows: ["I I", " A ", "I I"],
        keys: &[('I', "minecraft:iron_block"), ('A', "minecraft:anvil")],
        out: "sf:ARMOR_FORGE",
        out_count: 1,
        research: Some("basic"),
    },
    // ---- 工具 / 武器 / 食物（纯原版材料）----
    Shaped {
        key: "smelters_pickaxe",
        rows: ["III", " S ", " S "],
        keys: &[('I', "minecraft:iron_ingot"), ('S', "minecraft:stick")],
        out: "sf:SMELTERS_PICKAXE",
        out_count: 1,
        research: Some("tools"),
    },
    Shaped {
        key: "lumber_axe",
        rows: ["II ", "IS ", " S "],
        keys: &[('I', "minecraft:iron_ingot"), ('S', "minecraft:stick")],
        out: "sf:LUMBER_AXE",
        out_count: 1,
        research: Some("tools"),
    },
    Shaped {
        key: "blade_of_vampires",
        rows: [" G ", " G ", " S "],
        keys: &[('G', "minecraft:gold_ingot"), ('S', "minecraft:stick")],
        out: "sf:BLADE_OF_VAMPIRES",
        out_count: 1,
        research: Some("combat"),
    },
    Shaped {
        key: "fortune_cookie",
        rows: ["WWW", "WCW", "WWW"],
        keys: &[('W', "minecraft:wheat"), ('C', "minecraft:cocoa_beans")],
        out: "sf:FORTUNE_COOKIE",
        out_count: 4,
        research: Some("food"),
    },
    // ---- 技术组件（含粘液科技材料，仅在强化工作台合成）----
    Shaped {
        key: "steel_ingot",
        rows: [" I ", "ICI", " I "],
        keys: &[('I', "sf:IRON_DUST"), ('C', "sf:CARBON")],
        out: "sf:STEEL_INGOT",
        out_count: 1,
        research: Some("steel"),
    },
    Shaped {
        key: "damascus_steel_ingot",
        rows: [" I ", "ICI", " I "],
        keys: &[('I', "sf:STEEL_INGOT"), ('C', "sf:CARBON")],
        out: "sf:DAMASCUS_STEEL_INGOT",
        out_count: 1,
        research: Some("steel"),
    },
    Shaped {
        key: "bronze_ingot",
        rows: ["CT ", "   ", "   "],
        keys: &[('C', "sf:COPPER_DUST"), ('T', "sf:TIN_DUST")],
        out: "sf:BRONZE_INGOT",
        out_count: 2,
        research: Some("steel"),
    },
    Shaped {
        key: "heating_coil",
        rows: [" S ", "SCS", " S "],
        keys: &[('S', "sf:STEEL_INGOT"), ('C', "sf:CARBON")],
        out: "sf:HEATING_COIL",
        out_count: 1,
        research: Some("steel"),
    },
    Shaped {
        key: "motor",
        rows: ["SIS", "ICI", "SIS"],
        keys: &[
            ('S', "sf:STEEL_INGOT"),
            ('I', "sf:IRON_DUST"),
            ('C', "sf:COPPER_DUST"),
        ],
        out: "sf:MOTOR",
        out_count: 1,
        research: Some("steel"),
    },
    Shaped {
        key: "copper_wire",
        rows: [" C ", " C ", " C "],
        keys: &[('C', "sf:COPPER_DUST")],
        out: "sf:COPPER_WIRE",
        out_count: 3,
        research: Some("steel"),
    },
    Shaped {
        key: "battery",
        rows: ["CRC", "   ", "   "],
        keys: &[('C', "sf:COPPER_DUST"), ('R', "minecraft:redstone")],
        out: "sf:BATTERY",
        out_count: 1,
        research: Some("steel"),
    },
    // ---- 资源加工（含粘液科技材料）----
    Shaped {
        key: "synthetic_sapphire",
        rows: ["LLL", "LDL", "LLL"],
        keys: &[('L', "minecraft:lapis_lazuli"), ('D', "minecraft:diamond")],
        out: "sf:SYNTHETIC_SAPPHIRE",
        out_count: 1,
        research: Some("resources"),
    },
    Shaped {
        key: "synthetic_diamond",
        rows: ["SSS", "SSS", "SSS"],
        keys: &[('S', "sf:SYNTHETIC_SAPPHIRE")],
        out: "sf:SYNTHETIC_DIAMOND",
        out_count: 1,
        research: Some("resources"),
    },
    Shaped {
        key: "beef_jerky",
        rows: ["BC ", "   ", "   "],
        keys: &[('B', "minecraft:cooked_beef"), ('C', "sf:SALT")],
        out: "sf:BEEF_JERKY",
        out_count: 3,
        research: Some("food"),
    },
];
