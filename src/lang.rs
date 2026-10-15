//! 双语支持（简体中文 / English）。
//!
//! 语言按玩家客户端上报的 locale 前缀选择（`zh*` → 中文，其余 → 英文），
//! 与参考项目 two048 的做法保持一致：所有文案在服务端渲染，玩家无需安装任何资源包。
//!
//! 每条消息都是 [`Lang`] 上的一个方法，方法内部对 `Zh` / `En` 分支返回对应文案，
//! 因此漏翻或参数数量不符会在编译期暴露。

/// 当前使用的语言。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lang {
    /// 简体中文。
    Zh,
    /// 英文。
    En,
}

impl Lang {
    /// 根据客户端 locale 前缀选择语言。
    ///
    /// 简体、繁体、港澳中文一律回退到简体中文（对 `zh_tw` 读者而言简体远比英文可读）。
    /// 控制台 / 命令方块 / RCON 没有 locale，回退到英文。
    #[must_use]
    pub fn from_locale(locale: &str) -> Lang {
        if locale.starts_with("zh") {
            Lang::Zh
        } else {
            Lang::En
        }
    }

    // ---- 指南书 ----

    /// 指南书主菜单标题。
    #[must_use]
    pub fn guide_title(&self) -> &'static str {
        match self {
            Lang::Zh => "§8粘液科技 · 指南",
            Lang::En => "§8Slimefun · Guide",
        }
    }

    /// 分类页标题。
    #[must_use]
    pub fn category_title(&self) -> &'static str {
        match self {
            Lang::Zh => "§8粘液科技 · 分类",
            Lang::En => "§8Slimefun · Category",
        }
    }

    /// 物品配方页标题。
    #[must_use]
    pub fn recipe_title(&self) -> &'static str {
        match self {
            Lang::Zh => "§8粘液科技 · 配方",
            Lang::En => "§8Slimefun · Recipe",
        }
    }

    /// 返回上一页。
    #[must_use]
    pub fn back(&self) -> &'static str {
        match self {
            Lang::Zh => "§e« 返回",
            Lang::En => "§e« Back",
        }
    }

    /// 上一页。
    #[must_use]
    pub fn prev_page(&self) -> &'static str {
        match self {
            Lang::Zh => "§e« 上一页",
            Lang::En => "§e« Previous",
        }
    }

    /// 下一页。
    #[must_use]
    pub fn next_page(&self) -> &'static str {
        match self {
            Lang::Zh => "§e下一页 »",
            Lang::En => "§eNext »",
        }
    }

    /// 关闭按钮。
    #[must_use]
    pub fn close(&self) -> &'static str {
        match self {
            Lang::Zh => "§c关闭",
            Lang::En => "§cClose",
        }
    }

    /// 页码提示，如 `第 1 / 3 页`。
    #[must_use]
    pub fn page_indicator(&self, page: usize, total: usize) -> String {
        match self {
            Lang::Zh => format!("§7第 {page} / {total} 页"),
            Lang::En => format!("§7Page {page} / {total}"),
        }
    }

    /// 点击查看物品配方。
    #[must_use]
    pub fn click_view(&self) -> &'static str {
        match self {
            Lang::Zh => "§7点击查看配方",
            Lang::En => "§7Click to view recipe",
        }
    }

    /// 点击合成（强化工作台）。
    #[must_use]
    pub fn click_craft(&self) -> &'static str {
        match self {
            Lang::Zh => "§a点击合成",
            Lang::En => "§aClick to craft",
        }
    }

    /// 未解锁研究的物品提示。
    #[must_use]
    pub fn locked(&self) -> &'static str {
        match self {
            Lang::Zh => "§c✖ 未解锁",
            Lang::En => "§c✖ Locked",
        }
    }

    /// 研究名称提示，如 `研究：基础合成`。
    #[must_use]
    pub fn research_of(&self, name: &str) -> String {
        match self {
            Lang::Zh => format!("§7研究：§f{name}"),
            Lang::En => format!("§7Research: §f{name}"),
        }
    }

    /// 配方类型提示。
    #[must_use]
    pub fn recipe_type_of(&self, kind: &str) -> String {
        match self {
            Lang::Zh => format!("§7配方类型：§f{kind}"),
            Lang::En => format!("§7Recipe type: §f{kind}"),
        }
    }

    // ---- 机器 ----

    /// 机器配方列表标题。
    #[must_use]
    pub fn machine_title(&self, machine: &str) -> String {
        match self {
            Lang::Zh => format!("§8{machine} · 配方"),
            Lang::En => format!("§8{machine} · Recipes"),
        }
    }

    /// 输入 → 输出 的箭头说明。
    #[must_use]
    pub fn recipe_pairs_hint(&self) -> &'static str {
        match self {
            Lang::Zh => "§7左侧为输入，右侧为输出（每两格为一组配方）",
            Lang::En => "§7Left = input, right = output (two slots per recipe)",
        }
    }

    /// 手持物品不匹配任何配方。
    #[must_use]
    pub fn machine_no_recipe(&self, machine: &str) -> String {
        match self {
            Lang::Zh => format!("§e{machine}：该物品不匹配任何配方"),
            Lang::En => format!("§e{machine}: that item matches no recipe"),
        }
    }

    /// 合成成功。
    #[must_use]
    pub fn crafted(&self, output: &str) -> String {
        match self {
            Lang::Zh => format!("§a✔ 合成成功：§f{output}"),
            Lang::En => format!("§a✔ Crafted: §f{output}"),
        }
    }

    /// 强化工作台合成失败：缺少材料。
    #[must_use]
    pub fn missing_materials(&self) -> &'static str {
        match self {
            Lang::Zh => "§c材料不足",
            Lang::En => "§cNot enough materials",
        }
    }

    /// 背包已满。
    #[must_use]
    pub fn inventory_full(&self) -> &'static str {
        match self {
            Lang::Zh => "§c背包已满",
            Lang::En => "§cInventory full",
        }
    }

    /// 空手右键打开机器配方列表的提示。
    #[must_use]
    pub fn machine_hint(&self, machine: &str) -> String {
        match self {
            Lang::Zh => format!("§7{machine}：空手右键查看配方，手持原料右键直接加工"),
            Lang::En => format!("§7{machine}: right-click empty-handed to view recipes, or with an ingredient to process"),
        }
    }

    // ---- 命令 ----

    /// 仅玩家可执行。
    #[must_use]
    pub fn player_only(&self) -> &'static str {
        match self {
            Lang::Zh => "§c该命令只能由玩家执行",
            Lang::En => "§cThis command can only be run by a player",
        }
    }

    /// 物品未知。
    #[must_use]
    pub fn unknown_item(&self, id: &str) -> String {
        match self {
            Lang::Zh => format!("§c未知物品 id：{id}"),
            Lang::En => format!("§cUnknown item id: {id}"),
        }
    }

    /// 已给予物品。
    #[must_use]
    pub fn given(&self, amount: u32, name: &str) -> String {
        match self {
            Lang::Zh => format!("§a已给予 {amount} × §f{name}"),
            Lang::En => format!("§aGave {amount} × §f{name}"),
        }
    }

    /// 研究列表头。
    #[must_use]
    pub fn research_list_header(&self) -> &'static str {
        match self {
            Lang::Zh => "§b— 研究列表（/sf research unlock <key> 解锁）—",
            Lang::En => "§b— Researches (/sf research unlock <key>) —",
        }
    }

    /// 一条研究：名称 + 花费 + 状态。
    #[must_use]
    pub fn research_line(&self, key: &str, name: &str, cost: u32, unlocked: bool) -> String {
        let status = match (self, unlocked) {
            (Lang::Zh, true) => "§a已解锁",
            (Lang::Zh, false) => "§7未解锁",
            (Lang::En, true) => "§aunlocked",
            (Lang::En, false) => "§7locked",
        };
        match self {
            Lang::Zh => format!("§f{name} §8({key}) §7- 花费 {cost} 级经验 - {status}"),
            Lang::En => format!("§f{name} §8({key}) §7- costs {cost} levels - {status}"),
        }
    }

    /// 解锁成功。
    #[must_use]
    pub fn research_unlocked(&self, name: &str) -> String {
        match self {
            Lang::Zh => format!("§a✔ 已解锁研究：§f{name}"),
            Lang::En => format!("§a✔ Unlocked research: §f{name}"),
        }
    }

    /// 已经解锁。
    #[must_use]
    pub fn research_already(&self) -> &'static str {
        match self {
            Lang::Zh => "§e该研究已经解锁",
            Lang::En => "§eThat research is already unlocked",
        }
    }

    /// 经验不足。
    #[must_use]
    pub fn research_need_levels(&self, cost: u32) -> String {
        match self {
            Lang::Zh => format!("§c经验等级不足，需要 {cost} 级"),
            Lang::En => format!("§cNot enough experience levels, need {cost}"),
        }
    }

    /// 研究未知。
    #[must_use]
    pub fn research_unknown(&self, key: &str) -> String {
        match self {
            Lang::Zh => format!("§c未知研究 key：{key}"),
            Lang::En => format!("§cUnknown research key: {key}"),
        }
    }

    /// 个人统计头。
    #[must_use]
    pub fn stats_header(&self, name: &str) -> String {
        match self {
            Lang::Zh => format!("§b— {name} 的粘液科技统计 —"),
            Lang::En => format!("§b— Slimefun stats for {name} —"),
        }
    }

    /// 统计行。
    #[must_use]
    pub fn stats_line(&self, unlocked: usize, total: usize) -> String {
        match self {
            Lang::Zh => format!("§7已解锁研究：§f{unlocked}§7 / {total}"),
            Lang::En => format!("§7Unlocked researches: §f{unlocked}§7 / {total}"),
        }
    }

    /// 帮助头部。
    #[must_use]
    pub fn help_header(&self) -> &'static str {
        match self {
            Lang::Zh => "§b— 粘液科技 帮助 —",
            Lang::En => "§b— Slimefun Help —",
        }
    }

    /// 帮助里的命令说明。
    #[must_use]
    pub fn help_line(&self, cmd: &str) -> &'static str {
        // 命令字面量不翻译，说明按语言返回。
        match (self, cmd) {
            (Lang::Zh, "/sf guide") => "§7打开指南书",
            (Lang::Zh, "/sf give <id> [数量]") => "§7给自己一份粘液科技物品",
            (Lang::Zh, "/sf research") => "§7列出全部研究",
            (Lang::Zh, "/sf research unlock <key>") => "§7消耗经验等级解锁研究",
            (Lang::Zh, "/sf stats") => "§7查看个人统计",
            (Lang::Zh, "/sf reload") => "§7重载配置",
            (Lang::En, "/sf guide") => "§7Open the guide book",
            (Lang::En, "/sf give <id> [amount]") => "§7Give yourself a Slimefun item",
            (Lang::En, "/sf research") => "§7List all researches",
            (Lang::En, "/sf research unlock <key>") => "§7Unlock a research with XP levels",
            (Lang::En, "/sf stats") => "§7Show your personal stats",
            (Lang::En, "/sf reload") => "§7Reload configuration",
            _ => "§7...",
        }
    }

    /// 重载完成。
    #[must_use]
    pub fn reloaded(&self) -> &'static str {
        match self {
            Lang::Zh => "§a✔ 配置已重载",
            Lang::En => "§a✔ Configuration reloaded",
        }
    }

    /// 收到了指南书。
    #[must_use]
    pub fn guide_received(&self) -> &'static str {
        match self {
            Lang::Zh => "§a你收到了粘液科技指南书，右键可打开",
            Lang::En => "§aYou received the Slimefun Guide. Right-click to open it.",
        }
    }

    /// 研究解锁后物品才能合成。
    #[must_use]
    pub fn craft_locked(&self) -> &'static str {
        match self {
            Lang::Zh => "§c该物品所属研究尚未解锁",
            Lang::En => "§cThe research for this item is not unlocked yet",
        }
    }
}
