//! # pumpfun — 面向 Pumpkin 服务器的「粘液科技」移植插件
//!
//! 参考 `two048` 的工程结构与写法，把 Slimefun 4 的**核心玩法**用 Rust 重写为
//! Pumpkin 的 WebAssembly 插件：物品注册表与分类、指南书图形界面、
//! 多方块机器与配方、研究解锁，以及 `/slimefun` 主命令。
//!
//! Pumpkin 插件 API 的若干限制决定了实现方式：
//!
//! * **不能注册自定义方块 / 方块实体** → 机器用原版方块搭出的结构模拟，右键触发；
//! * **没有按方块的 tick 回调** → 只做「交互触发」的机器；
//! * **没有自定义容器类型** → 指南与机器界面都用 `generic_9x6`，
//!   点击靠全局 *inventory-click* 事件 + 每位玩家的界面状态处理；
//! * **物品恒为原版物品 + NBT** → 用 `pumpfun:id` 标记区分，作为跨语言身份。

mod command;
mod groups;
mod guide;
mod item;
mod lang;
mod machines;
mod recipes;
mod research;
mod storage;
mod text_util;

use pumpkin_plugin_api::common::{BlockPos, Hand};
use pumpkin_plugin_api::events::{EventPriority, EventHandler, FromIntoEvent, InteractAction};
use pumpkin_plugin_api::permission::{Permission, PermissionDefault};
use pumpkin_plugin_api::permissions::{FS_READ_DATA, FS_WRITE_DATA};
use pumpkin_plugin_api::recipe::{Ingredient, ShapedRecipeBuilder};
use pumpkin_plugin_api::wit::pumpkin::plugin::event::{
    Event, EventType, InventoryClickEventData, InventoryCloseEventData, PlayerInteractEventData,
    PlayerJoinEventData,
};
use pumpkin_plugin_api::wit::pumpkin::plugin::particles::Particle;
use pumpkin_plugin_api::wit::pumpkin::plugin::sounds::{Sound, SoundCategory};
use pumpkin_plugin_api::{
    Context, ItemStack, Player, Plugin, PluginMetadata, Screen, Server,
    register_plugin,
};

use lang::Lang;
use text_util::t;

/// 玩家数据文件名（位于插件数据目录内）。
const DATA_FILE: &str = "players.tsv";

/// 插件本体。
struct Pumpfun;

impl Plugin for Pumpfun {
    fn new() -> Self {
        Pumpfun
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "pumpfun".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["WorkBuddy".into()],
            description: "粘液科技(Slimefun)的 Pumpkin 移植 · Slimefun ported to Pumpkin".into(),
            dependencies: Vec::new(),
            permissions: vec![FS_READ_DATA.to_string(), FS_WRITE_DATA.to_string()],
        }
    }

    fn on_load(&self, context: Context) -> pumpkin_plugin_api::Result<()> {
        context.register_permission(&Permission {
            node: command::PERM_NODE.to_string(),
            description: "使用粘液科技命令".to_string(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        })?;

        // 载入玩家数据。
        let path = format!("{}/{DATA_FILE}", context.get_data_folder());
        storage::init(path);

        // 把「只用原版材料」的形状配方额外注册为原版工作台配方，解决起步问题。
        register_vanilla_recipes(&context);

        // 注册命令树。
        context.register_command(command::build(), command::PERM_NODE);

        // 注册事件。
        context.register_event_handler(JoinHandler, EventPriority::Normal, false)?;
        context.register_event_handler(InteractHandler, EventPriority::Normal, true)?;
        context.register_event_handler(ClickHandler, EventPriority::Normal, true)?;
        context.register_event_handler(CloseHandler, EventPriority::Normal, false)?;

        tracing::info!("pumpfun 已加载：/sf guide 打开指南，/sf give <id> 获取物品");
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        storage::save();
        tracing::info!("pumpfun 已卸载，玩家数据已保存");
        Ok(())
    }
}

register_plugin!(Pumpfun);

// ---------------------------------------------------------------------------
// 原版配方注册（起步引导）
// ---------------------------------------------------------------------------

/// 把「只用原版材料」的形状配方注册为原版工作台配方。
///
/// 否则玩家必须先有一个强化工作台才能合成强化工作台——按 Slimefun 原版的引导逻辑，
/// 这一步本应由工作台完成，因此这里把这类配方交还给原版工作台。
fn register_vanilla_recipes(context: &Context) {
    let manager = context.get_recipe_manager();
    for s in recipes::SHAPED {
        if !s.is_vanilla_only() {
            continue;
        }
        let Some(out) = recipes::build_token(s.out, Lang::En, s.out_count) else {
            continue;
        };
        let mut builder = ShapedRecipeBuilder::new(format!("pumpfun:{}", s.key), out).pattern(s.rows);
        for (ch, tok) in s.keys {
            builder = builder.key(*ch, Ingredient::item(tok));
        }
        if let Err(err) = builder.register(&manager) {
            tracing::warn!("pumpfun: 注册配方 {} 失败: {err:?}", s.key);
        }
    }
}

// ---------------------------------------------------------------------------
// 事件：玩家加入
// ---------------------------------------------------------------------------

/// 玩家加入：登记数据，首次进入时发放指南书。
struct JoinHandler;
impl EventHandler<PlayerJoinEvent> for JoinHandler {
    fn handle(&self, _server: Server, data: PlayerJoinEventData) -> PlayerJoinEventData {
        let player = &data.player;
        let uuid = player.get_id().to_string();
        let name = player.get_name();
        storage::touch(&uuid, &name);

        let inv = player.get_inventory();
        let storage_inv = inv.as_inventory();
        // 已经拥有指南书则不再发放。
        let mut has_guide = false;
        for slot in 0..storage_inv.get_size() {
            if let Some(stack) = storage_inv.get_item(slot)
                && item::is_guide(&stack)
            {
                has_guide = true;
                break;
            }
        }
        if !has_guide {
            let lang = Lang::from_locale(&player.get_locale());
            let book = item::guide_book(lang);
            if recipes::give_to_inventory(&storage_inv, book) {
                player.send_system_message(t(lang.guide_received()), false);
            }
        }
        data
    }
}

/// 玩家加入事件（API 未提供，自行定义）。
struct PlayerJoinEvent;
impl FromIntoEvent for PlayerJoinEvent {
    const EVENT_TYPE: EventType = EventType::PlayerJoinEvent;
    type Data = PlayerJoinEventData;
    fn data_from_event(event: Event) -> Self::Data {
        match event {
            Event::PlayerJoinEvent(d) => d,
            _ => panic!("unexpected event"),
        }
    }
    fn data_into_event(data: Self::Data) -> Event {
        Event::PlayerJoinEvent(data)
    }
}

/// 玩家互交事件（沿用 API 提供的定义）。
struct PlayerInteractEvent;
impl FromIntoEvent for PlayerInteractEvent {
    const EVENT_TYPE: EventType = EventType::PlayerInteractEvent;
    type Data = PlayerInteractEventData;
    fn data_from_event(event: Event) -> Self::Data {
        match event {
            Event::PlayerInteractEvent(d) => d,
            _ => panic!("unexpected event"),
        }
    }
    fn data_into_event(data: Self::Data) -> Event {
        Event::PlayerInteractEvent(data)
    }
}

/// 库存点击事件（API 未提供单元结构，自行定义）。
struct InventoryClickEvent;
impl FromIntoEvent for InventoryClickEvent {
    const EVENT_TYPE: EventType = EventType::InventoryClickEvent;
    type Data = InventoryClickEventData;
    fn data_from_event(event: Event) -> Self::Data {
        match event {
            Event::InventoryClickEvent(d) => d,
            _ => panic!("unexpected event"),
        }
    }
    fn data_into_event(data: Self::Data) -> Event {
        Event::InventoryClickEvent(data)
    }
}

/// 库存关闭事件（自行定义）。
struct InventoryCloseEvent;
impl FromIntoEvent for InventoryCloseEvent {
    const EVENT_TYPE: EventType = EventType::InventoryCloseEvent;
    type Data = InventoryCloseEventData;
    fn data_from_event(event: Event) -> Self::Data {
        match event {
            Event::InventoryCloseEvent(d) => d,
            _ => panic!("unexpected event"),
        }
    }
    fn data_into_event(data: Self::Data) -> Event {
        Event::InventoryCloseEvent(data)
    }
}

// ---------------------------------------------------------------------------
// 事件：右键方块
// ---------------------------------------------------------------------------

/// 玩家右键方块：打开指南书 / 触发多方块机器。
struct InteractHandler;
impl EventHandler<PlayerInteractEvent> for InteractHandler {
    fn handle(&self, _server: Server, mut data: PlayerInteractEventData) -> PlayerInteractEventData {
        if data.action != InteractAction::RightClickBlock {
            return data;
        }
        let Some(clicked) = data.clicked_pos else {
            return data;
        };
        if on_interact(&data.player, clicked) {
            data.cancelled = true;
        }
        data
    }
}

/// 处理一次右键方块，返回是否消费该次点击。
fn on_interact(player: &Player, clicked: BlockPos) -> bool {
    let lang = Lang::from_locale(&player.get_locale());
    let inv = player.get_inventory();
    let held = inv.get_item_in_hand(Hand::Right);

    // 1) 手持指南书 → 打开指南。
    if let Some(stack) = &held
        && item::is_guide(stack)
    {
        guide::open_main(player, lang);
        return true;
    }

    // 2) 是否是某台机器的结构中心。
    let world = player.get_world();
    let Some(machine) = machines::machine_at(&world, clicked) else {
        return false;
    };

    let holding_item = held.as_ref().is_some_and(|s| !is_air(s));
    if machine.kind == machines::Kind::CraftingTable {
        // 强化工作台：直接打开合成界面。
        guide::open_craft(player, lang, 0);
        if !holding_item {
            player.send_system_message(t(&lang.machine_hint(&machine_name(machine.id, lang))), false);
        }
        return true;
    }

    if holding_item {
        if let Some(stack) = held {
            process_machine(player, lang, machine, stack, clicked);
        }
    } else {
        guide::open_machine(player, lang, machine, 0);
    }
    true
}

/// 一件方块物品是否为空气。
fn is_air(stack: &ItemStack) -> bool {
    let key = stack.get_registry_key();
    key == "minecraft:air" || key == "air"
}

/// 机器名。
fn machine_name(id: &str, lang: Lang) -> String {
    item::find(id).map_or_else(|| id.to_string(), |d| item::name_of(d, lang).to_string())
}

/// 手持原料右键机器时执行加工。
fn process_machine(player: &Player, lang: Lang, machine: &machines::Machine, held: ItemStack, at: BlockPos) {
    let Some(recipe) = machine
        .recipes
        .iter()
        .find(|r| recipes::matches_token(&held, r.input) && u32::from(held.get_count()) >= u32::from(r.in_count))
    else {
        player.send_system_message(t(&lang.machine_no_recipe(&machine_name(machine.id, lang))), false);
        return;
    };

    let inv = player.get_inventory();
    let storage_inv = inv.as_inventory();
    let Some(out) = recipes::build_token(recipe.out, lang, recipe.out_count) else {
        return;
    };
    if !guide::has_room_for(&storage_inv, &out) {
        player.send_system_message(t(lang.inventory_full()), false);
        return;
    }

    // 扣手上原料。
    let remaining = u32::from(held.get_count()) - u32::from(recipe.in_count);
    let new_hand = if remaining == 0 {
        None
    } else {
        held.set_count(u8::try_from(remaining).unwrap_or(0));
        Some(held)
    };
    inv.set_item_in_hand(Hand::Right, new_hand);

    let _ = recipes::give_to_inventory(&storage_inv, out);
    player.send_system_message(t(&lang.crafted(&token_name(recipe.out, lang))), false);

    // 音效与粒子反馈。
    let world = player.get_world();
    let pos = (f64::from(at.x) + 0.5, f64::from(at.y) + 1.0, f64::from(at.z) + 0.5);
    world.play_sound(Sound::BlockAnvilLand, SoundCategory::Blocks, pos, 1.0, 1.0);
    world.spawn_particle(Particle::Crit, pos, (0.2, 0.2, 0.2), 0.05, 8);
}

/// token 的双语名。
fn token_name(tok: &str, lang: Lang) -> String {
    if let Some(id) = recipes::sf_id(tok) {
        machine_name(id, lang)
    } else {
        tok.strip_prefix("minecraft:").unwrap_or(tok).to_string()
    }
}

// ---------------------------------------------------------------------------
// 事件：库存点击 / 关闭
// ---------------------------------------------------------------------------

/// 处理指南 / 机器界面里的点击。
struct ClickHandler;
impl EventHandler<InventoryClickEvent> for ClickHandler {
    fn handle(&self, _server: Server, mut data: InventoryClickEventData) -> InventoryClickEventData {
        let handled = {
            let player = &data.player;
            let uuid = player.get_id().to_string();
            if guide::current(&uuid).is_none() {
                false
            } else if !matches!(data.window_type, Some(Screen::Generic9x6)) {
                // 只处理我们自己的界面。
                false
            } else {
                let slot = u32::try_from(data.slot.max(0)).unwrap_or(0);
                let lang = Lang::from_locale(&player.get_locale());
                guide::on_click(player, lang, slot)
            }
        };
        if handled {
            data.cancelled = true;
        }
        data
    }
}

/// 界面关闭时清掉玩家界面状态。
struct CloseHandler;
impl EventHandler<InventoryCloseEvent> for CloseHandler {
    fn handle(&self, _server: Server, data: InventoryCloseEventData) -> InventoryCloseEventData {
        let uuid = data.player.get_id().to_string();
        guide::clear(&uuid);
        data
    }
}
