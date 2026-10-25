# Zed AI Crew — Danh sách Spike (S1–S13)

| Mục | Nội dung |
| --- | -------- |
| Phiên bản | 1.1 · 2026-10-25 |
| Nguồn | Tổng hợp từ [Product Brief v3.5](brief.md) ([Phụ lục C](brief.md#ranh-gioi-nen-tang-zed-co-san-va-phan-crew-can-xay-them)), [Tech Spec v2.3](tech-spec.md) ([mục 20.1](tech-spec.md#muc-20), [Phụ lục B.3](tech-spec.md#phu-luc-b)) và [Kế hoạch triển khai v1.1](plan/00-overview.md) ([M0](plan/M0-prep-and-spikes.md)–[M7](plan/M7-model-and-retro.md)). |
| Quy tắc Spike | Code spike nằm trên nhánh `spike/<id>-<tên>` và **không bao giờ merge vào `main`** ([NEED 03](brief.md#need-03)). Kết quả là báo cáo `docs/crew/spikes/<id>.md` merge qua PR tài liệu, kèm cập nhật các task/spec bị ảnh hưởng. Không bao giờ đọc hay in nội dung file bí mật thật; luôn che email/token trước khi lưu fixture. |
| Nhãn | **[Đã kiểm]** đọc trong mã nguồn hoặc tài liệu gốc · **[Spike]** kết quả spike 2026-10-25 · **[Baseline]** theo baseline provider của Human · **[Nhớ]** kiến thức đáng tin, chưa kiểm lại · **[Suy luận]** cần kiểm |

## Mục lục

| Phần | Mục |
| ---- | --- |
| Tổng quan | [1. Bảng tổng hợp toàn bộ Spike (S1–S13)](#muc-1) · [2. Quy trình và nguyên tắc chạy Spike](#muc-2) |
| Chi tiết | [3. Phần 1: Các Spike đã hoàn tất (S1–S5, Q1b, Q3b)](#phan-1) · [4. Phần 2: Các Spike còn mở trong kế hoạch (S6–S13)](#phan-2) |

---

<a id="muc-1"></a>

## 1. Bảng tổng hợp toàn bộ Spike (S1–S13)

| Mã | Tên Spike | Trạng thái | Milestone / Task | Chặn bước / Task | Nhánh / Bàn giao |
| -- | --------- | ---------- | ---------------- | ---------------- | ---------------- |
| [S1](#s1) | Kênh nạp ROLE và tính bền vững qua compaction, load, resume (gồm Q1 & Q1b) | **Đã xong** (2026-10-25) | Trước M0 ([T0.1](plan/M0-prep-and-spikes.md)) | Đã tích hợp vào Spec 6.1, 6.2, 14 | `docs/crew/spikes/S1-S5.md` |
| [S2](#s2) | Trạng thái đăng nhập và hành vi lỗi `-32000` của 3 provider (Q2) | **Đã xong** (2026-10-25) | Trước M0 ([T0.1](plan/M0-prep-and-spikes.md)) | Đã tích hợp vào Spec 6.5 | `docs/crew/spikes/S1-S5.md` |
| [S3](#s3) | Ma trận sandbox của Claude, chặn đọc đường dẫn bí mật và `$TMPDIR` (gồm Q3 & Q3b) | **Đã xong** (2026-10-25) | Trước M0 ([T0.1](plan/M0-prep-and-spikes.md)) | Đã tích hợp vào Spec 6.2, 10.4 | `docs/crew/spikes/S1-S5.md` |
| [S4](#s4) | Hành vi Antigravity Ultra: danh sách model, chi phí cố định, song song, compaction (Q4) | **Đã xong** (2026-10-25) | Trước M0 ([T0.1](plan/M0-prep-and-spikes.md)) | Đã tích hợp vào Spec 6.1, 6.8, 13 | `docs/crew/spikes/S1-S5.md` |
| [S5](#s5) | Định dạng hạn mức `/status` của Codex và `/usage` của Antigravity (Q5) | **Đã xong** (2026-10-25) | Trước M0 ([T0.1](plan/M0-prep-and-spikes.md)) | Đã tích hợp vào Spec 6.7 | `docs/crew/spikes/S1-S5.md` |
| [S6](#s6) | Ext notification `_auth/status_update` và ext method `_session/steering` chạy qua H3 (API của crate 3.4.0 đã xác định) | **Còn mở** | M0 · [T0.7](plan/M0-prep-and-spikes.md) | Chặn Bước 0 → [T1.8](plan/M1-foundation.md), [T5.6](plan/M5-playbook.md) | `spike/S6-acp-ext` → `docs/crew/spikes/S6.md` |
| [S7](#s7) | Cầu nối MCP stdio với 3 provider (framing đã rõ; `McpServer` cần handler `initialize`) | **Còn mở** | M0 · [T0.8](plan/M0-prep-and-spikes.md) | Chặn Bước 0 → [T2.1](plan/M2-orchestrator.md) | `spike/S7-mcp-bridge` → `docs/crew/spikes/S7.md` |
| [S8](#s8) | Độ phủ gate và vùng ghi của từng provider ở mode worker | **Còn mở** | M3 · [T3.1](plan/M3-execution.md) | Chặn Bước 3 → [T3.4](plan/M3-execution.md), [T3.5](plan/M3-execution.md), [T3.6](plan/M3-execution.md) | `spike/S8-gate-coverage` → `docs/crew/spikes/S8.md` |
| [S9](#s9) | Hiệu lực thực tế của `maxBudgetUsd`, `maxTurns`, `permissions.deny` (Claude) và `disabledTools` (Antigravity) | **Còn mở** | M3 · [T3.2](plan/M3-execution.md) | Chặn Bước 3 → [T3.6](plan/M3-execution.md), [T3.13](plan/M3-execution.md) | `spike/S9-limits-deny` → `docs/crew/spikes/S9.md` |
| [S10](#s10) | Chi phí bộ nhớ và CPU của N Project không cửa sổ cùng N process agent (N = 1, 2, 4) | **Còn mở** | M3 · [T3.3](plan/M3-execution.md) | Chặn Bước 3 → [T3.13](plan/M3-execution.md), đầu vào cho [Q7](open-question.md#q7) | `spike/S10-resources` → `docs/crew/spikes/S10.md` |
| [S11](#s11) | ROLE của Codex qua `customInstructions` khi load/resume trong process mới | **Còn mở** | M2 · [T2.10](plan/M2-orchestrator.md) | Chặn Bước 2 → Phục hồi Orchestrator ([T2.10](plan/M2-orchestrator.md)) | `spike/S11-codex-role-resume` → `docs/crew/spikes/S11.md` |
| [S12](#s12) | Định dạng output thật của `/usage` và cấu trúc `_claude/rateLimit` trên Claude | **Còn mở** | M1 · [T1.11](plan/M1-foundation.md) | Chặn Bước 1 → Phần Claude của [T1.12](plan/M1-foundation.md) | `spike/S12-claude-quota` → `docs/crew/spikes/S12.md` |
| [S13](#s13) | Xác định nhóm hạn mức của `gemini-4-argon` trên Antigravity | **Còn mở** | M7 · [T7.1](plan/M7-model-and-retro.md) | Không chặn (trong lúc chờ, Argon có hạn mức `unknown`) | `spike/S13-argon-quota-group` → `docs/crew/spikes/S13.md` |

---

<a id="muc-2"></a>

## 2. Quy trình và nguyên tắc chạy Spike

Theo [Kế hoạch triển khai — mục 6](plan/00-overview.md):

1. **Nhánh tách biệt:** Mọi code thử nghiệm đặt trên nhánh `spike/<id>-<tên>` và **không bao giờ merge vào `main`** ([NEED 03](brief.md#need-03)).
2. **Bàn giao qua báo cáo tài liệu:** Kết quả của spike là một file báo cáo `docs/crew/spikes/<id>.md` được merge vào `main` qua PR tài liệu, gồm:
   - Câu hỏi cần trả lời;
   - Cách đo và cấu hình môi trường;
   - Kết quả thực tế có gắn nhãn (`[Spike]`, `[Đã kiểm]`);
   - Hệ quả đối với Tech Spec và các task trong `docs/crew/plan/`.
3. **Đồng bộ kế hoạch:** Cùng PR báo cáo spike phải cập nhật ngay các task bị ảnh hưởng trong `docs/crew/plan/` và `docs/crew/tech-spec.md`. Nếu kết quả làm thay đổi một quyết định sản phẩm mà Human đã duyệt thì dừng và hỏi Human trước.
4. **An toàn dữ liệu và hạn mức:**
   - Spike gọi tài khoản thật sẽ tốn hạn mức; báo cáo phải ghi rõ số prompt và lượng token đã dùng.
   - Không bao giờ đọc hay in nội dung file bí mật thật (`~/.ssh`, Keychain…); khi kiểm chặn đọc phải dùng file bí mật giả đặt trong thư mục tạm.
   - Che toàn bộ email, account ID và token trước khi đưa vào báo cáo hoặc `test_fixtures/contract/`.

---

<a id="phan-1"></a>

## 3. Phần 1: Các Spike đã hoàn tất (S1–S5, Q1b, Q3b — ngày 2026-10-25)

Thực hiện ngày 2026-10-25 trên máy macOS của Human (fork `20ea81ef29`; ba adapter `claude-agent-acp@0.95.0`, `codex-acp@2.2.2`, `antigravity-acp` 1.4.3; crate `agent-client-protocol` 3.4.0). Task [T0.1](plan/M0-prep-and-spikes.md) đóng gói toàn bộ kết quả này thành `docs/crew/spikes/S1-S5.md`.

<a id="s1"></a>

### S1 (Q1 & Q1b) · Kênh nạp ROLE và tính bền vững qua compaction, load, resume

| Hạng mục | Kết quả đã kiểm chứng **[Spike]** | Hệ quả thiết kế |
| -------- | --------------------------------- | --------------- |
| Crate ACP 3.4.0 | Crate `agent-client-protocol` 3.4.0 (schema 1.12.0) đã có trường `system_prompt` trên `new`, `load`, `fork`, `resume` với `content` là mảng content block. | Không cần nâng kết nối ACP của Zed lên v2 ([Tech Spec 1.2, 6.2](tech-spec.md#muc-6-2)). |
| Claude (`claude-acp`) | Nhận trường ACP `systemPrompt` (`mode: "append"`, `content: [{type: "text", text: "<ROLE>"}]`), ưu tiên hơn `_meta.systemPrompt`. Có hiệu lực qua 3 lượt, **còn sau `/compact`**. Khi load/resume, option mới thay option cũ; thiếu `systemPrompt` thì rơi về preset trần. Dự phòng `_meta.systemPrompt` phải là object `{ "append": "<ROLE>" }` (dạng chuỗi sẽ ghi đè toàn bộ prompt gốc). | `CrewConnection` chặn ở `Agent::server()` để mọi đường load/resume gửi lại nguyên vẹn `SessionOptions` ([Tech Spec 3.3, 6.1](tech-spec.md#muc-3-3)). |
| Antigravity (`antigravity-acp`) | Bỏ qua trường ACP `systemPrompt`; nhận `_meta.antigravity.systemPrompt.append` dạng chuỗi trên `new`, `load` và `resume` (kiểu khác bị bỏ qua im lặng). Không lưu ROLE cùng hội thoại; **còn sau compaction** (đã kiểm bằng mã vai trò chỉ có trong ROLE, vẫn nằm trong `<user_system_instructions>`). Đổi model không làm mất ROLE. | Tự kiểm kiểu chuỗi trước khi gửi; luôn gửi lại đủ `SessionOptions` ngay từ lần load/resume đầu tiên ([Tech Spec 6.1, 6.2](tech-spec.md#muc-6)). |
| Codex (`codex-acp`) | Qua đọc mã (`dist/index.js:28093-28099`): đọc `_meta.jetbrains.air.customInstructions` (chỉ nhận chuỗi) ở `newSession`, chuyển thành `developerInstructions` của `threadStart`; `loadSession`/`resumeSession` không đọc trường này **[Đã kiểm]**. | Kiểm chạy thật ở [T1.6](plan/M1-foundation.md) và kiểm hành vi sau resume ở [Spike S11](#s11) ([T2.10](plan/M2-orchestrator.md)). |

<a id="s2"></a>

### S2 (Q2) · Xác nhận trạng thái đăng nhập của ba provider

| Provider | Kết quả đã kiểm chứng **[Spike]** | Hệ quả thiết kế ([NEED 56](brief.md#need-56), [Spec 6.5](tech-spec.md#muc-6-5)) |
| -------- | --------------------------------- | -------------------------------------------------------------------------------- |
| Claude | Adapter dò bằng `claude auth status`, nên thông báo `_auth/status_update` báo sai “Not logged in” khi đăng nhập bằng `CLAUDE_CODE_OAUTH_TOKEN` trong môi trường dù prompt vẫn chạy. Khi thật sự chưa đăng nhập, `session/new` vẫn thành công và prompt đầu tiên mới trả lỗi `-32000`. | Bắt buộc xác nhận bằng healthcheck ping-pong trong session thăm dò dùng lại. |
| Codex | Gửi `_auth/status_update` ngay sau `initialize` và báo đúng (`kind = "account"` hoặc `kind = "none"`). | Bổ sung handler `_auth/status_update` (H3a); dùng healthcheck làm dự phòng khi quá hạn. |
| Antigravity | Báo lỗi `-32000` ngay khi gọi `session/new` nếu chưa đăng nhập. | Kiểm lỗi `-32000` lúc tạo session thăm dò, rồi chạy healthcheck ping-pong. |

<a id="s3"></a>

### S3 (Q3 & Q3b) · Ma trận sandbox của Claude, đường dẫn bí mật và `$TMPDIR`

| Hạng mục | Kết quả đã kiểm chứng **[Spike]** | Hệ quả thiết kế ([NEED 07](brief.md#need-07), [Spec 6.2, 10.4](tech-spec.md#muc-6-2)) |
| -------- | --------------------------------- | ------------------------------------------------------------------------------------- |
| Ghi ngoài worktree | Với `sandbox: { enabled: true }`, lệnh Bash ghi ra ngoài worktree hoặc vào `$HOME` bị chặn (`EPERM`). Lệnh Bash trong sandbox tự chạy không qua yêu cầu xin quyền (`autoAllowBashIfSandboxed`). | Đặt `enabled: true`, `failIfUnavailable: true`, `allowUnsandboxedCommands: false`. |
| Mạng | Mỗi host mới phát sinh yêu cầu xin quyền `SandboxNetworkAccess`; chỉ bị chặn khi từ chối hoặc bật `strictAllowlist: true` cùng `allowedDomains`. Host đã được cho phép giữ hiệu lực tới hết session. | Đặt `network.strictAllowlist: true` với `effective.network_allowlist`; gate kiểm host trong `SandboxNetworkAccess`. |
| Đọc đường dẫn bí mật (Q3b) | Mặc định Bash trong sandbox đọc được `~/.ssh`, token GitHub CLI và `~/Library/Keychains` (qua lệnh `security`). Cấu hình `filesystem.denyRead` hoặc `credentials.files` chặn được lệnh Bash đọc các đường dẫn này (kể cả `security`). Tuy nhiên tool `Read` không bị `denyRead` chặn mà đi qua yêu cầu xin quyền hoặc `settings.permissions.deny`. | Kết hợp cả hai: `filesystem.denyRead` cho Bash + luật `Read(//<đường dẫn>/**)` trong `settings.permissions.deny` cho tool Read + gate từ chối đọc/sửa ngoài worktree. Ghi nhận rủi ro còn lại R12 (chặn theo denylist). |
| `$TMPDIR` dùng chung (Q3b) | `$TMPDIR` trong sandbox Claude luôn bị ép về `/tmp/claude-501` cho mọi worker trên máy, kể cả khi session tự đặt biến `TMPDIR`. `filesystem.denyWrite: ["/tmp/claude-501"]` chặn ghi được; khi đó file tạm rơi vào worktree. | Chặn ghi `$TMPDIR` để cô lập dữ liệu giữa các project ([NEED 02](brief.md#need-02)); Crew chỉ commit file khai báo và xóa file không khai báo khi worker kết thúc ([T3.9](plan/M3-execution.md)). |
| Option `allow_always` | Sửa file ngoài cwd sinh option `allow_always` (“allow all edits in … during this session”). | Gate chỉ chọn theo `kind` là `allow_once` hoặc `reject_once`, cấm tuyệt đối `allow_always` / `reject_always`. |

<a id="s4"></a>

### S4 (Q4) · Hành vi Antigravity tài khoản Google AI Ultra

| Hạng mục | Kết quả đã kiểm chứng **[Spike]** | Hệ quả thiết kế |
| -------- | --------------------------------- | --------------- |
| Danh sách model | `gemini-4-argon`, `gemini-3.8-flash`, `claude-opus-5-5`, `claude-sonnet-5-5`, `gpt-oss-120b-medium`; `thinking_level` gồm `low`, `medium`, `high`. | Lưu ứng viên theo bộ ba `(provider, model, effort)` vì cùng model ID có thể xuất hiện ở nhiều provider ([Tech Spec 6.6](tech-spec.md#muc-6)). |
| Chi phí cố định | Prompt đầu tiên của mỗi session tốn khoảng **12,7 nghìn token** chỉ dẫn có sẵn. | Luôn dùng lại session thăm dò; cộng chi phí cố định khi ước tính chi phí worker Antigravity ([Tech Spec 6.5, 9.2](tech-spec.md#muc-6-5)). |
| Song song | Chạy được 4 prompt song song, adapter không đặt giới hạn. | Đặt trần song song ở tầng Crew (`parallel.per_provider.antigravity-acp`, [Q7](open-question.md#q7)). |
| Compaction | Diễn ra quanh **256 nghìn token**, thấp hơn nhiều so với cửa sổ ~1 triệu token mà adapter báo. ACP không gửi thông báo compaction; dấu hiệu duy nhất là `usage_update.used` tụt mạnh (ví dụ từ ~210k xuống ~23k token). | Phát hiện compaction bằng độ tụt `used` và tính ngân sách context của packet theo ngưỡng thực tế 256k token ([Tech Spec 6.8](tech-spec.md#muc-6), [T3.7](plan/M3-execution.md)). |
| Session lưu trữ | Session được lưu dưới `~/.gemini/antigravity-acp/conversations/`, hiện trong `session/list` và thread import; không có `session/delete` qua ACP. | Tái dùng session thăm dò; lọc khỏi thread import ở H5e; không xóa ngoài ACP ([NEED 61](brief.md#need-61)). |

<a id="s5"></a>

### S5 (Q5) · Định dạng văn bản hạn mức của Codex và Antigravity

| Provider | Kết quả đã kiểm chứng **[Spike]** | Hệ quả thiết kế ([Tech Spec 6.7](tech-spec.md#muc-6), [T1.12](plan/M1-foundation.md)) |
| -------- | --------------------------------- | ------------------------------------------------------------------------------ |
| Codex (`/status`) | Do adapter tự sinh (không tốn lượt gọi model), giống hệt nhau qua 4 phiên bản adapter. Dạng dòng: `**<limit> <label>:** <N>% left[ (resets <time>)]` với `<label>` ∈ `Nm limit`, `Nh limit`, `Nd limit`, `Weekly limit`, `Limit`. Giờ reset theo múi giờ và locale của process, không có năm. Output có dòng `**Account:**` chứa email. | Parse phần trăm theo cửa sổ; giờ reset parse best-effort (lỗi thì ghi `unknown`); **bắt buộc bỏ dòng `**Account:**`** trước khi lưu ([NEED 02](brief.md#need-02)). |
| Antigravity (`/usage`) | Adapter tự trả Markdown cố định: nhóm `- **<group>**`, dòng `*(Models within this group: …)*`, hai cửa sổ `Weekly Limit Remaining` và `Five Hour Limit Remaining` với giờ reset UTC tiếng Anh. Nhóm “Gemini Models” ghi “Gemini Flash, Gemini Pro”, chưa nhắc `gemini-4-argon`. | Các thông báo đặc biệt (“Unable to retrieve…”, “No quota limits reported.”…) chuyển thành `unknown`. Hạn mức của Argon là `unknown` cho tới khi chạy [Spike S13](#s13). |

---

<a id="phan-2"></a>

## 4. Phần 2: Các Spike còn mở trong kế hoạch (S6–S13)

<a id="s6"></a>

### S6 · Ext notification và ext method trên crate ACP 3.4.0 ([T0.7](plan/M0-prep-and-spikes.md))

| Mục | Nội dung |
| --- | -------- |
| **Câu hỏi** | Qua H3, client có (a) nhận được ext notification `_auth/status_update` và (b) gọi được ext method `_session/steering` không? API của crate `agent-client-protocol` 3.4.0 đã xác định khi kiểm thiết kế: `Builder::on_receive_notification` (`src/jsonrpc.rs:1579`), `ConnectionTo::send_request` (`:4300`), `UntypedMessage` (`:5676-5685`). H3 phải sửa `v1.rs`, vì field `connection` của `AcpConnection` là private (`v1.rs:75`) **[Đã kiểm]**. |
| **Căn cứ** | [Tech Spec 20.1 (S6)](tech-spec.md#muc-20), [3.2 (H3)](tech-spec.md#muc-3-2) |
| **Chặn** | Bước 0 → [T1.8](plan/M1-foundation.md) (handler đăng nhập), [T5.6](plan/M5-playbook.md) (steering khi đổi yêu cầu) |
| **Nhánh & Bàn giao** | Nhánh `spike/S6-acp-ext` (không merge) → Báo cáo `docs/crew/spikes/S6.md`, cập nhật T1.8 và T5.6 nếu cần |
| **Các bước** | 1. Đọc ba API trên trong mã crate (`~/.cargo/registry/src/*/agent-client-protocol-3.4.0/`) và cách Zed dựng client ở `crates/agent_servers/src/acp/v1.rs` (`client_builder`, `:368-442`).<br>2. Trên nhánh spike, viết test với agent giả: agent gửi `_auth/status_update` ngay sau initialize; client gửi request `_session/steering` và nhận response.<br>3. Ghi lại tên API, kiểu payload, cách đăng ký handler bên cạnh các `on_receive_notification` hiện có và giới hạn nếu có. |
| **Tiêu chí xong** | Báo cáo trả lời đủ (a) và (b), có đoạn code mẫu, và test trên nhánh spike đạt (0 prompt tốn hạn mức vì dùng agent giả). |

<a id="s7"></a>

### S7 · Cầu nối MCP với ba provider ([T0.8](plan/M0-prep-and-spikes.md))

| Mục | Nội dung |
| --- | -------- |
| **Câu hỏi** | Cả ba provider có khởi động MCP server stdio `<zed> --crew-mcp-bridge <socket>` và gọi được tool không? Framing đã rõ: JSON-RPC, mỗi message một dòng (`crates/context_server/src/listener.rs:360-394`), nên cầu nối chỉ chép byte; `McpServer` chỉ trả lời `tools/call`, `tools/list` và handler đã đăng ký (`listener.rs:214-242`), nên phải đăng ký handler `initialize` **[Đã kiểm]**. |
| **Căn cứ** | [Tech Spec 20.1 (S7)](tech-spec.md#muc-20), [7.2](tech-spec.md#muc-7) |
| **Chặn** | Bước 0 → [T2.1](plan/M2-orchestrator.md) (Cầu nối MCP và khung MCP server) |
| **Nhánh & Bàn giao** | Nhánh `spike/S7-mcp-bridge` (không merge) → Báo cáo `docs/crew/spikes/S7.md` |
| **Các bước** | 1. Trên nhánh spike, thêm tạm flag `--crew-mcp-bridge` vào `main.rs` theo mẫu `--askpass` (`crates/zed/src/main.rs:213-220`): sao chép hai chiều stdin ↔ socket.<br>2. Tạo `McpServer::new` với một tool `crew_ping` và handler `initialize` đăng ký bằng `handle_request` (socket `mcp.sock` trong thư mục tạm `zed-mcp*`, đặt quyền thư mục về 0700).<br>3. Với từng provider ở bản ghim, tạo session với `mcpServers: [{ name: "crew", command: std::env::current_exe(), args: ["--crew-mcp-bridge", <socket>] }]`, rồi gửi prompt yêu cầu gọi `crew_ping`.<br>4. Ghi lại cho từng provider: có gọi được tool không; có yêu cầu xin quyền cho tool MCP không và option là gì; Antigravity có nhận MCP stdio không. |
| **Tiêu chí xong** | Mỗi provider hoặc gọi được `crew_ping`, hoặc báo cáo nêu rõ lỗi và nguyên nhân. Chi phí khoảng 3 prompt ngắn. |

<a id="s8"></a>

### S8 · Độ phủ gate và vùng ghi ở mode worker ([T3.1](plan/M3-execution.md))

| Mục | Nội dung |
| --- | -------- |
| **Câu hỏi** | (1) Ở mode worker của từng provider, thao tác nào đi qua yêu cầu xin quyền? (2) Lệnh git ghi trong worktree có bị sandbox của Claude và Codex chặn không? (3) Khi nào Codex gửi yêu cầu nới quyền? (4) Sandbox Codex có chặn đọc không, và có cách nào qua ACP để loại `$TMPDIR` và `/tmp` khỏi vùng ghi của Codex không? (5) Mode `agent` của Codex tự duyệt những gì? |
| **Căn cứ** | [Tech Spec 20.1 (S8)](tech-spec.md#muc-20), [9.5](tech-spec.md#muc-9), [10.4](tech-spec.md#muc-10-4). Codex `workspace-write` có `excludeTmpdirEnvVar: false`, `excludeSlashTmp: false` (`dist/index.js:34264-34320`) **[Đã kiểm]** |
| **Chặn** | Bước 3 → [T3.4](plan/M3-execution.md) (Spawn worker), [T3.5](plan/M3-execution.md) (Luật gate cho worker), [T3.6](plan/M3-execution.md) (Sandbox Claude) |
| **Nhánh & Bàn giao** | Nhánh `spike/S8-gate-coverage` (không merge) → Báo cáo `docs/crew/spikes/S8.md`, cập nhật Spec 10.4 và T3.5 |
| **Các bước** | 1. Dùng harness T0.9 với repo thử có một git worktree tạo bằng `git worktree add` trên nhánh riêng, như `GitStore::create_worktree` (gitdir nằm ngoài worktree).<br>2. Với mỗi provider ở mode worker (Claude `default` có sandbox, Codex `workspace-write`, Antigravity `default`), yêu cầu agent: sửa file trong/ngoài worktree; chạy lệnh trong worktree; chạy `git commit` và `git push`; truy cập mạng; đọc một **file bí mật giả** trong thư mục tạm ngoài worktree; ghi vào `$TMPDIR` và `/tmp`.<br>3. Ghi lại có yêu cầu xin quyền không, các `kind` của option và kết quả thực tế (chỉ in `OK`/`DENIED`).<br>4. Lặp lại bước 2 với Codex ở mode `agent` để đối chiếu auto review. |
| **Tiêu chí xong** | Bảng trả lời đủ 5 câu hỏi; câu (4) nếu không chặn được qua ACP thì ghi vào danh sách rủi ro còn lại. Chi phí khoảng 20 lượt prompt. |

<a id="s9"></a>

### S9 · Giới hạn của Claude và chặn tool của Antigravity ([T3.2](plan/M3-execution.md))

| Mục | Nội dung |
| --- | -------- |
| **Câu hỏi** | `maxBudgetUsd`, `maxTurns` và luật `settings.permissions.deny` cho tool của Claude có hiệu lực khi chạy thật không? Luật deny có chặn được lệnh Bash vốn được tự cho phép trong sandbox (`autoAllowBashIfSandboxed`), ví dụ `Bash(git commit:*)`, không? `_meta.agy.disabledTools` của Antigravity có hiệu lực không? |
| **Căn cứ** | [Tech Spec 20.1 (S9)](tech-spec.md#muc-20), [6.2](tech-spec.md#muc-6-2), [9.2](tech-spec.md#muc-9); adapter Claude chuyển nguyên `_meta.claudeCode.options` cho SDK (`acp-agent.js:8570, 8688`) **[Đã kiểm]** |
| **Chặn** | Bước 3 → [T3.6](plan/M3-execution.md) (Sandbox Claude), [T3.13](plan/M3-execution.md) (Scheduler) |
| **Nhánh & Bàn giao** | Nhánh `spike/S9-limits-deny` (không merge) → Báo cáo `docs/crew/spikes/S9.md` |
| **Tiêu chí xong** | Mỗi tùy chọn có một test đạt hoặc kết luận không có hiệu lực; tùy chọn nào không có hiệu lực ở provider thì [T3.13](plan/M3-execution.md) thay bằng giới hạn ở tầng Crew. |

<a id="s10"></a>

### S10 · Tài nguyên bộ nhớ và CPU của N Project không cửa sổ ([T3.3](plan/M3-execution.md))

| Mục | Nội dung |
| --- | -------- |
| **Câu hỏi** | Bộ nhớ và CPU của N Project không cửa sổ cùng N process agent là bao nhiêu với N = 1, 2, 4? |
| **Căn cứ** | [Tech Spec 20.1 (S10)](tech-spec.md#muc-20); đầu vào cho [Brief Q7](open-question.md#q7) |
| **Chặn** | Bước 3 → [T3.13](plan/M3-execution.md) (giá trị đề xuất cho trần song song) và [T7.8](plan/M7-model-and-retro.md) |
| **Nhánh & Bàn giao** | Nhánh `spike/S10-resources` (không merge) → Báo cáo `docs/crew/spikes/S10.md` |
| **Cách đo** | Mở N worker giả lập bằng N worktree, mỗi worktree một `Project::local` không cửa sổ và một session ACP đang làm một việc ngắn có thật (ví dụ đọc và tóm tắt một file). Đo bộ nhớ và CPU của process Zed cùng các process adapter bằng công cụ của macOS, ở trạng thái rảnh và khi đang chạy. |
| **Tiêu chí xong** | Bảng có số đo cho N = 1, 2, 4 ở cả hai trạng thái, ghi cấu hình máy, đo lặp lại ít nhất hai lần. Spike cung cấp dữ kiện cho Human, không tự đặt trần. |

<a id="s11"></a>

### S11 · ROLE của Codex qua `customInstructions` khi load/resume ([T2.10](plan/M2-orchestrator.md))

| Mục | Nội dung |
| --- | -------- |
| **Câu hỏi** | ROLE của Codex truyền qua `_meta.jetbrains.air.customInstructions` có còn hiệu lực sau khi `load` hoặc `resume` trong process adapter mới không? (Phần hiệu lực ở session mới đã kiểm ở [T1.6](plan/M1-foundation.md)). |
| **Căn cứ** | [Tech Spec 20.1 (S11)](tech-spec.md#muc-20), [14.1 bước 4](tech-spec.md#muc-14); adapter Codex không đọc `customInstructions` khi `loadSession`/`resumeSession` (`dist/index.js:35227-35340`) **[Đã kiểm]** |
| **Chặn** | Bước 2 → Cách phục hồi thread Orchestrator trên Codex sau khi Zed khởi động lại ([T2.10](plan/M2-orchestrator.md)) |
| **Nhánh & Bàn giao** | Báo cáo `docs/crew/spikes/S11.md`; cập nhật `crates/crew/src/recovery.rs` và `ProviderProfile` của Codex |
| **Các bước** | Tạo session Codex có ROLE marker, gửi một lượt, tắt process adapter. Mở process mới, gọi `load` và `resume`, rồi hỏi để kiểm marker. Nếu mất ROLE: khi recovery, Crew mở thread Orchestrator mới và nắm lại trạng thái bằng `crew_get_state` (thread cũ chỉ xem). Nếu còn ROLE: resume trực tiếp qua `CrewConnection` như Claude. |
| **Tiêu chí xong** | Báo cáo có kết luận rõ ràng kèm contract test `role_codex_resume`. Chi phí khoảng 4 prompt. |

<a id="s12"></a>

### S12 · Định dạng hạn mức của Claude ([T1.11](plan/M1-foundation.md))

| Mục | Nội dung |
| --- | -------- |
| **Câu hỏi** | Định dạng output thật của lệnh `/usage` trên Claude là gì? Cấu trúc thật của `_meta["_claude/rateLimit"]` trong usage update là gì? |
| **Căn cứ** | [Tech Spec 20.1 (S12)](tech-spec.md#muc-20), [6.7](tech-spec.md#muc-6); mã adapter `usage-markdown.js`, `acp-agent.js:6084-6094` **[Đã kiểm]** |
| **Chặn** | Bước 1 → Phần parser hạn mức Claude trong [T1.12](plan/M1-foundation.md) |
| **Nhánh & Bàn giao** | Báo cáo `docs/crew/spikes/S12.md`; fixture đã che dữ liệu cá nhân trong `crates/crew_acp/test_fixtures/contract/` |
| **Các bước** | Dùng harness [T0.9](plan/M0-prep-and-spikes.md) gửi `/usage` trong session thăm dò của Claude, và ghi `_meta` của usage update sau một prompt ngắn. Che email và ID tài khoản trước khi lưu fixture. |
| **Tiêu chí xong** | Có fixture cho cả hai nguồn; báo cáo nêu rõ các cửa sổ (5 giờ, 7 ngày), đơn vị (`utilization`) và định dạng giờ reset (`resets_at`). |

<a id="s13"></a>

### S13 · Nhóm hạn mức của Gemini 4 Argon trên Antigravity ([T7.1](plan/M7-model-and-retro.md))

| Mục | Nội dung |
| --- | -------- |
| **Câu hỏi** | Model `gemini-4-argon` tính vào nhóm hạn mức nào trong output `/usage` của Antigravity? |
| **Căn cứ** | [Tech Spec 20.1 (S13)](tech-spec.md#muc-20), [6.7](tech-spec.md#muc-6). Output `/usage` chỉ ghi nhóm “Gemini Models” gồm “Gemini Flash, Gemini Pro”, không nhắc Argon **[Spike]** |
| **Ảnh hưởng** | Không chặn bước nào; trong lúc chờ kết quả, hạn mức của Argon là `unknown` (với chính sách mặc định `skip`, Argon bị bỏ qua; với `ask_human`, Crew hỏi Human). |
| **Nhánh & Bàn giao** | Báo cáo `docs/crew/spikes/S13.md`; cập nhật bảng ánh xạ model ↔ nhóm hạn mức trong `ProviderProfile` và contract test |
| **Cách đo** | 1. Đọc `/usage` trước khi chạy.<br>2. Chạy một loạt prompt đủ dài với `gemini-4-argon` để phần trăm còn lại của một nhóm giảm thấy rõ.<br>3. Đọc lại `/usage` và so sánh hai nhóm.<br>4. Lặp lại thêm một lần để loại trừ trùng hợp. |
| **Tiêu chí xong** | Xác định được nhóm hạn mức của Argon (và khóa vào contract test theo phiên bản adapter), hoặc kết luận không xác định được. Ghi rõ số prompt và lượng token đã dùng. |
