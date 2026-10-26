# Zed AI Crew — Tech Spec

| Field | Content |
| ----- | ------- |
| Version | 2.3 · bản nháp · 2026-10-25 |
| Nguồn | Product brief v3.5 (Source 0). Kế hoạch triển khai nằm ở `docs/crew/plan/` (bản 1.2). Báo cáo kiểm thiết kế với mã fork: [`design-check.md`](design-check.md). Brief là nguồn chính về yêu cầu; spec này mô tả cách đáp ứng, không thêm hay bớt yêu cầu. Chỗ nào khác NEED thì theo NEED. |
| Viết và duyệt | Tech lead (agent AI) viết theo brief mục 11.1. Human đã duyệt mọi quyết định thiết kế lớn của spec ngày 2026-10-25; các quyết định nằm thẳng trong mục liên quan. Bản 2.3 áp 24 đề xuất của báo cáo kiểm thiết kế, Human duyệt cùng ngày. |
| Cách trình bày | Thông tin đã kiểm chứng nằm thẳng trong mục liên quan. Mục 20 chỉ liệt kê việc còn mở; câu hỏi nào có câu trả lời thì đưa vào nội dung và bỏ khỏi danh sách. |
| Cơ sở đối chiếu | Mã nguồn fork `11caaae` (mã Rust giống commit spike `20ea81ef29`; nền upstream khoảng 2026-10-17), đọc ngày 2026-10-25. Bản 2.2 trở về trước đọc upstream `347f171` (2026-10-24), commit này không có trong fork. Mã adapter `claude-agent-acp@0.95.0` và `codex-acp@2.2.2` đọc từ gói npm; crate `agent-client-protocol` 3.4.0 và `agent-client-protocol-schema` 1.12.0 đọc từ crates.io. Spike ngày 2026-10-25 trên máy macOS của Human (fork `20ea81ef29`; ba adapter `claude-agent-acp@0.95.0`, `codex-acp@2.2.2`, `antigravity-acp` 1.4.3; crate `agent-client-protocol` 3.4.0). |
| Lưu ý dòng code | Số dòng là của fork `11caaae` và của gói npm, crate nêu trên. Đồng bộ upstream có thể làm lệch; bước 0 ([mục 18](#muc-18)) và mỗi lần đồng bộ kiểm lại trước khi sửa. |
| Nhãn | **[Đã kiểm]** mình đọc trong mã nguồn hoặc tài liệu gốc · **[Spike]** kết quả spike 2026-10-25 (chạy thật, hoặc đọc mã adapter trên máy Human) · **[Baseline]** theo baseline provider của Human · **[Nhớ]** kiến thức đáng tin, chưa kiểm lại · **[Suy luận]** cần spike hoặc test để xác nhận. |

## Mục lục

| Phần | Mục |
| ---- | --- |
| Tổng quan | [0. Tóm tắt](#muc-0) · [1. Phạm vi kỹ thuật](#muc-1) · [2. Kiến trúc](#muc-2) · [3. Crate, điểm nối và CrewConnection](#muc-3) |
| Lõi | [4. Run store](#muc-4) · [5. Cấu hình, ROLE và playbook](#muc-5) · [6. Lớp provider](#muc-6) · [7. Orchestrator và công cụ MCP](#muc-7) · [8. Chọn model và spawn worker](#muc-8) · [9. Pipeline và thực thi](#muc-9) · [10. Gate quyền](#muc-10) · [11. Kiểm chứng và Done](#muc-11) |
| Tích hợp | [12. GitHub](#muc-12) · [13. Dịch vụ model](#muc-13) · [14. Phục hồi và nâng cấp](#muc-14) · [15. Giao diện](#muc-15) · [16. Quan sát và log](#muc-16) |
| Triển khai | [17. Kiểm thử](#muc-17) · [18. Thứ tự triển khai](#muc-18) · [19. Rủi ro kỹ thuật](#muc-19) · [20. Việc còn mở](#muc-20) |
| Phụ lục | [A. Truy vết NEED](#phu-luc-a) · [B. Dữ kiện đã kiểm](#phu-luc-b) · [C. Lịch sử thay đổi](#phu-luc-c) |

---

<a id="muc-0"></a>

## 0. Tóm tắt

1. **Hình dạng.** Crew gồm 6 crate mới trong fork Zed. Core chỉ bị sửa ở các điểm nối H1–H12 ([3.2](#muc-3-2)).
2. **Mọi session Crew đi qua `CrewConnection`** ([3.3](#muc-3-3)). Mỗi provider có một agent id riêng của Crew (ví dụ `crew-claude-acp`), trỏ tới adapter đã ghim. Zed tự load lại thread ở nhiều đường (mở từ History, kết nối lại, nạp lại thread đã bị giải phóng), và mọi đường đều tạo agent server qua `Agent::server()` **[Đã kiểm]**. Crew chặn ở đó, nên mọi thread của agent id Crew đi qua `CrewConnection`, và mọi lần tạo, load, resume đều mang đủ ROLE, `_meta` và MCP server của Crew. Cần vậy vì hai lý do:
   - Provider không lưu ROLE cùng hội thoại **[Spike]**.
   - Khi load hay resume một session đang sống với option khác, adapter Claude hủy rồi dựng lại session theo option mới **[Đã kiểm]**. Một lần load thiếu option sẽ làm session mất ROLE, sandbox và công cụ của Crew.
3. **Orchestrator** là thread của agent id Crew trong Agent Panel. Nó gọi công cụ điều phối qua MCP server riêng của Crew, và chỉ đọc checkout của Human ([7.5](#muc-7-5)).
4. **Worker** là session do Crew tạo trong worktree riêng:
   - Nhận task packet tự đủ dùng; trả bàn giao qua công cụ có schema.
   - Sau khi Zed khởi động lại, Crew không giao tiếp việc cho worker cũ. Muốn làm tiếp thì spawn worker mới từ run store ([mục 14](#muc-14)).
5. **Kênh ROLE theo provider:**
   - Claude: trường ACP `systemPrompt` **[Spike]**.
   - Antigravity: `_meta.antigravity.systemPrompt.append` **[Spike]**.
   - Codex: `_meta.jetbrains.air.customInstructions` **[Đã kiểm trong mã]**, chưa chạy thật (S11).
6. **Chọn provider** dùng một quy trình chung ([mục 8](#muc-8)): dịch vụ model trả danh sách xếp hạng; Crew lọc theo đăng nhập, mức cách ly và hạn mức; chọn ứng viên đầu tiên đạt điều kiện rồi spawn.
7. **Đăng nhập** được xác nhận theo từng provider, mặc định bằng healthcheck ping-pong trong session thăm dò dùng lại ([6.5](#muc-6-5)).
8. **An ninh thực thi:**
   - Gate quyết định cho phép hay từ chối theo nội dung yêu cầu (`rawInput`), rồi chọn option theo loại (`kind`): chỉ `allow_once` hoặc `reject_once`, không bao giờ `allow_always` hay `reject_always`, vì hai loại này nới quyền cho cả session hoặc ghi chính sách lâu dài vào provider.
   - Câu hỏi agent gửi người dùng (elicitation) không qua yêu cầu xin quyền, nên Crew từ chối hết: worker hỏi qua bàn giao, Orchestrator qua `crew_ask_human` ([10.1](#muc-10)).
   - Sandbox của provider: Claude có sandbox, chặn đọc đường dẫn bí mật, chặn ghi `$TMPDIR`, mạng theo allowlist; Codex ở `workspace-write`.
   - Đọc và ghi file qua Zed (`fs/*`) của session Crew đi qua chính sách của Crew (H12): chỉ trong worktree của worker. Thiếu H12 thì Zed nhận ghi ra ngoài project qua worktree ẩn mà không xin quyền **[Đã kiểm]**. Antigravity ghi file theo đường này **[Baseline]**.
   - Crew quét secret trên diff trước khi commit. Rủi ro còn lại được ghi rõ ([10.4](#muc-10-4)).
9. **Mọi fact nằm trong run store SQLite.** Chỉ DoneEvaluator kết luận Done; quyết định của Human chỉ sinh ra từ tin nhắn thật của Human.

---

<a id="muc-1"></a>

## 1. Phạm vi kỹ thuật

### 1.1 Mục tiêu

Hiện thực 61 NEED của brief v3.5 theo thứ tự xây ở brief 4.4. Mục 18 chỉnh một điểm, có ghi lý do. Phụ lục A truy vết từng NEED tới thành phần và cách kiểm.

### 1.2 Ngoài phạm vi

| Mục | Lý do |
| --- | ----- |
| Windows, Linux, project remote/SSH, dev container | NEED 01. Crew từ chối khởi tạo cho project remote |
| Nhiều người dùng, đồng bộ cloud | NEED 01, 53 |
| Zed Agent native làm provider | NEED 25 chỉ gồm Claude, Codex, Antigravity |
| Subagent gốc của provider làm worker | Zed không khai capability `subagents` **[Đã kiểm]**; worker của Crew là session riêng |
| Chuyển kết nối ACP sang v2 | Zed chỉ chạy v1: `acp::connect` trả connection v1 (`v1.rs:320-340`); `AcpV2Connection` có sẵn nhưng chưa được dùng (`v2.rs:119`) **[Đã kiểm]**. Crate 3.4.0 đã có `systemPrompt` nên không cần nâng **[Spike]**. Đồng bộ upstream theo dõi việc chuyển sang v2 ([mục 17](#muc-17)) |
| Đọc, sửa hay dọn cấu hình, credential, home, dữ liệu session của provider | NEED 61. Session Antigravity không xóa được qua ACP thì để nguyên |
| Bọc process provider trong sandbox của OS | Nằm ngoài ACP (NEED 61) |

---

<a id="muc-2"></a>

## 2. Kiến trúc

### 2.1 Sơ đồ khối

```
┌──────────────────────────── Zed (fork) ────────────────────────────────┐
│ Agent Panel · Threads Sidebar · Settings · Onboarding · Status bar     │
│        ▲ crew_ui                                                        │
│ crew (core): CrewStore · SessionManager · Selection · Scheduler ·      │
│              PolicyGate · ModeGuard · Verification · DoneEvaluator ·   │
│              DecisionCapture · Recovery                                 │
│    │              │                 │                 │                 │
│ crew_acp       crew_mcp          crew_github       crew_models          │
│ (CrewConnection, (McpServer       (App, REST)       (catalog, score)    │
│  profile, health, theo session)       │                                 │
│  quota, probe)     │                  │                                 │
│ acp_thread · agent_servers (ACP v1) · project/agent_server_store       │
└────┼───────────────┼──────────────────┼─────────────────────────────────┘
     │ child process │ Unix socket      │ HTTPS
     ▼               │                  ▼
 Adapter ghim của Crew: Claude / Codex / Antigravity     GitHub REST API
 (agent chạy `zed --crew-mcp-bridge <socket>` như một MCP server stdio)
```

### 2.2 Tiến trình

| Tiến trình | Khởi chạy bởi | Vai trò |
| ---------- | ------------- | ------- |
| Zed (fork) | Human | Chạy Crew trong process chính |
| Adapter của Crew, mỗi provider một bản ghim (H10) | Zed, qua agent server store, khi mở session Crew | Orchestrator, worker, session thăm dò |
| Adapter của Zed (bản Zed tự cập nhật) | Zed | Thread thường của Human; Crew không dùng |
| `zed --crew-mcp-bridge <socket>` | Provider, khi khởi động MCP server khai trong request | Cầu nối stdio ↔ Unix socket (H1) |
| Lệnh kiểm chứng | Crew, qua API `Sandbox` (Seatbelt) | Sensor tất định |
| `git` | Crew, qua crate `git` và askpass của Zed | Worktree, commit, merge, push |

### 2.3 Ranh giới tin cậy

| Vùng | Mức tin cậy | Hệ quả |
| ---- | ----------- | ------ |
| Human | Quyết định sản phẩm | Quyết định chỉ từ tin nhắn trong hội thoại hoặc review trên GitHub (NEED 13, 14) |
| Zed + Crew | Tin cậy | Mọi gate tất định nằm ở đây |
| Process provider | Hợp tác, không giam được | Qua ACP, client không giam được agent vào worktree. compound-engineering, tiền lệ gần nhất, cũng ghi “No route currently confines the worker to its workspace” **[Đã kiểm]**. Vì vậy dựa vào sandbox của provider cộng gate |
| Dữ liệu provider trên máy (`~/.claude`, `~/.codex`, `~/.gemini`) | Ngoài phạm vi Crew | Không đọc, không sửa, không dọn (NEED 61) |
| Output của worker, web, file, công cụ | Dữ liệu | Không đổi mục tiêu, quyền, phạm vi (NEED 08) |
| MCP socket | Tiến trình cùng user | Mỗi session một socket (NEED 01). `McpServer` tạo thư mục tạm với quyền mặc định (0755 khi umask là 022) **[Đã kiểm]**, nên Crew tự đặt thư mục socket về 0700 và kiểm lại trước khi chạy agent |
| GitHub | Bên ngoài | Token installation theo đúng một repo (NEED 05) |

### 2.4 Mô hình luồng

- Entity và UI chạy trên foreground thread của GPUI. I/O (SQLite, HTTP, git, parse) chạy bằng `background_spawn`.
- Mọi task giữ `Task` handle để không bị hủy ngầm.
- Không `unwrap()`; lỗi đi lên UI, theo `.rules` của repo.

---

<a id="muc-3"></a>

## 3. Crate, điểm nối và CrewConnection

### 3.1 Crate mới

| Crate | Trách nhiệm |
| ----- | ----------- |
| `crew` | Run store, SessionManager, Selection, Scheduler, PolicyGate, ModeGuard, Verification, DoneEvaluator, DecisionCapture, Recovery, cấu hình theo tầng, ROLE và playbook |
| `crew_acp` | `CrewAgentServer`, `CrewConnection`, ProviderProfile, healthcheck, snapshot model, đọc hạn mức, phát hiện compaction |
| `crew_mcp` | McpServer theo session, bộ công cụ theo vai |
| `crew_github` | GitHub App, REST client, Issues, PR, review, ruleset, phát hiện lệch |
| `crew_models` | Danh mục model, nguồn benchmark, chấm điểm, xếp hạng ứng viên |
| `crew_ui` | Action, tổng quan session, view worker chỉ đọc, việc chờ Human, status bar, onboarding, Settings |

<a id="muc-3-2"></a>

### 3.2 Điểm nối H1–H12

Đây là toàn bộ chỗ sửa core (NEED 59). Thêm điểm nối mới thì phải cập nhật bảng này.

Ngoài core, fork chỉ khác upstream ở một chỗ: một mục ngắn (tối đa 10 dòng) ở cuối `.rules`, trỏ tới `docs/crew/AGENTS.md`. `AGENTS.md`, `CLAUDE.md` và `GEMINI.md` ở gốc repo đều là symlink tới `.rules` **[Đã kiểm]**, nên mục này tới được agent của cả ba provider (Human đồng ý 2026-10-25). Mục đặt ở cuối file để khi đồng bộ upstream, xung đột nếu có chỉ nằm ở đó.

| ID | Nơi sửa (fork `11caaae`) | Thay đổi | Lý do | NEED |
| -- | ---------------------------- | -------- | ----- | ---- |
| H1 | `crates/zed/src/main.rs` | Flag ẩn `--crew-mcp-bridge <socket>`: cầu nối hai chiều stdin/stdout ↔ Unix socket, thoát khi một phía đóng | Chế độ `--askpass` hiện có là một chiều (`askpass.rs:413-469`) **[Đã kiểm]** | 24, 26, 32 |
| H2 | `crates/agent_servers/src/acp/v1.rs` | Phương thức `AcpConnection::{new,load,resume}_session_with_options(…, SessionOptions)` ([6.3](#muc-6-3)): đặt `.system_prompt(...)` khi agent khai `sessionCapabilities.systemPrompt`; gắn `_meta`; dùng danh sách MCP của caller thay cho `mcp_servers_for_project` (`:9134`); không áp mode và config option mặc định từ settings `agent_servers.<agent id>` (`AcpConnectionDefaults`, `:94-168`). Trait `AgentConnection` giữ nguyên; `CrewConnection` lấy `AcpConnection` qua `into_any()` (`connection.rs:304`) | `new_session` (`:1238`), `load_session` (`:1418`) và `resume_session` (`:1457`) chỉ gửi cwd, thư mục thêm và **mọi** MCP server đang bật của project; không gửi system prompt; mặc định từ settings được áp sau new, load, resume mà không phát event (`:1310-1345`) **[Đã kiểm]** | 02, 55 |
| H3 | `crates/agent_servers/src/acp/v1.rs` | (a) Handler ext notification `_auth/status_update`; hiện client chỉ đăng ký notification của session và elicitation (`:368-442`). (b) Gọi ext method `_session/steering`. Crate đã có API cho cả hai (`Builder::on_receive_notification`, `ConnectionTo::send_request`, `UntypedMessage`), nhưng field `connection` của `AcpConnection` là private (`:75`), nên phải sửa `v1.rs` **[Đã kiểm]** | Zed bỏ qua thông báo đăng nhập **[Đã kiểm]**; adapter Claude và Codex đều khai `_session/steering` **[Đã kiểm]** | 36, 56 |
| H4 | `crates/acp_thread/src/acp_thread.rs`; với (b) có thể cả `crates/agent_servers/src/acp/v1.rs` | (a) Giữ `_meta` cấp trên của usage update, phát `used` và `size` theo từng update. (b) Ghi update bị bỏ: đọc từ `AcpConnection::subscribe_debug_messages()` (`v1.rs:488-495`), hoặc thêm hook ở tầng SDK trong `v1.rs`; không đặt ở nhánh `_ => {}` của `handle_session_update` (`:4526`) | `update_usage` (`:5366`) chỉ giữ `cost.meta` **[Đã kiểm]**. Update có kiểu lạ đã bị SDK loại khi parse (`agent-client-protocol` 3.4.0 `src/jsonrpc/handlers.rs:272-279`; enum `SessionUpdate` không có biến thể dự phòng), nên nhánh `_ => {}` chỉ thấy kiểu đã biết **[Đã kiểm]**. Luồng debug là công cụ gỡ lỗi, định dạng có thể đổi | 31, 43, 50, 60 |
| H5 | `crates/agent_ui` | (a) Action “Crew: New Session”. (b) Cờ chỉ đọc tường minh cho thread hoặc view: không composer, không hiện option `allow_always`. (c) Nhóm session Crew trong sidebar. (d) `Agent::server()` (`agent_ui.rs:578`) tra một registry factory đặt trong `agent_servers` trước khi tạo `CustomAgentServer`; Crew đăng ký factory cho agent id của Crew, trả về `CrewAgentServer`, trước khi panel khôi phục thread; agent id của Crew ẩn khỏi bộ chọn agent. (e) Thread import bỏ qua session Crew sở hữu và session có cwd là thư mục thăm dò. (f) Agent id của Crew không bao giờ thành agent đang chọn của panel | Zed tự load lại thread ở nhiều đường: mở thread có session id (`conversation_view.rs:1358-1371`), kết nối lại (`:1211-1218`), thread rảnh bị giải phóng (`agent_panel.rs:4300-4313`) rồi nạp lại khi Human mở. Thread import liệt kê session của mọi cwd (`thread_import.rs:803-807`). (b) không tái dùng được nhánh subagent: `is_subagent` chỉ là `parent_session_id.is_some()` (`thread_view.rs:1268`), không có setter, và giả parent id thì sidebar không lưu thread (`conversation_view.rs:1850`). (f): `set_base_view` gán agent đang chọn theo thread vừa hiện và lưu lại (`agent_panel.rs:4354`) **[Đã kiểm]** | 09, 50, 53, 55, 56 |
| H6 | `crates/onboarding` | Phần onboarding của Crew: sửa trang onboarding của Zed hoặc mở một item Crew riêng | Onboarding chỉ có một trang, không có registry trang (`onboarding.rs:299-301`) **[Đã kiểm]**; NEED 04 | 04 |
| H7 | `crates/settings_content`, `crates/settings_ui` | Một trường `crew` trên `ProjectSettingsContent`, nên hợp lệ ở cả `settings.json` của user (vì `SettingsContent` flatten nội dung project) lẫn `.zed/settings.json` **[Đã kiểm]**; thêm `crew` vào `flattened_deserialize!` (`settings_content/src/project.rs:90-96`); trang Settings | NEED 27 | 04, 27 |
| H8 | `crates/feature_flags/src/flags.rs` | `CrewFeatureFlag` với `enabled_for_staff() = false`: tắt mặc định, kể cả bản debug; bản debug bật bằng `"feature_flags": {"crew": "on"}`. Khi bắt đầu manual test, đổi `enabled_for_all()` thành `true` để bản release có Crew. Rollback là build lại với `enabled_for_all()` trả `false`, vì khi hàm này trả `true` thì settings không tắt được Crew (`store.rs:166-169`) **[Đã kiểm]** | Fork dùng được hằng ngày trong lúc xây (brief 9). Zed chỉ nhận override từ settings ở bản debug hoặc với tài khoản staff (`store.rs:105-107, 166-182`), và bản debug được coi như staff **[Đã kiểm]** | 03 |
| H9 | `crates/zed/src/main.rs` | Gọi `crew_ui::init(cx)` cạnh `agent_ui::init` (`main.rs:713`) **[Đã kiểm]** | Khởi tạo | — |
| H10 | `crates/project/src/agent_server_store.rs` | (a) Agent id của Crew cho mỗi provider, cài **đúng phiên bản ghim**: npx dùng spec chính xác thay cho `bounded_npm_package_spec` (`:1456`); binary cài vào thư mục riêng của agent id Crew, vì `remove_stale_versioned_archive_cache_dirs` (`:1055`, gọi ở `:1309`) xóa mọi bản khác trong cùng thư mục; dùng chung thì bản của Zed và của Crew xóa lẫn nhau **[Đã kiểm]**. Bản Zed tự cập nhật cho thread thường giữ nguyên. (b) Đăng ký agent id của Crew vào `external_agents` trước `AgentPanel::load`, và đăng ký lại sau mỗi lần `reregister_agents` (`:294`) xóa rồi dựng lại danh sách (`:348`) | Npx cài theo trần ở mỗi lần chạy; binary tự dọn bản cũ. Connection store chỉ giữ agent id tùy chỉnh có trong `external_agents` (`agent_connection_store.rs:277`) **[Đã kiểm]** | 28 |
| H11 | `crates/agent_servers/src/custom.rs` và luồng đăng nhập của Zed | Sửa lỗi đăng nhập ở tầng Zed (NEED 56): (a) chuyển tiếp `OPENAI_API_KEY` thay cho tên sai `OPEN_AI_API_KEY` (`custom.rs:243-244`); (b) ô nhập API key cho Codex, key lưu trong Keychain kể cả trên kênh dev ([mục 12](#muc-12)), gửi qua `_meta["api-key"]` của `authenticate`; (c) Antigravity: hiện URL đăng nhập lên UI khi không mở được trình duyệt | `custom.rs` đọc `OPEN_AI_API_KEY` **[Đã kiểm]** trong khi adapter đọc `CODEX_API_KEY`, `OPENAI_API_KEY` hoặc key trong `_meta`; Zed không có ô nhập key; Antigravity không đọc `NO_BROWSER`, URL chỉ ra stderr **[Baseline]** | 56 |
| H12 | `crates/agent_servers/src/acp/v1.rs` (handler `fs/*`), `crates/acp_thread/src/acp_thread.rs` (mở vị trí của tool call) | Với session Crew sở hữu: (a) mọi `fs/read_text_file` và `fs/write_text_file` hỏi chính sách của Crew trước khi chạy (Crew đăng ký chính sách qua một registry trong `agent_servers`, như H5d): chỉ cho đường dẫn đã chuẩn hóa và resolve symlink, nằm trong worktree của worker; từ chối với Orchestrator, session thăm dò và worker chỉ đọc. (b) Vị trí ngoài project trong tool call không còn được mở thành worktree ẩn; `acp_thread` không phụ thuộc `agent_servers`, nên thread nhận một cờ do connection đặt khi tạo thread | `fs/write_text_file` được áp mà không qua bước xin quyền nào (`v1.rs:9416-9449`). Kiểm đường dẫn nhận mọi worktree, kể cả worktree ẩn (`acp_thread.rs:7342` → `worktree_store.rs:463-472`), và so tiền tố mà không resolve symlink. Vị trí là đường dẫn tuyệt đối ngoài project được mở bằng `open_local_buffer` (`acp_thread.rs:1760`), và hàm này tạo worktree ẩn (`project.rs:3205`); việc này chạy ở mọi tool call và update, kể cả khi yêu cầu xin quyền bị từ chối (`acp_thread.rs:5419, 5589, 5654`) **[Đã kiểm]** | 07, 34 |

**Fork verification (T0.2, 2026-10-25):** H4 must preserve the fork's existing v1 path: `handle_session_update` converts `acp_v1::UsageUpdate` into `acp_v2::UsageUpdate`, forwarding `update.meta`; `update_usage` then discards that top-level metadata. The fork's `AcpThreadEvent::TokenUsageUpdated` is a unit variant, while upstream `347f171` carries `OutputTokensUpdate`. T1.10 adds a separate usage-details event carrying each update's `used`, `size` and `_meta`, preserving the current unit event and v1 APIs. The deliberate ACP `=3.4.0` pin (upstream `=3.5.0`) is accepted for M0 and checked by S6; both enable `schemars`, `unstable`, `unstable_protocol_v2`. See [verified hook locations](hook-points.md).

API có sẵn, **không cần sửa core** **[Đã kiểm]**:
- Gate: `AcpThreadEvent::ToolAuthorizationRequested` (`acp_thread.rs:3579`), `ElicitationRequested` (`:3581`), `authorize_tool_call` (`:6077`), `authorize_permission_request` (`:6092`).
- Worktree: `GitStore::create_worktree` với đích `NewBranch` (`git_store.rs:9276`); `Project::local` dựng project không cần cửa sổ (`project.rs:1232`), như `crates/eval_cli/src/main.rs:735` đã làm.
- MCP: `context_server::listener::McpServer`, `add_tool`, `handle_request` (handler cho method khác, như `initialize`), `McpServerTool`.
- Lưu trữ: `db::open_db` mở một file SQLite riêng, có migration (`db.rs:174`). Keychain: `CredentialsProvider` (kênh dev xem [mục 12](#muc-12)). Sandbox: API công khai `Sandbox::new`, `wrap`, `execute` (`sandbox.rs:519, 611, 638`) với `SandboxPolicy`; `wrap_invocation` nằm trong module private.
- Log update bị bỏ: `AcpConnection::subscribe_debug_messages()` (`v1.rs:488-495`).

<a id="muc-3-3"></a>

### 3.3 CrewConnection

```rust
// Phác thảo trong crew_acp; tên có thể đổi khi implement.
pub struct CrewAgentServer { provider: ProviderId, store: Entity<CrewStore> }   // impl AgentServer
pub struct CrewConnection {
    inner: Rc<AcpConnection>,   // connection v1 tới adapter ghim, tạo với agent id của Crew
    provider: ProviderId,
    store: Entity<CrewStore>,
}

impl AgentConnection for CrewConnection {
    // Chỉ tạo session khi được gọi từ “Crew: New Session” (session Orchestrator); lời gọi khác bị từ chối.
    fn new_session(self: Rc<Self>, project: Entity<Project>, work_dirs: PathList, cx: &mut App)
        -> Task<Result<Entity<AcpThread>>>;
    // Mọi đường load/resume của Zed đi qua hai hàm này.
    fn load_session(self: Rc<Self>, session_id: acp::SessionId, project: Entity<Project>,
        work_dirs: PathList, title: Option<SharedString>, cx: &mut App) -> Task<Result<Entity<AcpThread>>>;
    fn resume_session(/* như load_session */) -> Task<Result<Entity<AcpThread>>>;
    // Các phương thức còn lại của trait chuyển thẳng cho `inner`.
}
```

| Tình huống | Hành vi |
| ---------- | ------- |
| Session id không có trong `crew_acp_sessions` | Từ chối và báo lỗi. Agent id của Crew không mở session lạ |
| Session con của một session Crew | Adapter có thể báo session con trong `_meta.subagent_session_info`; Zed load session con trên cùng connection và bỏ qua lỗi load (`conversation_view.rs:1590-1620, 2438`) **[Đã kiểm]**. Crew ghi session con của session Crew đã biết vào `crew_acp_sessions` (kind `child`) và cho load ở chế độ chỉ xem, thay vì từ chối làm view hỏng mà không báo |
| Load hoặc resume session Crew | Dựng `SessionOptions` từ bản ghi (ROLE, `_meta`, MCP server) rồi gọi bản `*_with_options`. Trong cùng process adapter, option gửi lại phải giống hệt bản đã gửi, vì adapter Claude so fingerprint của cwd, MCP server, system prompt và `_meta.claudeCode.options`, khác thì hủy rồi dựng lại session (`acp-agent.js:489-510, 8379-8420`) **[Đã kiểm]** |
| Sau khi Zed khởi động lại | Socket MCP mới nên đường dẫn trong option đổi. Không sao, vì adapter là process mới, không còn session sống để dựng lại |
| Session worker | Load được để xem lịch sử, ở chế độ chỉ xem (H5b). Sau khi Zed khởi động lại, Crew không gửi prompt giao việc mới cho worker cũ |
| Session thăm dò | Chỉ Crew dùng; không hiện trong UI (H5e) |
| Kênh ROLE không dùng được (mất capability, adapter đổi) | Báo blocker, không tạo session (NEED 55) |
| Agent id lưu cùng thread | `AcpThread` giữ connection bên trong, và agent id lưu của thread lấy từ connection đó (`v1.rs:916-924`; `crates/agent_ui/src/thread_metadata_store.rs:1305`) **[Đã kiểm]**. Vì vậy `inner` được tạo với agent id của Crew; nếu không, mở lại thread sẽ đi về agent thường |
| Lời gọi theo từng thread | Prompt, cancel, mode, config option, close đi thẳng tới `inner` mà `AcpThread` giữ, không qua `CrewConnection` **[Đã kiểm]**. `CrewConnection` chỉ thấy new, load, resume, xác thực, danh sách session và định tuyến elicitation; phần còn lại Crew theo dõi bằng event của thread |
| Xử lý riêng theo agent id gốc | Zed xử lý riêng một số agent id: chỉ xóa `ANTHROPIC_API_KEY` khỏi môi trường của `claude-acp` (`custom.rs:236-237`); Codex, Gemini, Cursor cũng có xử lý riêng (`custom.rs:239-248`; `v1.rs:451, 649`) **[Đã kiểm]**. `CrewAgentServer` chép các xử lý này cho agent id của Crew. Thiếu thì adapter Claude chạy dưới `crew-claude-acp` có thể dùng API key trong môi trường thay cho gói đã đăng nhập (NEED 25) |
| Settings `agent_servers.<agent id>` | Không áp cho agent id của Crew: bản `*_with_options` của H2 bỏ qua chúng. Zed áp mode và model mặc định từ các settings này sau new, load, resume mà không phát event (`v1.rs:156-168, 1310-1345`) **[Đã kiểm]**; mode và model của session Crew chỉ do Crew đặt ([10.2](#muc-10)) |

Cơ sở cho thiết kế này **[Đã kiểm]**:
- `CustomAgentServer::new` chỉ được gọi trong `Agent::server()` (`agent_ui.rs:586`). Agent Panel, thread import và archive view đều tạo agent server qua hàm này (`agent_panel.rs:4665`, `thread_import.rs:704`, `threads_archive_view.rs:832`); trong mã production, `AgentServerDelegate::new` chỉ được gọi ở `agent_connection_store.rs:297`.
- Connection store chỉ giữ agent id tùy chỉnh có trong `external_agents` của agent server store (`agent_connection_store.rs:277`), và server đầu tiên được cache theo khóa sẽ được dùng tiếp (`:149-151`). Vì vậy Crew đăng ký agent id (H10b) trước `AgentPanel::load`, đăng ký lại sau mỗi lần `reregister_agents`, và đăng ký factory (H5d) trước khi panel khôi phục thread.
- Zed đã dùng đúng mẫu bọc này trong test: `FakeAcpAgentConnection` giữ `inner: Rc<AcpConnection>` và chuyển tiếp từng phương thức (`v1.rs:1841-1842`).
- Trait có 26 phương thức có cài đặt mặc định (`connection.rs:95-305`); bản bọc quên chuyển tiếp một phương thức vẫn biên dịch được, nên có test kiểm việc chuyển tiếp ([mục 17](#muc-17)).
- Hệ quả phụ: connection của Crew không hiện trong view ACP Logs, vì Zed chỉ downcast connection về `AcpConnection` (`agent_connection_store.rs:115`; `acp_tools.rs:133`).

Upstream có thể thêm đường mới, nên test ở [mục 17](#muc-17) khóa điều này mỗi lần đồng bộ.

---

<a id="muc-4"></a>

## 4. Run store

### 4.1 Nguyên tắc

- Một file SQLite riêng của Crew, mở bằng `db::open_db` (`db.rs:174`), đặt trong thư mục dữ liệu của Zed, không nằm trong repo nào (NEED 02, 58). Không dùng `db::static_connection!`, vì macro này dùng database chung của Zed (`db.rs:264-268`) **[Đã kiểm]**.
- **Độ bền:** với `ZED_STATELESS`, hoặc khi không mở được file, Zed âm thầm chuyển sang database trong bộ nhớ (`db.rs:178-180, 197-202`) **[Đã kiểm]**. Lúc khởi động, Crew kiểm run store là file trên đĩa; không phải thì readiness báo lỗi, Crew không mở session nào và Recovery dừng ([14.1](#muc-14)).
- `crew_events` chỉ ghi thêm và là nguồn chính; bảng projection dựng lại được từ đó.
- `project_id` là định danh của checkout chính: đường dẫn đã chuẩn hóa, gắn thêm repo GitHub sau Kickoff. Mọi bản ghi mang `project_id`; công cụ MCP chỉ truy vấn trong project của session.

### 4.2 Bảng chính

| Bảng | Cột chính | Ràng buộc |
| ---- | --------- | --------- |
| `crew_events` | `id`, `project_id`, `session_id`, `seq`, `kind`, `payload_json`, `created_at` | `(session_id, seq)` duy nhất; không `UPDATE`, không `DELETE` |
| `crew_sessions` | `id`, `project_id`, `type`, `state`, `input_version`, `config_hash`, `playbook_hash`, `orchestrator_provider`, `crew_version`, `schema_version`, `outcome` | Đã `done` thì các cột kết quả bị khóa (NEED 41) |
| `crew_acp_sessions` | `acp_session_id`, `provider`, `kind` (`orchestrator`, `worker`, `probe`, `child`), `parent_acp_session_id` (với `child`), `crew_session_id`, `spawn_id`, `cwd`, `options_json`, `options_hash`, `created_at` | Nguồn duy nhất để CrewConnection dựng lại option; `options_json` không chứa secret |
| `crew_inputs` | `session_id`, `version`, `source` (tin nhắn, tài liệu), `content_ref`, `created_at` | Chỉ ghi thêm |
| `crew_pipeline_versions` | `session_id`, `version`, `dag_json`, `reason`, `created_at` | Chỉ ghi thêm |
| `crew_tasks` | `id`, `session_id`, `pipeline_version`, `role`, `state`, `blocked_reason`, `deps_json`, `packet_artifact_id`, `problem_id` | `ready` chỉ khi mọi dependency `accepted` |
| `crew_spawns` | `id`, `task_id`, `provider`, `model`, `effort`, `role_id`, `role_version`, `role_hash`, `role_channel`, `adapter_version`, `mode`, `options_hash`, `ranked_json`, `skipped_json`, `quota_json`, `isolation`, `residual_risks_json`, `acp_session_id`, `worktree_path` | Bất biến sau khi tạo (NEED 26, 56) |
| `crew_handoffs` | `id`, `spawn_id`, `payload_json`, `schema_ok`, `accepted`, `reason`, `undeclared_files_json`, `secret_scan` | — |
| `crew_criteria` | `id`, `owner_kind`, `owner_id`, `text`, `state`, `evidence_ref` | `passed` bắt buộc có `evidence_ref` (NEED 43) |
| `crew_artifacts` | `id`, `kind`, `location`, `version`, `hash`, `derived_from_json`, `state` | Đổi nguồn thì phụ thuộc thành `stale` (NEED 42) |
| `crew_decisions` | `id`, `session_id`, `question_ids_json`, `versions_shown_json`, `human_message_ref`, `verbatim`, `interpretation`, `superseded_by` | `verbatim` lấy nguyên văn tin nhắn của Human |
| `crew_effects` | `id`, `session_id`, `kind`, `idempotency_key`, `intent_json`, `state`, `result_json` | `intent` ghi trước khi thực hiện (NEED 44) |
| `crew_verifications` | `id`, `task_id`, `kind`, `command_or_role`, `result`, `classification`, `evidence_ref` | `classification` là `product` hoặc `infra` |
| `crew_problems` | `id`, `session_id`, `description`, `failed_attempts`, `state` | Lần thử thứ 4 bị chặn (NEED 37) |
| `crew_health` | `provider`, `state`, `method`, `checked_at`, `detail` | `state`: `confirmed`, `not_logged_in`, `unconfirmed` |
| `crew_model_snapshots` | `provider`, `taken_at`, `source`, `models_json`, `stale` | — |
| `crew_quota` | `provider`, `group`, `window`, `percent_left`, `resets_at`, `state`, `source`, `observed_at` | `state`: `measured`, `estimated`, `unknown` |
| `crew_usage` | `spawn_id`, `observed_at`, `used`, `size`, `cost_json` | Dùng để phát hiện compaction |
| `crew_scores` | `provider`, `model`, `effort`, `harness_version`, `task_type`, `source`, `benchmark_version`, `value`, `stderr`, `self_reported`, `date` | Không gộp khác phiên bản hay khác harness |
| `crew_dropped_updates` | `provider`, `kind`, `count`, `last_seen` | — |

### 4.3 Máy trạng thái

| Đối tượng | Trạng thái | Quy tắc |
| --------- | ---------- | ------- |
| Session | `clarifying` → `planned` → `running` ⇄ `waiting_human` → `done` · `closed_not_done` · `stopped` | Chỉ DoneEvaluator chuyển sang `done` |
| Task | `pending` → `ready` → `queued` → `running` → `handed_off` → `verifying` → `accepted` · `rejected` · `blocked` · `waiting_human` · `stale` · `interrupted` | `blocked_reason`: quyền, hạn mức hết giữa chừng, blocker do worker báo. `interrupted` khi Zed tắt giữa lượt; Orchestrator quyết bước tiếp theo |
| Tiêu chí | `unverified` → `passed` · `failed` | Default-FAIL |
| Artifact | Tạm theo BMAD: `draft` → `ready` → `in_progress` → `in_review` → `done` · `blocked` · `dropped` · `superseded` | State machine đầy đủ theo loại artifact chốt khi đặc tả workflow (brief Q6) |
| Effect | `intent` → `done` · `failed` · `unknown` | Recovery xử lý `intent` và `unknown` |

### 4.4 Loại event

`session_created`, `input_versioned`, `pipeline_versioned`, `decision_requested`, `decision_captured`, `selection_made`, `selection_blocked`, `worker_spawned`, `gate_decision`, `mode_violation`, `handoff_received`, `handoff_rejected`, `secret_detected`, `verification_run`, `evaluation_verdict`, `attempt_recorded`, `effect_intent`, `effect_result`, `health_checked`, `quota_observed`, `quota_exhausted`, `compaction_detected`, `update_dropped`, `role_channel_unavailable`, `stop_requested`, `recovery_reconciled`, `guardrail_violation`, `done_evaluated`.

---

<a id="muc-5"></a>

## 5. Cấu hình, ROLE và playbook

### 5.1 Tầng (brief 5.6.2)

| Tầng | Vị trí | Gồm |
| ---- | ------ | --- |
| Đăng nhập provider | Zed và provider | Crew chỉ xác nhận ([6.5](#muc-6-5)) |
| Thiết lập riêng của provider | Home của provider | Crew không đọc, không sửa; chỉ ghi những gì provider báo qua ACP |
| Crew user-scope | Khóa `crew` trong `settings.json` của user; thư mục `crew/` trong thư mục cấu hình Zed cho ROLE và playbook | Provider và model được phép, ngưỡng, chính sách hạn mức, danh sách đường dẫn bí mật, **trần allowlist mạng**, ROLE, playbook |
| Crew project-scope | Khóa `crew` trong `.zed/settings.json` của checkout chính | Bước verification, quyền môi trường, GitHub, loại chủ sở hữu repo, allowlist mạng của project, thu hẹp provider |
| Quy ước trong target repo | `AGENTS.md`, `CLAUDE.md`, `.claude/` | Provider tự đọc |

### 5.2 Phác thảo

```jsonc
// settings.json (user-scope)
{
  "crew": {
    "enabled_providers": ["claude-acp", "codex-acp", "antigravity-acp"],
    "orchestrator_default_provider": "claude-acp",
    "allowed": { "codex-acp": { "models": ["*"], "efforts": ["*"] } },
    "pinned_adapters": { "claude-acp": "0.95.0", "codex-acp": "2.2.2", "antigravity-acp": "1.4.3" },
    "quota": {
      "near_limit_percent_left": null,   // brief Q7
      "unknown": "skip"                  // "skip" | "ask_human"; không có giá trị nào coi là còn đủ (NEED 26)
    },
    "snapshot_max_age": null,                                          // brief Q7
    "parallel": { "global": null, "per_provider": { "antigravity-acp": null } },  // brief Q7
    "secret_paths": ["~/.ssh", "~/.aws", "~/.config/gh", "~/Library/Keychains", "~/.gnupg", "~/.netrc"],
    "network_allowlist_ceiling": [],    // trần; project chỉ chọn trong trần
    "model_service": { "sources": ["models.dev", "terminal-bench-2.1"] },
    "default_candidates": {             // xếp hạng mặc định theo loại nhiệm vụ (mục 8)
      "implement": [{ "provider": "codex-acp", "model": "<id>", "effort": "<mức>" }]
    }
  }
}

// .zed/settings.json (project-scope, checkout chính)
{
  "crew": {
    "github": { "repository": "owner/repo" },
    "verification": {
      "baseline": [{ "label": "build", "command": "<lệnh>" }],
      "steps": [{ "label": "test", "command": "<lệnh>", "network": false }]
    },
    "permissions": { "merge": true, "release": false, "environments": [] },
    "network_allowlist": [],            // ⊆ network_allowlist_ceiling
    "secret_paths_extra": [],           // chỉ được thêm
    "providers": { "deny": [] },
    "artifacts_path": null              // chốt ở Kickoff
  }
}
```

Trong `secret_paths`, ba mục `~/.ssh`, token của GitHub CLI và `~/Library/Keychains` đã được spike chứng minh là đọc được khi không cấu hình gì thêm **[Spike]**. Các mục còn lại là đề xuất mặc định; Human chỉnh ở user-scope.

### 5.3 Quy tắc gộp

```text
effective.providers         = user.enabled_providers − project.providers.deny
effective.allowed           = user.allowed ∩ (project thu hẹp nếu có)
effective.network_allowlist = project.network_allowlist ∩ user.network_allowlist_ceiling
effective.secret_paths      = user.secret_paths ∪ project.secret_paths_extra
effective.permissions       = project.permissions          (NEED 06: quyền môi trường theo project)
effective.verification      = project.verification         (chỉ có ở project-scope)
```

- Khóa project-scope cố nới rộng thì bị bỏ qua và ghi cảnh báo.
- Cấu hình đã gộp được lưu snapshot kèm hash vào session lúc bắt đầu (NEED 28).
- **Không dùng giá trị đã gộp của Zed:** Zed cho project ghi đè user, ngược với quy tắc chỉ thu hẹp. Crew đọc riêng user-scope bằng `SettingsStore::raw_user_settings()` và project-scope bằng `SettingsStore::local_settings(root_id)`, chỉ lấy file ở gốc worktree **[Đã kiểm]**.
- **Restricted Mode:** với worktree chưa được tin cậy, Zed không nạp `.zed/settings.json` (`project_settings.rs:1362-1384`), nên `local_settings()` rỗng **[Đã kiểm]**. Crew phát hiện trường hợp này, dừng và chỉ dẫn Human tin cậy worktree; không chạy khi thiếu chính sách project.
- **Nguồn đọc:** Crew đọc project-scope từ checkout chính của Human, không bao giờ từ worktree của worker. Workspace của worker cũng nạp `.zed/settings.json` của worktree, nên Crew bỏ qua khóa `crew` ở các workspace này. Worker sửa khóa `crew` trong `.zed/settings.json` thì thay đổi bị đánh dấu trong bàn giao và cần Human quyết trước khi merge.

### 5.4 ROLE và playbook

ROLE là file Markdown có front matter, nằm trong `crew/roles/`. Playbook nằm trong `crew/playbooks/`.

```markdown
---
id: software-engineer
version: 3
task_kinds: [implement, debug]
allowed:                    # chỉ thu hẹp tập được phép ở user-scope
  providers: [claude-acp, codex-acp]
  efforts: [medium, high]
tools_deny: [WebFetch]      # map sang kênh chặn tool của từng provider (mục 6.1)
handoff_schema: crew.handoff/1
seed: ["<nguồn theo brief 5.3.1>"]
---
<trách nhiệm, chuẩn đầu ra, quy ước bàn giao, prompt của role>
```

- Seed theo bản đồ nguồn của brief 5.3.1. Release Manager, Platform/DevOps Engineer, Delivery Manager và Business Analyst được viết mới (NEED 55).
- Mỗi lần dùng, Crew lưu nội dung và hash của ROLE và playbook vào run store.
- Nạp ROLE không tự cấp quyền cho tool trong profile; quyền vẫn theo NEED 06.

---

<a id="muc-6"></a>

## 6. Lớp provider (`crew_acp`)

### 6.1 ProviderProfile

```rust
// Phác thảo; tên có thể đổi khi implement.
pub trait ProviderProfile {
    fn agent_id(&self) -> &'static str;                               // id gốc trong registry
    fn crew_agent_id(&self) -> &'static str;                          // id của Crew, ví dụ "crew-claude-acp"
    fn role_options(&self, role: &RoleDefinition) -> Result<SessionOptions>;
    fn worker_options(&self, policy: &WorkerPolicy) -> SessionOptions; // sandbox, chặn tool, giới hạn
    fn orchestrator_options(&self, policy: &OrchestratorPolicy) -> SessionOptions;
    fn worker_mode(&self) -> &'static str;
    fn orchestrator_mode(&self) -> &'static str;
    fn forbidden_modes(&self) -> &'static [&'static str];
    fn isolation(&self) -> IsolationLevel;
    fn auth_confirmation(&self) -> AuthConfirmation;
    fn quota_source(&self) -> QuotaSource;
    fn compaction_signal(&self) -> CompactionSignal;
    fn effective_context_tokens(&self, model: &ModelId) -> Option<u64>;
    fn supports_steering(&self) -> bool;
}
```

| Thuộc tính | Claude (`claude-acp`) | Codex (`codex-acp`) | Antigravity (`antigravity-acp`) |
| ---------- | --------------------- | ------------------- | ------------------------------- |
| Kênh ROLE | Trường ACP `systemPrompt`, mode `append`; dự phòng `_meta.systemPrompt` dạng object `{append}` **[Spike]** | `_meta.jetbrains.air.customInstructions` → `developerInstructions` của `threadStart`; giá trị phải là chuỗi **[Đã kiểm]**; chưa chạy thật (S11) | `_meta.antigravity.systemPrompt.append`, giá trị phải là chuỗi; kiểu khác bị bỏ qua im lặng **[Spike]** |
| ROLE khi load/resume | Option của request load/resume thay cho option cũ; thiếu `systemPrompt` thì về preset trần **[Spike]** | Adapter không đọc `customInstructions` khi load/resume **[Đã kiểm]**; ROLE có được giữ trong thread không thì chưa rõ (S11) | Không được lưu; phải gửi ngay từ lần load/resume đầu tiên. History đã có lượt thiếu ROLE thì gửi lại có thể không cứu được **[Spike]** |
| ROLE sau compaction | Còn **[Spike]** | Chưa kiểm | Còn **[Spike]** |
| Chặn tool theo ROLE | `settings.permissions.deny`; adapter cũng gộp `disallowedTools` của client (`acp-agent.js:8730`) **[Đã kiểm]** | Chưa có kênh qua ACP; ghi “không áp được” vào bản ghi spawn | `_meta.agy.disabledTools` **[Spike: đọc mã, chưa chạy thật]** |
| Mode worker | `default` | `workspace-write`. Mode mặc định của adapter là `agent`, dùng auto review nên ít yêu cầu tới gate hơn **[Đã kiểm]**; Crew đặt mode tường minh | `default` |
| Mode Orchestrator ([7.5](#muc-7-5)) | `default` | `read-only` **[Đã kiểm]** | `default` |
| Mode bị cấm | `bypassPermissions` | `agent-full-access` | `yolo` |
| Ghi file | Thẳng xuống đĩa; không gọi `fs/*` hay `terminal/*` của Zed **[Đã kiểm]** | Thẳng xuống đĩa; không gọi `fs/*` hay `terminal/*` của Zed **[Đã kiểm]** | Qua `fs/write_text_file` của Zed **[Baseline]**. Zed nhận cả đường dẫn trong worktree ẩn, tức là ngoài project, mà không xin quyền **[Đã kiểm]**; H12 giới hạn vào worktree của worker |
| Mức cách ly | Trung bình: sandbox chặn ghi và mạng, chặn đọc theo danh sách chặn ([10.4](#muc-10-4)) | Trung bình: sandbox OS chặn ghi ngoài worktree, mạng tắt; vẫn ghi được `$TMPDIR` và `/tmp` **[Đã kiểm]** | Thấp: tài khoản cá nhân không có sandbox **[Baseline]** |
| Xác nhận đăng nhập | Healthcheck bắt buộc; `_auth/status_update` báo sai khi đăng nhập bằng `CLAUDE_CODE_OAUTH_TOKEN` **[Spike]** | `_auth/status_update` báo đúng ngay sau initialize **[Spike]**; healthcheck dự phòng | Lỗi `-32000` khi tạo session, rồi healthcheck **[Spike]** |
| Nguồn hạn mức | `usage_update._meta["_claude/rateLimit"]` (H4) **[Đã kiểm]**; lệnh `/usage` (định dạng chờ S12) | Lệnh `/status`, adapter tự trả, không gọi model **[Spike]** | Lệnh `/usage`, adapter tự trả **[Spike]** |
| Tín hiệu compaction | `compaction_update` **[Spike]** | `compaction_update` **[Baseline]** | Không có; dấu hiệu là `usage_update.used` tụt mạnh **[Spike]** |
| Ngưỡng context thực tế | Theo `size` của usage update | Theo `size` của usage update | Khoảng 256 nghìn token, dù adapter báo khoảng 1 triệu **[Spike]** |
| Chi phí cố định | — | — | Prompt đầu tiên của mỗi session tốn khoảng 12,7 nghìn token chỉ dẫn có sẵn **[Spike]** |
| Steering | `_session/steering` | `_session/steering` | Không có: cancel rồi gửi prompt mới |
| Hỏi người dùng (elicitation) | Zed khai elicitation dạng form, nên adapter giữ tool `AskUserQuestion`; câu hỏi, MCP elicitation và hộp thoại dự phòng đi qua `elicitation/create` (`acp-agent.js:8586, 7779-7783, 8710-8724`) **[Đã kiểm]**. Crew chặn `AskUserQuestion` bằng `disallowedTools` và từ chối mọi elicitation | `request_user_input` đi qua elicitation dạng form, có thể không có timeout (`index.js:32938-32961`) **[Đã kiểm]**; Crew từ chối | Chưa kiểm; Crew từ chối mọi elicitation |
| Session thăm dò | Dùng lại qua resume | Dùng lại qua resume | Được lưu, hiện trong `session/list`, không xóa được qua ACP **[Spike]**; luôn dùng lại |

<a id="muc-6-2"></a>

### 6.2 Payload tạo session

Crew sinh payload từ ProviderProfile, không ghép tay. Payload dưới đây đã kiểm bằng spike, trừ chỗ có ghi chú.

**Claude worker:**

```jsonc
{ "method": "session/new", "params": {
  "cwd": "/abs/worker-worktree",
  "mcpServers": [{ "name": "crew", "command": "zed", "args": ["--crew-mcp-bridge", "<socket>"] }],
  "systemPrompt": { "mode": "append", "content": [{ "type": "text", "text": "<ROLE>" }] },
  "_meta": { "claudeCode": { "options": {
    "sandbox": {
      "enabled": true, "failIfUnavailable": true, "allowUnsandboxedCommands": false,
      "network": { "allowedDomains": ["<effective.network_allowlist>"], "strictAllowlist": true },
      "filesystem": { "denyRead": ["<effective.secret_paths>"], "denyWrite": ["<$TMPDIR thật của sandbox>"] }
    },
    "settings": { "permissions": { "deny": ["Read(//<đường dẫn bí mật>/**)", "<tool bị cấm theo ROLE>"] } },
    "disallowedTools": ["AskUserQuestion"],
    "settingSources": ["project"],
    "maxBudgetUsd": "<theo cấu hình>", "maxTurns": "<theo cấu hình>"
  } } }
} }
```

- `settings` luôn là object. Nếu là chuỗi, adapter coi đó là đường dẫn file tương đối theo cwd và tự đọc file (`acp-agent.js:8630-8633`) **[Đã kiểm]**; Crew không tạo file nào (NEED 61).
- `$TMPDIR` của sandbox luôn là `/tmp/claude-501` trên máy Human, kể cả khi session tự đặt `TMPDIR` **[Spike]**. Contract test ghi giá trị thật theo phiên bản adapter; Crew không đoán.
- `credentials.files` là cách thay thế đã kiểm cho phần chặn đọc của Bash **[Spike]**.
- `maxBudgetUsd` và `maxTurns` được adapter chuyển cho SDK **[Đã kiểm]**; hiệu lực khi chạy thật chờ S9.
- Dự phòng ROLE `_meta.systemPrompt` phải là object `{ "append": "<ROLE>" }`. Dạng chuỗi sẽ thay toàn bộ prompt có sẵn của Claude Code **[Spike]**.
- `disallowedTools: ["AskUserQuestion"]` và `settingSources: ["project"]` có ở mọi session Crew trên Claude (worker, Orchestrator, thăm dò). Adapter gộp `disallowedTools` của client vào danh sách của nó (`acp-agent.js:8730`) **[Đã kiểm]**. Câu hỏi của worker đi qua bàn giao `needs_context`, của Orchestrator qua `crew_ask_human` ([10.1](#muc-10)).
- `settingSources: ["project"]` thay giá trị mặc định `["user", "project", "local"]` của adapter; option của client được trải sau giá trị mặc định nên ghi đè được (`acp-agent.js:8668, 8688`) **[Đã kiểm]**. Nhờ vậy cấu hình user của Human và `.claude/settings.local.json` sót trong worktree không đổi được quyền của worker. Giữ `project` để `CLAUDE.md` và `.claude/settings.json` của repo vẫn có hiệu lực, và để không đổi cách đăng nhập **[Suy luận]**; contract test xác nhận, kèm việc luật deny của Crew thắng cấu hình trong worktree.

**Antigravity worker:**

```jsonc
{ "method": "session/new", "params": {
  "cwd": "/abs/worker-worktree", "mcpServers": [ /* crew */ ],
  "_meta": { "antigravity": { "systemPrompt": { "append": "<ROLE>" } } }
} }
```

Sau khi tạo, Crew đặt `mode` = `default`, rồi `model` và `thinking_level` bằng `session/set_config_option`. Đổi model không làm mất ROLE, vì adapter dựng lại agent với cùng system prompt **[Spike]**.

**Codex worker:**

```jsonc
{ "method": "session/new", "params": {
  "cwd": "/abs/worker-worktree", "mcpServers": [ /* crew */ ],
  "_meta": { "jetbrains": { "air": { "customInstructions": "<ROLE>" } } }
} }
```

Sau khi tạo, Crew đặt `mode` = `workspace-write`, rồi `model` và `reasoning_effort`.

**Không có `additionalDirectories` trong session worker** (mọi provider). Zed gửi mọi thư mục làm việc sau thư mục đầu dưới dạng `additionalDirectories` (`v1.rs:1157-1175`), và Codex thêm chúng vào vùng được ghi của sandbox (`codex-acp` `index.js:35878-35885`) **[Đã kiểm]**. Session worker chỉ có một thư mục làm việc là worktree của nó.

**Load và resume** chỉ đi qua CrewConnection ([3.3](#muc-3-3)) và luôn gửi lại **toàn bộ** SessionOptions, không chỉ ROLE:
- Claude đọc lại `_meta.claudeCode.options` khi load/resume; danh sách option buộc dựng lại session nằm ở `OPTION_REBUILDS_SESSION` (`acp-agent.js:395`), trong đó có `sandbox`, `settings`, `systemPrompt` **[Đã kiểm]**.
- Claude và Codex đọc `mcpServers` của request load/resume **[Đã kiểm]**. Thiếu MCP của Crew thì Orchestrator mất công cụ.

<a id="muc-6-3"></a>

### 6.3 SessionOptions (H2)

```rust
pub struct SessionOptions {
    pub system_prompt: Option<acp::SystemPrompt>,       // đặt khi agent khai capability systemPrompt
    pub meta: Option<serde_json::Map<String, Value>>,   // `_meta` của request
    pub mcp_servers: McpServerSelection,
}
pub enum McpServerSelection { ProjectDefault, Explicit(Vec<acp::McpServer>) }
```

- Session của Crew luôn dùng `Explicit`. Worker và Orchestrator chỉ có MCP của Crew; Orchestrator nhận thêm MCP của project mà profile cho phép. Session thăm dò không có MCP nào (NEED 02).
- Có `meta` hay `system_prompt` mà connection không hỗ trợ thì trả lỗi, không bỏ qua.
- Crew tự kiểm kiểu giá trị ROLE (chuỗi với Codex và Antigravity, mảng text block với Claude) trước khi gửi, vì hai adapter bỏ qua kiểu sai mà không báo lỗi.
- Thư mục làm việc truyền qua `work_dirs`, không nằm trong `SessionOptions`. Session worker chỉ có một thư mục ([6.2](#muc-6-2)).
- Bản `*_with_options` không áp mode và config option mặc định từ settings `agent_servers.<agent id>` ([3.3](#muc-3-3)); Crew tự đặt mode và config option, rồi ModeGuard kiểm ([10.2](#muc-10)).

### 6.4 Adapter được ghim (H10)

- Mỗi provider có một agent id của Crew, cài đúng bản ghim. Bản Zed tự cập nhật vẫn phục vụ thread thường của Human.
- Agent id của Crew được đăng ký vào agent server store trước khi panel nạp, và đăng ký lại mỗi lần Zed dựng lại danh sách agent (H10b, [3.3](#muc-3-3)).
- Hai bản dùng chung đăng nhập, vì credential nằm ở tầng provider.
- Đổi bản ghim chỉ khi Human đồng ý, sau khi bộ contract test ([mục 17](#muc-17)) đạt, và chỉ áp cho session mới.

<a id="muc-6-5"></a>

### 6.5 Xác nhận đăng nhập (NEED 56)

Chạy mỗi lần Zed khởi động, cho từng provider đã bật:

```text
connect(agent id của Crew) → initialize
match profile.auth_confirmation:
  StatusPush (Codex):  chờ `_auth/status_update` trong thời hạn cấu hình
                       kind = "account" → confirmed; kind = "none" → not_logged_in; không nhận được → Healthcheck
  Healthcheck (Claude) | SessionError + Healthcheck (Antigravity):
      s = probe_session(provider)   // resume bản ghi kind=probe; không có thì tạo mới trong thư mục thăm dò
      tạo session lỗi -32000        → not_logged_in        (Antigravity báo ở bước này)
      prompt "ping" có trả lời      → confirmed
      prompt lỗi -32000             → not_logged_in        (Claude báo ở bước này)
      lỗi khác hoặc quá hạn         → unconfirmed, kèm chi tiết
ghi crew_health; readiness (NEED 23) và UI đọc từ đây
```

- **Session thăm dò** có ROLE thăm dò (“chỉ trả lời `pong`, không dùng công cụ”), nên mọi session Crew đều có ROLE. Gate từ chối mọi yêu cầu xin quyền và mọi elicitation của session này; H12 từ chối mọi `fs/*`.
- **Thư mục thăm dò** là `crew/probe/` trong thư mục dữ liệu của Zed, nên session thăm dò không thuộc project nào. Thread import vẫn liệt kê session của mọi cwd, nên H5e lọc nó ra.
- **Chi phí:** healthcheck tốn hạn mức, nên luôn dùng lại session thăm dò. Với Antigravity, khoảng 12,7 nghìn token rơi vào prompt đầu tiên của session thăm dò **[Spike]**. Các lần ping sau trong cùng session được kỳ vọng rẻ hơn **[Suy luận]**.
- **`not_logged_in`:** Crew chỉ dẫn Human tới luồng đăng nhập của Zed (NEED 17, 56), không tự đăng nhập.

### 6.6 Snapshot model (NEED 56)

- Snapshot cập nhật từ config option trả về mỗi khi Crew tạo, load hoặc resume session. Claude và Codex trả config option cả khi load/resume (`acp-agent.js:8379-8420`; `index.js:43049-43241`) **[Đã kiểm]**; Antigravity chưa rõ **[Suy luận]**.
- Snapshot quá ngưỡng tuổi (brief Q7) thì resume session thăm dò để lấy config option. Provider không trả config option khi resume thì tạo session mới trong thư mục thăm dò; đọc model chỉ cần `session/new`, không cần prompt **[Spike]**. Với Antigravity, mỗi lần như vậy để lại một session không xóa được.
- Sync lỗi thì giữ snapshot cũ, gắn `stale`.
- Một model id có thể có ở nhiều provider; ví dụ Antigravity liệt kê cả `claude-opus-5-5` và `claude-sonnet-5-5` **[Spike]**. Ứng viên luôn là cặp (provider, model, effort); hạn mức và mức cách ly tính theo provider.

### 6.7 Đọc hạn mức (NEED 60)

Mọi nguồn đều qua ACP (NEED 61). Lệnh `/status` và `/usage` gửi trong session thăm dò, do adapter tự trả nên không tốn lượt gọi model **[Spike]**.

| Provider | Cách đọc | Quy tắc parse |
| -------- | -------- | ------------- |
| Codex | `/status` | Mỗi dòng `**<limit> <label>:** <N>% left[ (resets <time>)]`, với `<label>` ∈ `Nm limit`, `Nh limit`, `Nd limit`, `Weekly limit`, `Limit` **[Spike]**. Giờ reset theo múi giờ và locale của process adapter, không có năm, nên chỉ parse best effort; không parse được thì giờ reset là `unknown`, phần trăm vẫn dùng. **Bỏ dòng `**Account:**`** (chứa email) trước khi lưu |
| Antigravity | `/usage` | Markdown: nhóm `- **<group>**`, dòng `*(Models within this group: …)*`, cửa sổ `Weekly Limit Remaining` và `Five Hour Limit Remaining` với `N% remaining` hoặc `N units remaining`, giờ reset UTC tiếng Anh **[Spike]**. Output đặc biệt (“Unable to retrieve…”, “data not available yet”, “No quota limits reported.”, “Not applicable…”) → `unknown` |
| Claude | `_meta["_claude/rateLimit"]` trên usage update (H4); `/usage` khi cần | Cửa sổ 5 giờ và 7 ngày (`utilization`, `resets_at`) theo mã adapter **[Đã kiểm]**; định dạng thật chờ S12. Healthcheck lúc khởi động cũng làm mới giá trị này |

- **Gắn model với nhóm hạn mức:** model thuộc một nhóm chỉ khi provider báo nhóm đó chứa model ấy. Bảng so khớp tên hiển thị với model id nằm trong ProviderProfile và được khóa bằng contract test.
  - Ví dụ: Antigravity ghi nhóm “Gemini Models” gồm “Gemini Flash, Gemini Pro”, không nhắc Gemini 4 Argon **[Spike]**. Vì vậy hạn mức của Argon là `unknown` cho tới khi S13 xác định được nhóm của nó.
- **Một nhóm có nhiều cửa sổ** (tuần, 5 giờ): ứng viên chỉ đạt khi mọi cửa sổ đều trên ngưỡng.
- **Cập nhật:** Crew đọc lại trước khi chọn nếu bản ghi đã quá thời hạn cấu hình; usage update của worker được ghi theo thời gian thực.

### 6.8 Compaction và ngân sách context (NEED 31)

```text
compaction_detected khi:
  provider gửi compaction_update                                     (Claude, Codex)
  hoặc used_trước − used_sau ≥ tỷ lệ cấu hình × used_trước,
       và used_trước ≥ tỷ lệ cấu hình × ngưỡng context thực tế      (Antigravity)
```

- Ví dụ thật ở Antigravity: `used` tụt từ khoảng 210 nghìn xuống 23 nghìn token khi compaction **[Spike]**.
- Ngưỡng context thực tế của Antigravity (khoảng 256 nghìn token) lưu trong ProviderProfile và được đo lại khi đổi bản ghim adapter.
- Task packet và input của worker phải nhỏ hơn nhiều so với ngưỡng này, theo tỷ lệ trong cấu hình; vượt thì Orchestrator chia nhỏ task.
- ROLE còn sau compaction (Claude, Antigravity), nên Crew không gửi lại. Orchestrator dùng `crew_get_state` để nắm lại trạng thái.

---

<a id="muc-7"></a>

## 7. Orchestrator và công cụ MCP (`crew_mcp`)

### 7.1 Tạo session Crew

1. Human chạy “Crew: New Session” (H5a) và chọn provider; mặc định là Claude (NEED 24).
2. Crew kiểm `crew_health` của provider. Chưa `confirmed` thì dừng và chỉ dẫn đăng nhập.
3. Action mở một thread Agent Panel bằng `AgentPanel::create_thread_with_options` với `agent` là agent id của Crew. Không dùng action `NewExternalAgentThread`, vì action này đổi agent đang chọn của panel **[Đã kiểm]**. Zed gọi `CrewConnection::new_session` (H5d), hàm này:
   - tạo bản ghi `crew_sessions` và `crew_acp_sessions` (kind `orchestrator`);
   - tạo `McpServer` riêng cho session, đăng ký handler `initialize` và bộ công cụ Orchestrator ([7.2](#muc-7));
   - gọi `new_session_with_options` với ROLE Orchestrator, option chỉ đọc ([7.5](#muc-7-5)) và MCP server stdio `zed --crew-mcp-bridge <socket>`.
4. Model của Orchestrator theo cấu hình user-scope; Human đổi được bằng bộ chọn model của thread. Đây là điểm vào duy nhất để giao việc (NEED 09).

**Agent đang chọn của panel.** `create_thread_with_options` trả lại agent đang chọn sau khi tạo thread (`agent_panel.rs:3233-3265`), nhưng mỗi khi một thread thành view đang hiện, `set_base_view` gán agent đang chọn theo agent của thread đó và lưu lại (`agent_panel.rs:4354`) **[Đã kiểm]**. Không có H5f, sau khi Human xem một thread Crew, nút “+” của panel sẽ tạo session Crew. Vì vậy agent id của Crew không bao giờ thành agent đang chọn (H5f), và `CrewConnection::new_session` từ chối mọi lời gọi không đến từ “Crew: New Session”.

### 7.2 Cầu nối MCP (H1)

Công cụ của Crew đi qua MCP trên Unix socket với cầu nối stdio, không qua MCP HTTP trên localhost, để socket chỉ mở cho tiến trình cùng user trong thư mục 0700 ([2.3](#muc-2)).

- `zed --crew-mcp-bridge <socket>` kết nối socket, sao chép hai chiều stdin → socket và socket → stdout.
- Thoát mã 0 khi stdin EOF hoặc socket đóng; mã khác 0 khi không kết nối được.
- Không xử lý nội dung, không log payload.
- Framing của `McpServer` là JSON-RPC, mỗi message một dòng (`listener.rs:360-394`) **[Đã kiểm]**, nên cầu nối chỉ chép byte. Việc cả ba provider khởi động được MCP stdio này kiểm ở S7.

`McpServer` cần thêm ba điều **[Đã kiểm]**:
- **`initialize`:** server chỉ trả lời `tools/call`, `tools/list` và handler đã đăng ký; method khác nhận lỗi “unhandled method” (`listener.rs:214-242`). Crew đăng ký handler `initialize` bằng `handle_request`. Notification không có id bị bỏ qua (`listener.rs:215-217`).
- **Quyền thư mục:** thư mục socket được tạo với quyền mặc định, nên Crew đặt về 0700 ([2.3](#muc-2)).
- **Thu hồi khi Stop:** connection đang mở vẫn giữ bảng công cụ sau khi server bị drop (`listener.rs:206-242`), nên drop server không thu hồi quyền. Mọi công cụ kiểm trạng thái session trước khi chạy và từ chối khi session đã dừng.

### 7.3 Bộ công cụ

Mọi công cụ kiểm input theo JSON schema, cắt output dài kèm con trỏ, trả lỗi có hướng xử lý (NEED 33), và không nhận secret.

**Orchestrator:**

| Công cụ | Việc | Kiểm tất định | NEED |
| ------- | ---- | ------------- | ---- |
| `crew_get_state` | Projection gọn của session | — | 31 |
| `crew_set_session_type` | Loại session, OUTPUT chính, tiêu chí, quy mô | Loại có trong bảng Loại session | 18 |
| `crew_ask_human` | Ghi bộ câu hỏi sẽ trình Human | Mỗi câu có bối cảnh, ví dụ, khuyến nghị; giả định tách riêng; từ chối câu trùng chủ đề với quyết định đã chốt | 10, 52 |
| `crew_update_pipeline` | Phiên bản pipeline mới, kể cả chèn nhánh | DAG không chu trình; task đủ ROLE, mục tiêu, tiêu chí, phạm vi ngoài, dependency; task chạm dữ liệu, quyền, ranh giới tin cậy phải có abuse case và rollback | 21, 27, 29 |
| `crew_spawn_worker` | Chạy quy trình chọn ở [mục 8](#muc-8) rồi spawn | Task phải `ready`; lựa chọn nằm trong danh sách xếp hạng; chỉ chọn ứng viên xếp sau ứng viên đầu tiên đạt điều kiện khi có lý do ghi lại | 26, 29, 30 |
| `crew_worker_status` · `crew_get_handoff` | Trạng thái, bàn giao | — | 32, 50 |
| `crew_steer_worker` · `crew_stop_worker` | Đổi yêu cầu, dừng | Theo khả năng provider | 16, 36 |
| `crew_run_verification` | Sensor của project trong worktree | Chỉ lệnh trong cấu hình project | 46 |
| `crew_request_evaluation` | Spawn evaluator độc lập | Khác tác giả, context mới | 46 |
| `crew_register_problem` · `crew_record_attempt` | Đếm giải pháp đã thử | Lần thứ 4 bị chặn | 37 |
| `crew_integrate` | Gộp nhánh worker vào integration branch | Xung đột → task giải quyết hoặc hỏi Human | 34, 35 |
| `crew_github_*` | Issue, PR, review, merge | Merge chỉ khi head bằng bản Human đã duyệt | 05, 14, 35, 39 |
| `crew_write_artifact` | Tài liệu quyết định, báo cáo research, bản đồ quyết định, bản đồ tham chiếu | Phải có nguồn dẫn xuất | 11, 20, 49, 51 |
| `crew_propose_done` | Đề nghị đánh giá Done | DoneEvaluator quyết | 45 |

**Worker:** `crew_get_contract`, `crew_submit_handoff`, `crew_report_blocker`.

**Evaluator:** `crew_get_evaluation_packet`, `crew_submit_verdict`.

### 7.4 Ghi nhận quyết định

1. `crew_ask_human` ghi bộ câu hỏi và phiên bản INPUT/OUTPUT đang trình.
2. Tin nhắn kế tiếp của Human trong thread Orchestrator được gắn tất định với các câu hỏi đang mở; Crew lưu nguyên văn và id tin nhắn do Crew cấp.
3. Orchestrator ghi phần diễn giải riêng. Human sửa bằng tin nhắn sau, tạo quyết định thay thế.
4. Tin nhắn khác của Human tạo phiên bản INPUT mới (NEED 36).

**Nhận ra tin nhắn của Human.** Tin nhắn người dùng trong `AcpThread` không có id ổn định (`MessageIdentity::Legacy(None)`, `acp_thread.rs:6559-6566`); `SubmissionId` chỉ là bộ đếm trong bộ nhớ (`submission.rs:20`); composer và prompt do code gửi đều gọi `thread.send` (`thread_view.rs:2110`), nên không có cờ nguồn gốc **[Đã kiểm]**. Crew nghe `NewEntry` (`acp_thread.rs:5160`), ghi lại mọi prompt chính Crew gửi trước khi gửi, cấp id riêng cho từng tin nhắn, và coi các tin nhắn người dùng còn lại trong thread Orchestrator là của Human.

Elicitation bị từ chối ([10.1](#muc-10)), nên câu trả lời trong form của provider không bao giờ thành quyết định.

<a id="muc-7-5"></a>

### 7.5 Orchestrator chỉ đọc

Orchestrator chạy trên checkout chính của Human. Để agent không sửa cùng worktree với Human (tinh thần NEED 34), Orchestrator chỉ đọc; việc sửa code luôn đi qua worker trong worktree riêng.

| Provider | Mode | Lớp chặn sửa file và chạy lệnh |
| -------- | ---- | ------------------------------- |
| Claude | `default` | Gate từ chối mọi yêu cầu sửa file và chạy lệnh; `settings.permissions.deny` chặn `Write`, `Edit`, `NotebookEdit`, `Bash` |
| Codex | `read-only`: cần xin quyền để sửa file và truy cập mạng **[Đã kiểm]** | Sandbox chỉ đọc; gate từ chối yêu cầu sửa file và nới quyền |
| Antigravity | `default` | Gate từ chối yêu cầu sửa file và chạy lệnh; H12 từ chối mọi `fs/write_text_file` |

Hệ quả: Orchestrator trên Claude không chạy được lệnh, kể cả lệnh chỉ đọc như `git log`; việc cần lệnh được giao cho worker.

**Antigravity cần H12.** Antigravity ghi file qua `fs/write_text_file` của Zed **[Baseline]**, và Zed áp lệnh ghi này mà không qua yêu cầu xin quyền **[Đã kiểm]** (xem H12), nên gate một mình không giữ được Orchestrator Antigravity ở chế độ chỉ đọc. Chừng nào H12 chưa có, Crew không cho chọn Antigravity làm Orchestrator.

---

<a id="muc-8"></a>

## 8. Chọn model và spawn worker (NEED 26)

Một quy trình cho mọi provider và model:

```text
select(task) -> Selected | Blocked
  role   = roles.load(task.role)                                   // (1) ROLE và tập được phép của ROLE
  if session.at_cap()                { return Blocked("chạm trần session") }
  ranked = models.rank(task.kind, task.constraints)                // (2) chỉ (provider, model, effort) có trong snapshot;
                                                                   //     chưa có dữ liệu thì dùng default_candidates
                                                                   //     và trong effective.allowed ∩ role.allowed;
                                                                   //     mức cách ly là một yếu tố xếp hạng (NEED 60)
  skipped = []
  for c in ranked:                                                 // (3) theo thứ tự xếp hạng
    if health(c.provider) != confirmed { skip(c, "chưa xác nhận đăng nhập"); continue }
    if !isolation_ok(c.provider, task) { skip(c, "mức cách ly không đủ cho loại việc"); continue }
    q = quota.for_model(c)                                         // mọi cửa sổ của nhóm hạn mức chứa model
    if q.unknown():
      if config.quota.unknown == "ask_human" { return Blocked("hạn mức không rõ", ranked, skipped, c) }
      skip(c, "hạn mức không rõ"); continue
    if q.any_window_at_or_below(config.quota.near_limit_percent_left)
                                       { skip(c, "sắp hết hạn mức"); continue }
    return Selected { candidate: c, ranked, skipped, quota: q }    // (4) ứng viên đầu tiên đạt điều kiện
  return Blocked("không ứng viên nào đạt điều kiện", ranked, skipped)   // dừng phần việc, hỏi Human
```

- **Mức cách ly** vừa là yếu tố xếp hạng (NEED 60) vừa là điều kiện cứng:
  - Task cần chạy lệnh hay ghi file trên provider không có sandbox (Antigravity tài khoản cá nhân) chỉ qua được khi project cho phép, và mọi lệnh phải khớp allowlist của gate.
  - Task research, review, brainstorm qua được ở mọi mức.
- **Hai loại trần:**
  - Trần session (số worker, chi phí của cả session) chạm thì `Blocked` và hỏi Human (NEED 26).
  - Trần song song (NEED 30) thì không làm ứng viên bị bỏ qua: task được xếp hàng, và quy trình chọn chạy lại lúc lấy task ra, vì hạn mức có thể đã đổi.
- **Bản ghi spawn** gồm `ranked`, `skipped` kèm lý do, hạn mức đã thấy, mức cách ly, rủi ro còn lại, phiên bản adapter, kênh ROLE và hash của option (`crew_spawns`).
- **Xếp hạng mặc định:** trước khi dịch vụ model (mục 13) có dữ liệu, `rank` trả danh sách `default_candidates` của loại nhiệm vụ trong cấu hình user-scope (Human đồng ý 2026-10-25). Sau đó khóa này chỉ còn là dự phòng; bản ghi spawn ghi rõ nguồn xếp hạng.
- **Ví dụ:** ứng viên đầu là GPT-5.6 Sol trên Codex nhưng tài khoản sắp hết hạn mức. Crew bỏ qua với lý do “sắp hết hạn mức” và xét ứng viên kế tiếp. Gemini 4 Argon trên Antigravity chỉ được chọn khi hạn mức của nó đo được (S13); trong lúc chờ, Argon bị bỏ qua với lý do “hạn mức không rõ”, hoặc Human được hỏi nếu chính sách là `ask_human`.

**Spawn sau khi chọn:**
1. Tạo git worktree cho task trên một nhánh riêng có tên bằng `GitStore::create_worktree` với đích `NewBranch` (`git_store.rs:9276`), rồi dựng `Project::local` không cửa sổ cho worktree đó (`project.rs:1232`) **[Đã kiểm]**.
2. Tạo session qua `CrewConnection` trên Project đó, với đúng một thư mục làm việc ([6.2](#muc-6-2)); gộp `role_options` với `worker_options` của profile; ghi `crew_acp_sessions` (kind `worker`).
3. Đặt mode và config option. Đối chiếu giá trị agent trả về với lựa chọn; lệch thì hủy session và báo lỗi.
4. Đăng ký nghe event: gate, elicitation, ModeGuard, usage, compaction, update bị bỏ, dừng.
5. Gửi task packet làm prompt đầu tiên. Thread worker hiện ở chế độ chỉ đọc (H5b) từ workspace của Orchestrator.

Không dùng `create_worktree_workspace` của `git_ui_core`: hàm này cần `Workspace` và `Window` có gốc là `MultiWorkspace` (`worktree_service.rs:736-742, 1164`), từ chối chạy khi đang có lần tạo khác (`:781`), tạo worktree detached (`:504`), và thêm một Workspace cùng Project đầy đủ cho mỗi worker vào cửa sổ và sidebar của Human (`:1365-1367`) **[Đã kiểm]**.

---

<a id="muc-9"></a>

## 9. Pipeline và thực thi

### 9.1 Playbook → pipeline

- Playbook (user-scope, seed theo brief 5.3.1) định nghĩa workflow theo loại session.
- Orchestrator điều chỉnh bằng `crew_update_pipeline`; mỗi lần đổi là một phiên bản mới, giữ lịch sử (NEED 27).
- Workflow implement có bước **đối soát chỉ đọc trước khi implement** (seed: spec-kit `analyze`, GSD plan-checker). Bước này phải đạt thì task implement mới `ready` (NEED 22).

### 9.2 Scheduler

- Chạy task có dependency đã `accepted`; nhánh độc lập chạy song song (NEED 29).
- Trần toàn cục, theo provider, theo session; vượt trần song song thì xếp hàng (NEED 30).
- Timeout thực thi áp cho worker, không áp cho việc chờ Human (NEED 15).
- Trần chi phí:
  - Claude: `maxBudgetUsd` và `maxTurns` cho từng session.
  - Codex và Antigravity: số liệu usage cộng giới hạn ở Crew. Chi phí ước tính của Antigravity cộng thêm phần cố định của prompt đầu tiên.

### 9.3 Task packet (NEED 31)

```jsonc
{
  "task_id": "…", "session_id": "…", "pipeline_version": 3, "role": "software-engineer",
  "goal": "…",
  "inputs": [{ "artifact_id": "…", "version": 2 }],
  "scope_in": ["…"], "scope_out": ["…"],
  "acceptance_criteria": [{ "id": "AC-1", "given": "…", "when": "…", "then": "…" }],
  "abuse_cases": ["…"], "rollback": "…",
  "file_scope": ["crates/…"],
  "context_budget_tokens": 0,          // tính từ ngưỡng context thực tế của provider (mục 6.8)
  "constraints": ["…"], "deliverables": ["…"],
  "handoff_schema": "crew.handoff/1"
}
```

Nội dung từ nguồn ngoài nằm trong packet có nhãn dữ liệu (NEED 08).

### 9.4 Bàn giao (NEED 32)

```jsonc
{
  "status": "done | blocked | needs_context | partial",
  "summary": "…",
  "outputs": [{ "kind": "diff | doc | report", "location": "…", "version": "…" }],
  "evidence": [{ "kind": "command", "command": "…", "exit_code": 0 }],
  "limits": ["…"], "blockers": ["…"],
  "scope_expansion_requested": null,
  "attempt_of_problem": null
}
```

Bàn giao đi qua công cụ `crew_submit_handoff` có schema, không phân tích từ tin nhắn cuối của worker. Crew kiểm tất định; không đạt thì trả lại ngay kèm lý do:
- Đúng schema; artifact và đường dẫn có tồn tại.
- Diff của worktree khớp `outputs`; thay đổi nằm trong `file_scope`.
- File không được khai báo thì ghi vào `undeclared_files`.
- Diff không chứa secret: quét mẫu private key, token GitHub, API key. Có thì chặn commit, ghi `secret_detected` và báo Human mà không hiện giá trị (NEED 05, 17).
- Diff chạm khóa `crew` trong `.zed/settings.json` thì đánh dấu, cần Human quyết ([5.3](#muc-5)).

### 9.5 Worktree và commit

- Mỗi worker có một git worktree riêng trên nhánh riêng của task, với một `Project` không cửa sổ ([mục 8](#muc-8)); không để hai bên cùng sửa một worktree (NEED 34).
- **Worker không ghi git.** Các lớp chặn:
  - Gate từ chối lệnh git ghi khi agent xin quyền chạy lệnh.
  - Gitdir của một git worktree nằm trong `.git/worktrees/<tên>` của repo chính, tức là ngoài worktree. Sandbox `workspace-write` của Codex để `.git` chỉ đọc, nên `git commit` trong worktree bị chặn (gsd-core ADR 1239, dẫn `permissions.rs` của Codex) **[Đã kiểm qua tài liệu]**.
  - Sandbox Claude chặn ghi ngoài worktree **[Spike]**, nên lệnh git ghi trong sandbox nhiều khả năng cũng bị chặn **[Suy luận, S8]**.
  - Antigravity tài khoản cá nhân chỉ có gate.
- **Commit:** sau khi nhận bàn giao, Crew commit bằng danh tính GitHub App, **chỉ gồm**:
  - file đã theo dõi bị sửa trong `file_scope`;
  - file mới có trong `outputs`.
- **File không khai báo**, như file tạm của Claude rơi vào worktree khi `$TMPDIR` bị chặn ghi, không được commit. Crew ghi chúng trong bản ghi bàn giao và xóa khi worker kết thúc (NEED 07).
- **Tích hợp:** Crew gộp nhánh worker vào integration branch của story trong một worktree tích hợp riêng, không trong checkout của Human, rồi kiểm lại kết quả tích hợp (NEED 34, 35).
- **Prototype** nằm trên nhánh `crew/prototype/*`; gate chặn mở PR từ nhánh này (NEED 12).

### 9.6 Lỗi và lần thử (NEED 37, 38)

- Lỗi `infra` (port bận, timeout, thiếu tài nguyên, lỗi mạng, state bẩn) được phân loại theo mã thoát và mẫu đầu ra. Crew khôi phục môi trường và thử lại có giới hạn; không tính là một lần thử.
- Một lần thử = một bàn giao có `attempt_of_problem` mà kết quả kiểm là lỗi `product`. Đủ 3 lần thì vấn đề chuyển `waiting_human` và spawn cho vấn đề đó bị chặn.

### 9.7 Dừng, chờ, đổi yêu cầu

| Tình huống | Hành vi | NEED |
| ---------- | ------- | ---- |
| Stop/Cancel | Hủy mọi worker, dừng Scheduler, đối soát effect, báo phần đã dừng và tác động còn lại. Công cụ MCP của session từ chối mọi lời gọi sau khi dừng ([7.2](#muc-7)) | 16 |
| Chờ Human | `waiting_human`, không timeout; nhánh độc lập chạy tiếp | 15 |
| INPUT đổi | Phiên bản mới; phụ thuộc thành `stale`; worker nhận steering (Claude, Codex), hoặc bị cancel rồi gửi lại (Antigravity) | 36, 42 |
| Hết hạn mức giữa chừng | Không tự đổi provider. Worker chuyển `blocked` với lý do hạn mức, ghi `quota_exhausted`; Orchestrator hỏi Human (brief R6) | 26 |

---

<a id="muc-10"></a>

## 10. Gate quyền và an ninh thực thi

### 10.1 PolicyGate

Gate đăng ký `ToolAuthorizationRequested` và `ElicitationRequested` trên mọi session Crew: worker, Orchestrator ([7.5](#muc-7-5)) và session thăm dò. Yêu cầu của worker được trả lời trước khi hiện lên UI.

**Quyết định theo nội dung.** Gate quyết định cho phép hay từ chối từ nội dung yêu cầu (`rawInput`, loại tool, vị trí), rồi mới chọn option. `kind` không đủ để nhận ra yêu cầu nới sandbox của Codex: yêu cầu có `additionalPermissions` hay ngữ cảnh mạng đến dưới dạng `kind: execute` với option `allow_once` (`codex-acp` `index.js:28425, 32091, 32150-32156`), và yêu cầu permission profile đưa “grant these permissions for this turn” dưới loại `allow_once` (`index.js:32228-32234`) **[Đã kiểm]**.

**Cách chọn option.** Sau khi quyết định, gate chọn theo loại option (`kind`) của ACP, không theo tên hay id, vì mỗi provider đặt id khác nhau:

| Quyết định | Option được chọn |
| ---------- | ---------------- |
| Cho phép | Option loại `allow_once` |
| Từ chối | Option loại `reject_once`. Có nhiều option thì ưu tiên option cho agent làm tiếp (Codex `decline`) hơn option dừng lượt (Codex `cancel`) **[Đã kiểm]** |
| Không bao giờ chọn | Loại `allow_always` và `reject_always`: Claude “allow all edits in … during this session” **[Spike]**; Codex `allow_for_session`, `accept_execpolicy_amendment`, `apply_network_policy_amendment:*`, `allow_permissions_session` **[Đã kiểm]**; Antigravity “Allow Always”, “Allow Always (risky)” **[Baseline]**. Các option này nới quyền cho cả session, hoặc ghi chính sách lâu dài của provider (“…in the future”), trái NEED 07 và NEED 61 |
| Không có option phù hợp | Cancel lượt của worker và ghi blocker |

Yêu cầu sửa file của Codex chỉ có một option từ chối là `cancel`, và option này dừng lượt **[Đã kiểm]**. Khi đó Crew gửi tiếp một prompt nêu lý do từ chối để worker làm tiếp trong phạm vi.

**Không có `allow_always` trên UI.** Ở view gốc, phím tắt “allow always” trả lời yêu cầu đang chờ đầu tiên của bất kỳ session nào trong hội thoại (`conversation_view.rs:463-486`, `thread_view.rs:3012-3017`) **[Đã kiểm]**. Vì vậy view của session Crew không hiện option `allow_always` (H5b), và gate trả lời yêu cầu của worker trước khi chúng hiện lên.

**Elicitation.** Zed khai elicitation dạng form và URL với mọi agent (`v1.rs:470-474`) và đưa chúng lên dưới dạng `ElicitationRequested`, không qua `ToolAuthorizationRequested` (`acp_thread.rs:3581`) **[Đã kiểm]**. Gate không thấy chúng, worker không có composer thì không ai trả lời được, và câu trả lời trong form là một quyết định mà DecisionCapture không ghi. Crew chặn `AskUserQuestion` của Claude ([6.2](#muc-6-2)) và từ chối mọi elicitation của session Crew.

**Luật quyết định:**

| Yêu cầu | Quyết định |
| ------- | ---------- |
| Đọc hoặc sửa file trong worktree của worker | Cho phép |
| Đọc hoặc sửa file ngoài worktree, hoặc trong `effective.secret_paths` | Từ chối |
| Sửa file hoặc chạy lệnh từ Orchestrator hay session thăm dò | Từ chối ([7.5](#muc-7-5)) |
| `SandboxNetworkAccess {host}` của Claude | Cho phép khi host thuộc `effective.network_allowlist`, ngược lại từ chối. Host đã cho phép giữ hiệu lực tới hết session **[Spike]** |
| Lệnh shell trên provider có sandbox | Cho phép |
| Lệnh shell trên Antigravity tài khoản cá nhân | Chỉ lệnh khớp allowlist của project |
| Yêu cầu nới quyền sandbox của Codex (`additionalPermissions`, ngữ cảnh mạng hay permission profile trong `rawInput`) | Từ chối; S8 xem có trường hợp hợp lệ không |
| Elicitation (câu hỏi gửi người dùng, form hay URL) | Từ chối. Worker hỏi qua bàn giao `needs_context`, Orchestrator qua `crew_ask_human` |
| `fs/read_text_file`, `fs/write_text_file` qua Zed | Theo H12: chỉ trong worktree của worker; Orchestrator, session thăm dò và worker chỉ đọc bị từ chối |
| Lệnh git ghi | Từ chối ([9.5](#muc-9)) |
| Mạng ngoài allowlist | Từ chối |
| Công cụ MCP của Crew | Cho phép |
| Release, migration, tác động môi trường | Từ chối, trừ khi project có quyền và có artifact rollback đã duyệt (NEED 06) |

Mọi quyết định của gate được ghi event `gate_decision`. Bị từ chối thì worker báo blocker; Orchestrator hỏi Human trong hội thoại khi cần (NEED 13, 17).

### 10.2 ModeGuard

Kiểm mode trước khi spawn và mỗi khi có `ModeUpdated` hoặc `ConfigOptionsUpdated`, cho cả worker lẫn Orchestrator. Gặp mode bị cấm thì hủy session và ghi `mode_violation`.

Hai event này chỉ phát khi agent báo thay đổi (`acp_thread.rs:4504-4515`); thay đổi phía Zed (bộ chọn mode `v1.rs:9218`, `set_config_option` `:180`, mặc định từ settings `:1310-1345`) không phát gì **[Đã kiểm]**. Vì vậy ModeGuard đọc lại `session_modes()` và trạng thái config option sau mỗi lần Crew đổi mode hay config option và trước mỗi lượt của session. Crew không áp settings `agent_servers.<agent id>` cho agent id của Crew ([3.3](#muc-3-3)).

### 10.3 Lệnh kiểm chứng của Crew

Chạy qua API công khai `Sandbox` (`Sandbox::new`, `wrap`, `execute`; `sandbox.rs:519, 611, 638`); `wrap_invocation` nằm trong module private (`sandbox.rs:23-24`) **[Đã kiểm]**. Chỉ ghi được trong worktree và một thư mục scratch riêng cho mỗi lần chạy; mạng tắt mặc định, bật theo từng bước trong cấu hình project.

Seatbelt chỉ cho ghi vào đường dẫn được liệt kê và vài đường dẫn `/dev` (`macos_seatbelt.rs:310-341`) **[Đã kiểm]**, nên Crew đặt thư mục scratch làm `TMPDIR` của lệnh. Bước build cần unix socket thì khai trong policy của bước đó.

<a id="muc-10-4"></a>

### 10.4 Rủi ro còn lại

Mỗi bản ghi spawn ghi các rủi ro áp dụng, và UI hiển thị chúng (NEED 07):

| Rủi ro | Phạm vi |
| ------ | ------- |
| Chặn đọc là danh sách chặn: file bí mật ở đường dẫn ngoài danh sách vẫn đọc được bằng lệnh Bash **[Spike]** | Claude |
| Lệnh Bash trong sandbox chạy không qua yêu cầu xin quyền (`autoAllowBashIfSandboxed` mặc định bật), nên gate không thấy **[Spike]**. Chấp nhận được vì NEED 07 cho worker tự do trong worktree | Claude |
| `workspace-write` vẫn cho ghi `$TMPDIR` và `/tmp` (`excludeTmpdirEnvVar: false`, `excludeSlashTmp: false`) **[Đã kiểm]**. Sandbox không chặn đọc, vì policy không có giới hạn đọc **[Suy luận, S8]** | Codex |
| Không có sandbox. Ghi file đi qua Zed. Thiếu H12 thì Zed nhận ghi vào mọi worktree, kể cả worktree ẩn tạo từ vị trí ngoài project của tool call, mà không qua yêu cầu xin quyền **[Đã kiểm]**; H12 giới hạn vào worktree của worker. Lệnh shell chạy trong harness của provider, không qua Zed **[Baseline]**; lớp chặn duy nhất là gate | Antigravity tài khoản cá nhân |
| Gate chỉ thấy thao tác provider gửi qua yêu cầu xin quyền; độ phủ ở từng mode chờ S8 | Mọi provider |
| Zed chạy `terminal/create` không sandbox và không xin quyền (`v1.rs:9650-9709`) **[Đã kiểm]**. Adapter Claude và Codex ghim không gọi `terminal/*` hay `fs/*` **[Đã kiểm]**; Antigravity chạy lệnh trong harness của nó **[Baseline]**. Bản adapter mới có thể đổi, nên contract test khẳng định không có lưu lượng này | Mọi provider |

Đọc được secret không có nghĩa là secret ra ngoài được: mạng của worker theo allowlist (Claude) hoặc tắt (Codex), và Crew quét secret trên diff trước khi commit ([9.4](#muc-9)).

---

<a id="muc-11"></a>

## 11. Kiểm chứng và Done

### 11.1 Thứ tự kiểm (NEED 46)

1. **Sensor tất định của project:** build, type check, lint, test, schema/link check.
2. **Sensor của Crew:**
   - *Fail-before/pass-after:* test mới hoặc script tái hiện bug phải fail trên commit gốc và pass trên head (NEED 47, 48).
   - *Cờ sửa test:* test bị xóa assertion, bị skip hoặc bị nới điều kiện.
   - *Quét secret* trên diff ([9.4](#muc-9)).
   - *Render UI:* chạy app thật và chụp màn hình cho thay đổi UI.
3. **Evaluator độc lập:** khác tác giả, context mới, tự chạy lại test. Trả về:

```jsonc
{ "verdict": "pass | fail",
  "findings": [{ "category": "missing | partial | contradicts | unrequested | test_tampering | standards",
                 "criterion_id": "AC-1", "evidence": "đường_dẫn:dòng — trích dẫn" }],
  "not_verified": ["…"] }
```

### 11.2 DoneEvaluator (NEED 45)

Theo bảng Loại session của brief 4.3. Lời “xong” của agent không có tác dụng (NEED 43).

| Loại session | Điều kiện Done kiểm tự động |
| ------------ | --------------------------- |
| Onboarding (không phải session) | Kiểm tra sẵn sàng của NEED 05 đạt |
| Hỏi nhanh | Đã trả lời đúng câu hỏi; dữ kiện có nguồn, phần suy luận có nhãn |
| Kickoff | Baseline check chạy lại từ worktree sạch cho cùng kết quả; Human đã xác nhận các giả định; file thêm vào repo đi qua PR đã merged |
| Brainstorm | Bộ ý tưởng đạt tiêu chí đã thống nhất |
| Research | Có câu hỏi, phạm vi, nguồn của nhận định quan trọng, mức chắc chắn (NEED 49) |
| Refine | Story đạt điều kiện của NEED 22 |
| Implement | Sự kiện GitHub “PR merged” kèm `merge_commit_sha`; bằng chứng truy được. PR mở hoặc CI xanh chưa đủ |
| Release / migration | Rollout hoàn tất trong quyền đã cấp; kiểm sau triển khai đạt; rollback đã xác nhận còn dùng được |
| Incident | Triệu chứng gốc không còn khi kiểm lại; bản ghi có timeline, nguyên nhân đã tái hiện, tác động; việc phòng ngừa đã mở thành story |
| Review / retro | Mọi đề xuất đã có quyết định của Human; đề xuất được chấp nhận đã mở thành session/story |
| Retirement | PR gỡ đã merged; không còn tham chiếu hay hành vi cũ; dữ liệu xử lý theo phương án đã duyệt; ADR liên quan `superseded` |

### 11.3 Truy vết (NEED 49)

Mỗi bằng chứng gắn:
- Phiên bản INPUT và commit code.
- Cặp Model–Harness: provider, model, effort, phiên bản adapter, hash của playbook, ROLE, cấu hình và option session.
- Phần chưa chứng minh được.

---

<a id="muc-12"></a>

## 12. Tích hợp GitHub (`crew_github`)

| Phần | Thiết kế |
| ---- | -------- |
| Private key của App | Nhập từ file `.pem` qua UI onboarding, ghi thẳng vào Keychain của macOS; không qua hội thoại, log, run store (NEED 05, 17) |
| Keychain trên mọi kênh build | Fork build ở kênh `dev` (`crates/zed/RELEASE_CHANNEL`). Ở kênh này, `CredentialsProvider` của Zed dùng `DevelopmentCredentialsProvider` thay cho Keychain, trừ khi đặt `ZED_DEVELOPMENT_USE_KEYCHAIN` (`zed_credentials_provider.rs:46-62`) **[Đã kiểm]**. Crew luôn ghi secret của mình (private key của App, API key của Codex ở H11, key của Artificial Analysis) vào Keychain, bất kể kênh; không làm được thì từ chối lưu và báo Human. Có test riêng cho kênh `dev` |
| JWT | RS256 bằng `jsonwebtoken`, đã có trong workspace **[Đã kiểm]**. Client REST tự viết trên `http_client`, không dùng `octocrab` vì crate này dựa vào tokio **[Nhớ]** |
| Token installation | Theo đúng một repo, quyền tối thiểu (contents, issues, pull requests, checks; `workflows` khi cần), hết hạn khoảng 1 giờ **[Nhớ]** |
| Push | Qua crate `git` của Zed; askpass trả `x-access-token` và token. Tắt credential helper của user (`-c credential.helper=`) để không bao giờ push bằng danh tính Human và không lưu token vào Keychain của git **[Suy luận]** |
| Kiểm tra onboarding | Danh tính App, repo đã cài, quyền, tạo thử token, loại chủ sở hữu repo, ruleset (dismiss stale approvals, require approval of most recent push) |
| Issues | Hierarchy: repo của tổ chức dùng issue types cùng sub-issues; repo cá nhân dùng label cùng sub-issues. Issue do Crew tạo có marker `<!-- crew:artifact:<id> v<n> -->` và hash |
| Phát hiện lệch | Poll bằng conditional request (ETag) **[Nhớ]**. Hash khác bản Crew đã ghi nghĩa là Human đã sửa → báo, không ghi đè (NEED 42) |
| ADR, glossary, tài liệu quyết định | File dưới `artifacts_path` (chốt ở Kickoff), thay đổi qua PR; spec và ticket nằm trong Issues; ADR cũ thành `superseded` |
| PR và merge | PR từ integration branch, bằng danh tính App. Review `APPROVED` của Human chỉ hiệu lực khi `commit_id` trùng head; merge kèm `sha` = head đã duyệt **[Nhớ]**. PR bị đóng → báo, chờ Human |
| Effect | Mỗi thao tác ghi là một effect có khóa idempotency và marker; Recovery tìm marker |
| Không dùng được với App | API notifications **[Đã kiểm]**, Projects của tài khoản cá nhân → báo blocker |

---

<a id="muc-13"></a>

## 13. Dịch vụ model (`crew_models`, NEED 60)

| Thành phần | Thiết kế |
| ---------- | -------- |
| Loại nhiệm vụ | Từ playbook: orchestrate, architecture, implement, review, debug, research, ui, test, docs |
| Ứng viên | Cặp (provider, model, effort) có trong snapshot ACP và trong tập được phép. Cùng model qua harness khác là ứng viên khác |
| Metadata | models.dev: giá (cả giá theo ngày, tier), context, effort, ngày phát hành |
| Benchmark | Terminal-Bench 2.1 (cặp model + harness), models.dev `[[benchmarks]]`, Artificial Analysis (tùy chọn; key trong Keychain; ghi nguồn) |
| Lưu điểm | Theo `crew_scores`; không gộp khác phiên bản hay khác harness; điểm vendor tự báo được đánh dấu |
| Cách chấm | Điểm công khai là giá trị ban đầu theo loại nhiệm vụ. Kết quả của Crew theo cặp Model–Harness (tỷ lệ đạt, phải làm lại, chi phí, thời gian) điều chỉnh dần. Model mới ghi “chưa đánh giá” |
| Chi phí | Ước tính từ models.dev cộng chi phí cố định theo provider (Antigravity: prompt đầu khoảng 12,7 nghìn token) |
| Đầu ra | `rank(task_kind, constraints)` → danh sách xếp hạng kèm căn cứ; [mục 8](#muc-8) lọc tiếp theo đăng nhập, cách ly, hạn mức |
| Quy tắc | Điểm thiếu không bằng 0; không tự đổi tập được phép; chỉ tải dữ liệu công khai; không gửi dữ liệu project ra ngoài |

Không có benchmark công khai cho harness Antigravity, nên điểm của Antigravity chủ yếu đến từ eval của Crew.

---

<a id="muc-14"></a>

## 14. Phục hồi và nâng cấp

### 14.1 Recovery (NEED 44)

1. **Run store:** kiểm run store là file trên đĩa, không phải database trong bộ nhớ ([4.1](#muc-4)). Không phải thì dừng và báo Human; không đối soát, không mở session.
2. Nạp các session đang mở.
3. **Effect** `intent` hoặc `unknown`: đối soát với GitHub (marker) và git (nhánh, commit). Có kết quả thì ghi `done`; không có thì giữ `unknown` và báo. Không tự làm lại.
4. **Worker đang dở:** chuyển `interrupted`. Crew không gửi prompt giao việc mới cho session worker cũ. Muốn làm tiếp, Orchestrator yêu cầu spawn worker mới với packet dựng lại từ run store, cộng phần đã có trong worktree. Như vậy recovery không phụ thuộc việc provider có giữ ROLE khi resume hay không.
5. **Orchestrator:**
   - Claude và Antigravity: load qua CrewConnection với đầy đủ option.
   - Codex: chưa biết ROLE có còn sau resume (S11). Trong lúc chờ, Crew mở thread Orchestrator mới và nắm lại trạng thái bằng `crew_get_state`; thread cũ chỉ xem.
6. **Session thăm dò:** resume theo bản ghi `kind = probe`; không được thì tạo mới.
7. Hiện bản tóm tắt đối soát trong tổng quan session.

**Bất biến:** không đường code nào load hay resume session Crew mà không đi qua CrewConnection với đầy đủ option. Điều này gồm cả các đường Zed tự load lại thread (H5d) và thread import (H5e), và có test riêng ([mục 17](#muc-17)).

Crew không dọn dữ liệu session mà provider lưu trên máy (NEED 61).

### 14.2 Nâng cấp (NEED 28)

- Session ghim phiên bản Crew, phiên bản schema, hash cấu hình, hash playbook và ROLE, phiên bản adapter.
- Migration schema theo `Migrator` của crate `db`, chạy khi `open_db` mở run store ([4.1](#muc-4)). Session không tương thích thì dừng, kèm lý do.
- Đổi bản ghim adapter chỉ sau khi contract test đạt, và chỉ áp cho session mới. Ngưỡng context thực tế được đo lại.

---

<a id="muc-15"></a>

## 15. Giao diện (`crew_ui`, NEED 57)

| Bề mặt | Hiện thực | NEED |
| ------ | --------- | ---- |
| Điểm vào | Command Palette: “Crew: New Session”, “Crew: Open Session Overview”, “Crew: Stop Session”. Agent id của Crew ẩn khỏi bộ chọn agent (H5d) và không bao giờ thành agent đang chọn của panel (H5f) | 09, 16, 57 |
| Hội thoại Orchestrator | Thread Agent Panel của agent id Crew. Thread Orchestrator Codex mở lại sau khi Zed khởi động lại chỉ xem, cho tới khi S11 có kết quả | 09, 10 |
| Tổng quan session | Workspace item: loại, trạng thái, INPUT, tiêu chí OUTPUT, pipeline dạng cây có dependency, worker (ROLE, provider, model, work item age, mức cách ly, rủi ro còn lại), compaction, effect, bằng chứng | 43, 50 |
| Worker | Thread chỉ đọc theo cờ H5b: không composer, không option `allow_always`; mở từ workspace của Orchestrator | 50 |
| Việc chờ Human | Projection từ run store; bề mặt chốt bằng prototype | 12, 52 |
| Status bar | Worker đang chạy, việc chờ Human, trạng thái đăng nhập từng provider, hạn mức sắp chạm ngưỡng, tuổi lần cập nhật gần nhất | 50, 56 |
| Tài liệu quyết định | Markdown Preview | 51 |
| Tìm và lưu trữ | Threads Sidebar và Thread History, gắn nhãn Crew (H5c); thread import không hiện session Crew và session thăm dò (H5e) | 53 |
| Onboarding, Settings | H6 (trang onboarding của Zed hoặc item Crew riêng), H7; ô nhập API key Codex ở tầng Zed (H11) | 04, 05, 27, 56 |

Chỉ dùng component của crate `ui` và màu theme. Trạng thái có chữ hoặc icon đi kèm màu; mở được bằng bàn phím; không có Approve/Re-run; phần trăm tiến độ chỉ hiện khi có cơ sở đo.

---

<a id="muc-16"></a>

## 16. Quan sát và log

- Mọi sự kiện ở brief mục 10 là event trong `crew_events`; không gửi telemetry ra ngoài máy (NEED 02).
- Chỉ số bảo vệ (brief 3.3) được kiểm ngay khi xảy ra và ghi `guardrail_violation`.
- **Che dữ liệu trước khi lưu:** token, key, header `Authorization`, email tài khoản (dòng `**Account:**` của `/status`).
- **Update ACP bị bỏ** (H4b, đọc ở tầng SDK hoặc luồng debug) ghi vào `crew_dropped_updates`. Trạng thái luôn đối soát lại với git, GitHub và kết quả lệnh.

---

<a id="muc-17"></a>

## 17. Kiểm thử

| Lớp | Nội dung | Chạy khi |
| --- | -------- | -------- |
| Logic thuần | Selection, PolicyGate (quyết định theo `rawInput`, chọn theo `kind`, không bao giờ `*_always`, từ chối elicitation), ModeGuard, gộp cấu hình (chỉ thu hẹp), kiểm DAG, DoneEvaluator, đối soát effect, parser hạn mức, phát hiện compaction, quét secret | Mỗi thay đổi |
| Luồng điều phối | GPUI test với `FakeAcpAgentServer` (`v1.rs:1708`) **[Đã kiểm]**; GitHub giả qua HTTP client giả của Zed **[Nhớ]**; repo git tạm | Mỗi thay đổi |
| Bất biến CrewConnection | Mỗi đường Zed tự load lại thread (mở có session id, kết nối lại, nạp lại thread bị giải phóng) đi qua CrewConnection; thread import lọc session Crew; trong cùng process, option gửi lại giống hệt từng byte; `inner` mang agent id của Crew; đủ 26 phương thức có cài mặc định của trait được chuyển tiếp; agent id của Crew còn trong `external_agents` sau `reregister_agents`; agent id của Crew không thành agent đang chọn; session con của session Crew load được ở chế độ chỉ xem | Mỗi thay đổi |
| Chính sách fs (H12) | Đọc, ghi qua `fs/*` ngoài worktree của worker, qua symlink trỏ ra ngoài, vào worktree ẩn, hay từ Orchestrator và session thăm dò đều bị từ chối; vị trí ngoài project trong tool call không tạo worktree ẩn | Mỗi thay đổi |
| Recovery | Tiêm crash giữa intent và kết quả của effect; worker `interrupted` không tự chạy lại; run store không bền (`ZED_STATELESS`, file không mở được) thì Crew chặn và báo | Mỗi thay đổi |
| Secret | Trên kênh `dev`, secret của Crew vẫn vào Keychain, không vào file của `DevelopmentCredentialsProvider` | Mỗi thay đổi |
| **Contract test adapter** | Dựng từ harness của spike (bước 0): ROLE có hiệu lực ở new, load, resume cho từng provider; ma trận sandbox và `$TMPDIR` thật của Claude; parser `/status` và `/usage` trên fixture đã che email; `kind` và id của option xin quyền; `_auth/status_update` và healthcheck; ngưỡng compaction của Antigravity; load session do bản adapter của Zed tạo. Client tối thiểu khai đúng `client_capabilities_for_agent` của Zed (fs, terminal, elicitation) và khẳng định Claude, Codex không gửi `fs/*` hay `terminal/*`; elicitation bị từ chối và `AskUserQuestion` bị chặn; dạng yêu cầu nới sandbox của Codex; với `settingSources: ["project"]`, `CLAUDE.md` vẫn được nạp, đăng nhập không đổi và luật deny của Crew thắng cấu hình trong worktree | Khi đổi bản ghim, trước manual test |
| UI | Chạy app thật và chụp màn hình (NEED 03) | Thay đổi UI |
| Sensor của repo Crew | `cargo check`, clippy của repo, test các crate Crew và crate có điểm nối | Trước khi vào main |
| Kiểm chứng độc lập | Agent AI khác tác giả đối chiếu thay đổi với brief và spec | Trước khi vào main |
| Merge vào main | Agent merge khi cả kiểm chứng tự động lẫn kiểm chứng độc lập đạt; Human không review code (Human xác nhận 2026-10-25) | Mỗi PR |
| Đồng bộ upstream | Rebase định kỳ trên nhánh riêng, chạy toàn bộ test; xung đột ở H1–H12 được báo; báo khi upstream chuyển kết nối sang `AcpV2Connection` (`v2.rs`), vì H2, H3 và `Rc<AcpConnection>` trong CrewConnection sẽ hỏng | Theo nhịp (NEED 59) |
| NEED 61 | Check tĩnh: mã Crew không đọc/ghi `.claude`, `.codex`, `.gemini`, không gọi API riêng của provider; `settings` truyền cho Claude luôn là object; gate không có đường nào chọn `*_always` | Mỗi thay đổi |

---

<a id="muc-18"></a>

## 18. Thứ tự triển khai

Theo brief 4.4: thứ tự phụ thuộc, không phải slice; Human manual test khi đủ phạm vi.

Lọc thread import (H5e) làm ở bước 2, cùng lúc session Crew đầu tiên xuất hiện: nếu không lọc, session Crew có thể bị import dưới agent thường và mất ROLE, sandbox, gate.

Chính sách fs (H12) cũng làm ở bước 2, cùng Orchestrator chỉ đọc, vì Orchestrator trên Antigravity cần nó ([7.5](#muc-7-5)); worker dùng lại ở bước 3.

Khác brief 4.4 ở một điểm: phần **đọc hạn mức** đưa lên bước 1, vì nó thuộc trạng thái provider và quy trình chọn ở bước 2 cần hạn mức thật. Xếp hạng theo điểm benchmark (NEED 60) vẫn ở bước 7; trước đó bước 2 xếp hạng theo cấu hình mặc định, như brief 4.4 ghi. Brief cho phép tech spec chỉnh thứ tự.

| Bước | Hạng mục | Spike làm trước |
| ---- | -------- | --------------- |
| 0 | Kiểm lại H1–H12 trên fork, bắt đầu từ số dòng ở Phụ lục B.1 và báo cáo kiểm thiết kế; khung 6 crate; feature flag (H8, H9); biến harness spike thành contract test | S6, S7 |
| 1 · Nền | Run store (file riêng qua `open_db`, kiểm độ bền); Recovery (đối soát fact và effect); gộp cấu hình, kể cả Restricted Mode; ProviderProfile; agent id của Crew và đăng ký vào agent server store, CrewConnection, adapter ghim (H2, H5d, H10); healthcheck; snapshot; đọc hạn mức; H4; sửa lỗi đăng nhập ở tầng Zed (H11); secret vào Keychain trên mọi kênh build | S12 |
| 2 · Orchestrator | McpServer (handler `initialize`, thư mục 0700) và cầu nối (H1); “Crew: New Session” (H5a) và agent đang chọn (H5f); lọc thread import (H5e); ROLE và bất biến CrewConnection; Orchestrator chỉ đọc và chính sách fs (H12); từ chối elicitation; công cụ Orchestrator; DecisionCapture; Stop/Cancel; Selection | S11 |
| 3 · Thực thi | Worktree qua `GitStore` và Project không cửa sổ; PolicyGate (quyết định theo `rawInput`), ModeGuard; payload sandbox Claude; readiness và baseline; task packet; ngân sách context và phát hiện compaction; kiểm bàn giao, quét secret, chính sách commit; Verification qua API `Sandbox`; evaluator; đếm lần thử; Scheduler; gộp nhánh | S8, S9, S10 |
| 4 · GitHub | App và nhập key; onboarding; Issues; phát hiện lệch; PR, review, merge theo `sha`; push qua askpass; marker effect | — |
| 5 · Playbook | Playbook và ROLE từ seed; loại session và DoneEvaluator; đối soát trước implement; đổi yêu cầu; ghim khi nâng cấp; Kickoff; prototype; bản đồ quyết định và tham chiếu | — |
| 6 · UI | Tổng quan; worker chỉ đọc (H5b); việc chờ Human (prototype); status bar; sidebar (H5c); Settings (H7); onboarding (H6) | — |
| 7 · Model và retro | Nguồn benchmark, chấm điểm, xếp hạng; session retro; eval theo cặp Model–Harness | S13 |

---

<a id="muc-19"></a>

## 19. Rủi ro kỹ thuật

Bổ sung cho brief mục 12. “Mức” là đánh giá, không phải số đo.

| # | Rủi ro | Mức | Giảm thiểu |
| - | ------ | --- | ---------- |
| T1 | Orchestrator (LLM) không theo giao thức công cụ | Cao | Mọi thứ quan trọng qua gate tất định: Done, spawn, quyết định của Human |
| T2 | File UI của Zed lớn và đổi nhanh (`agent_panel.rs` gần 15.000 dòng **[Đã kiểm]**) | Cao | Ưu tiên đăng ký event; H5 mỏng; registry factory thay cho sửa từng đường mở thread |
| T3 | Crate ACP đổi khi đồng bộ (fork 3.4.0, upstream 3.5.0 cùng schema 1.12.0 **[Spike]**) | Thấp | Gói phần ACP vào `crew_acp`; contract test sau mỗi lần đồng bộ |
| T4 | Nhiều Project không cửa sổ và process agent tốn tài nguyên | Trung bình | Trần song song; S10 |
| T5 | Cầu nối MCP treo khi agent chết | Thấp | Thoát khi EOF hoặc socket đóng |
| T6 | Poll GitHub tốn rate limit | Thấp | ETag, backoff |
| T7 | DecisionCapture gắn nhầm tin nhắn | Trung bình | Id tin nhắn do Crew cấp; prompt của Crew được ghi trước khi gửi; gắn với lượt trình gần nhất; Orchestrator nhắc lại diễn giải |
| T8 | Seatbelt bị macOS khai tử **[Nhớ]** | Thấp | Theo hướng upstream |
| T9 | Upstream thêm một đường mở thread không đi qua `Agent::server()`, làm session Crew mất ROLE hoặc sandbox. Hiện mọi đường đều đi qua hàm này **[Đã kiểm]** | Trung bình | Registry factory (H5d); test bất biến cho từng đường; đồng bộ upstream chạy lại test này |
| T10 | Định dạng text `/status`, `/usage` đổi theo adapter | Trung bình | Ghim adapter; contract test trên fixture; parse lỗi thì `unknown` |
| T11 | Ngưỡng compaction thực tế thấp hơn cửa sổ được báo (Antigravity) | Trung bình | Ngân sách context theo ngưỡng đo được; đo lại khi đổi bản ghim |
| T12 | File tạm rơi vào worktree khi chặn ghi `$TMPDIR` | Thấp | Chỉ commit file khai báo và file đã theo dõi trong `file_scope`; xóa file không khai báo khi worker kết thúc |
| T13 | Healthcheck mỗi lần khởi động tốn hạn mức và khởi chạy cả ba adapter | Thấp | Session thăm dò dùng lại; Codex dùng thông báo trạng thái |
| T14 | Bản adapter của Zed và bản ghim của Crew cùng dùng dữ liệu session của provider; khác định dạng lưu trữ có thể làm lỗi load **[Suy luận]** | Trung bình | Crew chỉ load session của agent id Crew; contract test load chéo giữa hai bản |
| T15 | Nhóm hạn mức của Gemini 4 Argon không xác định, nên Argon bị bỏ qua theo chính sách mặc định | Trung bình | S13; trong lúc chờ, Human chọn chính sách `ask_human` nếu muốn dùng Argon |
| T16 | Session Antigravity (cả session thăm dò) tích lũy trên máy, không xóa được qua ACP | Thấp | Dùng lại session thăm dò; không dọn (NEED 61) |
| T17 | Upstream chuyển kết nối ACP sang `AcpV2Connection` (`v2.rs`, hiện chưa dùng **[Đã kiểm]**), làm hỏng H2, H3 và `Rc<AcpConnection>` trong CrewConnection | Thấp | Đồng bộ upstream theo dõi việc này ([mục 17](#muc-17)); phần ACP gói trong `crew_acp` |

---

<a id="muc-20"></a>

## 20. Việc còn mở

Mọi quyết định thiết kế lớn đã được Human duyệt (2026-10-25). Mục này chỉ còn việc chưa có câu trả lời.

### 20.1 Spike còn lại

| ID | Câu hỏi | Tiêu chí xong | Chặn bước |
| -- | ------- | ------------- | --------- |
| S6 | Đăng ký ext notification `_auth/status_update` và gọi ext method `_session/steering` có chạy được qua H3 không? API của crate 3.4.0 đã xác định: `Builder::on_receive_notification`, `ConnectionTo::send_request`, `UntypedMessage` (`src/jsonrpc.rs:1579, 4300, 5676-5685`) **[Đã kiểm]** | Test với agent giả | 0 |
| S7 | Cả ba provider khởi động được MCP stdio `zed --crew-mcp-bridge` và gọi được công cụ của `McpServer` có handler `initialize`. Framing đã rõ: JSON-RPC, mỗi message một dòng **[Đã kiểm]** | Mỗi provider gọi được một công cụ thử | 0 |
| S8 | Ở mode worker của từng provider, thao tác nào đi qua yêu cầu xin quyền? Lệnh git ghi trong worktree có bị sandbox chặn không? Khi nào Codex gửi yêu cầu nới quyền? Sandbox Codex có chặn đọc không, và có cách nào qua ACP để loại `$TMPDIR`, `/tmp` khỏi vùng ghi không? | Bảng độ phủ gate cho từng provider | 3 |
| S9 | `maxBudgetUsd`, `maxTurns` và luật `settings.permissions.deny` cho tool của Claude có hiệu lực khi chạy thật không, kể cả với lệnh Bash được tự cho phép trong sandbox? `_meta.agy.disabledTools` của Antigravity có hiệu lực không? | Mỗi tùy chọn một test đạt | 3 |
| S10 | Chi phí bộ nhớ và CPU của N Project không cửa sổ cùng N process agent | Số đo cho N = 1, 2, 4, làm căn cứ cho trần (brief Q7) | 3 |
| S11 | ROLE của Codex qua `customInstructions`: có hiệu lực ở session mới không, và còn sau load/resume trong process mới không? | Kết luận kèm test marker | 2 |
| S12 | Định dạng output `/usage` của Claude và cấu trúc `_claude/rateLimit` | Fixture đã che dữ liệu cá nhân, kèm parser | 1 |
| S13 | Gemini 4 Argon tính vào nhóm hạn mức nào của Antigravity? Cách đo: so `/usage` trước và sau một loạt prompt Argon | Nhóm xác định được, hoặc kết luận không xác định được | — (trong lúc chờ, Argon có hạn mức `unknown`) |

### 20.2 Câu hỏi mở của brief

Q6–Q9 của brief mục 7: state machine của artifact, giá trị mặc định của các ngưỡng, ngưỡng của chỉ số, repo dùng cho manual test đầu tiên.

---

<a id="phu-luc-a"></a>

## Phụ lục A. Truy vết NEED → thiết kế

| NEED | Thành phần | Cơ chế chính | Kiểm bằng |
| ---- | ---------- | ------------ | --------- |
| 01 | `crew`, `crew_ui` | Chỉ macOS; từ chối project remote | Test |
| 02 | `crew`, `crew_acp`, `crew_mcp` | Run store chia theo project; MCP của session do Crew quyết (H2); che dữ liệu trước khi lưu; session thăm dò tách thư mục; project-scope đọc từ checkout chính | Test truy cập chéo; test che dữ liệu |
| 03 | Quy trình | Sensor và kiểm chứng độc lập trước main; feature flag | Cổng trước main |
| 04 | `crew_ui`, `crew` | Onboarding (H6, H7); Kickoff là Initializer | Test Done theo loại |
| 05 | `crew_github` | App JWT, Keychain trên mọi kênh build, token theo repo, kiểm tra onboarding; quét secret trước commit | GitHub giả; test Keychain trên kênh `dev` |
| 06 | `crew` | Quyền môi trường theo project; release cần quyền và rollback | Test chính sách |
| 07 | `crew`, `crew_acp` | PolicyGate quyết định theo `rawInput`, chọn theo `kind`, không bao giờ `*_always`; từ chối elicitation; ModeGuard; payload sandbox Claude ([6.2](#muc-6-2)); chặn ghi `$TMPDIR`; chính sách fs cho session Crew (H12); ghi rủi ro còn lại ([10.4](#muc-10-4)) | Test gate; test H12; contract test sandbox |
| 08 | `crew` | Packet gắn nhãn dữ liệu; gate độc lập với LLM | Test với nội dung chứa chỉ thị |
| 09 | `crew_ui` | Thread Orchestrator là điểm vào duy nhất (H5a) | Test UI |
| 10 | `crew_mcp` | Schema `crew_ask_human`; chặn câu trùng | Test schema |
| 11 | `crew` | Artifact bản đồ quyết định | Kiểm playbook |
| 12 | `crew` | Nhánh `crew/prototype/*`, chặn PR | Test |
| 13 | `crew_ui`, `crew` | Không Approve/Re-run; Stop/Cancel; quyết định chỉ từ tin nhắn của Human (id do Crew cấp), elicitation bị từ chối | Review UI; test |
| 14 | `crew_github` | Review gắn `commit_id`; merge với `sha` | GitHub giả |
| 15 | `crew` | Chờ Human không timeout | Test Scheduler |
| 16 | `crew`, `crew_ui` | Stop/Cancel hủy worker, đối soát | GPUI test |
| 17 | `crew`, `crew_ui` | Hướng dẫn bước Human; nhập secret qua UI; chỉ dẫn đăng nhập | Review; test |
| 18 | `crew_mcp` | `crew_set_session_type`; đường nhanh cho việc nhỏ | Test |
| 19 | `crew` | Baseline, readiness; delta spec | Test |
| 20 | `crew` | Artifact bản đồ tham chiếu ↔ capability | Kiểm playbook |
| 21 | `crew_mcp` | Kiểm DAG và hợp đồng task; lát dọc trên target repo | Test |
| 22 | `crew` | Đối soát chỉ đọc chặn implement | Test |
| 23 | `crew_acp` | Readiness từ `crew_health`, snapshot, adapter ghim, baseline | Test |
| 24 | `crew_ui`, `crew_acp` | Orchestrator trên provider được chọn, qua agent id của Crew | Test |
| 25 | `crew_acp` | ProviderProfile cho 3 provider; trần Antigravity | Contract test |
| 26 | `crew`, `crew_models` | Quy trình chọn chung ([mục 8](#muc-8)): trần session, chính sách hạn mức `unknown`, không tự đổi provider khi hết hạn mức giữa chừng; bản ghi spawn có xếp hạng và lý do bỏ qua | Unit test |
| 27 | `crew` | Gộp cấu hình theo tầng; ROLE và playbook có phiên bản ([5.4](#muc-5)) | Test chỉ thu hẹp |
| 28 | `crew_acp`, `crew` | Adapter ghim (H10); snapshot cấu hình; contract test khi đổi bản ghim | Test; contract test |
| 29 | `crew` | Scheduler theo DAG; chèn nhánh | Test |
| 30 | `crew` | Trần song song, timeout, trần chi phí | Test |
| 31 | `crew_acp`, `crew_mcp` | Task packet; ngân sách context theo ngưỡng thực tế; phát hiện compaction ([6.8](#muc-6)) | Test |
| 32 | `crew_mcp`, `crew` | Schema bàn giao; file không khai báo | Test |
| 33 | `crew_mcp` | Kiểm input, cắt output, lỗi có hướng xử lý | Test |
| 34 | `crew` | Worktree riêng; chồng lấn `file_scope`; worktree tích hợp riêng; Orchestrator chỉ đọc; chính sách fs (H12) | Test repo git tạm; test H12 |
| 35 | `crew`, `crew_github` | Integration branch, PR | GitHub giả |
| 36 | `crew`, `crew_acp` | Phiên bản INPUT; steering hoặc cancel; `stale` | Test |
| 37 | `crew` | Đếm lần thử theo vấn đề | Test |
| 38 | `crew` | Phân loại lỗi infra | Test fixture |
| 39 | `crew_github` | Issues làm nguồn; hierarchy theo chủ sở hữu | GitHub giả |
| 40 | `crew_github` | Bên ghi tự động duy nhất; phát hiện sửa của Human | Test |
| 41 | `crew` | Khóa bản ghi đã Done | Test |
| 42 | `crew`, `crew_github` | ETag/hash; `derived_from` | Test |
| 43 | `crew` | Tiêu chí default-FAIL; log update bị bỏ (H4) | Test |
| 44 | `crew`, `crew_acp` | Nhật ký effect; worker không được giao tiếp việc sau khởi động lại; load Orchestrator qua CrewConnection; dừng khi run store không bền | Test tiêm crash |
| 45 | `crew` | DoneEvaluator theo loại session ([11.2](#muc-11)) | Test |
| 46 | `crew` | Sensor rồi evaluator độc lập | Test |
| 47 | `crew` | Fail-before/pass-after | Test |
| 48 | `crew` | Artifact tái hiện bug | Test |
| 49 | `crew` | Bằng chứng gồm cặp Model–Harness, phiên bản adapter, hash option | Test |
| 50 | `crew_ui` | Tổng quan, worker chỉ đọc, status bar | Chạy app, chụp màn hình |
| 51 | `crew` | Tài liệu quyết định một chiều | Test |
| 52 | `crew`, `crew_ui` | Projection việc chờ Human | Test, prototype |
| 53 | `crew_ui` | Sidebar, history; thread import lọc session Crew và thăm dò (H5c, H5e) | Test |
| 54 | `crew` | Retro; đề xuất cần Human quyết | Test |
| 55 | `crew_acp` | Kênh ROLE theo provider ([6.2](#muc-6-2)); CrewConnection gửi lại đủ option ở mọi lần tạo, load, resume, kể cả đường Zed tự load lại (H5d) | Contract test; test bất biến |
| 56 | `crew_acp` | Xác nhận đăng nhập theo provider, mặc định healthcheck ([6.5](#muc-6-5)); session thăm dò dùng lại; snapshot; sửa lỗi đăng nhập ở tầng Zed (H11) | Contract test |
| 57 | `crew_ui` | Component Zed; Command Palette | Review UI |
| 58 | Repo | Repo Crew private | Kiểm thủ công |
| 59 | Tất cả | 6 crate; H1–H12; rebase định kỳ | CI đồng bộ |
| 60 | `crew_models`, `crew_acp` | Xếp hạng ứng viên; parser hạn mức qua ACP ([6.7](#muc-6)); gắn model với nhóm hạn mức | Test parser trên fixture |
| 61 | `crew_acp` | Chỉ ACP; `settings` là object; gate không chọn option ghi chính sách lâu dài; không bọc process provider; check tĩnh | Check tĩnh |

---

<a id="phu-luc-b"></a>

## Phụ lục B. Dữ kiện đã kiểm

### B.1 Mã nguồn Zed (fork `11caaae`)

Bảng đầy đủ kèm phân tích nằm trong [`design-check.md`](design-check.md).

| Dữ kiện | Vị trí |
| ------- | ------ |
| MCP server trong process qua Unix socket; `add_tool`; `handle_request`; `McpServerTool`; thư mục socket tạo bằng `tempfile` với quyền mặc định | `crates/context_server/src/listener.rs:34-197, 402-423`; `:57`, `:143` |
| `McpServer` chỉ trả lời `tools/call`, `tools/list` và handler đã đăng ký; framing theo dòng; connection giữ bảng công cụ sau khi server bị drop | `crates/context_server/src/listener.rs:200-242, 360-394` |
| `--askpass` là cầu nối một chiều, chạy trước khi app khởi động | `crates/askpass/src/askpass.rs:413-469`; `crates/zed/src/main.rs:213-220` |
| `new_session` không nhận tùy chọn; trait có 26 phương thức có cài mặc định; `into_any` | `crates/acp_thread/src/connection.rs:104-109, 95-305, 304` |
| `new_session`, `load_session`, `resume_session` chỉ gửi cwd, thư mục thêm và MCP server đang bật của project | `crates/agent_servers/src/acp/v1.rs:1238, 1418, 1457, 1130-1153`; `:9134` (`mcp_servers_for_project`) |
| Client chỉ đăng ký handler cho notification của session và elicitation; khai capability fs, terminal, elicitation; field `connection` private; luồng debug | `crates/agent_servers/src/acp/v1.rs:368-442, 445-475, 75, 488-495` |
| Handler xin quyền; `fs/write_text_file` không qua bước xin quyền; `terminal/create` không sandbox | `crates/agent_servers/src/acp/v1.rs:9320, 9416-9449, 9650-9709` |
| Settings `agent_servers.<agent id>` áp mode và model sau new, load, resume; thư mục thêm thành `additionalDirectories`; xử lý riêng theo agent id | `crates/agent_servers/src/acp/v1.rs:156-168, 1310-1345, 1157-1175, 451, 649`; `crates/agent_servers/src/custom.rs:234-249` |
| `acp::connect` trả connection v1; `AcpV2Connection` có nhưng chưa dùng | `crates/agent_servers/src/acp/v1.rs:320-340`; `crates/agent_servers/src/acp/v2.rs:119` |
| Thread giữ connection bên trong; agent id lưu của thread lấy từ connection đó | `crates/agent_servers/src/acp/v1.rs:916-924`; `crates/agent_ui/src/thread_metadata_store.rs:1305` |
| Agent giả cho test; mẫu bọc `inner: Rc<AcpConnection>` | `crates/agent_servers/src/acp/v1.rs:1708, 1841-1842` |
| `Agent::server()` tạo `CustomAgentServer` cho mọi agent id tùy chỉnh, và là nơi duy nhất gọi `CustomAgentServer::new`; panel, thread import, archive view đều đi qua đây | `crates/agent_ui/src/agent_ui.rs:578-586`; `agent_panel.rs:4665`; `thread_import.rs:704`; `threads_archive_view.rs:832`; `AgentServerDelegate::new` ở `agent_connection_store.rs:297` |
| Connection store chỉ giữ agent id có trong `external_agents` và cache server theo khóa; ACP Logs chỉ downcast `AcpConnection`; `reregister_agents` xóa rồi dựng lại danh sách | `crates/agent_ui/src/agent_connection_store.rs:277, 149-151, 115`; `crates/project/src/agent_server_store.rs:294, 348` |
| Các đường Zed tự load/resume thread; session con load trên cùng connection, lỗi load bị bỏ | `crates/agent_ui/src/conversation_view.rs:1358-1371` (load, nếu không thì resume), `:1211-1218` (kết nối lại), `:1590-1620, 2438` (session con); `crates/agent_ui/src/agent_panel.rs:4300-4313` (giải phóng thread rảnh có hỗ trợ load) |
| Thread import liệt kê session không lọc cwd | `crates/agent_ui/src/thread_import.rs:803-807` |
| Agent đang chọn: `create_thread_with_options` trả lại giá trị cũ; `set_base_view` gán theo thread đang hiện | `crates/agent_ui/src/agent_panel.rs:3233-3265, 4354` |
| Đọc và ghi file qua ACP nhận mọi worktree, kể cả worktree ẩn, và so tiền tố không resolve symlink; vị trí ngoài project thành worktree ẩn ở mọi tool call | `crates/acp_thread/src/acp_thread.rs:7250-7272, 7330-7345, 1760, 5419, 5589, 5654`; `crates/project/src/worktree_store.rs:424-428, 463-472`; `crates/project/src/project.rs:3205`; `crates/util/src/paths.rs:220-222` |
| Event và API trả lời xin quyền; elicitation là event riêng | `crates/acp_thread/src/acp_thread.rs:3569-3599, 6077, 6092` |
| `ModeUpdated`, `ConfigOptionsUpdated` chỉ phát khi agent báo; đổi mode và config option phía Zed | `crates/acp_thread/src/acp_thread.rs:4504-4515`; `crates/agent_servers/src/acp/v1.rs:9218, 180` |
| Update lạ bị bỏ không log; `update_usage` chỉ giữ `cost.meta` | `crates/acp_thread/src/acp_thread.rs:4526, 5366` |
| Tin nhắn người dùng không có id ổn định; `SubmissionId` là bộ đếm; composer gọi `thread.send` | `crates/acp_thread/src/acp_thread.rs:6559-6566, 5160`; `crates/acp_thread/src/submission.rs:20`; `crates/agent_ui/src/conversation_view/thread_view.rs:2110` |
| `is_subagent` là `parent_session_id.is_some()`; phím tắt “allow always” trả lời yêu cầu đang chờ của mọi session trong hội thoại; sidebar không lưu thread có parent | `crates/agent_ui/src/conversation_view/thread_view.rs:1268-1269, 3012-3017`; `crates/agent_ui/src/conversation_view.rs:463-486, 1850` |
| Agent ngoài không được liệt kê model trước | `crates/agent_ui/src/agent_panel.rs:5008` |
| Chuyển tiếp env của Codex dùng tên sai `OPEN_AI_API_KEY`; chỉ xóa `ANTHROPIC_API_KEY` với `claude-acp` | `crates/agent_servers/src/custom.rs:236-245` |
| `create_worktree_workspace` cần cửa sổ, chạy từng lần một, tạo worktree detached, thêm workspace vào cửa sổ; `GitStore::create_worktree`; `Project::local` không cửa sổ | `crates/git_ui_core/src/worktree_service.rs:736-742, 781, 504, 1164, 1365-1367`; `crates/project/src/git_store.rs:9276`; `crates/project/src/project.rs:1232`; `crates/eval_cli/src/main.rs:735` |
| Sandbox: API công khai `Sandbox`; module Seatbelt private; Seatbelt chỉ cho ghi vào đường dẫn được liệt kê | `crates/sandbox/src/sandbox.rs:23-24, 85-118, 519, 611, 638`; `crates/sandbox/src/macos_seatbelt.rs:41-150, 310-341` |
| Keychain; kênh `dev` dùng provider phát triển thay cho Keychain; kênh của fork | `crates/credentials_provider/src/credentials_provider.rs:11-34`; `crates/zed_credentials_provider/src/zed_credentials_provider.rs:46-62`; `crates/zed/RELEASE_CHANNEL` |
| SQLite: `open_db` mở file riêng có migration, chuyển sang bộ nhớ khi stateless hoặc lỗi; `static_connection!` dùng database chung | `crates/db/src/db.rs:174-202, 247-268` |
| Feature flag: override chỉ ở bản debug hoặc staff; `enabled_for_all` luôn thắng | `crates/feature_flags/src/flags.rs:45-51`; `crates/feature_flags/src/store.rs:105-107, 166-182` |
| Restricted Mode giữ lại `.zed/settings.json`; `flattened_deserialize!` của project settings | `crates/project/src/project_settings.rs:1362-1384`; `crates/settings_content/src/project.rs:90-96` |
| Khởi tạo `agent_ui` trong `main.rs`; onboarding chỉ có một trang | `crates/zed/src/main.rs:713`; `crates/onboarding/src/onboarding.rs:299-301` |
| Cài npx theo trần; dọn bản binary cũ; thư mục cài theo agent id | `crates/project/src/agent_server_store.rs:1456-1467, 1055, 1309, 403-405, 1387-1390` |
| Dependency có sẵn | `Cargo.toml`: `agent-client-protocol = "=3.4.0"` (upstream đã lên 3.5.0), `jsonwebtoken`, `rsa`, `octocrab`, `sha2` |

### B.2 Mã adapter và crate ACP

| Dữ kiện | Vị trí |
| ------- | ------ |
| Claude nhận `systemPrompt` (append/override) và `_meta.systemPrompt` | `claude-agent-acp@0.95.0` `dist/client-system-prompt.js`, `dist/acp-agent.js:8532-8555` |
| Claude chuyển `_meta.claudeCode.options` cho SDK; `settings` là object hoặc đường dẫn file theo cwd; gộp `disallowedTools` của client | `dist/acp-agent.js:8570, 8630-8633, 8688, 8730` |
| Claude so fingerprint (cwd, MCP, system prompt, option) khi load/resume; khác thì hủy rồi dựng lại session; trả config option | `dist/acp-agent.js:395-470` (`OPTION_REBUILDS_SESSION`), `:489-510`, `:1544-1568`, `:8379-8420` |
| Claude gửi `_claude/rateLimit` trong usage update; có `/usage` với cửa sổ 5 giờ và 7 ngày | `dist/acp-agent.js:6084-6094`; `dist/usage-markdown.js` |
| Claude và Codex khai `_session/steering` và `_auth/status_update` | `dist/acp-agent.js:249, 1273`; `codex-acp@2.2.2` `dist/index.js:39759, 39605` |
| Codex: bốn mode, mặc định `agent` (auto review); `workspace-write` cho ghi `$TMPDIR`, `/tmp`, mạng tắt | `dist/index.js:34264-34320` |
| Codex đọc `customInstructions` (chỉ nhận chuỗi) chỉ ở `newSession`; `loadSession`/`resumeSession` không đọc nhưng đọc `mcpServers` | `dist/index.js:28093-28099, 35227-35340` |
| Codex trả config option ở new, load, resume | `dist/index.js:43049-43241` |
| Codex: id và `kind` của option xin quyền (lệnh, sửa file, nới quyền, MCP) | `dist/index.js:32022-32041, 32144-32250` |
| Codex chỉ đưa hạn mức qua text `/status` | `dist/index.js` (`handleRateLimitsUpdated`, `formatSingleRateLimit`) |
| Claude giữ `AskUserQuestion` khi client có form elicitation; câu hỏi, MCP elicitation, hộp thoại dự phòng đi qua `elicitation/create` | `claude-agent-acp@0.95.0` `dist/acp-agent.js:8586, 7779-7783, 8710-8724` |
| Claude mặc định `settingSources: ["user", "project", "local"]`; option của client trải sau nên ghi đè được | `dist/acp-agent.js:8668, 8688` |
| Claude và Codex chạy Read, Write, Edit, Bash trong process của mình (Claude qua `query` của SDK với bộ tool `claude_code`; Codex trong sandbox của nó); mã adapter không có lời gọi `fs/*` hay `terminal/*` tới client | `dist/acp-agent.js:8591-8592, 8877-8884`; `codex-acp@2.2.2` `dist/index.js:35610` |
| Codex: yêu cầu có `additionalPermissions` hay ngữ cảnh mạng là `kind: execute` với option `allow_once`; permission profile dùng `allow_once` cho cả lượt | `dist/index.js:28425, 32091, 32150-32156, 32228-32234` |
| Codex: `request_user_input` đi qua form elicitation | `dist/index.js:32938-32961` |
| Codex thêm `additionalDirectories` vào vùng ghi của sandbox `workspace-write` | `dist/index.js:35878-35885` |
| Crate: đăng ký ext notification, gửi ext request, message không định kiểu | `agent-client-protocol` 3.4.0 `src/jsonrpc.rs:1579, 4300, 5676-5685` |
| Crate: notification parse lỗi thì không tới handler; `SessionUpdate` không có biến thể dự phòng | `src/jsonrpc/handlers.rs:272-279`; `agent-client-protocol-schema` 1.12.0 `src/v1/client.rs:103` |
| Schema có `system_prompt` và `meta` trên new, load, resume | `agent-client-protocol-schema` 1.12.0 `src/v1/agent.rs:824-836, 1015-1027, 1401-1413` |

### B.3 Spike ngày 2026-10-25 (máy macOS của Human, fork `20ea81ef29`)

| Dữ kiện | Ảnh hưởng |
| ------- | --------- |
| Crate 3.4.0 (schema 1.12.0) có `system_prompt` trên new, load, fork, resume; `content` là mảng content block | [6.2](#muc-6-2) |
| ROLE Claude qua `systemPrompt` có hiệu lực 3 lượt, còn sau `/compact`; resume dựng lại option từ request | [6.1](#muc-6), [14](#muc-14) |
| ROLE Antigravity qua `_meta…append` có hiệu lực 3 lượt; không được lưu; còn sau compaction (kiểm bằng mã vai trò); compaction quanh 256 nghìn token; ACP không báo compaction | [6.1](#muc-6), [6.8](#muc-6) |
| Ma trận sandbox Claude; yêu cầu `SandboxNetworkAccess`; option `allow_always` cho sửa ngoài cwd; Bash trong sandbox tự được cho phép; `$TMPDIR` dùng chung `/tmp/claude-501`; `denyRead`, `credentials.files`, luật `Read(//…)` | [6.2](#muc-6-2), [10](#muc-10) |
| Antigravity tài khoản Ultra: danh sách model, `/usage`, 4 session song song, session thăm dò được lưu, prompt đầu khoảng 12,7 nghìn token | [6](#muc-6), [8](#muc-8), [13](#muc-13) |
| Codex `/status`: adapter tự sinh, giống hệt qua 4 phiên bản, giờ reset theo TZ/locale, có email | [6.7](#muc-6) |
| Đăng nhập: Claude báo sai với token môi trường, prompt đầu lỗi `-32000` khi thật sự chưa đăng nhập; Codex báo đúng; Antigravity lỗi `-32000` khi tạo session | [6.5](#muc-6-5) |

### B.4 Tài liệu tham chiếu

| Dữ kiện | Nguồn |
| ------- | ----- |
| “No route currently confines the worker to its workspace” | compound-engineering `skills/ce-work/references/cross-model-execution.md:84` |
| Sandbox `workspace-write` của Codex để `.git` chỉ đọc; gitdir của git worktree nằm ngoài worktree, nên `git commit` trong worktree bị chặn | gsd-core `docs/adr/1239-gsd-embeddable-orchestration-engine.md:269` |
| Claude Code coi `.git/` là đường dẫn được bảo vệ, từ chối agent ghi vào | obra-superpowers `RELEASE-NOTES.md:263` |

---

<a id="phu-luc-c"></a>

## Phụ lục C. Lịch sử thay đổi

| Bản | Nội dung |
| --- | -------- |
| 1.0 | Bản nháp đầu từ brief v3.1: kiến trúc, H1–H10, TD1–TD9, spike S1–S10, truy vết NEED |
| 2.0 | Viết lại theo brief v3.5, các spike và mã adapter. CrewConnection với agent id riêng là đường duy nhất tạo, load, resume session Crew, kể cả khi Zed tự load lại thread (TD10, H5d, H5e). Payload đã kiểm cho từng provider; gửi lại đủ option khi load/resume. Worker không được giao tiếp việc sau khởi động lại. Healthcheck theo provider với ROLE thăm dò. Quy trình chọn chung với trần session và chính sách hạn mức `unknown`. Parser hạn mức. Phát hiện compaction. Gate chọn option theo `kind`. Orchestrator chỉ đọc (TD11). Quét secret trước commit. Sửa lỗi đăng nhập ở tầng Zed (H11). S1–S5 đã có kết quả nên đưa vào nội dung; thêm S11–S13 |
| 2.1 | Human duyệt toàn bộ quyết định thiết kế trong mục 20.1 của bản 2.0 (MCP qua Unix socket, bàn giao qua công cụ có schema, SQLite, client GitHub tự viết, ghim adapter bằng agent id riêng, ghi quyết định tất định, Crew commit bằng App, cấu hình project trong `.zed/settings.json`, nơi đặt artifact, CrewConnection, Orchestrator chỉ đọc). Quyết định nằm thẳng trong mục liên quan; bỏ bảng quyết định chờ duyệt |
| 2.2 | Khớp với kế hoạch triển khai 1.0, theo ba điểm Human đồng ý: agent merge vào main khi đủ hai cổng kiểm chứng; thêm khóa `default_candidates`; thêm mục Crew ở cuối `.rules`. Cập nhật từ việc đọc mã Zed khi lập kế hoạch: cách bật `CrewFeatureFlag` cho bản release (H8), đọc riêng cấu hình user và project (H7, 5.3), thư mục cài riêng cho adapter ghim (H10), mở thread Orchestrator bằng `create_thread_with_options` (7.1), lọc thread import chuyển lên bước 2 (18) |
| 2.3 | Áp 24 đề xuất của báo cáo kiểm thiết kế với mã fork ([`design-check.md`](design-check.md)), Human duyệt 2026-10-25. Thêm H12: chính sách của Crew cho `fs/*` và không tạo worktree ẩn, vì Zed nhận ghi ra ngoài project mà không xin quyền. Từ chối elicitation và chặn `AskUserQuestion`; gate quyết định theo `rawInput`; không hiện `allow_always`. H5b thành cờ chỉ đọc thật; thêm H5f (agent đang chọn). Agent id của Crew đăng ký vào agent server store, `inner` mang id Crew, chép xử lý riêng theo agent id gốc, bỏ qua settings `agent_servers`, nhận session con. Run store là file riêng qua `open_db`, chặn khi không bền. Restricted Mode. `settingSources: ["project"]`; không gửi `additionalDirectories`. Worktree của worker qua `GitStore` và Project không cửa sổ. `McpServer` cần handler `initialize`, thư mục socket 0700, công cụ kiểm trạng thái khi Stop. Id tin nhắn do Crew cấp. ModeGuard đọc lại trạng thái. API `Sandbox` với `TMPDIR` riêng. Secret luôn vào Keychain. Rollback feature flag cần build lại. H9 ở `main.rs`. Số dòng theo fork `11caaae` |
