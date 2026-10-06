use ratatui::layout::Rect;

pub(crate) const ONBOARDING_TITLE: &str = "  herdr";
pub(crate) const ONBOARDING_SUBTITLE: &str = "  適用於程式開發代理的終端機工作區管理工具";
pub(crate) const ONBOARDING_DESCRIPTION: [&str; 3] = [
    "  這是一個以滑鼠操作為主的終端機。",
    "  點擊側邊欄切換工作區、拖曳窗格",
    "  邊框以調整大小，按右鍵開啟內容選單。",
];
pub(crate) const ONBOARDING_PREFIX_SUFFIX: &str = " 進入前綴模式 · ";
pub(crate) const ONBOARDING_HELP_LABEL: &str = "?";
pub(crate) const ONBOARDING_HELP_SUFFIX: &str = " 顯示快捷鍵與設定";
pub(crate) const ONBOARDING_NEXT: &str =
    "  接下來：安裝選用的代理整合，以取得更可靠的狀態";

pub(crate) fn onboarding_welcome_continue_rect(area: Rect) -> Rect {
    super::widgets::continue_button_rect(area)
}
