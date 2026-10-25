# Zed AI Crew — Câu hỏi mở, giả định và việc cần Human quyết định

| Mục | Nội dung |
| --- | -------- |
| Phiên bản | 1.0 · 2026-10-25 |
| Nguồn | Tổng hợp từ [Product Brief v3.5](brief.md) ([mục 7](brief.md#muc-7)), [Tech Spec v2.2](tech-spec.md) ([mục 20.2](tech-spec.md#muc-20)) và [Kế hoạch triển khai v1.1](plan/00-overview.md) ([mục 9](plan/00-overview.md)). |
| Nguyên tắc | Chỉ giữ các câu hỏi và quyết định chưa có câu trả lời tại thời điểm hiện tại. Câu hỏi nào đã có câu trả lời (như Q1–Q5, Q1b, Q3b và D1–D21) đã được đưa thẳng vào nội dung liên quan trong Brief và Tech Spec, không giữ danh sách hỏi đáp lịch sử. |
| Nhãn | **[Đã kiểm]** đọc trong mã nguồn hoặc tài liệu gốc · **[Spike]** kết quả spike 2026-10-25 · **[Baseline]** theo baseline provider của Human · **[Nhớ]** kiến thức đáng tin, chưa kiểm lại · **[Suy luận]** cần spike hoặc test để xác nhận |

## Mục lục

| Phần | Mục |
| ---- | --- |
| Câu hỏi & Giả định | [1. Câu hỏi mở của Product Brief & Tech Spec (Q6–Q9)](#muc-1) · [2. Giả định nền tảng (A1–A5)](#muc-2) |
| Kiểm chứng & Hành động | [3. Các điểm `[Suy luận]` cần xác nhận khi triển khai](#muc-3) · [4. Việc Human cần làm theo mốc triển khai](#muc-4) · [5. Lịch sử câu hỏi đã giải quyết (Q1–Q5)](#muc-5) |

---

<a id="muc-1"></a>

## 1. Câu hỏi mở của Product Brief & Tech Spec (Q6–Q9)

Không có hạn chót theo ngày vì dự án Crew không có deadline ([NEED 03](brief.md#need-03)); cột **Khi nào** là mốc phụ thuộc phải có câu trả lời trước khi bước tương ứng hoàn tất.

| Mã | Câu hỏi mở | Bối cảnh & Chi tiết cần chốt | Ảnh hưởng | Ai trả lời | Khi nào (Mốc chặn) |
| -- | ---------- | ---------------------------- | --------- | ---------- | ------------------ |
| <a id="q6"></a>**Q6** | **Danh mục artifact SDLC, bộ state và điều kiện chuyển state đầy đủ là gì?** | Bản 2.x và [Brief mục 5.5](brief.md#sdlc-artifacts) mới nêu ví dụ tầm nhìn (`product-brief`, `PRD`, `SRS`, `Idea → Epic → Feature → User Story → Task → Sub-task`). [Tech Spec mục 4.3](tech-spec.md#muc-4) tạm dùng state của BMAD (`draft` → `ready` → `in_progress` → `in_review` → `done` · `blocked` · `dropped` · `superseded`). Cần chốt danh mục artifact chính thức, bộ state và điều kiện chuyển state theo từng workflow. | [NEED 39](brief.md#need-39), [NEED 43](brief.md#need-43), bảng `crew_artifacts` ([Tech Spec 4.2](tech-spec.md#muc-4)) | Agent AI đề xuất từ nguồn seed (state machine của BMAD); **Human duyệt** | Khi đặc tả workflow ở **M5** ([T5.3](plan/M5-playbook.md), [T5.10](plan/M5-playbook.md)) |
| <a id="q7"></a>**Q7** | **Giá trị mặc định của ngưỡng “sắp hết hạn mức”, ngưỡng tuổi snapshot model và giới hạn song song?** | Trong [Tech Spec mục 5.2](tech-spec.md#muc-5), các khóa `crew.quota.near_limit_percent_left`, `crew.snapshot_max_age`, `crew.parallel.global` và `crew.parallel.per_provider.antigravity-acp` đang để `null` (“chưa cấu hình”, theo [NEED 30](brief.md#need-30) không tự chọn số). Cần số đo thực nghiệm từ [Spike S10](spike-list.md#s10) (tài nguyên N workspace) và [Spike S13](spike-list.md#s13) (nhóm hạn mức Argon) để Human đặt giá trị. | [NEED 25](brief.md#need-25), [NEED 26](brief.md#need-26), [NEED 30](brief.md#need-30), [NEED 56](brief.md#need-56) | **Human**, đặt trong cấu hình user-scope (`settings.json`) | Trước manual test ở **M7** ([T7.8](plan/M7-model-and-retro.md)) |
| <a id="q8"></a>**Q8** | **Ngưỡng cho các chỉ số “chưa đặt ngưỡng” ở [Brief mục 3.2](brief.md#chi-so-chinh) là bao nhiêu?** | Ba chỉ số đang theo dõi xu hướng chưa đặt ngưỡng cứng: (1) Thời gian Human chờ tới lượt quyết định và số lượt quyết định mỗi session; (2) Tỷ lệ PR implement bị Human yêu cầu sửa vì lệch kiến trúc hoặc lệch hệ thống thật; (3) Tỷ lệ session đạt Done theo tiêu chí của loại session. | [Brief mục 3.2](brief.md#chi-so-chinh), [mục 3.4](brief.md#baseline) | **Human**, sau khi đo baseline vài session theo quy trình hiện tại | Sau khi có baseline, trước/trong manual test ([T7.8](plan/M7-model-and-retro.md)) |
| <a id="q9"></a>**Q9** | **Target repo nào dùng cho lần manual test đầu tiên?** | Theo [Brief mục 9](brief.md#muc-9), khi Crew đủ phạm vi từ M0–M7, Human sẽ manual test trên một target repo thử nghiệm ít rủi ro trước khi bật cho các project khác qua onboarding. | [Brief mục 9](brief.md#muc-9), [NEED 03](brief.md#need-03), [NEED 04](brief.md#need-04), [NEED 05](brief.md#need-05) | **Human** | Khi Crew đủ phạm vi ở **M7** ([T7.8](plan/M7-model-and-retro.md)) |

---

<a id="muc-2"></a>

## 2. Giả định nền tảng (A1–A5)

Tổng hợp từ [Brief mục 7.2](brief.md#muc-7). Nếu một giả định không còn đúng trong thực tế, hệ thống xử lý theo cột **Hành vi khi giả định sai**.

| Mã | Giả định | Hành vi khi giả định sai | NEED liên quan |
| -- | -------- | ------------------------ | -------------- |
| <a id="a1"></a>**A1** | Human đã đăng nhập các provider qua Zed trên máy macOS này. | Crew phát hiện trạng thái `not_logged_in` qua healthcheck lúc khởi động và chỉ dẫn Human tới luồng đăng nhập của Zed; không tự đăng nhập thay. | [NEED 17](brief.md#need-17), [NEED 56](brief.md#need-56) |
| <a id="a2"></a>**A2** | Target repo nằm trên GitHub; brief không định nghĩa forge khác (GitLab, Bitbucket…). | Phải thiết kế và viết lại các yêu cầu về danh tính App, PR, Issues và phát hiện lệch. | [NEED 05](brief.md#need-05), [NEED 35](brief.md#need-35), [NEED 39](brief.md#need-39)–[42](brief.md#need-42) |
| <a id="a3"></a>**A3** | Adapter ở phiên bản đã ghim giữ đúng hành vi đã quan sát (kênh system prompt, mode, thông báo trạng thái đăng nhập, định dạng hạn mức). | Bộ contract test ([Tech Spec mục 17](tech-spec.md#muc-17)) bắt lỗi ngay khi thử đổi bản ghim; khi runtime mất kênh thì `CrewConnection` báo blocker và từ chối tạo session. | [NEED 28](brief.md#need-28), [NEED 55](brief.md#need-55) |
| <a id="a4"></a>**A4** | Zed upstream giữ ACP Registry và các bề mặt UI (`AgentPanel`, `AcpThread`, `SettingsStore`, `McpServer`) mà Crew dùng tại các điểm nối H1–H11. | Quy trình đồng bộ upstream ([T1.14](plan/M1-foundation.md)) và checklist điểm nối ([T0.2](plan/M0-prep-and-spikes.md)) phát hiện và chặn merge nhánh đồng bộ cho tới khi cập nhật điểm nối. | [NEED 59](brief.md#need-59) |
| <a id="a5"></a>**A5** | Provider cho phép dùng gói/tài khoản hiện tại khi bị điều khiển tự động qua adapter ACP chính thức. | Giới hạn độ song song (như trần riêng cho Antigravity) hoặc tắt provider đó trong cấu hình `enabled_providers`. | [NEED 25](brief.md#need-25) |

---

<a id="muc-3"></a>

## 3. Các điểm `[Suy luận]` trong Tech Spec & Kế hoạch cần xác nhận khi triển khai

Các điểm dưới đây được gắn nhãn **[Suy luận]** hoặc **[Nhớ]** trong [Tech Spec v2.2](tech-spec.md) và các file milestone, cần được xác nhận bằng spike hoặc unit/integration test trong quá trình thực hiện task:

| # | Điểm cần xác nhận | Vị trí trong Spec / Plan | Cách xác nhận |
| - | ----------------- | ------------------------ | ------------- |
| 1 | Các lần ping healthcheck thứ hai trở đi trong cùng session thăm dò của Antigravity có tốn ít token hơn nhiều so với prompt đầu (~12,7 nghìn token) không? | [Tech Spec 6.5](tech-spec.md#muc-6-5), [T1.8](plan/M1-foundation.md) | Đo `usage_update` ở lần resume + ping thứ 2 trong contract test `health_` ([T1.8](plan/M1-foundation.md)). |
| 2 | Antigravity có trả lại `configOptions` (danh sách model/thinking_level) khi gọi `session/load` hoặc `session/resume` không, hay chỉ trả ở `session/new`? | [Tech Spec 6.6](tech-spec.md#muc-6), [T1.9](plan/M1-foundation.md) | Kiểm trong contract test của [T1.6](plan/M1-foundation.md) / [T1.9](plan/M1-foundation.md); nếu không trả khi resume thì tạo `session/new` trong thư mục thăm dò khi cần làm mới snapshot. |
| 3 | Tên event chính xác trên `AcpThread` để `DecisionCapture` bắt tin nhắn mới của Human trong thread Orchestrator là gì? | [T2.6](plan/M2-orchestrator.md) | Kiểm mã `crates/acp_thread/src/acp_thread.rs` khi viết [T2.6](plan/M2-orchestrator.md). |
| 4 | Lệnh `git commit` trong git worktree của worker Claude (khi bật sandbox) có bị chặn ghi vào `.git/worktrees/<tên>` nằm ngoài worktree không? | [Tech Spec 9.5](tech-spec.md#muc-9), [T3.1 (S8)](plan/M3-execution.md) | Xác nhận thực nghiệm trong [Spike S8](spike-list.md#s8). |
| 5 | Sandbox `workspace-write` của Codex có giới hạn đọc file không, và có tùy chọn ACP nào loại bỏ `$TMPDIR` cùng `/tmp` khỏi vùng ghi không? | [Tech Spec 10.4](tech-spec.md#muc-10-4), [T3.1 (S8)](plan/M3-execution.md) | Xác nhận thực nghiệm trong [Spike S8](spike-list.md#s8); nếu không có thì ghi vào `residual_risks_json` ở [T3.5](plan/M3-execution.md). |
| 6 | Khi Zed mở file ngoài worktree và tạo worktree ẩn trong workspace, worker Antigravity có ghi được vào đường dẫn đó qua `fs/write_text_file` không? | [Tech Spec 10.4](tech-spec.md#muc-10-4) | Kiểm tra hành vi `worktree_store.rs` trong test gate của [T3.5](plan/M3-execution.md). |
| 7 | Chạy `git` với `-c credential.helper=` kết hợp `ZED_ASKPASS_SOCKET` có đảm bảo bỏ qua hoàn toàn credential của Human và không lưu token installation vào Keychain không? | [Tech Spec 12](tech-spec.md#muc-12), [T4.3](plan/M4-github.md) | Kiểm bằng unit test với repo bare cục bộ và credential helper giả ở [T4.3](plan/M4-github.md). |
| 8 | Bề mặt UI hiển thị “Việc chờ Human” (panel riêng, mục trong Tổng quan session, hay badge trên status bar) dùng phương án nào? | [Brief mục 8](brief.md#muc-8), [Tech Spec 15](tech-spec.md#muc-15), [T6.3](plan/M6-ui.md) | Dựng prototype trên nhánh `crew/prototype/pending-human` ở [T6.3](plan/M6-ui.md) để **Human chọn**. |
| 9 | Điều khoản sử dụng API miễn phí của Artificial Analysis yêu cầu ghi nguồn (attribution) cụ thể ra sao trên UI? | [T7.3](plan/M7-model-and-retro.md) | Đọc trang điều khoản gốc khi hiện thực bộ nạp ở [T7.3](plan/M7-model-and-retro.md). |
| 10 | Khác biệt định dạng lưu trữ session giữa bản adapter Zed tự cập nhật và bản ghim của Crew có gây lỗi khi load chéo không? | [Tech Spec 19 (T14)](tech-spec.md#muc-19) | Crew chỉ load session thuộc `crew_acp_sessions`; kiểm thêm bằng contract test load chéo. |

---

<a id="muc-4"></a>

## 4. Việc Human cần làm theo mốc triển khai

Tổng hợp từ [Kế hoạch triển khai — mục 9](plan/00-overview.md) và các task từ M0 đến M7:

| Mốc | Công việc của Human | Task liên quan | Kết quả cần đạt |
| --- | ------------------- | -------------- | --------------- |
| **M0** | Xác nhận repo fork trên GitHub có visibility là `PRIVATE`. | [T0.1](plan/M0-prep-and-spikes.md) | `gh repo view --json visibility -q .visibility` trả về `PRIVATE` ([NEED 58](brief.md#need-58)). |
| **M0–M3** | Có mặt trên máy macOS đã đăng nhập thật cả 3 provider khi agent chạy contract test hoặc các spike tốn hạn mức. | [T0.9](plan/M0-prep-and-spikes.md), [S6–S12](spike-list.md#phan-2) | Các báo cáo spike `docs/crew/spikes/S6..S12.md` và fixture contract test hoàn tất. |
| **M4** | Tạo GitHub App trên `github.com` theo hướng dẫn, cài vào repo thử với quyền tối thiểu, tải file `.pem` và chọn file qua hộp thoại onboarding. | [T4.1](plan/M4-github.md), [T4.2](plan/M4-github.md) | Private key lưu vào macOS Keychain; bộ kiểm sẵn sàng ([NEED 05](brief.md#need-05)) báo đạt. |
| **M5** | Duyệt danh mục artifact và state machine SDLC do agent đề xuất từ seed (**Q6**). | [T5.3](plan/M5-playbook.md), [T5.10](plan/M5-playbook.md) | Chốt state machine cho từng loại artifact/workflow. |
| **M6** | Xem các phương án prototype của bề mặt “Việc chờ Human” và chọn phương án chính thức trong hội thoại. | [T6.3](plan/M6-ui.md) | Quyết định được ghi nhận qua `DecisionCapture` và hiện thực vào `crates/crew_ui/src/pending.rs`. |
| **M7 (Trước manual test)** | Đặt giá trị cấu hình cho **Q7** (ngưỡng hạn mức, tuổi snapshot, trần song song) dựa trên báo cáo S10 & S13; chọn target repo thử nghiệm ít rủi ro (**Q9**). | [T7.8](plan/M7-model-and-retro.md) | Cấu hình user-scope hoàn chỉnh; bản build release bật `CrewFeatureFlag::enabled_for_all() = true` sẵn sàng chạy thử. |
| **M7 (Sau baseline)** | Ghi lại vài session làm theo quy trình hiện tại ([Brief 3.4](brief.md#baseline)) và đặt ngưỡng cho các chỉ số ở **Q8**. | [T7.8](plan/M7-model-and-retro.md) | Có mốc so sánh định lượng cho đợt manual test. |

---

<a id="muc-5"></a>

## 5. Lịch sử câu hỏi đã giải quyết (Q1–Q5, Q1b, Q3b)

Theo quy tắc của repo (`.rules`) và [Brief v3.3–v3.5](brief.md#phu-luc-e), các câu hỏi kỹ thuật Q1–Q5 cùng Q1b, Q3b đã được trả lời bằng đợt spike ngày 2026-10-25 và tích hợp trực tiếp vào nội dung chính:

- **Q1 & Q1b (Kênh nạp ROLE và tính bền vững sau compaction):** Đã đưa vào [NEED 55](brief.md#need-55), [NEED 31](brief.md#need-31) và [Tech Spec 6.1–6.2](tech-spec.md#muc-6).
- **Q2 (Trạng thái đăng nhập provider qua ACP):** Đã đưa vào [NEED 56](brief.md#need-56) và [Tech Spec 6.5](tech-spec.md#muc-6-5).
- **Q3 & Q3b (Sandbox của Claude, chặn đọc đường dẫn bí mật và `$TMPDIR`):** Đã đưa vào [NEED 07](brief.md#need-07), rủi ro R12 và [Tech Spec 6.2, 10.4](tech-spec.md#muc-6-2).
- **Q4 (Hành vi, model, chi phí cố định và compaction của Antigravity):** Đã đưa vào [NEED 25](brief.md#need-25), [NEED 30](brief.md#need-30), [NEED 56](brief.md#need-56) và [Tech Spec 6.1, 6.8](tech-spec.md#muc-6).
- **Q5 (Định dạng hạn mức `/status` của Codex và `/usage` của Antigravity):** Đã đưa vào [NEED 60](brief.md#need-60) và [Tech Spec 6.7](tech-spec.md#muc-6).

Chi tiết kết quả thực nghiệm của S1–S5 và danh sách spike còn lại S6–S13 xem tại [Danh sách Spike (`spike-list.md`)](spike-list.md).
