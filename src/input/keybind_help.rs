use std::borrow::Cow;

use crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    config::{ActionKeybinds, IndexedKeybind, Keybinds},
    input::TerminalKey,
};

pub(crate) type KeybindHelpEntry = (String, Cow<'static, str>);
pub(crate) type KeybindHelpGroup = (&'static str, Vec<KeybindHelpEntry>);

pub(crate) fn keybind_help_text_char(key: &TerminalKey) -> Option<char> {
    if !key.modifiers.difference(KeyModifiers::SHIFT).is_empty() {
        return None;
    }
    if let Some(character) = key.shifted_codepoint.and_then(char::from_u32) {
        return Some(character);
    }
    let KeyCode::Char(character) = key.code else {
        return None;
    };
    Some(character)
}

fn entry(key: impl Into<String>, label: &'static str) -> KeybindHelpEntry {
    (key.into(), Cow::Borrowed(label))
}

fn binding_label(bindings: &ActionKeybinds) -> String {
    bindings.label().unwrap_or_else(|| "未設定".to_owned())
}

fn indexed_label(bindings: &[IndexedKeybind]) -> String {
    if bindings.is_empty() {
        return "未設定".to_owned();
    }
    let mut parts = Vec::new();
    let mut index = 0;
    while index < bindings.len() {
        if let Some(prefix) = indexed_range_prefix(&bindings[index..]) {
            parts.push(format!("{prefix}1..9"));
            index += 9;
        } else {
            parts.push(bindings[index].label.clone());
            index += 1;
        }
    }
    parts.join(" / ")
}

fn indexed_range_prefix(bindings: &[IndexedKeybind]) -> Option<&str> {
    let run = bindings.get(..9)?;
    let prefix = run[0].label.strip_suffix('1')?;
    for (offset, binding) in run.iter().enumerate() {
        let digit = char::from(b'1' + offset as u8);
        if binding.label.strip_suffix(digit) != Some(prefix) {
            return None;
        }
    }
    Some(prefix)
}

pub(crate) fn keybind_help_groups(
    keybinds: &Keybinds,
    prefixes: &[crate::config::KeyCombo],
) -> Vec<KeybindHelpGroup> {
    let mut groups = vec![
        (
            "全域",
            vec![
                entry(crate::config::format_prefix_combos(prefixes), "前綴模式"),
                entry(binding_label(&keybinds.help), "快捷鍵"),
                entry(binding_label(&keybinds.settings), "設定"),
                entry(binding_label(&keybinds.detach), "中斷連線"),
                entry(binding_label(&keybinds.reload_config), "重新載入設定"),
                entry(
                    binding_label(&keybinds.open_notification_target),
                    "開啟通知目標",
                ),
            ],
        ),
        (
            "導覽",
            vec![
                entry("esc", "返回"),
                entry(
                    format!(
                        "{} / {}",
                        binding_label(&keybinds.navigate.workspace_up),
                        binding_label(&keybinds.navigate.workspace_down)
                    ),
                    "工作區清單",
                ),
                entry(
                    format!(
                        "{} / {} / {} / {} / left / right",
                        binding_label(&keybinds.navigate.pane_left),
                        binding_label(&keybinds.navigate.pane_down),
                        binding_label(&keybinds.navigate.pane_up),
                        binding_label(&keybinds.navigate.pane_right)
                    ),
                    "移動焦點",
                ),
                entry("tab / shift+tab", "切換窗格"),
                entry("enter", "開啟工作區"),
                entry("1..9", "切換工作區"),
            ],
        ),
        (
            "工作區 / 分頁",
            vec![
                entry(
                    binding_label(&keybinds.workspace_picker),
                    "工作區導覽",
                ),
                entry(binding_label(&keybinds.goto), "工作階段導覽器"),
                entry(binding_label(&keybinds.new_workspace), "新增工作區"),
                entry(binding_label(&keybinds.new_worktree), "新增工作樹"),
                entry(binding_label(&keybinds.open_worktree), "開啟工作樹"),
                entry(
                    binding_label(&keybinds.remove_worktree),
                    "刪除工作樹工作目錄",
                ),
                entry(
                    binding_label(&keybinds.rename_workspace),
                    "重新命名工作區",
                ),
                entry(binding_label(&keybinds.close_workspace), "關閉工作區"),
                entry(
                    binding_label(&keybinds.previous_workspace),
                    "上一個工作區",
                ),
                entry(binding_label(&keybinds.next_workspace), "下一個工作區"),
                entry(
                    indexed_label(&keybinds.switch_workspace),
                    "切換工作區 1-9",
                ),
                entry(binding_label(&keybinds.previous_agent), "上一個代理"),
                entry(binding_label(&keybinds.next_agent), "下一個代理"),
                entry(indexed_label(&keybinds.focus_agent), "聚焦代理 1-9"),
                entry(binding_label(&keybinds.new_tab), "新增分頁"),
                entry(binding_label(&keybinds.rename_tab), "重新命名分頁"),
                entry(binding_label(&keybinds.previous_tab), "上一個分頁"),
                entry(binding_label(&keybinds.next_tab), "下一個分頁"),
                entry(binding_label(&keybinds.move_tab_previous), "向左移動分頁"),
                entry(binding_label(&keybinds.move_tab_next), "向右移動分頁"),
                entry(indexed_label(&keybinds.switch_tab), "切換分頁 1-9"),
                entry(binding_label(&keybinds.close_tab), "關閉分頁"),
            ],
        ),
        (
            "窗格",
            vec![
                entry(binding_label(&keybinds.split_vertical), "垂直分割"),
                entry(
                    binding_label(&keybinds.split_horizontal),
                    "水平分割",
                ),
                entry(binding_label(&keybinds.close_pane), "關閉窗格"),
                entry(binding_label(&keybinds.rename_pane), "重新命名窗格"),
                entry(binding_label(&keybinds.edit_scrollback), "編輯捲動記錄"),
                entry(binding_label(&keybinds.clear_pane), "清除窗格"),
                entry(binding_label(&keybinds.copy_mode), "複製模式"),
                entry(binding_label(&keybinds.zoom), "最大化窗格"),
                entry(binding_label(&keybinds.resize_mode), "調整大小模式"),
                entry(
                    binding_label(&keybinds.resize_pane_left),
                    "向左調整窗格大小",
                ),
                entry(
                    binding_label(&keybinds.resize_pane_down),
                    "向下調整窗格大小",
                ),
                entry(binding_label(&keybinds.resize_pane_up), "向上調整窗格大小"),
                entry(
                    binding_label(&keybinds.resize_pane_right),
                    "向右調整窗格大小",
                ),
                entry(binding_label(&keybinds.toggle_sidebar), "切換側邊欄"),
                entry(binding_label(&keybinds.focus_pane_left), "聚焦左側窗格"),
                entry(binding_label(&keybinds.focus_pane_down), "聚焦下方窗格"),
                entry(binding_label(&keybinds.focus_pane_up), "聚焦上方窗格"),
                entry(
                    binding_label(&keybinds.focus_pane_right),
                    "聚焦右側窗格",
                ),
                entry(binding_label(&keybinds.cycle_pane_next), "切換至下一個窗格"),
                entry(
                    binding_label(&keybinds.cycle_pane_previous),
                    "切換至上一個窗格",
                ),
                entry(binding_label(&keybinds.last_pane), "上一個窗格"),
            ],
        ),
    ];

    if !keybinds.custom_commands.is_empty() {
        groups.push((
            "自訂",
            keybinds
                .custom_commands
                .iter()
                .map(|binding| {
                    (
                        binding.label.clone(),
                        binding
                            .description
                            .clone()
                            .map(Cow::Owned)
                            .unwrap_or(Cow::Borrowed("自訂命令")),
                    )
                })
                .collect(),
        ));
    }
    groups
}

pub(crate) fn filter_keybind_help_groups(
    groups: Vec<KeybindHelpGroup>,
    query: &str,
) -> Vec<KeybindHelpGroup> {
    if query.is_empty() {
        return groups;
    }
    let query = query.to_lowercase();
    groups
        .into_iter()
        .filter_map(|(group, entries)| {
            let entries = entries
                .into_iter()
                .filter(|(key, label)| {
                    key.to_lowercase().contains(&query) || label.to_lowercase().contains(&query)
                })
                .collect::<Vec<_>>();
            (!entries.is_empty()).then_some((group, entries))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn groups() -> Vec<KeybindHelpGroup> {
        vec![
            (
                "workspaces / tabs",
                vec![entry("w", "workspace navigation"), entry("c", "new tab")],
            ),
            (
                "panes",
                vec![entry("v", "split vertical"), entry("x", "close pane")],
            ),
        ]
    }

    #[test]
    fn filter_matches_labels_and_shortcuts_case_insensitively() {
        let filtered = filter_keybind_help_groups(groups(), "WoRk");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].1[0].1, "workspace navigation");

        let filtered = filter_keybind_help_groups(groups(), "x");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].1[0].1, "close pane");
        assert!(filter_keybind_help_groups(groups(), "panes").is_empty());
    }

    #[test]
    fn help_lists_every_configured_prefix() {
        let groups = keybind_help_groups(
            &Keybinds::default(),
            &[
                (KeyCode::Char(' '), KeyModifiers::CONTROL),
                (KeyCode::Char('s'), KeyModifiers::CONTROL),
            ],
        );
        let global = &groups[0].1;
        assert_eq!(global[0].0, "ctrl+space / ctrl+s");
        assert_eq!(global[0].1, "prefix mode");
    }
}
