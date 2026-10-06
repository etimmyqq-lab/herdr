# herdr-zh：herdr 繁體中文版

[herdr](https://github.com/herdrdev/herdr) 是給 AI 程式開發代理（Claude Code、Codex 等）用的終端機工作區管理工具。這個 repo 是它的**非官方**繁體中文（台灣）版：把互動介面翻成繁體中文，其餘功能與官方版相同。

> 本專案與 herdr 官方團隊無關，也未經其背書。問題請回報到[這個 repo 的 Issues](../../issues)，不要回報給官方。

## 安裝

還沒裝過 herdr，或已經裝了官方英文版，都是同一行指令。

**Windows**（PowerShell）

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://raw.githubusercontent.com/etimmyqq-lab/herdr-zh/zh-tw/distribution/install.ps1 | iex"
```

**macOS / Linux**

```sh
curl -fsSL https://raw.githubusercontent.com/etimmyqq-lab/herdr-zh/zh-tw/distribution/install.sh | sh
```

裝完後開新的終端機視窗，輸入 `herdr` 啟動。指令名稱和官方版一樣是 `herdr`。

如果安裝前官方英文版正在執行，請先結束它（`herdr server stop` 會關掉所有窗格裡的程式），再啟動才會整個換成中文。

## 更新

```sh
herdr update
```

繁中版的更新來源是這個 repo，不是官方網站，所以更新後仍然是繁中版。

## 換回官方英文版

重新執行官方的安裝指令即可：

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://herdr.dev/install.ps1 | iex"
```

```sh
curl -fsSL https://herdr.dev/install.sh | sh
```

## 版號

繁中版的版號是「官方版號的最後一碼乘以 100，再加上繁中修訂號」。例如 `0.9.300` 是以官方 `0.9.3` 為基礎的第一版，`0.9.301` 是同一個官方版本上的翻譯修正。

## 翻譯範圍

| 已翻譯 | 維持英文 |
|---|---|
| 歡迎畫面、設定、側邊欄、分頁列 | 指令列（CLI）的說明與輸出 |
| 快捷鍵說明、模式列、右鍵選單 | 按鍵名稱（`ctrl+b`、`esc` 等） |
| 工作樹對話框、通知、連線錯誤訊息 | 設定檔的鍵與值 |

## 已知限制

- **macOS 與 Linux 的畫面尚未經人實際檢查**：這兩個平台只經過自動化的「安裝並確認版本」測試。看到排版錯位或漏翻，歡迎開 Issue 並附上截圖。
- **自動化測試未涵蓋翻譯**：官方測試以英文字串為準，繁中版的建置流程只編譯、不執行那些測試。
- **預覽版頻道**：`herdr channel set preview` 會切到官方的英文預覽版。

## 回報翻譯問題

用詞不順、漏翻、排版跑掉，請開 [Issue](../../issues)，並附上畫面截圖與作業系統。

## 授權

herdr 以 [Apache License 2.0](../LICENSE) 授權，本專案沿用相同授權，並在原始碼上做了修改（介面翻譯、產品名稱顯示為 `herdr_zh`、更新來源改為本 repo、兩處中日韓文字寬度的顯示修正）。原始著作權屬於 herdr 的作者與貢獻者。
