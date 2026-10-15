//! 玩家数据持久化。
//!
//! 数据存在插件数据目录里的 `players.tsv`，每行一位玩家：
//!
//! ```text
//! <uuid>    <name>    <research_key,research_key,...>
//! ```

use std::collections::{BTreeSet, HashMap};
use std::sync::{LazyLock, Mutex, MutexGuard};

/// 一位玩家的持久化数据。
#[derive(Clone, Default)]
pub struct PlayerData {
    /// 最后见到的玩家名（仅用于可读性）。
    pub name: String,
    /// 已解锁的研究 key。
    pub unlocked: BTreeSet<String>,
}

/// 全服数据。
#[derive(Default)]
pub struct Store {
    /// uuid → 数据。
    pub players: HashMap<String, PlayerData>,
}

/// 内存中的全服数据。
static STORE: LazyLock<Mutex<Store>> = LazyLock::new(|| Mutex::new(Store { players: HashMap::new() }));

/// 数据文件路径（`on_load` 时写入）。
static DATA_FILE: Mutex<Option<String>> = Mutex::new(None);

/// 加锁访问全服数据。
pub fn store() -> MutexGuard<'static, Store> {
    STORE.lock().unwrap_or_else(|e| e.into_inner())
}

/// 记录数据文件路径并从磁盘载入。
pub fn init(path: String) {
    *DATA_FILE.lock().unwrap_or_else(|e| e.into_inner()) = Some(path.clone());
    load_from(&path);
}

/// 从磁盘载入。
fn load_from(path: &str) {
    let Ok(content) = std::fs::read_to_string(path) else {
        return;
    };
    let mut s = store();
    for line in content.lines() {
        let mut parts = line.split('\t');
        let (Some(uuid), Some(name), Some(keys)) = (parts.next(), parts.next(), parts.next()) else {
            continue;
        };
        if uuid.is_empty() {
            continue;
        }
        let mut data = PlayerData {
            name: name.to_string(),
            ..PlayerData::default()
        };
        for k in keys.split(',').filter(|k| !k.is_empty()) {
            data.unlocked.insert(k.to_string());
        }
        s.players.insert(uuid.to_string(), data);
    }
}

/// 写回磁盘。
pub fn save() {
    let Some(path) = DATA_FILE
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone()
    else {
        return;
    };
    let s = store();
    let mut out = String::new();
    for (uuid, data) in &s.players {
        let keys: Vec<&str> = data.unlocked.iter().map(String::as_str).collect();
        out.push_str(uuid);
        out.push('\t');
        out.push_str(&data.name);
        out.push('\t');
        out.push_str(&keys.join(","));
        out.push('\n');
    }
    if let Err(err) = std::fs::write(&path, out) {
        tracing::warn!("pumpfun: failed to save player data: {err}");
    }
}

/// 确保玩家条目存在并更新名字。
pub fn touch(uuid: &str, name: &str) {
    let mut s = store();
    let entry = s.players.entry(uuid.to_string()).or_default();
    if entry.name != name {
        entry.name = name.to_string();
    }
}

/// 是否已解锁某项研究。
#[must_use]
pub fn is_unlocked(uuid: &str, key: &str) -> bool {
    store()
        .players
        .get(uuid)
        .is_some_and(|d| d.unlocked.contains(key))
}

/// 解锁一项研究，返回是否发生了变化。
pub fn unlock(uuid: &str, key: &str) -> bool {
    store()
        .players
        .entry(uuid.to_string())
        .or_default()
        .unlocked
        .insert(key.to_string())
}

/// 已解锁研究的数量。
#[must_use]
pub fn unlocked_count(uuid: &str) -> usize {
    store()
        .players
        .get(uuid)
        .map_or(0, |d| d.unlocked.len())
}
