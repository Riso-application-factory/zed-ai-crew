# Zed AI Crew — Kế hoạch triển khai: tổng quan

| Mục | Nội dung |
| --- | -------- |
| Phiên bản | 1.2 · 2026-10-25 |
| Nguồn | Product brief v3.5 và tech spec v2.3. Kế hoạch không thêm yêu cầu. Spec 2.3 đã khớp với kế hoạch này; các thay đổi của bản 2.3 và số dòng theo fork lấy từ báo cáo [`docs/crew/design-check.md`](../design-check.md). |
| Người dùng | Agent AI implementer, agent AI kiểm chứng độc lập, và Human (theo dõi tiến độ, trả lời câu hỏi, manual test cuối) |
| Cách dùng | Mỗi task là một gói việc tự đủ dùng. Giao **một task cho một agent**; agent đọc `docs/crew/AGENTS.md`, phần task và các mục spec được dẫn, không cần lịch sử hội thoại |
| Nhãn | Như spec: **[Đã kiểm]** đọc trong mã nguồn hoặc tài liệu gốc · **[Spike]** kết quả spike 2026-10-25 · **[Baseline]** theo baseline provider của Human · **[Nhớ]** kiến thức đáng tin, chưa kiểm lại · **[Suy luận]** cần kiểm |

## 1. Danh sách file

| File | Milestone | Ứng với spec mục 18 |
| ---- | --------- | ------------------- |
| [M0-prep-and-spikes.md](M0-prep-and-spikes.md) | M0 · Chuẩn bị và spike chặn | Bước 0 |
| [M1-foundation.md](M1-foundation.md) | M1 · Nền | Bước 1 |
| [M2-orchestrator.md](M2-orchestrator.md) | M2 · Orchestrator và giao việc | Bước 2 |
| [M3-execution.md](M3-execution.md) | M3 · Thực thi và kiểm chứng | Bước 3 |
| [M4-github.md](M4-github.md) | M4 · GitHub | Bước 4 |
| [M5-playbook.md](M5-playbook.md) | M5 · Playbook và pipeline | Bước 5 |
| [M6-ui.md](M6-ui.md) | M6 · Giao diện | Bước 6 |
| [M7-model-and-retro.md](M7-model-and-retro.md) | M7 · Chọn model, retro và bàn giao manual test | Bước 7 |

## 2. Bản đồ milestone

| Milestone | Mục tiêu | Cần có trước | Spike | Xong khi |
| --------- | -------- | ------------ | ----- | -------- |
| M0 | Repo sẵn sàng cho agent: tài liệu trong repo, điểm nối đã kiểm trên fork, 6 crate rỗng sau feature flag, check NEED 61, harness contract test | — | S6, S7 | Mọi task M0 đạt; báo cáo S6, S7 đã merge |
| M1 | Lớp nền: run store, cấu hình theo tầng, recovery, adapter ghim, CrewConnection, xác nhận đăng nhập, snapshot model, đọc hạn mức, sửa lỗi đăng nhập ở tầng Zed | M0 | S12 | Crew (bật flag) khởi chạy adapter ghim, xác nhận đăng nhập, đọc hạn mức; mọi đường load của Zed đi qua CrewConnection |
| M2 | Human mở session Crew và nói chuyện với Orchestrator chỉ đọc; Orchestrator làm rõ, lập pipeline, chọn provider | M1 | S11 | Một session Crew chạy được trên cả 3 provider, tới bước có pipeline và lựa chọn cho từng task |
| M3 | Worker chạy trong worktree, gate và sandbox, bàn giao, kiểm chứng, evaluator, scheduler, tích hợp nhánh | M2 | S8, S9, S10 | Một pipeline implement chạy hết trên repo thử cục bộ, tới nhánh tích hợp đã kiểm |
| M4 | GitHub App, Issues, PR, review theo version, merge theo `sha` | M3 | — | Từ nhánh tích hợp ra PR; merge chỉ khi head đúng bản Human đã duyệt |
| M5 | ROLE, playbook, loại session, Done, Kickoff, đổi yêu cầu, nâng cấp | M2, M3, M4 | — | Mọi loại session trong brief 4.3 có workflow và điều kiện Done chạy được |
| M6 | Giao diện quan sát và quyết định | M1, M5 | — | Mọi bề mặt ở spec mục 15 chạy trong app thật, có ảnh chụp |
| M7 | Dịch vụ chấm điểm model, retro, chuẩn bị manual test | M1, M3, M5 | S13 | Human có bản build bật Crew và danh sách việc cần làm trước manual test |

Có thể làm song song: M4 với cuối M3; M6 bắt đầu khi các projection của M5 đã có; T7.2–T7.4 bắt đầu sau M1 vì chỉ cần snapshot model.

## 3. Phụ thuộc giữa các crate

Spec 3.1 chia 6 crate. Kế hoạch chốt thêm chiều phụ thuộc để tránh vòng:

```
zed ──► crew_ui ──► crew ──► crew_acp ──► agent_servers, acp_thread, project
  │                  │  ├──► crew_mcp ──► context_server
  │                  │  ├──► crew_github ──► http_client, credentials_provider, git
  │                  │  └──► crew_models ──► http_client
  └──► crew_mcp (chỉ hàm cầu nối của H1, không cần GPUI)
agent_ui ──► agent_servers   (agent_ui và agent_servers không phụ thuộc crate Crew)
```

Quy tắc:
- **`agent_ui` và `agent_servers` không phụ thuộc crate Crew.** Mọi chỗ core cần “biết” về Crew (H5b, H5d, H5e, H5f, H12) đi qua một registry trong `agent_servers`, do Crew điền lúc khởi tạo. Nhờ vậy điểm nối nhỏ và không kéo crate Crew vào core. `acp_thread` nằm dưới `agent_servers` nên không tra registry được; chỗ nào trong `acp_thread` cần biết session thuộc Crew (H12) thì nhận một cờ do `agent_servers` đặt lúc tạo thread.
- **`crew_acp` không phụ thuộc `crew`.** CrewConnection cần đọc bản ghi session; `crew_acp` khai trait truy cập bản ghi, `crew` hiện thực trait đó trên run store và đăng ký lúc khởi tạo.
- **`crew_mcp` chỉ là khung**: MCP server theo session, đăng ký tool, kiểm schema, cầu nối. Công cụ cụ thể (`crew_spawn_worker`…) nằm trong `crew`, vì cần truy cập lõi.

## 4. Quy trình cho mỗi task

1. Tạo nhánh `crew/<task-id>-<tên-ngắn>` từ `main` của fork, ví dụ `crew/T1.4-session-options`.
2. Đọc `docs/crew/AGENTS.md`, phần task, và các mục spec được dẫn trong task.
3. Viết test trước theo “Tiêu chí nghiệm thu”, xác nhận test fail đúng lý do.
4. Code trong phạm vi “Được sửa”. Cần sửa ngoài phạm vi thì dừng và báo, không tự mở rộng.
5. Chạy mục “Kiểm chứng” của task cùng các lệnh chung ở mục 7.
6. Mở PR vào `main` của fork theo quy tắc PR trong `.rules`. Body PR gồm: ID task, thay đổi, bằng chứng (lệnh và kết quả), phần chưa kiểm được.
7. **Kiểm chứng độc lập:** một agent khác tác giả, context mới, ưu tiên khác provider, nhận phần task và diff. Agent này tự chạy lại kiểm chứng, đối chiếu với tiêu chí nghiệm thu, brief và spec, rồi bắt phần thiếu, phần mâu thuẫn và code không được yêu cầu. Kết quả là `pass` hoặc `fail` kèm bằng chứng (brief 11.1).
8. Merge khi cả hai cổng đạt: kiểm chứng tự động và kiểm chứng độc lập (NEED 03). Human không review code (Human xác nhận 2026-10-25); Human chỉ được hỏi khi task chạm quyết định sản phẩm.

## 5. Định nghĩa Xong cho mọi task

- [ ] Mọi tiêu chí nghiệm thu đạt, có bằng chứng (tên test, lệnh, kết quả).
- [ ] `cargo test` cho các crate bị sửa, `./script/clippy` và `cargo fmt --all -- --check` đạt.
- [ ] Mọi hành vi Crew nằm sau `CrewFeatureFlag`; tắt flag thì Zed chạy như upstream. Ngoại lệ là các sửa lỗi của tầng Zed ở H11.
- [ ] Core chỉ bị sửa ở điểm nối có trong `docs/crew/hook-points.md`. Điểm nối mới phải được thêm vào file này và vào spec trong cùng PR.
- [ ] Check tĩnh NEED 61 đạt (sau T0.6).
- [ ] Không có secret trong code, log, fixture hay body PR; fixture từ provider đã được che dữ liệu.
- [ ] Thay đổi UI: đã chạy app thật và đính ảnh chụp vào PR.
- [ ] Kiểm chứng độc lập: `pass`.

## 6. Spike

- Code spike nằm trên nhánh `spike/<id>-<tên>` và **không bao giờ merge vào `main`** (NEED 03).
- Kết quả là một báo cáo `docs/crew/spikes/<id>.md`, merge qua PR tài liệu: câu hỏi, cách đo, kết quả có nhãn, hệ quả cho task và spec.
- Cùng PR đó cập nhật các task bị ảnh hưởng trong `docs/crew/plan/`. Kết quả làm đổi một quyết định Human đã duyệt thì dừng và hỏi Human trước.
- Spike dùng tài khoản thật thì tốn hạn mức; báo cáo ghi số prompt đã dùng. Không bao giờ đọc hay in nội dung file bí mật thật; khi cần, dùng file giả trong thư mục tạm.

## 7. Lệnh dùng chung

Mọi task chạy trên máy macOS của Human, vì Crew chỉ hỗ trợ macOS (NEED 01) và contract test cần đăng nhập provider thật.

```sh
cargo test -p <crate>                          # test của crate bị sửa
./script/clippy                                 # .rules yêu cầu dùng script này thay cho cargo clippy
cargo fmt --all -- --check
cargo run                                       # chạy app bản debug, cho thay đổi UI
CREW_CONTRACT=1 cargo test -p crew_acp --test contract -- --ignored --test-threads=1
```

Contract test gọi provider thật nên tốn hạn mức; chỉ chạy khi task yêu cầu, khi đổi bản ghim adapter, và trước manual test.

Bật Crew trong bản debug: thêm `"feature_flags": { "crew": "on" }` vào `settings.json` của user. Zed chỉ nhận override này ở bản debug hoặc với tài khoản staff (`crates/feature_flags/src/store.rs:105-107, 171-176`) **[Đã kiểm]**; bản release của Human bật Crew ở T7.8.

## 8. Khi kế hoạch cần đổi

| Nguyên nhân | Cách xử lý |
| ----------- | ---------- |
| Kết quả spike | Như mục 6 |
| Upstream đổi điểm nối | Quy trình đồng bộ (T1.14); cập nhật `docs/crew/hook-points.md`, task và spec trong cùng PR |
| Task lớn hơn dự kiến | Tách task con `T<x>.<y>a`, `T<x>.<y>b`; giữ nguyên tiêu chí nghiệm thu gốc |
| Câu hỏi sản phẩm mới | Hỏi Human; đưa câu trả lời vào thẳng task và spec, không giữ danh sách hỏi đáp riêng |

## 9. Việc Human làm trong kế hoạch

| Lúc | Việc | Task |
| --- | ---- | ---- |
| M0 | Xác nhận repo fork là private | T0.1 |
| M0–M3 | Có mặt khi agent chạy contract test hoặc spike tốn hạn mức, vì cần đăng nhập thật | T0.9, S6–S12 |
| M4 | Tạo GitHub App trên github.com, cài vào repo thử, tải file `.pem` theo hướng dẫn | T4.1 |
| Trước manual test | Đặt giá trị cho brief Q7 (ngưỡng hạn mức, tuổi snapshot, trần song song) dựa trên S10; chọn target repo (Q9) | T7.8 |
| Sau baseline | Đặt ngưỡng chỉ số (Q8) | T7.8 |

## 10. Truy vết NEED → task

Bảng sinh từ cột NEED của danh sách task trong từng file milestone; sửa ở đó trước, rồi cập nhật bảng này.

| NEED | Task | NEED | Task | NEED | Task |
| ---- | ---- | ---- | ---- | ---- | ---- |
| 01 | T2.3 | 22 | T5.5 | 43 | T1.1, T1.10, T5.4, T6.1 |
| 02 | T1.1, T1.2, T1.4, T1.12, T3.6 | 23 | T1.8, T3.10, T5.8 | 44 | T1.3, T2.10, T4.6 |
| 03 | T0.1, T0.3, T0.5, T7.8 | 24 | T0.8, T2.3 | 45 | T5.4 |
| 04 | T5.8, T6.7 | 25 | T1.6 | 46 | T3.10, T3.11 |
| 05 | T4.1, T4.2, T4.3, T6.7 | 26 | T2.8, T3.4, T3.13, T7.6 | 47 | T3.11 |
| 06 | T3.5 | 27 | T1.2, T5.1, T5.3, T5.8, T6.6 | 48 | T3.11 |
| 07 | T2.2, T3.1, T3.5, T3.6, T3.9 | 28 | T0.9, T1.5, T5.7 | 49 | T5.10, T6.1 |
| 08 | T3.15 | 29 | T2.5, T3.13 | 50 | T6.1, T6.2, T6.4 |
| 09 | T2.3 | 30 | T3.2, T3.3, T3.13 | 51 | T4.7, T6.8 |
| 10 | T2.5, T2.6 | 31 | T1.10, T3.7 | 52 | T6.3 |
| 11 | T5.10 | 32 | T0.8, T3.8 | 53 | T2.3, T6.5 |
| 12 | T5.9, T6.3 | 33 | T2.1, T2.5, T3.8 | 54 | T7.5, T7.7 |
| 13 | T2.7, T6.8 | 34 | T3.4, T3.9, T3.14 | 55 | T0.9, T1.4, T1.6, T1.7, T2.4, T2.10, T3.2, T3.4, T5.1, T5.2 |
| 14 | T4.5 | 35 | T3.14, T4.5 | 56 | T0.7, T1.8, T1.9, T1.13, T6.4 |
| 15 | T2.7, T3.13 | 36 | T0.7, T2.6, T5.6 | 57 | T6.8 |
| 16 | T2.9, T3.15 | 37 | T3.12 | 58 | T0.1 |
| 17 | T2.7, T4.1, T6.7 | 38 | T3.12 | 59 | T0.2, T0.4, T1.14 |
| 18 | T2.5, T5.3, T5.4 | 39 | T4.2, T4.4, T5.8 | 60 | T1.10, T1.11, T1.12, T6.4, T7.1, T7.2, T7.3, T7.4, T7.5, T7.6 |
| 19 | T3.10, T5.8 | 40 | T4.4 | 61 | T0.6, T1.6, T2.2 |
| 20 | T5.10 | 41 | T1.1, T4.4 |  |  |
| 21 | T2.5, T5.10 | 42 | T1.9, T4.4, T5.6 |  |  |
