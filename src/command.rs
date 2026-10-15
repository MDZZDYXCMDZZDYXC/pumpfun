//! `/slimefun`（别名 `/sf`）命令树。

use pumpkin_plugin_api::command::{
    Arg, ArgumentType, Command, CommandError, CommandNode, CommandSender, ConsumedArgs, StringType,
};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::wit::pumpkin::plugin::command::{Number, PermissionLevel};
use pumpkin_plugin_api::{Player, Server};

use crate::guide;
use crate::item;
use crate::lang::Lang;
use crate::research;
use crate::storage;
use crate::text_util::t;

/// 命令所需的权限节点。
pub const PERM_NODE: &str = "pumpfun.command";

/// 取发送者语言（控制台 / 命令方块回落英文）。
fn lang_of(sender: &CommandSender) -> Lang {
    sender
        .as_player()
        .map_or(Lang::En, |p| Lang::from_locale(&p.get_locale()))
}

/// 取字符串参数。
fn arg_string(args: &ConsumedArgs, key: &str) -> Option<String> {
    match args.get_value(key) {
        Arg::Simple(s) | Arg::Msg(s) => Some(s),
        _ => None,
    }
}

/// 取整数参数。
fn arg_int(args: &ConsumedArgs, key: &str) -> Option<i32> {
    match args.get_value(key) {
        Arg::Num(Ok(Number::Int32(v))) => Some(v),
        Arg::Num(Ok(Number::Int64(v))) => Some(i32::try_from(v).unwrap_or(i32::MAX)),
        Arg::Num(Ok(Number::Float32(v))) => Some(v as i32),
        Arg::Num(Ok(Number::Float64(v))) => Some(v as i32),
        _ => None,
    }
}

/// 要求玩家上下文。
fn require_player(sender: &CommandSender, lang: Lang) -> Result<Player, ()> {
    match sender.as_player() {
        Some(p) => Ok(p),
        None => {
            sender.send_message(t(lang.player_only()));
            Err(())
        }
    }
}

/// 判断是否为管理员（权限等级 >= 2，即游戏管理员 / OP）。
fn is_admin(sender: &CommandSender) -> bool {
    sender.has_permission_level(PermissionLevel::Two)
}

// ---------------------------------------------------------------------------
// 各子命令
// ---------------------------------------------------------------------------

/// 打开指南书。
struct GuideHandler;
impl CommandHandler for GuideHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let lang = lang_of(&sender);
        if let Ok(player) = require_player(&sender, lang) {
            guide::open_main(&player, lang);
        }
        Ok(1)
    }
}

/// 帮助。
struct HelpHandler;
impl CommandHandler for HelpHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let lang = lang_of(&sender);
        let mut msg = t(lang.help_header());
        let commands = [
            "/sf guide",
            "/sf give <id> [数量]",
            "/sf research",
            "/sf research unlock <key>",
            "/sf stats",
            "/sf reload",
        ];
        for cmd in commands {
            msg = msg.add_child(t(&format!("\n§f{cmd} ")));
            msg = msg.add_child(t(lang.help_line(cmd)));
        }
        sender.send_message(msg);
        Ok(1)
    }
}

/// 给予物品（`amount` 是否从参数读取）。
struct GiveHandler(bool);
impl CommandHandler for GiveHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let lang = lang_of(&sender);
        if !is_admin(&sender) {
            sender.send_message(t("§c权限不足"));
            return Ok(0);
        }
        let Ok(player) = require_player(&sender, lang) else {
            return Ok(0);
        };
        let Some(raw) = arg_string(&args, "item") else {
            sender.send_message(t("§c缺少物品 id"));
            return Ok(0);
        };
        let id = raw.to_uppercase();
        let Some(def) = item::find(&id) else {
            sender.send_message(t(&lang.unknown_item(&raw)));
            return Ok(0);
        };
        let amount = if self.0 {
            arg_int(&args, "amount").unwrap_or(1).clamp(1, 64)
        } else {
            1
        };
        let count = u8::try_from(amount).unwrap_or(1);
        let inv = player.get_inventory().as_inventory();
        let stack = item::build_def(def, lang, count);
        if !crate::recipes::give_to_inventory(&inv, stack) {
            sender.send_message(t(lang.inventory_full()));
            return Ok(0);
        }
        sender.send_message(t(&lang.given(u32::from(count), item::name_of(def, lang))));
        Ok(1)
    }
}

/// 列出研究。
struct ResearchListHandler;
impl CommandHandler for ResearchListHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let lang = lang_of(&sender);
        let uuid = sender.as_player().map(|p| p.get_id().to_string());
        let mut msg = t(lang.research_list_header());
        for r in research::RESEARCHES {
            let unlocked = uuid.as_deref().is_some_and(|u| storage::is_unlocked(u, r.key));
            let line = lang.research_line(r.key, research::name(r, lang), r.cost, unlocked);
            msg = msg.add_child(t(&format!("\n{line}")));
        }
        sender.send_message(msg);
        Ok(1)
    }
}

/// 解锁研究。
struct ResearchUnlockHandler;
impl CommandHandler for ResearchUnlockHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let lang = lang_of(&sender);
        let Ok(player) = require_player(&sender, lang) else {
            return Ok(0);
        };
        let Some(key) = arg_string(&args, "key") else {
            sender.send_message(t("§c缺少研究 key"));
            return Ok(0);
        };
        let Some(r) = research::find(&key) else {
            sender.send_message(t(&lang.research_unknown(&key)));
            return Ok(0);
        };
        let uuid = player.get_id().to_string();
        if storage::is_unlocked(&uuid, r.key) {
            sender.send_message(t(lang.research_already()));
            return Ok(0);
        }
        let level = player.get_experience_level();
        if level < i32::try_from(r.cost).unwrap_or(i32::MAX) {
            sender.send_message(t(&lang.research_need_levels(r.cost)));
            return Ok(0);
        }
        player.set_experience_level(level - i32::try_from(r.cost).unwrap_or(0));
        storage::unlock(&uuid, r.key);
        storage::save();
        sender.send_message(t(&lang.research_unlocked(research::name(r, lang))));
        Ok(1)
    }
}

/// 个人统计。
struct StatsHandler;
impl CommandHandler for StatsHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let lang = lang_of(&sender);
        let Ok(player) = require_player(&sender, lang) else {
            return Ok(0);
        };
        let uuid = player.get_id().to_string();
        let unlocked = storage::unlocked_count(&uuid);
        let mut msg = t(&lang.stats_header(&player.get_name()));
        msg = msg.add_child(t(&format!("\n{}", lang.stats_line(unlocked, research::RESEARCHES.len()))));
        sender.send_message(msg);
        Ok(1)
    }
}

/// 重载。
struct ReloadHandler;
impl CommandHandler for ReloadHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        if !is_admin(&sender) {
            sender.send_message(t("§c权限不足"));
            return Ok(0);
        }
        let lang = lang_of(&sender);
        sender.send_message(t(lang.reloaded()));
        Ok(1)
    }
}

/// 构建完整的命令树。
#[must_use]
pub fn build() -> Command {
    let names = ["slimefun".to_string(), "sf".to_string()];
    let desc = "粘液科技主命令 · Slimefun main command";

    Command::new(&names, desc)
        .execute(HelpHandler)
        .then(CommandNode::literal("help").execute(HelpHandler))
        .then(CommandNode::literal("guide").execute(GuideHandler))
        .then(CommandNode::literal("open").execute(GuideHandler))
        .then(
            CommandNode::literal("give")
                .then(
                    CommandNode::argument("item", &ArgumentType::String(StringType::SingleWord))
                        .execute(GiveHandler(false))
                        .then(
                            CommandNode::argument("amount", &ArgumentType::Integer((None, None)))
                                .execute(GiveHandler(true)),
                        ),
                ),
        )
        .then(
            CommandNode::literal("research")
                .execute(ResearchListHandler)
                .then(
                    CommandNode::literal("unlock")
                        .then(
                            CommandNode::argument(
                                "key",
                                &ArgumentType::String(StringType::SingleWord),
                            )
                            .execute(ResearchUnlockHandler),
                        ),
                ),
        )
        .then(CommandNode::literal("stats").execute(StatsHandler))
        .then(CommandNode::literal("reload").execute(ReloadHandler))
}
