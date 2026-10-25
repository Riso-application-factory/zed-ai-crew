# M6 · Giao diện

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Human nhìn thấy cả team làm việc, biết ngay việc nào đang cần mình, tìm lại được session và OUTPUT, thiết lập Crew qua Settings và onboarding. Mọi bề mặt dùng component của Zed |
| Spec | Mục 18 bước 6; 15; 3.2 (H5b, H5c, H6, H7) |
| Cần có trước | M1, M5 (projection và artifact để hiển thị) |
| Spike | — |
| Quy tắc chung cho mọi task | Chỉ dùng component của crate `ui` và màu theme. Trạng thái có chữ hoặc icon đi kèm màu. Mở được bằng bàn phím. Không có nút Approve hay Re-run (NEED 13). Phần trăm tiến độ chỉ hiện khi có cơ sở đo. Số liệu ghi rõ là đo được, ước tính hay không có (NEED 30). Mọi task chạy app thật và đính ảnh chụp (NEED 03) |

## Danh sách task

| Task | Tên | Phụ thuộc | NEED |
| ---- | --- | --------- | ---- |
| T6.1 | Tổng quan session | M5 | 43, 49, 50 |
| T6.2 | Thread worker chỉ đọc (H5b) | M3 | 50 |
| T6.3 | Việc chờ Human: prototype rồi hiện thực | T5.9 | 12, 52 |
| T6.4 | Status bar | M1 | 50, 56, 60 |
| T6.5 | Sidebar và History (H5c) | M2 | 53 |
| T6.6 | Trang Settings (H7, phần UI) | T1.2 | 27 |
| T6.7 | Onboarding (H6) | T4.2 | 04, 05, 17 |
| T6.8 | Tài liệu quyết định; rà soát tuân thủ Zed UI | T6.1–T6.7 | 13, 51, 57 |

---

### T6.1 · Tổng quan session

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Một màn hình cho biết session đang ở đâu, ai đang làm gì, bằng chứng nằm đâu |
| Căn cứ | NEED 43, 49, 50; spec 15 (Tổng quan session) |
| Được sửa | `crates/crew_ui/src/overview.rs`; action “Crew: Open Session Overview” |
| Bàn giao | Workspace item Tổng quan session |

**Nội dung:** loại session, trạng thái, phiên bản INPUT, tiêu chí OUTPUT và trạng thái từng tiêu chí, pipeline dạng cây có dependency. Với mỗi worker: ROLE, provider, model, work item age, mức cách ly, rủi ro còn lại (T3.5). Thêm compaction, effect, bằng chứng (spec 11.3).

Mọi dữ liệu đọc từ projection của run store, không suy từ lời của agent (NEED 43).

**Tiêu chí nghiệm thu**
- Cho session có 3 task ở 3 trạng thái khác nhau, khi mở Tổng quan, thì cây pipeline hiện đúng trạng thái và dependency.
- Cho worker Antigravity tài khoản cá nhân, khi xem dòng của nó, thì thấy mức cách ly “thấp” và rủi ro còn lại.
- Cho tiêu chí `passed`, khi mở, thì đi được tới bằng chứng của tiêu chí đó.
- Cho chỉ dùng bàn phím, khi điều hướng Tổng quan, thì mở được mọi mục.

**Kiểm chứng:** `cargo test -p crew_ui overview`; chạy app, chụp Tổng quan của một session thật trên repo thử.

---

### T6.2 · Thread worker chỉ đọc (H5b)

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Human xem được hội thoại của worker nhưng không gõ vào được và không trả lời được yêu cầu xin quyền của worker |
| Căn cứ | NEED 50; spec 3.2 (H5b), 3.3 (Session worker), 10.1 |
| Được sửa | `crates/agent_ui/src/conversation_view/thread_view.rs` và `crates/agent_ui/src/conversation_view.rs` (H5b: cờ chỉ đọc cho thread hoặc view; không hiện option `allow_always` cho session Crew); predicate “session Crew” và “session worker của Crew” trong registry của `agent_servers` |
| Bàn giao | View chỉ đọc cho thread worker, hiện từ workspace của Orchestrator |

**Thiết kế**
- **H5b là cờ chỉ đọc thật**, không tái dùng nhánh subagent. `is_subagent` là `parent_session_id.is_some()` (`thread_view.rs:1268-1269`), chỉ được đặt trong `AcpThread::new` (`acp_thread.rs:3686`), `AcpConnection` truyền `None` và không có setter; giả một parent id thì sidebar không lưu thread (`conversation_view.rs:1850`) **[Đã kiểm]**.
- View chỉ đọc không có composer. Worker có `Project` không cửa sổ (T3.4), nên thread worker hiện trong workspace của Orchestrator.
- Session Crew không bao giờ hiện option `allow_always` trong UI. Phím tắt “allow always” của view gốc trả lời yêu cầu đang chờ đầu tiên của mọi session trong conversation (`conversation_view.rs:463-486`, `thread_view.rs:3012-3017`) **[Đã kiểm]**; với session Crew, phím tắt này không làm gì. Gate trả lời yêu cầu của worker trước khi view hiện chúng (T3.5).
- Thread worker mở lại sau khi Zed khởi động lại vẫn đi qua CrewConnection (T1.7), nhưng Crew không gửi prompt giao việc mới (spec 14.1 bước 3).

**Tiêu chí nghiệm thu**
- Cho thread worker, khi mở, thì không có ô nhập và không gửi được tin nhắn.
- Cho thread thường của Human, khi mở, thì vẫn có ô nhập như cũ.
- Cho thread worker, khi Zed khởi động lại, thì thread vẫn có trong sidebar và vẫn chỉ đọc.
- Cho session Crew có yêu cầu xin quyền đang chờ, khi Human bấm phím tắt “allow always”, thì không option nào được chọn.
- Cho yêu cầu xin quyền của session Crew có option `allow_always`, khi view hiện yêu cầu, thì không có nút cho option đó.

**Kiểm chứng:** `cargo test -p agent_ui`; chạy app, chụp thread worker.

---

### T6.3 · Việc chờ Human: prototype rồi hiện thực

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Human biết ngay việc nào đang cần mình, ở đâu, vì sao |
| Căn cứ | NEED 12, 52; spec 15 (Việc chờ Human) |
| Được sửa | `crates/crew_ui/src/pending.rs` |
| Bàn giao | Prototype để Human chọn bề mặt; bản hiện thực theo lựa chọn |

**Các bước**
1. Làm prototype theo T5.9, nhánh `crew/prototype/pending-human`, với hai hoặc ba phương án bề mặt. Ví dụ: panel riêng, mục trong Tổng quan, badge trên status bar.
2. Human chọn phương án trong hội thoại; quyết định được ghi qua T2.6.
3. Hiện thực phương án đã chọn trên nhánh task, đọc từ projection “việc chờ Human” của run store.

**Tiêu chí nghiệm thu**
- Cho prototype, khi xong, thì có artifact ghi câu hỏi thiết kế, các phương án, quyết định của Human.
- Cho câu hỏi đang mở từ `crew_ask_human`, khi xem bề mặt đã chọn, thì thấy câu hỏi kèm bối cảnh và đường tới thread Orchestrator.
- Cho bước chỉ Human làm được (T2.7), khi xem, thì thấy hướng dẫn thực hiện.

**Kiểm chứng:** chạy app, chụp từng phương án của prototype và bản hiện thực.

---

### T6.4 · Status bar

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Liếc là biết Crew đang chạy gì và có gì bất thường |
| Căn cứ | NEED 50, 56, 60; spec 15 (Status bar) |
| Được sửa | `crates/crew_ui/src/status_bar.rs` |
| Bàn giao | Status bar item |

**Nội dung:** số worker đang chạy, số việc chờ Human, trạng thái đăng nhập từng provider (T1.8), provider có hạn mức sắp chạm ngưỡng (T1.12), tuổi của lần cập nhật gần nhất.

**Tiêu chí nghiệm thu**
- Cho Claude ở `not_logged_in`, khi xem status bar, thì thấy trạng thái bằng chữ hoặc icon, không chỉ bằng màu, và bấm vào thì tới chỉ dẫn đăng nhập.
- Cho hạn mức `unknown`, khi xem, thì hiện “không rõ”, không hiện 0 hay 100%.

**Kiểm chứng:** `cargo test -p crew_ui status_bar`; chạy app, chụp status bar.

---

### T6.5 · Sidebar và History (H5c)

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Tìm và quay lại đúng session hoặc OUTPUT |
| Căn cứ | NEED 53; spec 3.2 (H5c), 15 (Tìm và lưu trữ) |
| Được sửa | `crates/agent_ui` (nhóm thread Crew trong sidebar, qua registry của `agent_servers`); `crates/crew_ui` |
| Bàn giao | Thread Crew có nhãn và được nhóm theo session trong Threads Sidebar và Thread History |

Thread import đã lọc session Crew và session thăm dò từ T2.3.

**Tiêu chí nghiệm thu**
- Cho một session Crew có Orchestrator và 2 worker, khi mở sidebar, thì ba thread nằm chung một nhóm có nhãn Crew.
- Cho thread Crew mở từ History, khi load, thì đi qua CrewConnection (test bất biến của T1.7 vẫn đạt).
- Cho session thăm dò, khi xem sidebar và History, thì không hiện.

**Kiểm chứng:** `cargo test -p agent_ui -p crew_ui`; chạy app, chụp sidebar.

---

### T6.6 · Trang Settings (H7, phần UI)

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Human xem và sửa cấu hình Crew mà không cần viết JSON |
| Căn cứ | NEED 27; spec 5 |
| Được sửa | `crates/settings_ui` (trang Crew); `crates/crew_ui` |
| Bàn giao | Trang Settings cho Crew |

**Thiết kế**
- Tách rõ user-scope và project-scope.
- Với khóa project, hiện giá trị hiệu lực sau khi gộp và cảnh báo khi project cố nới rộng (T1.2).
- Hiện snapshot model kèm tuổi, cùng nguồn với readiness (NEED 56).
- Ngưỡng Q7 chưa đặt thì ghi rõ “chưa cấu hình”, không điền sẵn số.

**Tiêu chí nghiệm thu**
- Cho project đặt allowlist rộng hơn trần, khi mở Settings, thì thấy giá trị hiệu lực là phần giao, kèm cảnh báo.
- Cho Human sửa một khóa user-scope trong Settings, khi lưu, thì `settings.json` của user có giá trị mới, và lần gộp kế tiếp dùng giá trị đó.

**Kiểm chứng:** `cargo test -p settings_ui -p crew_ui`; chạy app, chụp trang Settings.

---

### T6.7 · Onboarding (H6)

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Thiết lập Crew cho một project từ đầu tới lúc sẵn sàng |
| Căn cứ | NEED 04, 05, 17; brief 4.3 (Onboarding); spec 3.2 (H6) |
| Được sửa | `crates/onboarding` (H6); `crates/crew_ui` |
| Bàn giao | Trang onboarding |

Onboarding của Zed chỉ có một trang và không có registry trang (`crates/onboarding/src/onboarding.rs:299-301`) **[Đã kiểm]**. H6 hoặc thêm một mục Crew vào trang đó, hoặc mở một item Crew riêng từ đó; cách đã chọn ghi vào `docs/crew/hook-points.md`.

**Các bước trên trang**
1. Trạng thái đăng nhập từng provider; chưa đăng nhập thì chỉ dẫn tới luồng của Zed.
2. Kết nối GitHub App: hướng dẫn (T4.1), nhập key (T4.2), kiểm sẵn sàng (T4.2).
3. Mở session Kickoff đầu tiên (T5.8).

Mỗi bước chỉ Human làm được có hướng dẫn thực hiện theo mẫu T2.7.

**Tiêu chí nghiệm thu**
- Cho project chưa kết nối App, khi mở onboarding, thì bước 2 hiện “chưa sẵn sàng” kèm việc còn thiếu.
- Cho mọi bước đạt, khi xem, thì onboarding báo sẵn sàng theo NEED 05 và mở được Kickoff.

**Kiểm chứng:** chạy app, chụp từng bước.

---

### T6.8 · Tài liệu quyết định; rà soát tuân thủ Zed UI

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Tài liệu quyết định đọc được ngay trong Zed; toàn bộ UI của Crew tuân thủ Zed UI |
| Căn cứ | NEED 13, 51, 57; spec 15 |
| Được sửa | `crates/crew_ui` |
| Bàn giao | Mở tài liệu quyết định bằng Markdown Preview; báo cáo rà soát UI |

**Rà soát**, theo checklist ở đầu file:
- component và màu;
- trạng thái có chữ hoặc icon;
- bàn phím;
- không có Approve hay Re-run;
- phần trăm có cơ sở đo;
- số liệu có nhãn đo được, ước tính hay không có.

Agent kiểm chứng độc lập chạy app và đối chiếu từng bề mặt với checklist.

**Tiêu chí nghiệm thu**
- Cho một tài liệu quyết định, khi mở từ Tổng quan, thì hiện bằng Markdown Preview.
- Cho mọi bề mặt của Crew, khi rà, thì không mục nào trong checklist bị vi phạm.

**Kiểm chứng:** chạy app; báo cáo rà soát kèm ảnh chụp từng bề mặt.

---

## Điều kiện xong M6

- [ ] T6.1–T6.8 đạt theo Định nghĩa Xong chung.
- [ ] Mọi bề mặt ở spec mục 15 chạy trong app thật, có ảnh chụp trong PR.
