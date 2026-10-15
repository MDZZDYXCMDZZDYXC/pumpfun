//! 物品分类（对应 Slimefun 的 `ItemGroup`）。
//!
//! 只保留有实际内容的分类，键名沿用 Slimefun 的 `NamespacedKey`，便于对照原插件。

use crate::lang::Lang;

/// 一个分类。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GroupId {
    /// 武器。
    Weapons,
    /// 工具。
    Tools,
    /// 资源（矿粉、金属锭原料等）。
    Resources,
    /// 基础机器（多方块结构）。
    BasicMachines,
    /// 技术组件。
    TechnicalComponents,
    /// 魔法物品。
    MagicalItems,
    /// 食物。
    Food,
}

/// 全部分类，按 tier 排序（与原插件一致：基础类在前）。
pub const GROUPS: &[GroupId] = &[
    GroupId::Weapons,
    GroupId::Tools,
    GroupId::Resources,
    GroupId::BasicMachines,
    GroupId::TechnicalComponents,
    GroupId::MagicalItems,
    GroupId::Food,
];

#[allow(dead_code)]
impl GroupId {
    /// Slimefun 原插件里的 `NamespacedKey`。
    #[must_use]
    pub fn key(self) -> &'static str {
        match self {
            GroupId::Weapons => "weapons",
            GroupId::Tools => "tools",
            GroupId::Resources => "resources",
            GroupId::BasicMachines => "basic_machines",
            GroupId::TechnicalComponents => "tech_misc",
            GroupId::MagicalItems => "magical_items",
            GroupId::Food => "food",
        }
    }

    /// 双语显示名。
    #[must_use]
    pub fn name(self, lang: Lang) -> &'static str {
        match (self, lang) {
            (GroupId::Weapons, Lang::Zh) => "§7武器",
            (GroupId::Weapons, Lang::En) => "§7Weapons",
            (GroupId::Tools, Lang::Zh) => "§7工具",
            (GroupId::Tools, Lang::En) => "§7Tools",
            (GroupId::Resources, Lang::Zh) => "§7资源",
            (GroupId::Resources, Lang::En) => "§7Resources",
            (GroupId::BasicMachines, Lang::Zh) => "§7基础机器",
            (GroupId::BasicMachines, Lang::En) => "§7Basic Machines",
            (GroupId::TechnicalComponents, Lang::Zh) => "§7技术组件",
            (GroupId::TechnicalComponents, Lang::En) => "§7Technical Components",
            (GroupId::MagicalItems, Lang::Zh) => "§7魔法物品",
            (GroupId::MagicalItems, Lang::En) => "§7Magical Items",
            (GroupId::Food, Lang::Zh) => "§7食物",
            (GroupId::Food, Lang::En) => "§7Food",
        }
    }

    /// 分类图标的原版物品键。
    #[must_use]
    pub fn icon_base(self) -> &'static str {
        match self {
            GroupId::Weapons => "minecraft:golden_sword",
            GroupId::Tools => "minecraft:iron_pickaxe",
            GroupId::Resources => "minecraft:gunpowder",
            GroupId::BasicMachines => "minecraft:crafting_table",
            GroupId::TechnicalComponents => "minecraft:iron_ingot",
            GroupId::MagicalItems => "minecraft:glowstone_dust",
            GroupId::Food => "minecraft:cookie",
        }
    }

    /// 分类 tier（仅用于排序）。
    #[must_use]
    pub fn tier(self) -> u8 {
        match self {
            GroupId::Weapons | GroupId::Tools | GroupId::Resources | GroupId::BasicMachines => 1,
            GroupId::TechnicalComponents | GroupId::MagicalItems | GroupId::Food => 2,
        }
    }
}
