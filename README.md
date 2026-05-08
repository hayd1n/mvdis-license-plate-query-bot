# MVDIS License Plate Query Bot (監理服務網車牌查詢機器人)

這是一個基於 Rust 開發的自動化查詢工具，旨在協助使用者從台灣「監理服務網 (MVDIS)」快速查詢並過濾特定號碼的車牌。當尋找到符合條件的車牌時，程式可以自動將結果推播至 **Telegram** 或 **ntfy.sh**。

## ✨ 功能特色

- **高併發查詢**：支援自訂併發數量 (Concurrency) 與重試次數，以最高效率搜集可用車牌。
- **多重查詢條件設定**：可同時設定多組查詢條件，包含車牌版本、車種、特定監理站以及要匹配的號碼模式。
- **多平台通知推播**：支援 Telegram Bot 及 ntfy.sh 進行即時推播，且採用 Markdown 格式確保各平台呈現效果一致。
- **效能測量**：自動計算並顯示每次查詢所耗費的時間。

## 🚀 快速開始

### 前置作業

確保您的開發環境中已安裝 [Rust 及 Cargo](https://rustup.rs/)。

### 安裝與執行

1. 複製本專案到本地端：

   ```bash
   git clone <YOUR_REPOSITORY_URL>
   cd mvdis-license-plate-query-bot
   ```

2. 複製設定檔範本並進行設定（詳見下方設定檔說明）：
   > 專案根目錄下需有一份 `config.toml`

3. 編譯並執行程式：

   ```bash
   cargo run --release
   ```

## ⚙️ 設定檔說明 (`config.toml`)

本程式依賴 `config.toml` 來決定查詢的行為與通知的目標。以下為設定檔的詳細結構：

### 1. 用戶端連線設定 `[client]`

```toml
[client]
retry_times = 3      # 遇到錯誤時的重試次數（預設：3）
concurrency = 20     # 同時發出請求的併發數（預設為系統的 CPU 核心數）
```

### 2. 通知服務設定 `[notification]`

支援 **Telegram** 與 **ntfy** 兩種通知管道。您可以根據需求啟用其中一個或同時啟用。

**Telegram 設定**

```toml
[notification.telegram]
bot_token = "YOUR_TELEGRAM_BOT_TOKEN"  # 向 @BotFather 申請的機器人 Token
chat_id = "YOUR_CHAT_ID"               # 接收通知的對象（您的 User ID 或群組 ID）
# message_prefix = "\\[MVDIS License Plate Bot]" # 可選：自訂訊息前綴
```

**ntfy 設定**

```toml
[notification.ntfy]
server_url = "https://ntfy.sh" # 可選：若您使用自建的 ntfy 伺服器，可在此修改
topic = "your_secret_topic"    # 必填：您想發送與訂閱的主題名稱
```

### 3. 查詢規則設定 `[[query]]`

您可以設定多個 `[[query]]` 區塊來一次搜括多種不同的車型或號碼。

```toml
[[query]]
plate_ver = "new"                           # 車牌版本，例如："new" (新式車牌)
plate_type = "motorcycle550ccBelow"         # 車種類型，例如：黃牌重機 ("motorcycle550ccBelow")
# stations = ["taipeiCity", "shilin"]       # 可選：限制只查詢特定監理站，若註解掉則查詢全台
matchs = ["0001", "0390", "587"]            # 必填：您想尋找的車牌號碼 (支援部分匹配)

[[query]]
plate_ver = "new"
plate_type = "motorcycleNormalHeavy"        # 白牌重型機車
matchs = ["0001"]
```

### 附錄：查詢規則可用參數列表

在編寫 `config.toml` 時，請使用以下表格中定義的 **CamelCase 字串**。

#### `plate_ver` (車牌樣式)

| 參數名稱 (`camelCase`) | 說明 |
| :--- | :--- |
| `old` | 原型式車牌 |
| `new` | 新式車牌 |

#### `plate_type` (車牌別)

**汽車 (Car)**

| 參數名稱 (`camelCase`) | 說明 |
| :--- | :--- |
| `carOwn` | 自用小客貨車 |
| `carRent` | 租賃小客貨車 |
| `carBusiness` | 營業小客車 |
| `carBusinessT` | 營業小貨車 |
| `carBigOwn` | 自用大客車 |
| `carBigRent` | 自用大貨車 |
| `carBigBusiness` | 營業大客車 |
| `carBigBusinessT` | 營業大貨車 |
| `carBigBusinessContainer` | 營業貨櫃曳引 |
| `carTour` | 遊覽大客車 |
| `carElectricOwn` | 電動自小客 |
| `carElectricRent` | 電動租賃車 |
| `carElectricBusiness` | 電動小營客車 |
| `carElectricBigBusiness` | 電動大營客車 |

**機車 (Motorcycle)**

| 參數名稱 (`camelCase`) | 說明 |
| :--- | :--- |
| `motorcycle550ccBelow` | 550cc 以下重機 (黃牌) |
| `motorcycle550ccAbove` | 550cc 以上重機 (紅牌) |
| `motorcycleNormalHeavy` | 普通重型機車 (白牌) |
| `motorcycleNormalLight` | 普通輕型機車 (綠牌) |
| `motorcycleElectric550ccBelow` | 電動 550cc 以下重機 |
| `motorcycleElectric550ccAbove` | 電動 550cc 以上重機 |
| `motorcycleElectricNormalHeavy` | 電動普通重型機車 |
| `motorcycleElectricNormalLight` | 電動普通輕型機車 |

**拖車 (Trailer)**

| 參數名稱 (`camelCase`) | 說明 |
| :--- | :--- |
| `trailerOwn` | 自用拖車 |
| `trailerBusiness` | 營業拖車 |

#### `stations` (監理站 - 可填入多個作為過濾條件)

| 管轄區域 | 參數名稱 (`camelCase`) | 監理站名稱 |
| :--- | :--- | :--- |
| **臺北市** | `taipeiCity` | 臺北市區監理所 |
| | `shilin` | 士林監理站 |
| | `keelung` | 基隆監理站 |
| | `kinmen` | 金門監理站 |
| | `lienchiang` | 連江監理站 |
| **高雄市** | `kaohsiungCity` | 高雄市區監理所 |
| | `lingya` | 苓雅監理站 |
| | `qishan` | 旗山監理站 |
| **臺北區** | `taipeiDistrict` | 臺北區監理所 |
| | `banqiao` | 板橋監理站 |
| | `yilan` | 宜蘭監理站 |
| | `hualien` | 花蓮監理站 |
| | `yuli` | 玉里監理分站 |
| | `luzhou` | 蘆洲監理站 |
| **新竹區** | `hsinchuDistrict` | 新竹區監理所 |
| | `hsinchuCity` | 新竹市監理站 |
| | `taoyuan` | 桃園監理站 |
| | `zhongli` | 中壢監理站 |
| | `miaoli` | 苗栗監理站 |
| **臺中區** | `taichungDistrict` | 臺中區監理所 |
| | `taichungCity` | 臺中市監理站 |
| | `puli` | 埔里監理分站 |
| | `fengyuan` | 豐原監理站 |
| | `changhua` | 彰化監理站 |
| | `nantou` | 南投監理站 |
| **嘉義區** | `chiayiDistrict` | 嘉義區監理所 |
| | `dongshi` | 東勢監理分站 |
| | `yunlin` | 雲林監理站 |
| | `xinying` | 新營監理站 |
| | `tainan` | 臺南監理站 |
| | `madou` | 麻豆監理站 |
| | `chiayiCity` | 嘉義市監理站 |
| **高雄區** | `kaohsiungDistrict` | 高雄區監理所 |
| | `taitung` | 臺東監理站 |
| | `pingtung` | 屏東監理站 |
| | `hengchun` | 恆春監理分站 |
| | `penghu` | 澎湖監理站 |
