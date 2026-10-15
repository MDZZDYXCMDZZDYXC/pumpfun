//! 文本组件构造辅助。
//!
//! WIT 的 `text-component` 是「链式消费」资源：诸如 `add-child` / `color-named` 的方法
//! 会**消耗**自身并返回新的组件，因此调用处必须重新赋值（`msg = msg.add_child(..)`）。

use pumpkin_plugin_api::text::TextComponent;

/// 由字符串构造一个纯文本组件。
#[must_use]
pub fn t(s: &str) -> TextComponent {
    TextComponent::text(s)
}
