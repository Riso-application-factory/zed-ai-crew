# M1 · Nền

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Lớp nền mà mọi phần sau dựa vào: run store, cấu hình theo tầng, recovery, gửi option cho session, adapter ghim, CrewConnection, xác nhận đăng nhập, snapshot model, đọc hạn mức, giữ `_meta` của usage, sửa lỗi đăng nhập ở tầng Zed, quy trình đồng bộ upstream |
| Spec | Mục 18 bước 1; 3.2 (H2, H3a, H4, H5d, H10, H11), 3.3, 4, 5, 6, 14.1 |
| Cần có trước | M0 (S6 cho T1.8) |
| Spike | S12 (T1.11) |
| Ghi chú | Lọc thread import (H5e) làm ở T2.3, cùng lúc session Crew đầu tiên xuất hiện (spec 2.2, mục 18) |

## Danh sách task

| Task | Tên | Phụ thuộc | NEED |
| ---- | --- | --------- | ---- |
| T1.1 | Run store | M0 | 02, 41, 43 |
| T1.2 | Cấu hình Crew theo tầng (H7, phần nội dung) | M0 | 02, 27 |
| T1.3 | Nhật ký effect và Recovery lõi | T1.1 | 44 |
| T1.4 | Gửi option khi tạo, load, resume session (H2) | M0 | 02, 55 |
| T1.5 | Adapter ghim theo agent id của Crew (H10) | M0 | 28 |
| T1.6 | ProviderProfile cho ba provider | T1.4, T0.9 | 25, 55, 61 |
| T1.7 | CrewAgentServer, CrewConnection, registry trong `Agent::server()` (H5d) | T1.1, T1.4, T1.5, T1.6 | 55 |
| T1.8 | Xác nhận đăng nhập và session thăm dò (H3a) | T1.7, S6 | 23, 56 |
| T1.9 | Snapshot danh mục model | T1.8 | 42, 56 |
| T1.10 | Giữ `_meta` của usage update, báo update bị bỏ (H4) | M0 | 31, 43, 60 |
| T1.11 | Spike S12: hạn mức của Claude | T1.10 | 60 |
| T1.12 | Đọc hạn mức | T1.8, T1.10, T1.11 | 02, 60 |
| T1.13 | Sửa lỗi đăng nhập ở tầng Zed (H11) | M0 | 56 |
| T1.14 | Quy trình đồng bộ upstream | T0.2 | 59 |

---

### T1.1 · Run store

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Lưu mọi fact của Crew vào SQLite theo spec mục 4 |
| Căn cứ | NEED 02, 41, 43; spec 4.1–4.4 |
| Được sửa | `crates/crew/src/store.rs` (và module con nếu cần); `crates/crew/Cargo.toml` (thêm `db`, `sqlez`, `paths`) |
| Bàn giao | Domain `CrewDb`, schema v1, API ghi event và projection, kiểm store bền |

**Thiết kế**
- File SQLite riêng của Crew, mở bằng `db::open_db::<CrewDb>` (`crates/db/src/db.rs:174`), theo mẫu `GlobalKeyValueStore` (`crates/db/src/kvp.rs:243-250`) **[Đã kiểm]**. Không dùng `db::static_connection!`: macro này trỏ vào database chung của app Zed (`AppDatabase::global`, `db.rs:266-268`) **[Đã kiểm]**, trái spec 4.1. File nằm trong thư mục dữ liệu của Zed (crate `paths`), không nằm trong repo nào (NEED 02).
- **Store phải bền.** Khi có `ZED_STATELESS`, hoặc mở file thất bại, `open_db` âm thầm dùng database trong bộ nhớ (`db.rs:178-180, 197-202`) **[Đã kiểm]**. Lúc khởi động Crew kiểm store nằm trên file; không thì chặn mọi hành vi Crew và báo Human kèm lý do, vì dữ liệu sẽ mất khi tắt app.
- `impl Domain for CrewDb` với `NAME = "crew"` và `MIGRATIONS`. Migration chỉ thêm mới: sqlez mặc định không cho sửa migration đã chạy (`should_allow_migration_change` trả `false`) **[Đã kiểm]**.
- Schema v1 gồm mọi bảng ở spec 4.2.
- Trigger SQLite chặn `UPDATE` và `DELETE` trên `crew_events`, và chặn sửa các cột kết quả của session đã `done` (NEED 41).
- API: `append_event` (cấp `seq` kế tiếp trong cùng transaction với việc cập nhật projection), các hàm đọc projection, `lock_done`. Không có hàm sửa hay xóa event.
- `project_id` = hash của đường dẫn đã chuẩn hóa của checkout chính.

**Tiêu chí nghiệm thu**
- Cho hai lần ghi event đồng thời vào một session, khi đọc lại, thì `seq` liên tục và không trùng.
- Cho một event đã ghi, khi chạy `UPDATE` hoặc `DELETE` trực tiếp bằng SQL, thì bị từ chối.
- Cho session đã `done`, khi cố sửa `outcome`, thì bị từ chối (NEED 41).
- Cho hai project, khi truy vấn theo `project_id` của project A, thì không thấy bản ghi của B (NEED 02).
- Cho projection bị xóa, khi dựng lại từ `crew_events`, thì kết quả trùng với projection trước đó.
- Cho store đã mở, khi tìm bảng `crew_events`, thì bảng nằm trong file riêng của Crew, không trong database chung của Zed.
- Cho `ZED_STATELESS=1` hoặc thư mục database không ghi được, khi Crew khởi động, thì Crew bị chặn kèm lý do và không tạo session nào.

**Kiểm chứng:** `cargo test -p crew store` (dùng `db::open_test_db::<CrewDb>`).

---

### T1.2 · Cấu hình Crew theo tầng (H7, phần nội dung)

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Đọc cấu hình user-scope và project-scope riêng biệt, gộp theo quy tắc chỉ thu hẹp |
| Căn cứ | NEED 02, 27; spec 5.1–5.3; brief 5.6.2 |
| Được sửa | `crates/settings_content/src/project.rs` (thêm một trường, thêm `crew` vào `flattened_deserialize!`), `crates/settings_content/src/crew.rs` (mới, kiểu nội dung), `crates/crew/src/config.rs` |
| Bàn giao | `CrewSettingsContent`, hàm gộp, snapshot cấu hình kèm hash, phát hiện Restricted Mode |

**Thiết kế**
- Thêm `pub crew: Option<CrewSettingsContent>` vào `ProjectSettingsContent`.
  - `SettingsContent` flatten `ProjectSettingsContent` (`settings_content.rs:175-176`), nên khóa `crew` hợp lệ trong `settings.json` của user.
  - `.zed/settings.json` được parse thành `ProjectSettingsContent` (`crates/settings/src/settings_store.rs:1110`), nên khóa này cũng hợp lệ ở project **[Đã kiểm]**.
  - Thêm `crew` vào danh sách trường của `flattened_deserialize!(ProjectSettingsContent { … })` (`crates/settings_content/src/project.rs:90-96`) **[Đã kiểm]**.
- **Restricted Mode:** với worktree chưa được tin, Zed giữ lại `.zed/settings.json` thay vì áp dụng (`crates/project/src/project_settings.rs:1357-1384`, kiểm qua `TrustedWorktrees`) **[Đã kiểm]**, nên `local_settings()` rỗng. Crew phát hiện trường hợp này và chặn, kèm chỉ dẫn tin worktree theo luồng của Zed; không chạy với cấu hình thiếu phần project.
- **Không dùng giá trị đã gộp của Zed** (`Settings::get`): Zed cho project ghi đè user, ngược với quy tắc chỉ thu hẹp. Crew đọc riêng hai nguồn:
  - user: `SettingsStore::raw_user_settings()` (`settings_store.rs:506`);
  - project: `SettingsStore::local_settings(root_id)` (`:1198`), chỉ cho worktree gốc của checkout chính **[Đã kiểm]**. Hàm này trả cả file `.zed/settings.json` nằm trong thư mục con, kèm đường dẫn tương đối **[Đã kiểm]**; Crew chỉ lấy file ở gốc worktree (đường dẫn rỗng).
- Gộp theo spec 5.3. Khóa chỉ dành cho user-scope xuất hiện trong file project thì bị bỏ qua và ghi cảnh báo. Workspace của worker bị bỏ qua.
- Giá trị Q7 chưa đặt (`null`) nghĩa là “chưa cấu hình”. Chỗ nào cần giá trị đó thì chặn và hỏi Human, không tự chọn số (NEED 30).
- Snapshot cấu hình đã gộp kèm hash, lưu vào session lúc bắt đầu (NEED 28).

**Tiêu chí nghiệm thu** (test theo bảng)
- Cho project đặt allowlist mạng rộng hơn trần user-scope, khi gộp, thì kết quả là phần giao, kèm cảnh báo.
- Cho project thêm đường dẫn bí mật, khi gộp, thì kết quả là phần hợp.
- Cho project chặn một provider, khi gộp, thì provider đó bị loại.
- Cho file project chứa `pinned_adapters` (khóa của user), khi gộp, thì khóa bị bỏ qua, kèm cảnh báo.
- Cho workspace của worker có `.zed/settings.json` khác checkout chính, khi Crew đọc cấu hình, thì dùng checkout chính.
- Cho cùng nội dung cấu hình, khi tính hash hai lần, thì cho cùng giá trị.
- Cho `.zed/settings.json` có khóa `crew`, khi parse thành `ProjectSettingsContent`, thì nhận được giá trị của khóa.
- Cho worktree chưa được tin có `.zed/settings.json` chứa khóa `crew`, khi Crew đọc cấu hình, thì bị chặn kèm chỉ dẫn, không gộp như project không có cấu hình.

**Kiểm chứng:** `cargo test -p crew config` và `cargo test -p settings_content`.

---

### T1.3 · Nhật ký effect và Recovery lõi

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Ghi ý định trước mọi tác động ra ngoài; khi khởi động lại thì đối soát, không tự làm lại |
| Căn cứ | NEED 44; spec 4.2 (`crew_effects`), 14.1 bước 1–3 và 6 |
| Được sửa | `crates/crew/src/effects.rs`, `crates/crew/src/recovery.rs` |
| Bàn giao | API effect; khung đối soát với reconciler theo loại; reconciler cho git |

**Thiết kế**
- `begin_effect(kind, idempotency_key, intent)` ghi `intent` trước khi thực hiện; `complete_effect` và `fail_effect` ghi kết quả.
- Khi khởi động: mỗi effect ở `intent` hoặc `unknown` được giao cho reconciler theo loại.
  - Reconciler git (nhánh, commit) có ngay ở task này. Reconciler GitHub thêm ở T4.6.
  - Tìm thấy kết quả thì ghi `done`. Không tìm thấy thì ghi `unknown` và báo. Không bao giờ tự thực hiện lại.
- Task đang `running` khi Zed tắt chuyển `interrupted`.
- Store không bền (T1.1) thì Recovery dừng và báo, không đối soát gì.

**Tiêu chí nghiệm thu**
- Cho effect tạo commit bị ngắt giữa `intent` và kết quả, khi khởi động lại và commit có trong repo, thì effect thành `done`.
- Cho trường hợp như trên nhưng commit không có, khi khởi động lại, thì effect thành `unknown`, có báo, và không có commit mới nào được tạo.
- Cho task `running` lúc tắt, khi khởi động lại, thì task là `interrupted`.
- Cho store đang chạy trong bộ nhớ, khi khởi động, thì Recovery dừng có báo và không effect nào đổi trạng thái.

**Kiểm chứng:** `cargo test -p crew recovery` (repo git tạm, tiêm crash bằng cách drop giữa hai bước).

---

### T1.4 · Gửi option khi tạo, load, resume session (H2)

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Crew gửi được system prompt, `_meta` và danh sách MCP tùy chọn qua connection ACP của Zed |
| Căn cứ | NEED 02, 55; spec 3.2 (H2), 6.3 |
| Được sửa | Chỉ `crates/agent_servers/src/acp/v1.rs` |
| Bàn giao | `SessionOptions`; `AcpConnection::{new,load,resume}_session_with_options` |

**Thiết kế**
- `SessionOptions` khai trong `agent_servers`, gần `AcpConnection`, vì core không phụ thuộc crate Crew.
- Đặt `.system_prompt(...)` chỉ khi agent đã khai `sessionCapabilities.systemPrompt`. Option có `system_prompt` mà agent không khai capability thì trả lỗi, không bỏ qua im lặng.
- Gắn `_meta` của caller vào request.
- `McpServerSelection::Explicit` thay cho `mcp_servers_for_project` (`v1.rs:9134`) **[Đã kiểm]**.
- Bản `*_with_options` không áp mode và config option mặc định từ settings `agent_servers.<agent id>` (`AcpConnectionDefaults`, `v1.rs:94-168`). Bản hiện có áp chúng sau new, load, resume mà không phát event (`v1.rs:1310-1345`) **[Đã kiểm]**; session Crew chỉ nhận mode và config option do Crew đặt (T1.7, T2.2).
- `new_session`, `load_session`, `resume_session` hiện có giữ nguyên hành vi. Trait `AgentConnection` không đổi.

**Tiêu chí nghiệm thu** (dùng `FakeAcpAgentServer` từ module `test_support` của `agent_servers`, bật feature `test-support`, `v1.rs:1708, 1767` **[Đã kiểm]**)
- Cho agent có capability `systemPrompt`, khi gọi bản `*_with_options` cho new, load và resume, thì request agent nhận có `systemPrompt`, `_meta` và đúng danh sách MCP của caller.
- Cho agent không có capability, khi option có `system_prompt`, thì gọi hàm trả lỗi và không gửi request.
- Cho `new_session` cũ, khi gọi, thì request vẫn mang MCP của project như trước.
- Cho settings có `agent_servers.<agent id>` với `default_mode`, khi gọi bản `*_with_options`, thì agent không nhận yêu cầu đổi mode nào; gọi `new_session` cũ thì vẫn áp như trước.

**Kiểm chứng:** `cargo test -p agent_servers`

---

### T1.5 · Adapter ghim theo agent id của Crew (H10)

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Mỗi provider có một agent id của Crew, cài đúng phiên bản ghim; bản Zed tự cập nhật không bị ảnh hưởng |
| Căn cứ | NEED 28, 61 (ghim adapter là tầng Zed); spec 3.2 (H10), 6.4 |
| Được sửa | `crates/project/src/agent_server_store.rs`; dữ liệu bản ghim trong `crates/crew_acp/src/pinned.rs` |
| Bàn giao | Agent id `crew-claude-acp`, `crew-codex-acp`, `crew-antigravity-acp` |

**Thiết kế**
- **Npx:** dùng spec chính xác `@agentclientprotocol/claude-agent-acp@0.95.0` và `@agentclientprotocol/codex-acp@2.2.2` (registry) **[Đã kiểm]**. Không đi qua `bounded_npm_package_spec`, vì hàm này đổi `pkg@X` thành `pkg@0.0.0 - X`, nên npm có thể dùng bản cũ hơn trong cache (`agent_server_store.rs:1456-1468`) **[Đã kiểm]**.
- **Binary Antigravity 1.4.3:** URL và sha256 của bản ghim lưu trong `crew_acp` vì registry chỉ giữ bản mới nhất. Bản darwin-aarch64 là `https://dl.google.com/agy-extensions/releases/macos/agy-acp-server-1.4.3-darwin-arm64.zip`, sha256 `c57db0a9b8b712daa9c3c17cdb990a61842ea569f279e8bd79d7aa85a303b20b`, lệnh `./agy_acp_server.par` **[Đã kiểm, registry]**.
- **Thư mục cài riêng** cho agent id của Crew. Hàm `remove_stale_versioned_archive_cache_dirs` (`:1055`, gọi ở `:1309`) xóa mọi bản khác bản đang chạy trong cùng thư mục **[Đã kiểm]**. Dùng chung thư mục với agent gốc thì hai bên sẽ xóa bản của nhau. Agent registry cài vào thư mục theo agent id (`:403-405`) **[Đã kiểm]**; entry của Crew cũng đặt thư mục theo agent id Crew.
- **Đăng ký trong store.** Connection store chỉ giữ agent id custom có trong `external_agents` của store (`crates/agent_ui/src/agent_connection_store.rs:277`), và `reregister_agents` xóa sạch rồi dựng lại danh sách này từ settings và registry (`agent_server_store.rs:348`) **[Đã kiểm]**. Vì vậy agent id của Crew được thêm vào ngay trong mỗi lần dựng lại, để có mặt trước `AgentPanel::load` (`crates/zed/src/zed.rs:864`) và sau mỗi lần settings hay registry đổi.
- Env dùng chung như agent gốc của Zed. Các nhánh env theo agent id gốc ở `custom.rs` không áp cho id Crew; T1.7 chép những nhánh cần thiết.

**Tiêu chí nghiệm thu**
- Cho agent id Crew của Claude, khi Zed dựng lệnh chạy, thì lệnh dùng `@agentclientprotocol/claude-agent-acp@0.95.0` đúng nguyên văn.
- Cho settings `agent_servers` đổi làm store dựng lại danh sách, khi tra `external_agents`, thì ba agent id Crew vẫn có.
- Cho bản Antigravity của Zed vừa cập nhật lên bản mới hơn, khi chạy agent gốc rồi chạy agent id của Crew, thì thư mục bản 1.4.3 vẫn còn và lệnh của Crew trỏ vào đó.
- Cho file tải về có sha256 sai, khi cài, thì cài thất bại kèm lỗi.

**Kiểm chứng:** `cargo test -p project agent_server_store` (filesystem giả).

---

### T1.6 · ProviderProfile cho ba provider

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Mọi khác biệt giữa provider nằm trong một chỗ, sinh payload tất định, không ghép tay |
| Căn cứ | NEED 25, 55, 61; spec 6.1, 6.2, 6.3 |
| Được sửa | `crates/crew_acp/src/profile.rs` và một file cho mỗi provider |
| Bàn giao | Trait `ProviderProfile` và 3 hiện thực; contract test ROLE |

**Thiết kế**
- Hàm thuần sinh `SessionOptions` cho từng loại session: worker, Orchestrator, thăm dò. Kèm mode, mode bị cấm, mức cách ly, cách xác nhận đăng nhập, nguồn hạn mức, tín hiệu compaction, ngưỡng context thực tế, hỗ trợ steering.
- Payload đúng spec 6.2:
  - Claude: `systemPrompt` mode `append` với `content` là mảng text block; `_meta.claudeCode.options` gồm `sandbox`, `settings` (luôn là object), `disallowedTools: ["AskUserQuestion"]` và `settingSources: ["project"]`. Hai option sau có ở mọi session Crew trên Claude (worker, Orchestrator, thăm dò).
  - Antigravity: `_meta.antigravity.systemPrompt.append` là chuỗi.
  - Codex: `_meta.jetbrains.air.customInstructions` là chuỗi.
- **`AskUserQuestion`:** Zed khai form elicitation, nên adapter Claude giữ tool này (`acp-agent.js:8586`) và câu hỏi của agent đi đường elicitation, không qua gate (T2.2). Adapter gộp `disallowedTools` của client vào danh sách của nó (`:8730`) **[Đã kiểm]**.
- **`settingSources`:** adapter mặc định đọc `["user", "project", "local"]` (`acp-agent.js:8668`), và option của client ghi đè được giá trị này (`:8688`) **[Đã kiểm]**. Chỉ dùng `project`: bỏ settings user của Human và `.claude/settings.local.json` còn sót trong worktree, vì hai nguồn này đổi được quyền của session; giữ `CLAUDE.md`, `.claude/settings.json` của repo và cách đăng nhập như cũ **[Suy luận]**, contract test bên dưới xác nhận.
- Tự kiểm kiểu giá trị ROLE trước khi gửi, vì adapter Antigravity và Codex bỏ qua kiểu sai mà không báo lỗi **[Spike]** **[Đã kiểm]**.
- Đường dẫn `$TMPDIR` thật của sandbox Claude lưu theo phiên bản adapter. Ban đầu là `/tmp/claude-501` **[Spike]**; T3.6 đo lại bằng contract test.

**Tiêu chí nghiệm thu**
- Cho một ROLE mẫu, khi sinh payload cho từng provider, thì JSON trùng snapshot trong test.
- Cho mọi tổ hợp option, khi sinh payload Claude, thì `settings` luôn là object, không bao giờ là chuỗi (NEED 61, spec 6.2).
- Cho ROLE rỗng hoặc sai kiểu, khi sinh payload, thì trả lỗi.
- Cho mọi loại session Claude, khi sinh payload, thì `disallowedTools` có `AskUserQuestion` và `settingSources` đúng bằng `["project"]`.
- **Contract:** cho ROLE yêu cầu marker, khi tạo session mới, load và resume trong process mới, thì marker có ở cả ba trường hợp với Claude và Antigravity. Với Codex chỉ kiểm session mới; phần resume là S11 (T2.10). Chi phí khoảng 8 prompt ngắn.
- **Contract:** cho worker Claude với `settingSources: ["project"]` trên repo thử có `CLAUDE.md` chứa marker, khi hỏi marker, thì agent trả đúng, và session vẫn chạy bằng gói đã đăng nhập như trước. Cho session Claude, khi prompt yêu cầu agent hỏi người dùng trước khi làm, thì không có `elicitation/create` nào. Chi phí khoảng 2 prompt.

**Kiểm chứng**
```sh
cargo test -p crew_acp profile
CREW_CONTRACT=1 cargo test -p crew_acp --test contract role_ -- --ignored --test-threads=1
CREW_CONTRACT=1 cargo test -p crew_acp --test contract claude_options_ -- --ignored --test-threads=1
```

---

### T1.7 · CrewAgentServer, CrewConnection và registry trong `Agent::server()` (H5d)

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Mọi đường Zed mở hoặc load lại thread của agent id Crew đều đi qua `CrewConnection`, và mỗi lần load/resume gửi đủ option |
| Căn cứ | NEED 55; spec 3.3, 6.2 (Load và resume), 14.1 (Bất biến) |
| Được sửa | `crates/agent_servers` (module registry mới); `crates/agent_ui/src/agent_ui.rs` (`Agent::server()`, `:578-591`); `crates/crew_acp/src/connection.rs`; `crates/crew` (hiện thực trait bản ghi) |
| Bàn giao | Registry factory; `CrewAgentServer`; `CrewConnection`; bảng `crew_acp_sessions` được dùng |

**Thiết kế**
- Registry trong `agent_servers`: `register_agent_server_factory(agent_id, factory)` và hàm tra cứu.
  - `Agent::server()` tra registry trước khi tạo `CustomAgentServer`. Đây là nơi duy nhất gọi `CustomAgentServer::new` (`agent_ui.rs:586`). Agent Panel, thread import và archive view đều đi qua hàm này (`agent_panel.rs:4665`, `thread_import.rs:704`, `threads_archive_view.rs:832`) **[Đã kiểm]**.
  - Factory đăng ký trong `crew_ui::init` (T0.5), trước khi `AgentPanel::load` khôi phục thread. Connection store giữ server đầu tiên được cache cho mỗi agent id (`agent_connection_store.rs:149-151`) **[Đã kiểm]**, nên đăng ký muộn thì factory không bao giờ được dùng.
  - Ẩn agent id của Crew khỏi bộ chọn agent và trang cấu hình agent.
- `CrewAgentServer` tạo `AcpConnection` bên trong bằng chính agent id Crew. `AcpThread` giữ connection này (`v1.rs:919-923`) và agent id lưu của thread lấy từ nó (`crates/agent_ui/src/thread_metadata_store.rs:1305`) **[Đã kiểm]**; tạo bằng id gốc thì mở lại thread sẽ đi tới agent gốc.
  - Chép các nhánh env mà Zed chỉ áp cho id gốc: xóa `ANTHROPIC_API_KEY` như với `claude-acp` (`custom.rs:236-237`), chuyển tiếp key của Codex (`:239-245`) **[Đã kiểm]**. Thiếu nhánh Claude thì adapter có thể đăng nhập bằng API key trong env thay cho gói đã đăng nhập (NEED 25).
  - Id Crew không nhận `agent_servers.<id>` từ settings: `observe_settings` áp mode và config mặc định theo id sau new, load, resume mà không phát event (`v1.rs:156-168, 1310-1345`) **[Đã kiểm]**. `CrewConnection` chỉ gọi bản `*_with_options` của H2, vốn không áp các mặc định này (T1.4). ModeGuard (T2.2) vẫn đọc lại trạng thái sau đó.
- `CrewConnection` giữ `Rc<AcpConnection>`, lấy qua `into_any()` (`connection.rs:304`) **[Đã kiểm]**, theo mẫu `FakeAcpAgentConnection` trong `v1.rs` **[Đã kiểm]**.
  - `load_session`, `resume_session`: dựng `SessionOptions` từ `crew_acp_sessions.options_json`, gửi giống hệt từng byte trong cùng process. Adapter Claude so fingerprint và dựng lại session khi option khác (`acp-agent.js:489-510, 8379-8420`) **[Đã kiểm]**.
  - Session id không có trong bản ghi: từ chối. Ngoại lệ là session con mà adapter báo trong `_meta.subagent_session_info` của một session Crew đã biết: Zed load session con trên cùng connection và bỏ qua lỗi load (`conversation_view.rs:1590-1620, 2438`) **[Đã kiểm]**, nên từ chối sẽ làm view của session con lỗi mà không báo. Crew ghi session con vào `crew_acp_sessions` (kind `child`) và cho load ở chế độ chỉ xem.
  - Sau khi Zed khởi động lại: thay đường dẫn socket MCP, giữ nguyên phần còn lại.
  - `new_session` từ UI (session Orchestrator) nối ở T2.3. Ở task này `CrewConnection` có API nội bộ `create_session(kind, options)`.
  - Lời gọi theo thread (prompt, cancel, mode, config option, close) đi thẳng tới connection bên trong mà `AcpThread` giữ (`v1.rs:919-923`) **[Đã kiểm]**. `CrewConnection` chỉ thấy new, load, resume, authenticate, danh sách session và định tuyến elicitation, nên logic Crew không đặt ở các method theo thread.
  - Trait `AgentConnection` có 26 method có hiện thực mặc định (`connection.rs:95-304`) **[Đã kiểm]**. Wrapper quên chuyển tiếp một method vẫn biên dịch, nên có test kiểm từng method.
- `crew_acp` khai trait truy cập bản ghi session; `crew` hiện thực trait trên run store và đăng ký lúc khởi tạo (tổng quan mục 3).

**Tiêu chí nghiệm thu**
- Cho agent id đã đăng ký trong registry, khi gọi `Agent::server()`, thì nhận server của factory; với agent id khác thì nhận `CustomAgentServer` như cũ.
- Cho session Crew đã tạo, khi load hai lần trong cùng process, thì hai request agent nhận giống hệt từng byte ở phần `systemPrompt`, `_meta` và `mcpServers`.
- Cho session id không có trong `crew_acp_sessions` và không phải session con của session Crew, khi load qua agent id Crew, thì bị từ chối.
- Cho session con mà adapter báo trong một session Crew, khi Zed load nó, thì load thành công, có bản ghi kind `child`, và view chỉ xem.
- Cho thread Crew đã lưu, khi đọc metadata, thì agent id là id Crew.
- Cho env có `ANTHROPIC_API_KEY`, khi `CrewAgentServer` dựng lệnh cho `crew-claude-acp`, thì biến này rỗng như với `claude-acp`.
- Cho settings có `agent_servers.crew-claude-acp` với `default_mode`, khi tạo session Crew, thì mode không đổi theo settings.
- Cho từng method có hiện thực mặc định của `AgentConnection`, khi gọi trên `CrewConnection`, thì kết quả trùng với gọi trên connection bên trong.
- Cho từng đường Zed tự load lại thread, khi chạy với agent id Crew, thì agent giả nhận request load có đủ ROLE, `_meta` và MCP. Các đường cần phủ: khôi phục lúc khởi động (`AgentPanel::load`, `zed.rs:864`), mở thread có session id (`conversation_view.rs:1360-1371`), kết nối lại (`:1211-1268`), thread rảnh bị giải phóng rồi mở lại (`agent_panel.rs:4300-4325`), session con (`conversation_view.rs:2438`) **[Đã kiểm]**.

**Kiểm chứng:** `cargo test -p crew_acp connection`, `cargo test -p agent_ui`

---

### T1.8 · Xác nhận đăng nhập và session thăm dò (H3a)

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Mỗi lần Zed khởi động, Crew biết provider nào đã đăng nhập, bằng cơ chế riêng của từng provider |
| Căn cứ | NEED 23, 56; spec 6.5; báo cáo S6 |
| Được sửa | `crates/agent_servers/src/acp/v1.rs` (H3a: handler `_auth/status_update`); `crates/crew_acp/src/health.rs` |
| Bàn giao | Bảng `crew_health` được cập nhật mỗi lần khởi động; session thăm dò dùng lại |

**Thiết kế**
- Thuật toán theo spec 6.5:
  - Codex tin thông báo `_auth/status_update`.
  - Claude bắt buộc healthcheck ping-pong.
  - Antigravity: lỗi `-32000` khi tạo session nghĩa là chưa đăng nhập; còn lại thì healthcheck.
- Session thăm dò:
  - Nằm trong thư mục `crew/probe/` dưới thư mục dữ liệu của Zed (crate `paths`).
  - Bản ghi `kind = probe`. Luôn resume bản ghi cũ; chỉ tạo mới khi không resume được.
  - Có ROLE thăm dò: “chỉ trả lời `pong`, không dùng công cụ”.
  - Mọi yêu cầu xin quyền bị từ chối và mọi elicitation bị decline. PolicyGate đầy đủ có ở T2.2; ở task này dùng bộ trả lời “từ chối tất cả” cho session thăm dò.
- Thời hạn chờ lấy từ cấu hình.
- `not_logged_in` chỉ dẫn tới luồng đăng nhập của Zed, không tự đăng nhập (NEED 17, 56).

**Tiêu chí nghiệm thu** (agent giả)
- Cho Codex gửi `kind = "account"`, khi khởi động, thì `confirmed`; gửi `kind = "none"` thì `not_logged_in`; không gửi gì trong thời hạn thì chạy healthcheck.
- Cho Antigravity trả `-32000` khi tạo session, khi khởi động, thì `not_logged_in`.
- Cho Claude tạo session được nhưng prompt trả `-32000`, khi khởi động, thì `not_logged_in`.
- Cho prompt `ping` có trả lời, khi khởi động, thì `confirmed`.
- Cho lỗi khác hoặc quá hạn, khi khởi động, thì `unconfirmed` kèm chi tiết.
- Cho lần khởi động thứ hai, khi chạy healthcheck, thì dùng lại session thăm dò cũ, không tạo session mới.
- **Contract:** cho ba provider thật đã đăng nhập, khi chạy healthcheck, thì cả ba `confirmed`. Chi phí 2 prompt (Claude, Antigravity); Antigravity tốn khoảng 12,7 nghìn token cho prompt đầu **[Spike]**.

**Kiểm chứng:** `cargo test -p crew_acp health`, `cargo test -p agent_servers`, cộng contract test `health_`.

---

### T1.9 · Snapshot danh mục model

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Danh mục model của từng provider luôn có, ghi tuổi và nguồn, không hardcode |
| Căn cứ | NEED 42, 56; spec 6.6 |
| Được sửa | `crates/crew_acp/src/snapshot.rs` |
| Bàn giao | Bảng `crew_model_snapshots` được cập nhật; sự kiện model xuất hiện hoặc biến mất |

**Thiết kế**
- Lấy từ config option mỗi khi Crew tạo, load hoặc resume session. Claude và Codex trả config option cả khi load/resume **[Đã kiểm]**.
- Quá `snapshot_max_age` thì resume session thăm dò. Provider không trả config option khi resume thì tạo session mới trong thư mục thăm dò. Với Antigravity, mỗi lần như vậy để lại một session không xóa được **[Spike]**.
- Sync lỗi thì giữ snapshot cũ và gắn `stale`; không bao giờ coi là danh sách rỗng.
- Model biến mất mà cấu hình hay playbook còn tham chiếu thì báo lệch (NEED 42); không tự sửa cấu hình.

**Tiêu chí nghiệm thu**
- Cho config option của một session mới, khi nhận, thì snapshot cập nhật kèm thời điểm và nguồn.
- Cho lần sync lỗi, khi đọc snapshot, thì nhận bản cũ gắn `stale`.
- Cho model bị rút mà cấu hình còn tham chiếu, khi sync, thì có sự kiện báo lệch và cấu hình giữ nguyên.

**Kiểm chứng:** `cargo test -p crew_acp snapshot`

---

### T1.10 · Giữ `_meta` của usage update, báo update bị bỏ (H4)

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Crew nhận được `used`, `size` và `_meta` của từng usage update, và biết khi Zed bỏ qua một update |
| Căn cứ | NEED 31, 43, 60; spec 3.2 (H4) |
| Được sửa | H4a: chỉ `update_usage` trong `crates/acp_thread/src/acp_thread.rs` (`:5366`). H4b: `crates/crew_acp/src/dropped_updates.rs`, đọc luồng debug của `AcpConnection`; chỉ khi luồng này không đủ mới thêm handler ở tầng SDK trong `crates/agent_servers/src/acp/v1.rs` |
| Bàn giao | Event usage mang `used`, `size`, `_meta`; sự kiện `update_dropped { kind }` của Crew |

Hiện `update_usage` chỉ giữ `cost.meta` **[Đã kiểm]**. Giữ diff nhỏ nhất có thể, vì `acp_thread.rs` đổi thường xuyên ở upstream.

H4b không đặt ở nhánh `_ => {}` của `handle_session_update` (`acp_thread.rs:4526`): update có loại lạ bị SDK từ chối ngay lúc parse (`agent-client-protocol-3.4.0 src/jsonrpc/handlers.rs:272-279`; enum `SessionUpdate` không có biến thể dự phòng) **[Đã kiểm]**, nên nhánh đó chỉ thấy loại đã biết. Crew đọc `AcpConnection::subscribe_debug_messages()` (`v1.rs:488-495`) và ghi update có loại lạ hoặc loại Zed không xử lý. Đây là công cụ gỡ lỗi, định dạng có thể đổi, nên test của task này chạy lại mỗi lần đồng bộ upstream (T1.14). Chỉ ghi loại update, không ghi payload.

**Tiêu chí nghiệm thu**
- Cho usage update có `_meta["_claude/rateLimit"]` ở cấp trên, khi thread xử lý, thì event phát ra mang nguyên `_meta`.
- Cho chuỗi usage update, khi xử lý, thì mỗi update phát một event với `used` và `size` của chính nó.
- Cho agent giả gửi một session update có loại lạ, khi Crew nhận, thì có `update_dropped` kèm loại, không có payload, và state của thread không đổi.

**Kiểm chứng:** `cargo test -p acp_thread`, `cargo test -p crew_acp dropped_updates`

---

### T1.11 · Spike S12: hạn mức của Claude

| Mục | Nội dung |
| --- | -------- |
| Câu hỏi | Định dạng output thật của `/usage` của Claude là gì? Cấu trúc thật của `_claude/rateLimit` trong usage update là gì? |
| Căn cứ | Spec 20.1 (S12), 6.7; mã adapter `usage-markdown.js`, `acp-agent.js:6084-6094` **[Đã kiểm]** |
| Chặn | Phần Claude của T1.12 |
| Bàn giao | `docs/crew/spikes/S12.md`; fixture đã che dữ liệu trong `crates/crew_acp/test_fixtures/contract/` |

**Các bước:** dùng harness T0.9 gửi `/usage` trong session thăm dò, và ghi `_meta` của usage update sau một prompt ngắn. Che email và id tài khoản trước khi lưu.

**Tiêu chí xong:** fixture cho cả hai nguồn; báo cáo nêu các cửa sổ (5 giờ, 7 ngày), đơn vị và định dạng giờ reset.

---

### T1.12 · Đọc hạn mức

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Biết phần trăm còn lại theo cửa sổ của từng nhóm model, chỉ từ những gì provider báo qua ACP |
| Căn cứ | NEED 02, 60, 61; spec 6.7; báo cáo S1–S5 (mục Q4, Q5) và S12 |
| Được sửa | `crates/crew_acp/src/quota.rs` và một parser cho mỗi provider |
| Bàn giao | Bảng `crew_quota` được cập nhật; hàm `quota_for_model` |

**Thiết kế**
- **Codex:** gửi `/status` trong session thăm dò. Parse mỗi dòng `**<limit> <label>:** <N>% left[ (resets <time>)]` **[Spike]**. Giờ reset chỉ parse best effort; không parse được thì `unknown`, phần trăm vẫn dùng. Bỏ dòng `**Account:**` (chứa email) trước khi lưu.
- **Antigravity:** gửi `/usage`. Parse nhóm, danh sách “Models within this group”, hai cửa sổ tuần và 5 giờ, và các output đặc biệt → `unknown` (spec 6.7).
- **Claude:** `_meta["_claude/rateLimit"]` (từ H4) và `/usage`, theo định dạng S12.
- **Gắn model với nhóm:** dùng bảng tên hiển thị ↔ model id trong ProviderProfile, khóa bằng contract test. Không khớp nhóm nào thì `unknown`. Gemini 4 Argon là `unknown` cho tới S13 **[Spike]**.
- Ứng viên chỉ đạt khi mọi cửa sổ của nhóm đều trên ngưỡng (logic chọn nằm ở T2.8; task này cung cấp dữ liệu).

**Tiêu chí nghiệm thu** (fixture lấy từ mẫu trong báo cáo S1–S5 và S12)
- Cho output `/status` mẫu, khi parse, thì được `5h` 95% và `Weekly` 99%, và không lưu dòng `**Account:**`.
- Cho output `/status` theo locale `vi_VN` (`13:18 on 1 thg 11`), khi parse, thì phần trăm đúng và giờ reset là `unknown`.
- Cho output `/usage` mẫu của Antigravity, khi parse, thì có 2 nhóm × 2 cửa sổ với giờ reset UTC.
- Cho “No quota limits reported.” hoặc “data not available yet”, khi parse, thì `unknown`.
- Cho `gemini-4-argon`, khi tra nhóm, thì `unknown`.
- **Contract:** cho adapter thật ở bản ghim, khi gửi `/status` và `/usage`, thì parser đọc được và fixture mới (đã che dữ liệu) khớp định dạng đã khóa.

**Kiểm chứng:** `cargo test -p crew_acp quota`, cộng contract test `quota_`.

---

### T1.13 · Sửa lỗi đăng nhập ở tầng Zed (H11)

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Sửa ba lỗi đăng nhập phía Zed mà brief NEED 56 giao cho tầng Zed của fork |
| Căn cứ | NEED 56; spec 3.2 (H11) |
| Được sửa | `crates/agent_servers/src/custom.rs`; luồng authenticate trong `crates/agent_servers/src/acp/v1.rs`; UI đăng nhập agent trong `crates/agent_ui` |
| Bàn giao | Ba bản sửa, mỗi bản có test |

Đây là sửa lỗi của Zed, nên **không** nằm sau `CrewFeatureFlag`. Ghi cả ba vào `docs/crew/hook-points.md`.

**Các bước**
1. `custom.rs:243-244` đang chuyển tiếp `OPEN_AI_API_KEY` **[Đã kiểm]**, nhưng adapter chỉ đọc `CODEX_API_KEY` và `OPENAI_API_KEY` **[Baseline]**. Đổi thành `OPENAI_API_KEY`.
2. Ô nhập API key cho phương thức `api-key` của Codex:
   - Key luôn lưu trong Keychain, kể cả trên kênh phát hành `dev`. Không dùng `zed_credentials_provider::global(cx)`: trên kênh `dev` hàm này trả `DevelopmentCredentialsProvider`, lưu credential vào file JSON thường `development_credentials` trong thư mục config, trừ khi có `ZED_DEVELOPMENT_USE_KEYCHAIN` (`crates/zed_credentials_provider/src/zed_credentials_provider.rs:46-62, 116`), và `crates/zed/RELEASE_CHANNEL` của fork là `dev` **[Đã kiểm]**. Gọi API credentials của GPUI (`cx.write_credentials`), như `KeychainCredentialsProvider` làm (`:69`). Không ghi được vào Keychain thì từ chối lưu và báo Human.
   - Gửi trong `_meta["api-key"]` của request `authenticate`; adapter đọc key từ đây **[Baseline]**.
   - Hiện Zed gửi `authenticate` chỉ với method id, không có `_meta` **[Baseline]**; T0.2 ghi vị trí trên fork.
   - Key không bao giờ xuất hiện trong log.
3. Antigravity không đọc `NO_BROWSER`, và khi không mở được trình duyệt thì URL đăng nhập chỉ ra stderr **[Baseline]**. Khi đang authenticate, bắt URL đăng nhập trong stderr của adapter và hiện lên UI.

**Tiêu chí nghiệm thu**
- Cho env có `OPENAI_API_KEY`, khi Zed dựng env cho `codex-acp`, thì env có `OPENAI_API_KEY`.
- Cho key đã nhập, khi authenticate bằng `api-key`, thì agent giả nhận `_meta["api-key"]` có key; log không chứa key.
- Cho kênh phát hành `dev` và không có `ZED_DEVELOPMENT_USE_KEYCHAIN`, khi lưu key, thì key nằm trong credential store của nền tảng (Keychain) và file `development_credentials` không chứa key.
- Cho stderr của adapter có URL đăng nhập, khi đang authenticate, thì UI hiện URL đó.

**Kiểm chứng:** `cargo test -p agent_servers -p agent_ui`; chạy app và chụp ô nhập key.

---

### T1.14 · Quy trình đồng bộ upstream

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Đồng bộ với upstream Zed theo nhịp mà không phá Crew |
| Căn cứ | NEED 59; spec 17 (Đồng bộ upstream) |
| Được sửa | `docs/crew/upstream-sync.md` (mới) |
| Bàn giao | Quy trình và một lần chạy thử trên nhánh |

**Quy trình**
1. Lấy upstream về nhánh `sync/<ngày>`.
2. Gộp và ghi lại xung đột ở H1–H12 và ở mục Crew cuối `.rules`.
3. Chạy lại checklist của T0.2 và cập nhật `docs/crew/hook-points.md`. Kiểm riêng upstream đã chuyển sang `AcpV2Connection` chưa: lớp này có trong `crates/agent_servers/src/acp/v2.rs:119` nhưng chưa được dùng, `acp::connect` vẫn trả bản v1 (`v1.rs:320-340`) **[Đã kiểm]**. Nếu đã chuyển thì H2, H3 và `Rc<AcpConnection>` trong `CrewConnection` hỏng; dừng và báo tech lead trước khi gộp.
4. Chạy toàn bộ test của crate Crew và các crate có điểm nối, cộng test bất biến và test chuyển tiếp method của CrewConnection (T1.7) và test update bị bỏ (T1.10).
5. Merge vào `main` theo quy trình chung. Đồng bộ làm đổi crate `agent-client-protocol` thì chạy cả contract test.

**Tiêu chí nghiệm thu:** cho một lần chạy thử, khi xong, thì có báo cáo gồm số xung đột, điểm nối bị ảnh hưởng và kết quả test.

**Kiểm chứng:** agent kiểm chứng độc lập làm theo `docs/crew/upstream-sync.md` trên một nhánh mới, và đạt cùng kết quả mà không phải hỏi thêm.

---

## Điều kiện xong M1

- [ ] T1.1–T1.14 đạt theo Định nghĩa Xong chung.
- [ ] Bật flag, khởi động Zed: `crew_health` có trạng thái cho cả ba provider; `crew_quota` có số liệu cho Codex và Antigravity.
- [ ] Test bất biến CrewConnection phủ đủ các đường load ở T1.7.
- [ ] Báo cáo S12 đã merge.
