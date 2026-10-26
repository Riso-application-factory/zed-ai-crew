# M0 · Chuẩn bị và spike chặn

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Repo fork sẵn sàng cho agent làm việc: tài liệu nằm trong repo; điểm nối đã kiểm trên fork; 6 crate rỗng biên dịch được sau feature flag; check tĩnh NEED 61; harness contract test. Trả lời S6 và S7, hai spike chặn M1–M2 |
| Spec | Mục 18 bước 0; 3.1, 3.2, 17, 20.1 |
| Cần có trước | — |
| Spike | S6, S7 |
| Thứ tự | T0.1 → T0.2, T0.3 → T0.4 → T0.5, T0.6. T0.7, T0.8, T0.9 chạy song song sau T0.4 |

## Danh sách task

| Task | Tên | Phụ thuộc | NEED |
| ---- | --- | --------- | ---- |
| T0.1 | Đưa tài liệu nguồn vào repo | — | 03, 58 |
| T0.2 | Kiểm lại điểm nối trên fork | T0.1 | 59 |
| T0.3 | Quy tắc cho agent trong repo Crew | T0.1 | 03 |
| T0.4 | Khung 6 crate | T0.3 | 59 |
| T0.5 | `CrewFeatureFlag` và khởi tạo (H8, H9) | T0.4 | 03 |
| T0.6 | Check tĩnh NEED 61 | T0.4 | 61 |
| T0.7 | Spike S6: ext notification và ext method | T0.4 | 36, 56 |
| T0.8 | Spike S7: cầu nối MCP với ba provider | T0.4 | 24, 32 |
| T0.9 | Harness contract test | T0.4 | 28, 55 |

---

### T0.1 · Đưa tài liệu nguồn vào repo

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Agent nào cũng đọc được brief, spec, kế hoạch và kết quả spike ngay trong repo |
| Căn cứ | NEED 03, 58; spec, dòng “Nguồn” |
| Được sửa | `docs/crew/**` (mới) |
| Không được sửa | Mọi file khác |
| Bàn giao | `docs/crew/brief.md`, `docs/crew/tech-spec.md`, `docs/crew/design-check.md`, `docs/crew/plan/*.md`, `docs/crew/spikes/S1-S5.md` |

**Các bước**
1. Chép brief v3.5 thành `docs/crew/brief.md`, tech spec v2.3 thành `docs/crew/tech-spec.md` và báo cáo design check thành `docs/crew/design-check.md`. Phiên bản ghi trong nội dung, không trong tên file, để đường dẫn ổn định.
2. Chép các file kế hoạch này vào `docs/crew/plan/`.
3. Viết `docs/crew/spikes/S1-S5.md` từ báo cáo spike Q1–Q5 của Human, cộng hai kết quả bổ sung:
   - Q1b: ROLE của Antigravity còn sau compaction.
   - Q3b: chặn đọc đường dẫn bí mật và `$TMPDIR` của Claude.

   Nội dung hai kết quả này lấy từ spec Phụ lục B.3 và brief NEED 07, 31, 55. Không chép email hay dữ liệu tài khoản.
4. Kiểm repo là private.

**Tiêu chí nghiệm thu**
- Cho repo fork, khi mở `docs/crew/`, thì có đủ các tài liệu ở mục Bàn giao.
- Cho mọi file trong `docs/crew/`, khi kiểm liên kết `#anchor` và liên kết tương đối, thì không có liên kết hỏng.
- Cho repo trên GitHub, khi kiểm visibility, thì kết quả là `PRIVATE` (NEED 58).

**Kiểm chứng**
```sh
gh repo view --json visibility -q .visibility      # PRIVATE
```
Kiểm liên kết bằng một script ngắn: liệt kê `](#…)` và `<a id="…">` của từng file, báo anchor thiếu. Script nằm trong body PR, không commit.

---

### T0.2 · Kiểm lại điểm nối trên fork

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Xác nhận H1–H12 và các API “có sẵn” ở spec 3.2 còn đúng trên `main` hiện tại của fork, và ghi vị trí theo fork |
| Căn cứ | NEED 59; spec 3.2, Phụ lục B.1; báo cáo [`docs/crew/design-check.md`](../design-check.md) (bảng số dòng spec → fork và các khẳng định đã kiểm) |
| Được sửa | `docs/crew/hook-points.md` (mới). Không sửa code |
| Bàn giao | Bảng: ID, symbol, đường dẫn và dòng trên fork, dòng ghi trong spec, trạng thái (`khớp`, `dời dòng`, `đổi API`) |

**Các bước**
1. Bắt đầu từ bảng số dòng spec → fork và bảng khẳng định đã kiểm của báo cáo design check. Báo cáo đọc fork ở commit `11caaae`, nên mọi dòng vẫn phải mở lại trên `main` hiện tại.
2. Với mỗi điểm nối H1–H12 và mỗi dữ kiện ở Phụ lục B.1, tìm theo **tên symbol**, không theo số dòng: `new_session`, `mcp_servers_for_project`, `handle_session_update`, `handle_write_text_file`, `Agent::server`, `bounded_npm_package_spec`, `remove_stale_versioned_archive_cache_dirs`, `GitStore::create_worktree`, `McpServer::new`…
3. Ghi version và feature của crate `agent-client-protocol` trong `Cargo.toml` của fork. Spec ghi 3.4.0 với feature `unstable` **[Spike]**.
4. Kiểm `AGENTS.md`, `CLAUDE.md`, `GEMINI.md` ở gốc repo vẫn là symlink tới `.rules`. Trên upstream, cả ba là symlink **[Đã kiểm]**. T0.3 dựa vào điều này.
5. Dòng nào trạng thái `đổi API` thì báo tech lead cập nhật spec trước khi task M1–M2 chạm điểm đó bắt đầu.

**Tiêu chí nghiệm thu**
- Cho bảng hook-points, khi đối chiếu với spec 3.2, thì mỗi H1–H12 và mỗi API “có sẵn” có đúng một dòng.
- Cho một dòng bất kỳ, khi mở đường dẫn và dòng ghi trong bảng, thì thấy đúng symbol.

**Kiểm chứng:** agent kiểm chứng độc lập chọn ngẫu nhiên 5 dòng và mở lại.

---

### T0.3 · Quy tắc cho agent trong repo Crew

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Mọi agent làm việc trong fork tự đọc được quy tắc của Crew mà không cần ai dặn |
| Căn cứ | NEED 03; brief 11.1 |
| Được sửa | `docs/crew/AGENTS.md` (mới); cuối file `.rules` (thêm tối đa 10 dòng) |
| Bàn giao | `docs/crew/AGENTS.md`; một mục ngắn ở cuối `.rules` trỏ tới file đó |

`AGENTS.md`, `CLAUDE.md` và `GEMINI.md` ở gốc repo đều là symlink tới `.rules` **[Đã kiểm]**, nên một mục thêm vào `.rules` tới được cả ba provider. Mục thêm vào ở **cuối file** và ngắn, để khi đồng bộ upstream chỉ có thể xung đột ở đúng mục đó. Thay đổi này được ghi trong `docs/crew/hook-points.md` như một chỗ fork khác upstream, dù không phải code core.

**Nội dung `docs/crew/AGENTS.md`**
1. Tài liệu nguồn và cách đọc một task.
2. Lệnh build, test, clippy, fmt, chạy app, contract test (mục 7 của tổng quan), kèm cảnh báo contract test tốn hạn mức.
3. Ranh giới:
   - Core chỉ sửa ở các điểm nối trong `hook-points.md`.
   - Không đọc, ghi hay gọi gì của provider ngoài ACP (NEED 61).
   - Không code thử trong `main`.
   - Không đưa secret vào code, log, fixture hay PR.
   - Mọi hành vi Crew nằm sau `CrewFeatureFlag`.
4. Chiều phụ thuộc giữa các crate (tổng quan mục 3).
5. Quy trình task và Định nghĩa Xong (tổng quan mục 4, 5).
6. Nhãn dữ kiện dùng trong báo cáo.

**Tiêu chí nghiệm thu**
- Cho diff của `.rules`, khi xem, thì chỉ có một mục mới ở cuối file, tối đa 10 dòng, trỏ tới `docs/crew/AGENTS.md`.
- Cho một agent mới chưa có context, khi được hỏi “chạy contract test thế nào” và “được sửa file core nào”, thì trả lời đúng chỉ bằng cách đọc `docs/crew/AGENTS.md`.

**Kiểm chứng:** `git diff main -- .rules` cho thấy chỉ có phần thêm ở cuối file. Agent kiểm chứng độc lập, mở trong context mới, trả lời hai câu hỏi ở tiêu chí thứ hai mà không đọc file nào khác ngoài `docs/crew/AGENTS.md`.

---

### T0.4 · Khung 6 crate

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Có 6 crate rỗng biên dịch được, đúng chiều phụ thuộc |
| Căn cứ | NEED 59; spec 3.1 |
| Được sửa | `crates/crew`, `crates/crew_acp`, `crates/crew_mcp`, `crates/crew_github`, `crates/crew_models`, `crates/crew_ui` (mới); `Cargo.toml` gốc (`[workspace] members` và `[workspace.dependencies]`) |
| Bàn giao | 6 crate với `Cargo.toml` và file gốc của thư viện |

**Các bước**
1. Theo mẫu `crates/feature_flags/Cargo.toml` **[Đã kiểm]**: `edition.workspace = true`, `publish.workspace = true`, `[lints] workspace = true`, `[lib] path = "src/<tên_crate>.rs"`, `doctest = false`. Giữ trường `license` và symlink `LICENSE-GPL` như các crate khác của repo. `.rules` yêu cầu dùng `[lib] path` có tên mô tả thay cho `lib.rs`, và không dùng `mod.rs`.
2. Thêm 6 đường dẫn vào `[workspace] members` và 6 dòng `<tên> = { path = "crates/<tên>" }` vào `[workspace.dependencies]`.
3. Chỉ khai phụ thuộc theo sơ đồ ở tổng quan mục 3. Crate rỗng chưa cần phụ thuộc nào thì chưa khai.

**Tiêu chí nghiệm thu**
- Cho workspace, khi `cargo check` 6 crate, thì đạt.
- Cho cây phụ thuộc, khi kiểm, thì `agent_ui`, `agent_servers`, `acp_thread` không phụ thuộc crate Crew nào, và `crew_acp` không phụ thuộc `crew`.

**Kiểm chứng**
```sh
cargo check -p crew -p crew_acp -p crew_mcp -p crew_github -p crew_models -p crew_ui
cargo tree -p agent_ui -e normal | grep -c " crew" || true     # kỳ vọng 0
cargo tree -p crew_acp -e normal | grep -E "^.* crew v" || true   # không có crate `crew`
```

---

### T0.5 · `CrewFeatureFlag` và khởi tạo (H8, H9)

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Mọi hành vi Crew nằm sau một feature flag. Tắt flag thì fork chạy như Zed upstream |
| Căn cứ | NEED 03; brief 9; spec 3.2 (H8, H9) |
| Được sửa | `crates/feature_flags/src/flags.rs` (H8), `crates/zed/src/main.rs` (H9), `crates/crew_ui` |
| Bàn giao | `CrewFeatureFlag`; `crew_ui::init(cx)` được gọi lúc khởi động |

**Thiết kế**
- Khai theo mẫu các flag khác trong `flags.rs` **[Đã kiểm]**: struct `CrewFeatureFlag`, `const NAME = "crew"`, `type Value = PresenceFlag`, `register_feature_flag!(CrewFeatureFlag)`.
- **`enabled_for_staff()` trả `false`.** Bản debug coi người dùng như staff (`store.rs:180`) **[Đã kiểm]**, nên nếu để mặc định thì Crew tự bật trong mọi bản debug. Đặt `false` thì Crew tắt mặc định, và bật bằng `"feature_flags": { "crew": "on" }` trong bản debug; khóa override là `on`/`off` **[Đã kiểm]**.
- **Bản release của Human** không nhận override từ settings vì Human không phải staff (`store.rs:105-107`) **[Đã kiểm]**. Vì vậy bản fork hằng ngày luôn tắt Crew, đúng brief mục 9. T7.8 đổi `enabled_for_all()` để bật cho manual test.
- **H9:** gọi `crew_ui::init(cx)` trong `crates/zed/src/main.rs`, ngay sau `agent_ui::init` (`:713`), không trong `zed.rs` **[Đã kiểm]**. Workspace và `AgentPanel::load` (`crates/zed/src/zed.rs:864`) chạy sau đó, nên agent id của Crew (T1.5) và factory (T1.7) được đăng ký trước khi panel khôi phục thread.
- `crew_ui::init(cx)` đăng ký action và observer. Mọi đường vào kiểm `cx.has_flag::<CrewFeatureFlag>()` và theo dõi khi flag đổi.

**Tiêu chí nghiệm thu**
- Cho flag tắt, khi mở Command Palette, thì không có action “Crew: …”; Crew không khởi chạy adapter nào và không ghi gì vào database.
- Cho bản debug có override `"crew": "on"`, khi mở Command Palette, thì thấy “Crew: New Session”; action này báo “chưa hỗ trợ” cho tới T2.3.
- Cho flag đổi từ bật sang tắt khi app đang chạy, khi mở Command Palette, thì action biến mất.

**Kiểm chứng**
```sh
cargo test -p crew_ui -p feature_flags
cargo run      # chụp Command Palette khi tắt và khi bật flag
```

---

### T0.6 · Check tĩnh NEED 61

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Một test chặn mọi đoạn mã Crew điều khiển provider ngoài ACP |
| Căn cứ | NEED 61; spec 17 (dòng NEED 61) |
| Được sửa | `crates/crew/src/need61_check.rs` (chỉ test) |
| Bàn giao | Test quét mã nguồn các crate `crew*` |

**Thiết kế**

Test đọc mọi file `.rs` trong `crates/crew*/src` và báo lỗi khi gặp:
- đường dẫn home của provider: `.claude`, `.codex`, `.gemini`, `antigravity-acp/conversations`;
- host API riêng của provider: `api.anthropic.com`, `api.openai.com`, `chatgpt.com`, `generativelanguage.googleapis.com`;
- lệnh chạy thẳng CLI của provider: `Command::new("claude")`, `Command::new("codex")`, `app-server`.

Danh sách cho phép (allowlist) nằm ngay trong test, mỗi mục có lý do. Ví dụ: agent id `claude-acp` và `codex-acp` là định danh registry, không phải đường dẫn.

**Tiêu chí nghiệm thu**
- Cho mã nguồn hiện tại, khi chạy test, thì đạt.
- Cho một chuỗi mẫu chứa `"~/.codex/config.toml"` đưa vào hàm quét, khi chạy test âm, thì hàm quét báo lỗi.

**Kiểm chứng:** `cargo test -p crew need61`

---

### T0.7 · Spike S6: ext notification và ext method trên crate ACP 3.4.0

| Mục | Nội dung |
| --- | -------- |
| Câu hỏi | Dùng API nào của crate `agent-client-protocol` 3.4.0 để (a) nhận ext notification `_auth/status_update` và (b) gọi ext method `_session/steering`? |
| Căn cứ | Spec 20.1 (S6), 3.2 (H3) |
| Chặn | T1.8 (handler đăng nhập), T5.6 (steering) |
| Nhánh | `spike/S6-acp-ext`, không merge |
| Bàn giao | `docs/crew/spikes/S6.md`, cập nhật T1.8 và T5.6 nếu cần |

Báo cáo design check đã xác định API **[Đã kiểm]**: `on_receive_notification` của builder (`agent-client-protocol-3.4.0 src/jsonrpc.rs:1579`), `ConnectionTo::send_request` (`:4300`) và `UntypedMessage` (`:5676-5685`). H3 phải sửa `v1.rs`, vì trường `AcpConnection.connection` là private (`v1.rs:75`). Spike chỉ còn phải chạy được test với agent giả.

**Các bước**
1. Đọc các API trên và cách Zed dựng client ở `crates/agent_servers/src/acp/v1.rs` (`client_builder`, `:368-442`) **[Đã kiểm]**.
2. Trên nhánh spike, viết test với agent giả: agent gửi `_auth/status_update` ngay sau initialize; client gửi request `_session/steering` và nhận response.
3. Ghi lại: kiểu payload, cách đăng ký handler bên cạnh các `on_receive_notification` hiện có, phần sửa cần có trong `v1.rs`, giới hạn nếu có.

**Tiêu chí xong:** báo cáo trả lời được (a) và (b), có đoạn code mẫu, và test trên nhánh spike đạt.

---

### T0.8 · Spike S7: cầu nối MCP với ba provider

| Mục | Nội dung |
| --- | -------- |
| Câu hỏi | Cả ba provider có khởi động MCP server stdio `<zed> --crew-mcp-bridge <socket>` và gọi được tool qua cầu nối chỉ chép byte không? |
| Căn cứ | Spec 20.1 (S7), 7.2 |
| Chặn | T2.1 |
| Nhánh | `spike/S7-mcp-bridge`, không merge |
| Bàn giao | `docs/crew/spikes/S7.md` |

Báo cáo design check đã trả lời phần framing **[Đã kiểm]**: `McpServer` dùng JSON-RPC theo dòng (`crates/context_server/src/listener.rs:360-394`), nên cầu nối chỉ chép byte. Server không tự trả lời `initialize`: nó chỉ xử lý `tools/call`, `tools/list` và handler đã đăng ký (`:214-242`). Spike còn phải chạy ba provider qua cầu nối.

**Các bước**
1. Trên nhánh spike, thêm tạm flag `--crew-mcp-bridge` vào `main.rs` theo mẫu `--askpass` (`crates/zed/src/main.rs:215-219`) **[Đã kiểm]**: sao chép stdin → socket và socket → stdout.
2. Tạo `McpServer::new` với một tool `crew_ping` và một handler `initialize` đăng ký bằng `McpServer::handle_request` (`listener.rs:143`). Server tạo socket `mcp.sock` trong thư mục tạm `zed-mcp*` (`listener.rs:55-60`) **[Đã kiểm]**.
3. Với từng provider ở bản ghim, tạo session với `mcpServers: [{ name: "crew", command: <đường dẫn app>, args: ["--crew-mcp-bridge", <socket>] }]`, rồi gửi prompt yêu cầu gọi `crew_ping`.
   - `command` phải là đường dẫn tuyệt đối của chính app Zed đang chạy (`std::env::current_exe()`), như askpass làm (`crates/askpass/src/askpass.rs:278-281`) **[Đã kiểm]**. `zed` trên PATH là CLI, không phải app **[Nhớ]**.
4. Ghi lại cho từng provider: có gọi được tool không; có yêu cầu xin quyền cho tool MCP không và option là gì (Codex có bộ option riêng cho MCP **[Đã kiểm]**); Antigravity có nhận MCP stdio không.

**Tiêu chí xong:** mỗi provider hoặc gọi được `crew_ping`, hoặc báo cáo nêu rõ lỗi và nguyên nhân. Chi phí khoảng 3 prompt ngắn.

---

### T0.9 · Harness contract test

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Có khung contract test chạy với adapter thật, để các task sau thêm test cho từng hành vi đã quan sát ở spike |
| Căn cứ | NEED 28, 55; spec 17 (Contract test adapter) |
| Được sửa | `crates/crew_acp/tests/contract.rs` và module hỗ trợ; `crates/crew_acp/test_fixtures/contract/` |
| Bàn giao | Harness và test đầu tiên (initialize) |

Harness của spike ở `/tmp/crew-spike` không được giữ lại **[Spike]**, nên khung này viết mới bằng Rust.

**Thiết kế**
- Test gắn `#[ignore]` và chỉ chạy khi có `CREW_CONTRACT=1`.
- Lệnh adapter lấy từ biến môi trường `CREW_CONTRACT_CLAUDE_CMD`, `CREW_CONTRACT_CODEX_CMD`, `CREW_CONTRACT_ANTIGRAVITY_CMD`, cho tới khi T1.5 cung cấp đường dẫn bản ghim.
- Client ACP tối thiểu qua stdio, dùng thẳng crate `agent-client-protocol`, cùng kiểu builder như `v1.rs`, để test không cần GPUI. Client có:
  - initialize với đúng capability Zed khai trong `client_capabilities_for_agent` (`v1.rs:445-475`): fs đọc và ghi, terminal, elicitation dạng form và URL **[Đã kiểm]**. Adapter đổi hành vi theo capability, ví dụ Claude chỉ giữ `AskUserQuestion` khi client có form elicitation (`acp-agent.js:8586`) **[Đã kiểm]**, nên khai khác Zed thì test không phản ánh hành vi dưới Zed;
  - ghi lại capability của agent;
  - trả lời yêu cầu xin quyền và elicitation theo chính sách của từng test;
  - hiện thực `fs/read_text_file` và `fs/write_text_file` chỉ trong thư mục của test, vì Antigravity ghi file qua đó **[Baseline]**;
  - ghi traffic thô vào thư mục tạm;
  - che dữ liệu (email, token) trước khi ghi fixture vào `test_fixtures/contract/`.
- **Kiểm chung cho mọi contract test với Claude và Codex:** không có request `fs/*` hay `terminal/*` nào từ agent. Zed ghi file qua `fs/write_text_file` mà không xin quyền (`v1.rs:9416-9449`) và chạy `terminal/create` không sandbox (`v1.rs:9650-9709`); hai adapter này ở bản ghim không dùng hai nhóm đó **[Đã kiểm]**, nên một bản cập nhật bắt đầu dùng thì test phải fail.
- Không test nào in hay lưu nội dung file bí mật thật. Test về đường dẫn bí mật dùng file giả trong thư mục tạm.

**Tiêu chí nghiệm thu**
- Cho máy Human đã đăng nhập ba provider, khi chạy lệnh contract test, thì test initialize đạt cho cả ba adapter và ghi lại capability.
- Cho adapter Claude, khi initialize, thì capability có `sessionCapabilities.systemPrompt` **[Spike]**.
- Cho request initialize của harness, khi đọc, thì `clientCapabilities` có fs, terminal và elicitation (form, URL) như `client_capabilities_for_agent` của Zed.
- Cho một traffic mẫu của Claude hoặc Codex có request `fs/write_text_file` hay `terminal/create`, khi harness chạy bước kiểm chung, thì test fail (kiểm chính harness, không gọi provider).
- Cho môi trường không có `CREW_CONTRACT`, khi chạy `cargo test -p crew_acp`, thì contract test bị bỏ qua, không gọi provider.

**Kiểm chứng**
```sh
CREW_CONTRACT=1 cargo test -p crew_acp --test contract -- --ignored --test-threads=1
cargo test -p crew_acp
```

---

## Điều kiện xong M0

- [ ] T0.1–T0.9 đạt theo Định nghĩa Xong chung.
- [ ] `docs/crew/hook-points.md` không còn dòng `đổi API` chưa xử lý.
- [ ] Báo cáo S6 và S7 đã merge, và các task bị ảnh hưởng đã cập nhật.


## Execution record — 2026-10-25

Implementation branch: `m0/prep-and-spikes`, based on `11caaae9f7`. This records evidence, not acceptance of unmet completion gates.

| Task | Evidence |
| --- | --- |
| T0.1 | Brief v3.5, spec v2.2, plans and [S1–S5 report](../spikes/S1-S5.md) exist. Explicit fork visibility check returned PUBLIC; NEED 58 remains blocked pending Human's decision. |
| T0.2 | [Hook map](../hook-points.md) covers H1–H11, existing APIs and B.1. Independent reviewer reopened five rows. H4 fork drift is reflected in spec and T1.10/T1.11; ACP remains deliberately pinned to 3.4.0. |
| T0.3 | [Agent instructions](../AGENTS.md) plus one four-line tail section in `.rules`. Independent reviewer answered contract command and core scope using only AGENTS. |
| T0.4 | Six crate skeletons passed `cargo check`; normal dependency trees contain no Crew dependency from agent_ui, agent_servers or acp_thread, nor crew from crew_acp. |
| T0.5 | Default-off flag, synchronous post-palette startup integration and guarded NewSession action implemented. GPUI test passed for staff default off and settings on→off transitions. Real-app evidence captured on the dev build with an isolated data dir: flag off hides every `crew` action in the Command Palette; `"crew": "on"` surfaces "crew: new session", which prompts "Crew sessions are not supported yet"; switching the setting to `"off"` while running hides the action again. |
| T0.6 | NEED 61 scanner passed three tests, including forbidden provider access and allowed ACP registry identifiers. Negative test was observed failing before implementation. |
| T0.7 | [S6](../spikes/S6.md): fake-agent notification/steering roundtrip passed on separate spike branch; T1.8/T5.6 updated. |
| T0.8 | [S7](../spikes/S7.md): actual app/McpServer framing and all three pinned providers passed crew_ping; exactly three prompts. T2.1 updated. |
| T0.9 | Default harness run passed one fixture test and ignored three live tests. Explicit live initialize passed all three adapters, including Claude systemPrompt capability; sanitized fixtures recorded. Zero prompts for initialize. |

The changed-crate test command passed 13 tests, with three real-adapter tests ignored:

```sh
cargo test -p crew_ui -p feature_flags -p crew -p crew_acp -p crew_mcp -p crew_github -p crew_models
```

`./script/clippy` (workspace release clippy, cargo-shear, typos), full-workspace formatting (`cargo fmt --all -- --check`), and real-app UI evidence all pass. `m0/prep-and-spikes` is pushed and open as PR #3; review/merge and independent verification remain pending. Human decided to proceed while the fork stays PUBLIC, so NEED 58 remains formally unmet and M0 stays open. Spike source remains isolated on `spike/S6-acp-ext` (`2fdbd33f34`) and `spike/S7-mcp-bridge` (`ec6efc014d`); only reports belong in the product branch.
