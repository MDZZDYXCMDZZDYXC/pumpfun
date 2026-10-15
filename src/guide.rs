//! 指南书图形界面与机器配方界面。
//!
//! 所有界面都基于原版 `generic_9x6` 容器（Pumpkin 只提供有限的几种原生容器类型，
//! 且不提供 GUI 实例 id），点击一律通过全局的 *inventory-click* 事件处理，
//! 并用「每位玩家当前的界面状态」来判定该次点击发生在哪个界面、哪个按钮上。

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use pumpkin_plugin_api::gui::Gui;
use pumpkin_plugin_api::{ItemStack, ItemStackExt, Player, Screen};

use crate::groups::{GroupId, GROUPS};
use crate::item::{self, ItemDef};
use crate::lang::Lang;
use crate::machines::{self, Machine};
use crate::research;
use crate::recipes::{self, Shaped};
use crate::storage;
use crate::text_util::t;

/// 每页物品数量。
const PAGE_SIZE: usize = 45;
/// 关闭 / 返回按钮槽位。
const SLOT_BACK: u32 = 49;
/// 上一页按钮槽位。
const SLOT_PREV: u32 = 45;
/// 下一页按钮槽位。
const SLOT_NEXT: u32 = 53;
/// 主菜单里第一个分类的槽位。
const SLOT_GROUP0: u32 = 10;
/// 配方界面里放大显示产物的槽位。
const SLOT_PREVIEW: u32 = 4;
/// 机器配方列表每页条数（每条占用 3 格）。
const MACHINE_PER_PAGE: usize = 15;

/// 当前正在浏览的界面。
#[derive(Clone)]
pub enum Ui {
    /// 主菜单（分类列表）。
    Main,
    /// 某个分类的物品列表。
    Group {
        /// 分类。
        group: GroupId,
        /// 页码（从 0 开始）。
        page: usize,
    },
    /// 某件物品的配方界面。
    Recipe {
        /// 物品 id。
        item: String,
        /// 返回时回到的分类。
        from_group: GroupId,
        /// 返回时回到的页码。
        from_page: usize,
    },
    /// 一台机器的配方列表。
    Machine {
        /// 机器 id。
        id: &'static str,
        /// 页码。
        page: usize,
    },
    /// 强化工作台的形状合成列表。
    Craft {
        /// 页码。
        page: usize,
    },
}

/// 每位玩家的界面状态。
static UIS: LazyLock<Mutex<HashMap<String, Ui>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn set_ui(uuid: &str, ui: Ui) {
    UIS.lock().unwrap_or_else(|e| e.into_inner()).insert(uuid.to_string(), ui);
}

/// 读取玩家当前界面状态。
#[must_use]
pub fn current(uuid: &str) -> Option<Ui> {
    UIS.lock().unwrap_or_else(|e| e.into_inner()).get(uuid).cloned()
}

/// 清除玩家界面状态。
pub fn clear(uuid: &str) {
    UIS.lock().unwrap_or_else(|e| e.into_inner()).remove(uuid);
}

/// 构造一个带名字与描述的展示物品。
fn button(base: &str, name: &str, lore: &[&str]) -> ItemStack {
    let stack = ItemStack::of(base, 1);
    stack.set_custom_name(Some(t(name)));
    if !lore.is_empty() {
        stack.set_lore(lore.iter().map(|l| t(l)).collect());
    }
    stack
}

/// 灰色填充。
fn filler() -> ItemStack {
    button("minecraft:gray_stained_glass_pane", "§8·", &[])
}

// ---------------------------------------------------------------------------
// 打开各类界面
// ---------------------------------------------------------------------------

/// 打开指南主菜单。
pub fn open_main(player: &Player, lang: Lang) {
    let gui = Gui::new(Screen::Generic9x6, t(lang.guide_title()));
    fill(&gui);
    for (i, group) in GROUPS.iter().enumerate() {
        let slot = SLOT_GROUP0 + u32::try_from(i).unwrap_or(0);
        let icon = button(
            group.icon_base(),
            group.name(lang),
            &[lang.click_view()],
        );
        gui.set_item(slot, icon);
    }
    gui.set_item(SLOT_BACK, button("minecraft:barrier", lang.close(), &[]));
    gui.set_allow_grab_items(false);
    gui.set_allow_put_items(false);
    let uuid = player.get_id().to_string();
    set_ui(&uuid, Ui::Main);
    player.open_gui(gui);
}

/// 打开某个分类。
pub fn open_group(player: &Player, lang: Lang, group: GroupId, page: usize) {
    let all = item::in_group(group);
    let total_pages = all.len().div_ceil(PAGE_SIZE).max(1);
    let page = page.min(total_pages - 1);

    let title = format!(
        "{} {} {}",
        lang.category_title(),
        group.name(lang),
        lang.page_indicator(page + 1, total_pages)
    );
    let gui = Gui::new(Screen::Generic9x6, t(&title));
    fill(&gui);

    let uuid = player.get_id().to_string();
    for (i, def) in all.iter().skip(page * PAGE_SIZE).take(PAGE_SIZE).enumerate() {
        let slot = u32::try_from(i).unwrap_or(0);
        gui.set_item(slot, item_button(def, lang, &uuid));
    }

    if page > 0 {
        gui.set_item(SLOT_PREV, button("minecraft:arrow", lang.prev_page(), &[]));
    }
    if page + 1 < total_pages {
        gui.set_item(SLOT_NEXT, button("minecraft:arrow", lang.next_page(), &[]));
    }
    gui.set_item(SLOT_BACK, button("minecraft:barrier", lang.back(), &[]));
    gui.set_allow_grab_items(false);
    gui.set_allow_put_items(false);
    set_ui(&uuid, Ui::Group { group, page });
    player.open_gui(gui);
}

/// 物品列表里的单个物品按钮（未解锁时标注）。
fn item_button(def: &ItemDef, lang: Lang, uuid: &str) -> ItemStack {
    let mut lore: Vec<String> = Vec::new();
    match def.research {
        Some(key) if !storage::is_unlocked(uuid, key) => {
            let name = research::find(key).map_or(key, |r| research::name(r, lang));
            lore.push(lang.research_of(name));
            lore.push(lang.locked().to_string());
        }
        _ => lore.push(lang.click_view().to_string()),
    }
    let stack = item::build_def(def, lang, 1);
    let mut full = vec![t(item::name_of(def, lang))];
    full.extend(lore.iter().map(|l| t(l)));
    stack.set_lore(full);
    stack
}

/// 打开某件物品的配方界面。
pub fn open_recipe(player: &Player, lang: Lang, id: &str, from_group: GroupId, from_page: usize) {
    let Some(def) = item::find(id) else {
        return;
    };
    let gui = Gui::new(Screen::Generic9x6, t(lang.recipe_title()));
    fill(&gui);
    gui.set_item(SLOT_PREVIEW, item::build_def(def, lang, 1));

    if let Some(m) = machines::find(id) {
        // 机器方块：显示结构说明与配方对。
        render_machine_recipes(&gui, lang, m, 0);
    } else if let Some(s) = shaped_for(id) {
        render_shaped(&gui, lang, s);
    } else {
        gui.set_item(22, button("minecraft:knowledge_book", &lang.recipe_type_of("-"), &[]));
    }

    gui.set_item(SLOT_BACK, button("minecraft:barrier", lang.back(), &[]));
    gui.set_allow_grab_items(false);
    gui.set_allow_put_items(false);
    let uuid = player.get_id().to_string();
    set_ui(
        &uuid,
        Ui::Recipe {
            item: id.to_string(),
            from_group,
            from_page,
        },
    );
    player.open_gui(gui);
}

/// 打开一台机器的配方列表。
pub fn open_machine(player: &Player, lang: Lang, m: &'static Machine, page: usize) {
    let name = item::find(m.id).map_or(m.id, |d| item::name_of(d, lang));
    let total_pages = m.recipes.len().div_ceil(MACHINE_PER_PAGE).max(1);
    let page = page.min(total_pages - 1);
    let title = format!(
        "{} {}",
        lang.machine_title(name),
        lang.page_indicator(page + 1, total_pages)
    );
    let gui = Gui::new(Screen::Generic9x6, t(&title));
    fill(&gui);
    render_machine_recipes(&gui, lang, m, page);
    if page > 0 {
        gui.set_item(SLOT_PREV, button("minecraft:arrow", lang.prev_page(), &[]));
    }
    if page + 1 < total_pages {
        gui.set_item(SLOT_NEXT, button("minecraft:arrow", lang.next_page(), &[]));
    }
    gui.set_item(SLOT_BACK, button("minecraft:barrier", lang.close(), &[]));
    gui.set_allow_grab_items(false);
    gui.set_allow_put_items(false);
    let uuid = player.get_id().to_string();
    set_ui(&uuid, Ui::Machine { id: m.id, page });
    player.open_gui(gui);
}

/// 打开强化工作台（形状合成）界面。
pub fn open_craft(player: &Player, lang: Lang, page: usize) {
    let total_pages = recipes::SHAPED.len().div_ceil(PAGE_SIZE).max(1);
    let page = page.min(total_pages - 1);
    let title = format!(
        "{} {}",
        lang.category_title(),
        lang.page_indicator(page + 1, total_pages)
    );
    let gui = Gui::new(Screen::Generic9x6, t(&title));
    fill(&gui);

    let uuid = player.get_id().to_string();
    for (i, s) in recipes::SHAPED.iter().skip(page * PAGE_SIZE).take(PAGE_SIZE).enumerate() {
        let slot = u32::try_from(i).unwrap_or(0);
        gui.set_item(slot, shaped_button(s, lang, &uuid));
    }
    if page > 0 {
        gui.set_item(SLOT_PREV, button("minecraft:arrow", lang.prev_page(), &[]));
    }
    if page + 1 < total_pages {
        gui.set_item(SLOT_NEXT, button("minecraft:arrow", lang.next_page(), &[]));
    }
    gui.set_item(SLOT_BACK, button("minecraft:barrier", lang.close(), &[]));
    gui.set_allow_grab_items(false);
    gui.set_allow_put_items(false);
    set_ui(&uuid, Ui::Craft { page });
    player.open_gui(gui);
}

/// 形状配方列表里的一个按钮。
fn shaped_button(s: &Shaped, lang: Lang, uuid: &str) -> ItemStack {
    let mut lore: Vec<String> = Vec::new();
    // 材料清单。
    let reqs = recipes::shaped_requirements(s);
    let mut line = String::from("§7材料： ");
    for (idx, (tok, n)) in reqs.iter().enumerate() {
        if idx > 0 {
            line.push_str("§8, ");
        }
        let token_name = token_display_name(tok, lang);
        line.push_str(&format!("§f{token_name}§8×{n}"));
    }
    lore.push(line);
    match s.research {
        Some(key) if !storage::is_unlocked(uuid, key) => {
            let name = research::find(key).map_or(key, |r| research::name(r, lang));
            lore.push(lang.research_of(name));
            lore.push(lang.locked().to_string());
        }
        _ => lore.push(lang.click_craft().to_string()),
    }
    let stack = recipes::build_token(s.out, lang, s.out_count)
        .unwrap_or_else(|| ItemStack::of("minecraft:stone", 1));
    let out_name = token_display_name(s.out, lang);
    let mut full = vec![t(&format!("§f{out_name}§8×{}", s.out_count))];
    full.extend(lore.iter().map(|l| t(l)));
    stack.set_lore(full);
    stack
}

/// token 的双语显示名（用于材料/产物提示）。
fn token_display_name(tok: &str, lang: Lang) -> String {
    if let Some(id) = recipes::sf_id(tok) {
        item::find(id).map_or_else(|| id.to_string(), |d| item::name_of(d, lang).to_string())
    } else {
        tok.strip_prefix("minecraft:").unwrap_or(tok).to_string()
    }
}

/// 在 GUI 中渲染一台机器的配方对（输入 → 输出）。
fn render_machine_recipes(gui: &Gui, lang: Lang, m: &'static Machine, page: usize) {
    if m.recipes.is_empty() {
        gui.set_item(22, button("minecraft:knowledge_book", &lang.machine_hint(m.id), &[lang.recipe_pairs_hint()]));
        return;
    }
    for (i, r) in m
        .recipes
        .iter()
        .skip(page * MACHINE_PER_PAGE)
        .take(MACHINE_PER_PAGE)
        .enumerate()
    {
        let row = u32::try_from(i / 3).unwrap_or(0);
        let col = u32::try_from(i % 3).unwrap_or(0);
        let in_slot = row * 9 + col * 3;
        let out_slot = in_slot + 1;
        if let Some(stack) = recipes::build_token(r.input, lang, r.in_count) {
            gui.set_item(in_slot, stack);
        }
        gui.set_item(out_slot, button("minecraft:arrow", "§8→", &[]));
        if let Some(stack) = recipes::build_token(r.out, lang, r.out_count) {
            gui.set_item(out_slot + 1, stack);
        }
    }
}

/// 在 GUI 中渲染一份形状配方（3×3 图案 + 产物）。
fn render_shaped(gui: &Gui, lang: Lang, s: &Shaped) {
    // 3×3 材料区：槽位 10..12 / 19..21 / 28..30
    const GRID: [u32; 9] = [10, 11, 12, 19, 20, 21, 28, 29, 30];
    let mut idx = 0usize;
    for row in s.rows {
        for ch in row.chars() {
            let slot = GRID[idx];
            idx += 1;
            if ch == ' ' {
                continue;
            }
            if let Some((_, tok)) = s.keys.iter().find(|(c, _)| *c == ch)
                && let Some(stack) = recipes::build_token(tok, lang, 1)
            {
                gui.set_item(slot, stack);
            }
        }
    }
    gui.set_item(24, button("minecraft:arrow", "§8→", &[]));
    if let Some(stack) = recipes::build_token(s.out, lang, s.out_count) {
        gui.set_item(25, stack);
    }
    let _ = lang;
}

/// 用灰色填充空白槽位。
fn fill(gui: &Gui) {
    for slot in 0..54u32 {
        gui.set_item(slot, filler());
    }
}

/// 查找以某物品为产物的形状配方。
#[must_use]
pub fn shaped_for(item_id: &str) -> Option<&'static Shaped> {
    recipes::SHAPED.iter().find(|s| s.out == format!("sf:{item_id}"))
}

// ---------------------------------------------------------------------------
// 点击处理
// ---------------------------------------------------------------------------

/// 处理一次 GUI 点击。返回 `true` 表示本次点击已被本插件消费（应取消原版行为）。
pub fn on_click(player: &Player, lang: Lang, slot: u32) -> bool {
    let uuid = player.get_id().to_string();
    let Some(ui) = current(&uuid) else {
        return false;
    };

    match ui {
        Ui::Main => {
            if slot == SLOT_BACK {
                clear(&uuid);
                return true;
            }
            if let Some(idx) = slot.checked_sub(SLOT_GROUP0)
                && let Some(group) = GROUPS.get(idx as usize)
            {
                open_group(player, lang, *group, 0);
                return true;
            }
            true
        }
        Ui::Group { group, page } => {
            match slot {
                SLOT_BACK => {
                    open_main(player, lang);
                }
                SLOT_PREV => {
                    open_group(player, lang, group, page.saturating_sub(1));
                }
                SLOT_NEXT => {
                    open_group(player, lang, group, page + 1);
                }
                s if s < PAGE_SIZE as u32 => {
                    let idx = page * PAGE_SIZE + s as usize;
                    if let Some(def) = item::in_group(group).get(idx) {
                        let id = def.id.to_string();
                        open_recipe(player, lang, &id, group, page);
                    }
                }
                _ => {}
            }
            true
        }
        Ui::Recipe {
            item,
            from_group,
            from_page,
        } => {
            if slot == SLOT_BACK {
                open_group(player, lang, from_group, from_page);
            }
            let _ = item;
            true
        }
        Ui::Machine { id, page } => {
            match slot {
                SLOT_BACK => {
                    clear(&uuid);
                }
                SLOT_PREV => {
                    if let Some(m) = machines::find(id) {
                        open_machine(player, lang, m, page.saturating_sub(1));
                    }
                }
                SLOT_NEXT => {
                    if let Some(m) = machines::find(id) {
                        open_machine(player, lang, m, page + 1);
                    }
                }
                _ => {}
            }
            true
        }
        Ui::Craft { page } => {
            match slot {
                SLOT_BACK => {
                    clear(&uuid);
                }
                SLOT_PREV => {
                    open_craft(player, lang, page.saturating_sub(1));
                }
                SLOT_NEXT => {
                    open_craft(player, lang, page + 1);
                }
                s if s < PAGE_SIZE as u32 => {
                    let idx = page * PAGE_SIZE + s as usize;
                    if let Some(shaped) = recipes::SHAPED.get(idx) {
                        craft_shaped(player, lang, shaped);
                        open_craft(player, lang, page);
                    }
                }
                _ => {}
            }
            true
        }
    }
}

/// 尝试在强化工作台里合成一份形状配方。
pub fn craft_shaped(player: &Player, lang: Lang, s: &Shaped) {
    let uuid = player.get_id().to_string();

    // 研究门槛。
    if let Some(key) = s.research
        && !storage::is_unlocked(&uuid, key)
    {
        player.send_system_message(t(lang.craft_locked()), false);
        return;
    }

    let inv = player.get_inventory().as_inventory();
    if !s.can_craft(&inv) {
        player.send_system_message(t(lang.missing_materials()), false);
        return;
    }

    let Some(out) = recipes::build_token(s.out, lang, s.out_count) else {
        return;
    };
    // 先扣材料，再给产物；若背包放不下则把材料退回比较麻烦，因此先确认有空间。
    if !has_room_for(&inv, &out) {
        player.send_system_message(t(lang.inventory_full()), false);
        return;
    }
    s.consume(&inv);
    let _ = recipes::give_to_inventory(&inv, out);
    let name = token_display_name(s.out, lang);
    player.send_system_message(t(&lang.crafted(&name)), false);
}

/// 判断背包中是否还能容纳一份物品（同类堆叠或存在空位）。
#[must_use]
pub fn has_room_for(inv: &pumpkin_plugin_api::inventory::Inventory, stack: &ItemStack) -> bool {
    for slot in 0..inv.get_size() {
        match inv.get_item(slot) {
            None => return true,
            Some(existing) => {
                if recipes::same_kind(&existing, stack) {
                    let max = u32::from(existing.get_max_count());
                    let cur = u32::from(existing.get_count());
                    if cur + u32::from(stack.get_count()) <= max {
                        return true;
                    }
                }
            }
        }
    }
    false
}
