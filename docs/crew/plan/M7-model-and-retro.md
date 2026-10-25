# M7 · Chọn model, retro và bàn giao manual test

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Dịch vụ chấm điểm model theo loại nhiệm vụ thay cho xếp hạng mặc định; session retro biến kết quả đã kiểm chứng thành đề xuất cho Human; chuẩn bị bản build và danh sách việc cho manual test |
| Spec | Mục 18 bước 7; 13; 8 (xếp hạng); 11.2 (Review / retro) |
| Cần có trước | M1 (snapshot), M3 (kết quả worker), M5 (loại session) |
| Spike | S13 (T7.1) |
| Làm sớm được | T7.2–T7.4 chỉ cần snapshot model của M1, nên làm song song với M3–M6 được |

## Danh sách task

| Task | Tên | Phụ thuộc | NEED |
| ---- | --- | --------- | ---- |
| T7.1 | Spike S13: nhóm hạn mức của Gemini 4 Argon | T1.12 | 60 |
| T7.2 | Nạp danh mục models.dev | M1 | 60 |
| T7.3 | Nạp Terminal-Bench 2.1 và Artificial Analysis | T7.2 | 60 |
| T7.4 | Chấm điểm theo loại nhiệm vụ | T7.3 | 60 |
| T7.5 | Eval của Crew theo cặp Model–Harness | T7.4, M3 | 54, 60 |
| T7.6 | Nối `rank()` vào quy trình chọn | T7.4, T2.8 | 26, 60 |
| T7.7 | Session retro | T7.5, T5.4 | 54 |
| T7.8 | Chuẩn bị manual test | Mọi task trước | 03 |

---

### T7.1 · Spike S13: nhóm hạn mức của Gemini 4 Argon

| Mục | Nội dung |
| --- | -------- |
| Câu hỏi | Gemini 4 Argon tính vào nhóm hạn mức nào của Antigravity? |
| Căn cứ | Spec 20.1 (S13), 6.7. `/usage` ghi nhóm “Gemini Models” gồm “Gemini Flash, Gemini Pro”, không nhắc Argon **[Spike]** |
| Ảnh hưởng | Khi chưa có kết quả, hạn mức của Argon là `unknown`. Với chính sách `skip`, Argon luôn bị bỏ qua |
| Bàn giao | `docs/crew/spikes/S13.md`; cập nhật bảng gắn model với nhóm trong ProviderProfile và contract test |

**Cách đo**
1. Đọc `/usage`.
2. Chạy một loạt prompt đủ dài với `gemini-4-argon`, đủ để phần trăm còn lại của một nhóm giảm thấy rõ.
3. Đọc lại `/usage` và so hai nhóm.
4. Lặp lại một lần để loại trùng hợp với việc dùng ở nơi khác.

Báo cáo ghi số prompt và lượng token đã dùng.

**Tiêu chí xong:** xác định được nhóm của Argon, hoặc kết luận không xác định được. Nếu xác định được, contract test khóa mapping theo phiên bản adapter, vì bản sau có thể đổi.

---

### T7.2 · Nạp danh mục models.dev

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Có metadata của model: giá, context, effort, ngày phát hành |
| Căn cứ | NEED 60; spec 13 (Metadata) |
| Được sửa | `crates/crew_models/src/catalog.rs` |
| Bàn giao | Bộ nạp `https://models.dev/api.json`, có cache và phiên bản |

**Thiết kế**
- Nguồn: `https://models.dev/api.json` **[Đã kiểm, README của repo models.dev]**.
- Chỉ tải dữ liệu công khai; không gửi dữ liệu project ra ngoài (NEED 02, 60).
- Giữ cả giá theo ngày và theo tier. Ví dụ: giá giới thiệu của Argon tăng gấp đôi từ 2026-12-01 **[Đã kiểm, dữ liệu models.dev]**.
- Ghép với snapshot ACP theo model id. Model có trong snapshot mà không có trong models.dev thì ghi “chưa có metadata”.

**Tiêu chí nghiệm thu**
- Cho fixture `api.json`, khi nạp, thì có giá, context, effort cho các model trong snapshot.
- Cho mạng lỗi, khi nạp, thì giữ bản cache cũ, gắn `stale`.
- Cho model có giá phụ thuộc ngày, khi tính giá ở một ngày, thì dùng đúng mức giá của ngày đó.

**Kiểm chứng:** `cargo test -p crew_models catalog`

---

### T7.3 · Nạp Terminal-Bench 2.1 và Artificial Analysis

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Có điểm benchmark công khai, ưu tiên benchmark đo cặp model và harness |
| Căn cứ | NEED 60; spec 13 (Benchmark) |
| Được sửa | `crates/crew_models/src/benchmarks.rs` |
| Bàn giao | Bộ nạp cho từng nguồn; bảng `crew_scores` |

**Nguồn**
- **Terminal-Bench 2.1:** repo `github.com/harbor-framework/terminal-bench-2-1`, file `leaderboard/submissions/*.json`, Apache-2.0 **[Đã kiểm qua repo clone]**. Mỗi dòng có agent và phiên bản agent, model, effort, accuracy ± stderr, cost, thời gian.
- **models.dev `[[benchmarks]]`:** có `harness`, `version`, `source`, `date`. Phần lớn do vendor tự báo, nên gắn `self_reported` **[Đã kiểm]**.
- **Artificial Analysis** (tùy chọn): API dùng header `x-api-key`; key nằm trong Keychain. Điều khoản bản miễn phí yêu cầu ghi nguồn **[Suy luận: mới thấy qua kết quả tìm kiếm, chưa đọc trang gốc]**, nên mọi chỗ hiện điểm từ nguồn này phải ghi nguồn.

**Quy tắc lưu:** không gộp điểm khác phiên bản benchmark hay khác harness. Lưu `harness_version` và `benchmark_version`. Ví dụ: models.dev gom Terminal-Bench 2.0, 2.1 và 4.0 chung một tên, nên phải tách theo `version` **[Đã kiểm]**.

**Tiêu chí nghiệm thu**
- Cho fixture submission của Terminal-Bench 2.1, khi nạp, thì mỗi dòng thành một điểm theo cặp (agent, model, effort) kèm stderr.
- Cho hai điểm cùng model nhưng khác phiên bản harness, khi lưu, thì là hai bản ghi riêng.
- Cho điểm vendor tự báo, khi lưu, thì có `self_reported = true`.
- Cho không có key Artificial Analysis, khi nạp, thì bỏ qua nguồn này và ghi lý do.

**Kiểm chứng:** `cargo test -p crew_models benchmarks`

---

### T7.4 · Chấm điểm theo loại nhiệm vụ

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Mỗi cặp (provider, model, effort) có điểm theo loại nhiệm vụ, kèm căn cứ |
| Căn cứ | NEED 60; spec 13 (Cách chấm, Quy tắc) |
| Được sửa | `crates/crew_models/src/scoring.rs` |
| Bàn giao | Hàm `rank(task_kind, constraints)` trả danh sách xếp hạng kèm căn cứ |

**Thiết kế**
- Loại nhiệm vụ lấy từ playbook: orchestrate, architecture, implement, review, debug, research, ui, test, docs.
- Điểm công khai chỉ là giá trị ban đầu; kết quả của chính Crew (T7.5) điều chỉnh dần.
- Benchmark đo cặp model + harness được ưu tiên hơn benchmark đo model đơn lẻ. Điểm vendor tự báo bị giảm trọng số.
- Thiếu dữ liệu thì ghi “chưa đánh giá”, không coi là 0 và không coi là tốt nhất.
- Xếp hạng gồm cả chi phí, tốc độ và mức cách ly của provider.
- Không có benchmark công khai cho harness Antigravity, nên điểm của ứng viên Antigravity chủ yếu đến từ T7.5.

**Tiêu chí nghiệm thu**
- Cho một ứng viên không có điểm nào, khi xếp hạng, thì ứng viên có nhãn “chưa đánh giá” và vị trí theo quy tắc đã ghi, không theo điểm 0.
- Cho hai ứng viên cùng model nhưng khác harness, khi xếp hạng, thì là hai ứng viên riêng.
- Cho mỗi ứng viên trong kết quả, khi đọc, thì có căn cứ: nguồn, phiên bản, ngày, mức tin cậy.

**Kiểm chứng:** `cargo test -p crew_models scoring`

---

### T7.5 · Eval của Crew theo cặp Model–Harness

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Điểm phản ánh kết quả thật trên việc của Human |
| Căn cứ | NEED 54, 60; spec 13 (Cách chấm) |
| Được sửa | `crates/crew_models/src/crew_eval.rs`; đọc dữ liệu từ run store của `crew` qua một trait, giữ đúng chiều phụ thuộc |
| Bàn giao | Chỉ số theo cặp Model–Harness tính từ run store |

**Chỉ số:** tỷ lệ bàn giao được nhận ở lần đầu, số lần phải làm lại, chi phí, thời gian. Tách theo loại nhiệm vụ, provider, model, effort, phiên bản adapter.

**Tiêu chí nghiệm thu**
- Cho các bản ghi spawn và handoff trong run store, khi tính, thì chỉ số khớp với tính tay trên fixture.
- Cho hai phiên bản adapter khác nhau, khi tính, thì chỉ số tách riêng.

**Kiểm chứng:** `cargo test -p crew_models crew_eval`

---

### T7.6 · Nối `rank()` vào quy trình chọn

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Quy trình chọn dùng xếp hạng của dịch vụ model thay cho danh sách mặc định |
| Căn cứ | NEED 26, 60; spec 8 |
| Được sửa | `crates/crew/src/selection.rs` |
| Bàn giao | `select` gọi `rank`; `default_candidates` chỉ còn là dự phòng |

**Thiết kế:** `rank` không có dữ liệu cho loại nhiệm vụ thì dùng `default_candidates` (T2.8). Điểm không tự đổi tập được phép và không đổi provider của worker đang chạy (NEED 60).

**Tiêu chí nghiệm thu**
- Cho dịch vụ model có dữ liệu, khi chọn, thì bản ghi spawn có danh sách xếp hạng kèm căn cứ từ `rank`.
- Cho dịch vụ model chưa có dữ liệu cho loại nhiệm vụ, khi chọn, thì dùng `default_candidates` và ghi rõ nguồn xếp hạng.
- Các test của T2.8 vẫn đạt.

**Kiểm chứng:** `cargo test -p crew selection`

---

### T7.7 · Session retro

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Cải tiến dựa trên kết quả đã kiểm chứng, và mọi thay đổi đều do Human quyết |
| Căn cứ | NEED 54; brief 4.3 (Review / retro); seed mattpocock `retro` |
| Được sửa | `crates/crew/assets/playbooks/retro.md`; ROLE liên quan |
| Bàn giao | Workflow retro |

**Thiết kế**
- Đề xuất cải tiến phải dẫn bằng chứng từ run store: sensor fail lặp lại, phải làm lại nhiều, lựa chọn model kém.
- Đề xuất có thể là một sensor tất định mới, một chỉnh sửa playbook, hoặc một thay đổi trong tập được phép.
- Không tự áp dụng: Human quyết từng đề xuất; đề xuất được chấp nhận mở thành session hoặc story mới (NEED 54). Không dùng cơ chế tự tiến hóa (brief 5.3.1, danh sách “không dùng”).

**Tiêu chí nghiệm thu**
- Cho một đề xuất không có bằng chứng, khi Orchestrator ghi, thì bị từ chối.
- Cho retro còn đề xuất chưa có quyết định của Human, khi đề nghị Done, thì bị từ chối (T5.4).
- Cho đề xuất được chấp nhận, khi Done, thì đã có session hoặc story mới tương ứng.

**Kiểm chứng:** `cargo test -p crew retro`

---

### T7.8 · Chuẩn bị manual test

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Human có bản build bật Crew và danh sách việc để bắt đầu manual test |
| Căn cứ | NEED 03; brief 9 (Bàn giao), Q7–Q9 |
| Được sửa | `crates/feature_flags/src/flags.rs` (`CrewFeatureFlag::enabled_for_all`); `docs/crew/manual-test.md` (mới) |
| Bàn giao | Bản release của fork bật Crew; checklist manual test |

**Các bước**
1. Chạy toàn bộ contract test ở bản ghim hiện tại và ghi kết quả.
2. Human đặt các giá trị Q7, dựa trên số đo S10 và kết quả S13:
   - ngưỡng “sắp hết hạn mức”;
   - tuổi tối đa của snapshot model;
   - trần song song, kể cả trần riêng cho Antigravity.
3. Human chọn target repo cho lần manual test đầu tiên (Q9). Repo này phải có rủi ro thấp (brief 9).
4. Đổi `CrewFeatureFlag::enabled_for_all()` thành `true`. Bản release không nhận override từ settings (T0.5), nên đây là cách bật cho Human. Rollback là đổi lại thành `false`.
5. Viết `docs/crew/manual-test.md`: thứ tự loại session nên thử, cách đọc Tổng quan, cách dừng session, cách gỡ Crew khỏi project (brief 9, Rollback).
6. Q8 (ngưỡng chỉ số) được đặt sau khi có baseline từ các session thử.

**Tiêu chí nghiệm thu**
- Cho bản release mới build, khi mở Command Palette, thì có các action Crew.
- Cho contract test, khi chạy toàn bộ, thì đạt, hoặc từng lỗi có lý do và đã được Human chấp nhận.
- Cho checklist, khi Human đọc, thì làm theo được mà không cần hỏi thêm.

**Kiểm chứng:** build bản release (`cargo build --release`), mở app và chụp Command Palette có action Crew; báo cáo contract test đính kèm PR.

---

## Điều kiện xong M7

- [ ] T7.1–T7.8 đạt theo Định nghĩa Xong chung.
- [ ] Quy trình chọn dùng `rank()` và bản ghi spawn ghi đủ căn cứ.
- [ ] Human đã đặt Q7, chọn Q9, và có bản build bật Crew.
