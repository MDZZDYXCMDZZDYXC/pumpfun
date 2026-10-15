//! 物品注册表。
//!
//! Pumpkin 无法注册新的原版物品类型，因此一件「粘液科技物品」由三部分构成：
//!
//! * 一个**原版基础物品**（`base`，决定图标与大致外观）；
//! * 一个**显示名 / 描述**（按语言渲染）；
//! * 一个写在物品自定义数据里的 **`pumpfun:id` 标记**——这是身份的唯一来源，
//!   也是跨语言识别（中文名物品与英文名物品被视为同一件物品）的依据。

use pumpkin_plugin_api::common::NbtTag;
use pumpkin_plugin_api::persistent_data::string_tree;
use pumpkin_plugin_api::{ItemStack, ItemStackExt};

use crate::groups::GroupId;
use crate::lang::Lang;

/// 自定义数据的命名空间（沿用本插件自己的名字，不借用 Slimefun 的命名空间）。
pub const NS: &str = "pumpfun";
/// 存放物品 id 的键。
pub const KEY_ID: &str = "id";
/// 指南书标记键。
pub const KEY_GUIDE: &str = "guide";

/// 一件粘液科技物品的定义。
pub struct ItemDef {
    /// 唯一 id（大写，与 Slimefun 原插件一致）。
    pub id: &'static str,
    /// 原版基础物品键。
    pub base: &'static str,
    /// 所属分类。
    pub group: GroupId,
    /// 所属研究（`None` 表示无需研究）。
    pub research: Option<&'static str>,
    /// 中文显示名。
    pub zh: &'static str,
    /// 英文显示名。
    pub en: &'static str,
    /// 中文描述。
    pub desc_zh: &'static str,
    /// 英文描述。
    pub desc_en: &'static str,
}

/// 构造一条物品定义，减少表格噪音。
#[allow(clippy::too_many_arguments)]
const fn it(
    id: &'static str,
    base: &'static str,
    group: GroupId,
    research: Option<&'static str>,
    zh: &'static str,
    en: &'static str,
    desc_zh: &'static str,
    desc_en: &'static str,
) -> ItemDef {
    ItemDef {
        id,
        base,
        group,
        research,
        zh,
        en,
        desc_zh,
        desc_en,
    }
}

use GroupId::{BasicMachines, Food, MagicalItems, Resources, TechnicalComponents, Tools, Weapons};

/// 全部物品。
pub const ITEMS: &[ItemDef] = &[
    // ---- 资源 ----
    it("STONE_CHUNK", "minecraft:flint", Resources, Some("resources"), "§6碎石块", "§6Stone Chunk", "§7研磨石头得到的碎块", "§7A chunk of stone"),
    it("SIFTED_ORE", "minecraft:gunpowder", Resources, Some("resources"), "§6筛矿", "§6Sifted Ore", "§7可洗出金属矿粉", "§7Can be washed into metal dust"),
    it("SALT", "minecraft:sugar", Resources, Some("resources"), "§f盐", "§fSalt", "§7由沙子洗出", "§7Washed from sand"),
    it("SULFATE", "minecraft:gunpowder", Resources, Some("resources"), "§7硫酸盐", "§7Sulfate", "§7由岩浆块粉碎得到", "§7Crushed from magma blocks"),
    it("CARBON", "minecraft:black_dye", Resources, Some("resources"), "§8碳", "§8Carbon", "§7冶炼合金的基础", "§7Base material for alloys"),
    it("COMPRESSED_CARBON", "minecraft:coal", Resources, Some("resources"), "§8压缩碳", "§8Compressed Carbon", "§7高密度碳", "§7High density carbon"),
    it("IRON_DUST", "minecraft:gunpowder", Resources, Some("resources"), "§6铁粉", "§6Iron Dust", "§7可冶炼成铁锭", "§7Smelts into iron ingot"),
    it("GOLD_DUST", "minecraft:gunpowder", Resources, Some("resources"), "§6金粉", "§6Gold Dust", "§7可冶炼成金锭", "§7Smelts into gold ingot"),
    it("COPPER_DUST", "minecraft:gunpowder", Resources, Some("resources"), "§6铜粉", "§6Copper Dust", "§7可冶炼成铜锭", "§7Smelts into copper ingot"),
    it("TIN_DUST", "minecraft:gunpowder", Resources, Some("resources"), "§6锡粉", "§6Tin Dust", "§7青铜的原料之一", "§7One ingredient of bronze"),
    it("SILVER_DUST", "minecraft:gunpowder", Resources, Some("resources"), "§6银粉", "§6Silver Dust", "§7可冶炼成银锭", "§7Smelts into silver ingot"),
    it("LEAD_DUST", "minecraft:gunpowder", Resources, Some("resources"), "§6铅粉", "§6Lead Dust", "§7沉重的金属粉末", "§7Heavy metal dust"),
    it("ALUMINUM_DUST", "minecraft:gunpowder", Resources, Some("resources"), "§6铝粉", "§6Aluminum Dust", "§7轻金属原料", "§7Light metal material"),
    it("ZINC_DUST", "minecraft:gunpowder", Resources, Some("resources"), "§6锌粉", "§6Zinc Dust", "§7可用于镀层", "§7Used for plating"),
    it("MAGNESIUM_DUST", "minecraft:gunpowder", Resources, Some("resources"), "§6镁粉", "§6Magnesium Dust", "§7易燃的轻金属粉末", "§7Flammable light metal dust"),
    it("SYNTHETIC_SAPPHIRE", "minecraft:lapis_lazuli", Resources, Some("resources"), "§b人造蓝宝石", "§bSynthetic Sapphire", "§7可替代青金石使用", "§7Works like lapis lazuli"),
    it("SYNTHETIC_DIAMOND", "minecraft:diamond", Resources, Some("resources"), "§b人造钻石", "§bSynthetic Diamond", "§7由蓝宝石压缩而成", "§7Compressed from sapphires"),
    it("GOLD_4K", "minecraft:gold_ingot", Resources, Some("resources"), "§6金锭 (4K)", "§6Gold Ingot (4K)", "§7低纯度金锭", "§7Low purity gold ingot"),
    it("GOLD_6K", "minecraft:gold_ingot", Resources, Some("resources"), "§6金锭 (6K)", "§6Gold Ingot (6K)", "§7低纯度金锭", "§7Low purity gold ingot"),
    it("GOLD_8K", "minecraft:gold_ingot", Resources, Some("resources"), "§6金锭 (8K)", "§6Gold Ingot (8K)", "§7中纯度金锭", "§7Medium purity gold ingot"),
    it("GOLD_10K", "minecraft:gold_ingot", Resources, Some("resources"), "§6金锭 (10K)", "§6Gold Ingot (10K)", "§7中纯度金锭", "§7Medium purity gold ingot"),
    it("GOLD_12K", "minecraft:gold_ingot", Resources, Some("resources"), "§6金锭 (12K)", "§6Gold Ingot (12K)", "§7中纯度金锭", "§7Medium purity gold ingot"),
    it("GOLD_14K", "minecraft:gold_ingot", Resources, Some("resources"), "§6金锭 (14K)", "§6Gold Ingot (14K)", "§7中纯度金锭", "§7Medium purity gold ingot"),
    it("GOLD_16K", "minecraft:gold_ingot", Resources, Some("resources"), "§6金锭 (16K)", "§6Gold Ingot (16K)", "§7高纯度金锭", "§7High purity gold ingot"),
    it("GOLD_18K", "minecraft:gold_ingot", Resources, Some("resources"), "§6金锭 (18K)", "§6Gold Ingot (18K)", "§7高纯度金锭", "§7High purity gold ingot"),
    it("GOLD_20K", "minecraft:gold_ingot", Resources, Some("resources"), "§6金锭 (20K)", "§6Gold Ingot (20K)", "§7高纯度金锭", "§7High purity gold ingot"),
    it("GOLD_22K", "minecraft:gold_ingot", Resources, Some("resources"), "§6金锭 (22K)", "§6Gold Ingot (22K)", "§7极高纯度金锭", "§7Very high purity gold ingot"),
    it("GOLD_24K", "minecraft:gold_ingot", Resources, Some("resources"), "§6金锭 (24K)", "§6Gold Ingot (24K)", "§7纯金", "§7Pure gold"),
    // ---- 技术组件 ----
    it("STEEL_INGOT", "minecraft:iron_ingot", TechnicalComponents, Some("steel"), "§b钢锭", "§bSteel Ingot", "§7铁与碳的合金", "§7An alloy of iron and carbon"),
    it("DAMASCUS_STEEL_INGOT", "minecraft:iron_ingot", TechnicalComponents, Some("steel"), "§b大马士革钢锭", "§bDamascus Steel Ingot", "§7强韧的高级合金", "§7Tough advanced alloy"),
    it("BRONZE_INGOT", "minecraft:copper_ingot", TechnicalComponents, Some("steel"), "§b青铜锭", "§bBronze Ingot", "§7铜与锡的合金", "§7An alloy of copper and tin"),
    it("ALUMINUM_INGOT", "minecraft:iron_ingot", TechnicalComponents, Some("steel"), "§b铝锭", "§bAluminum Ingot", "§7轻质金属锭", "§7Light metal ingot"),
    it("MAGNESIUM_INGOT", "minecraft:iron_ingot", TechnicalComponents, Some("steel"), "§b镁锭", "§bMagnesium Ingot", "§7易燃的轻金属锭", "§7Flammable light metal ingot"),
    it("HEATING_COIL", "minecraft:redstone", TechnicalComponents, Some("steel"), "§c加热线圈", "§cHeating Coil", "§7一切加热机器的核心", "§7Core of every heating machine"),
    it("MOTOR", "minecraft:piston", TechnicalComponents, Some("steel"), "§7电动机", "§7Electric Motor", "§7机械动力来源", "§7Source of mechanical power"),
    it("COPPER_WIRE", "minecraft:string", TechnicalComponents, Some("steel"), "§6铜线", "§6Copper Wire", "§7导电的细线", "§7Thin conductive wire"),
    it("BATTERY", "minecraft:redstone_block", TechnicalComponents, Some("steel"), "§c电池", "§cBattery", "§7可以储存一点能量", "§7Stores a bit of energy"),
    // ---- 魔法物品 ----
    it("MAGIC_LUMP_1", "minecraft:slime_ball", MagicalItems, Some("magic"), "§d魔法结晶 I", "§dMagic Lump I", "§7最低级的魔法物质", "§7Lowest tier magic material"),
    it("MAGIC_LUMP_2", "minecraft:magma_cream", MagicalItems, Some("magic"), "§d魔法结晶 II", "§dMagic Lump II", "§7由魔法结晶 I 研磨而来", "§7Ground from Magic Lump I"),
    it("MAGIC_LUMP_3", "minecraft:glowstone_dust", MagicalItems, Some("magic"), "§d魔法结晶 III", "§dMagic Lump III", "§7由魔法结晶 II 研磨而来", "§7Ground from Magic Lump II"),
    it("ENDER_LUMP_1", "minecraft:chorus_fruit", MagicalItems, Some("magic"), "§5末影结晶 I", "§5Ender Lump I", "§7最低级的末影物质", "§7Lowest tier ender material"),
    it("ENDER_LUMP_2", "minecraft:popped_chorus_fruit", MagicalItems, Some("magic"), "§5末影结晶 II", "§5Ender Lump II", "§7由末影结晶 I 研磨而来", "§7Ground from Ender Lump I"),
    it("ENDER_LUMP_3", "minecraft:ender_pearl", MagicalItems, Some("magic"), "§5末影结晶 III", "§5Ender Lump III", "§7由末影结晶 II 研磨而来", "§7Ground from Ender Lump II"),
    // ---- 基础机器 ----
    it("ENHANCED_CRAFTING_TABLE", "minecraft:crafting_table", BasicMachines, Some("basic"), "§e强化工作台", "§eEnhanced Crafting Table", "§7可以合成更复杂的东西", "§7Crafts more complex things"),
    it("GRIND_STONE", "minecraft:dispenser", BasicMachines, Some("basic"), "§b研磨机", "§bGrind Stone", "§7把东西磨成更细的形态", "§7Grinds items into finer forms"),
    it("ORE_CRUSHER", "minecraft:dispenser", BasicMachines, Some("basic"), "§b打粉机", "§bOre Crusher", "§7把矿石打成双倍矿粉", "§7Crushes ores into double dust"),
    it("ORE_WASHER", "minecraft:cauldron", BasicMachines, Some("basic"), "§b洗矿机", "§bOre Washer", "§7把筛矿洗成金属粉末", "§7Washes sifted ore into dust"),
    it("COMPRESSOR", "minecraft:dispenser", BasicMachines, Some("basic"), "§b压缩机", "§bCompressor", "§7把松散的东西压实", "§7Compresses loose materials"),
    it("SMELTERY", "minecraft:dispenser", BasicMachines, Some("basic"), "§b冶炼厂", "§bSmeltery", "§7冶炼矿粉成金属锭", "§7Smelts dust into ingots"),
    it("ARMOR_FORGE", "minecraft:anvil", BasicMachines, Some("basic"), "§b盔甲锻造台", "§bArmor Forge", "§7锻造盔甲", "§7Forges armor"),
    // ---- 工具 ----
    it("SMELTERS_PICKAXE", "minecraft:iron_pickaxe", Tools, Some("tools"), "§a自动冶炼镐", "§aSmelter's Pickaxe", "§7挖矿时自动冶炼矿石", "§7Auto-smelts blocks you mine"),
    it("LUMBER_AXE", "minecraft:iron_axe", Tools, Some("tools"), "§a伐木斧", "§aLumber Axe", "§7一次砍倒整棵树", "§7Fells a whole tree at once"),
    // ---- 武器 ----
    it("BLADE_OF_VAMPIRES", "minecraft:golden_sword", Weapons, Some("combat"), "§c吸血鬼之刃", "§cBlade of Vampires", "§7攻击有几率回复生命", "§7Chance to heal on hit"),
    // ---- 食物 ----
    it("FORTUNE_COOKIE", "minecraft:cookie", Food, Some("food"), "§6幸运饼干", "§6Fortune Cookie", "§7吃下会获得一点经验", "§7Grants a little experience"),
    it("BEEF_JERKY", "minecraft:cooked_beef", Food, Some("food"), "§6牛肉干", "§6Beef Jerky", "§7耐饿的干粮", "§7Hearty dried meat"),
    it("DIET_COOKIE", "minecraft:cookie", Food, Some("food"), "§6减肥饼干", "§6Diet Cookie", "§7吃了会变小", "§7Shrinks you for a while"),
];

/// 按 id 查找物品定义。
#[must_use]
pub fn find(id: &str) -> Option<&'static ItemDef> {
    ITEMS.iter().find(|d| d.id == id)
}

/// 取出某个分类下的全部物品。
#[must_use]
pub fn in_group(group: GroupId) -> Vec<&'static ItemDef> {
    ITEMS.iter().filter(|d| d.group == group).collect()
}

/// 读取物品上的 `pumpfun:id` 标记。
#[must_use]
pub fn id_of(stack: &ItemStack) -> Option<String> {
    let tree = stack.get_custom_data(NS, KEY_ID)?;
    match tree.tags.get(tree.root as usize)? {
        NbtTag::StringTag(s) => Some(s.clone()),
        _ => None,
    }
}

/// 构造一件粘液科技物品（含显示名、描述与 id 标记）。
///
/// 若 id 未知，返回 `None`。
#[must_use]
pub fn build(id: &str, lang: Lang, count: u8) -> Option<ItemStack> {
    let def = find(id)?;
    Some(build_def(def, lang, count))
}

/// 用一条定义构造物品。
#[must_use]
pub fn build_def(def: &ItemDef, lang: Lang, count: u8) -> ItemStack {
    let stack = ItemStack::of(def.base, count);
    stack.set_custom_name(Some(crate::text_util::t(name_of(def, lang))));
    let desc = match lang {
        Lang::Zh => def.desc_zh,
        Lang::En => def.desc_en,
    };
    stack.set_lore(vec![
        crate::text_util::t(desc),
        crate::text_util::t(""),
        crate::text_util::t(&format!("§8Slimefun · {}", def.id)),
    ]);
    stack.set_custom_data(NS, KEY_ID, &string_tree(def.id));
    stack
}

/// 物品的双语显示名。
#[must_use]
pub fn name_of(def: &ItemDef, lang: Lang) -> &'static str {
    match lang {
        Lang::Zh => def.zh,
        Lang::En => def.en,
    }
}

/// 构造一本指南书物品。
#[must_use]
pub fn guide_book(lang: Lang) -> ItemStack {
    let stack = ItemStack::of("minecraft:enchanted_book", 1);
    stack.set_custom_name(Some(crate::text_util::t(match lang {
        Lang::Zh => "§b粘液科技指南书",
        Lang::En => "§bSlimefun Guide",
    })));
    stack.set_lore(vec![
        crate::text_util::t(match lang {
            Lang::Zh => "§7右键打开",
            Lang::En => "§7Right-click to open",
        }),
        crate::text_util::t(""),
        crate::text_util::t("§8Slimefun Guide"),
    ]);
    stack.set_custom_data(NS, KEY_GUIDE, &string_tree("1"));
    stack
}

/// 判断一件物品是否为指南书。
#[must_use]
pub fn is_guide(stack: &ItemStack) -> bool {
    stack.has_custom_data(NS, KEY_GUIDE)
}
