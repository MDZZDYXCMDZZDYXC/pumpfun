//! 研究系统。
//!
//! 与原插件一致：研究的花费是**原版经验等级**（不是某种独立的货币）。
//! 解锁某研究后，属于该研究的物品才能在指南与合成中使用。

use crate::lang::Lang;

/// 一项研究。
pub struct Research {
    /// 唯一 key。
    pub key: &'static str,
    /// 中文名。
    pub zh: &'static str,
    /// 英文名。
    pub en: &'static str,
    /// 解锁所需的原版经验等级。
    pub cost: u32,
}

/// 全部研究。
pub const RESEARCHES: &[Research] = &[
    Research { key: "basic", zh: "基础工程", en: "Basic Engineering", cost: 1 },
    Research { key: "resources", zh: "资源加工", en: "Resource Processing", cost: 3 },
    Research { key: "food", zh: "烹饪", en: "Cooking", cost: 2 },
    Research { key: "tools", zh: "工具改良", en: "Improved Tools", cost: 5 },
    Research { key: "steel", zh: "冶金学", en: "Metallurgy", cost: 10 },
    Research { key: "magic", zh: "魔法物质", en: "Magical Materials", cost: 12 },
    Research { key: "combat", zh: "战斗装备", en: "Combat Gear", cost: 15 },
];

/// 按 key 查找研究。
#[must_use]
pub fn find(key: &str) -> Option<&'static Research> {
    RESEARCHES.iter().find(|r| r.key == key)
}

/// 研究的双语名。
#[must_use]
pub fn name(r: &Research, lang: Lang) -> &'static str {
    match lang {
        Lang::Zh => r.zh,
        Lang::En => r.en,
    }
}
