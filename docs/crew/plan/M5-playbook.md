# M5 · Playbook và pipeline

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Mọi loại session trong brief 4.3 có workflow và điều kiện Done chạy được: danh mục ROLE theo scrum team, playbook theo loại session, DoneEvaluator, đối soát trước khi implement, đổi yêu cầu giữa chừng, ghim phiên bản khi nâng cấp, Kickoff, prototype, bản đồ quyết định và tham chiếu |
| Spec | Mục 18 bước 5; 5.4, 9.1, 9.7, 11.2, 11.3, 14.2 |
| Cần có trước | M2, M3, M4 |
| Spike | — |
| Nguồn nội dung | Bản đồ seed ở brief 5.3.1. Nguyên tắc: lấy nội dung, không lấy cơ chế. Hook, slash command và tool riêng của Claude Code không chạy trên Codex hay Antigravity, nên Crew dựng lại các cổng kiểm thành sensor tất định của mình |
| Bản nháp | [`docs/crew/seed/`](../seed/index.html) (2026-10-25): 16 ROLE và 10 playbook viết sẵn từ bản đồ seed, kèm danh sách khoảng trống của spec. T5.2, T5.3 và T5.5 lấy làm điểm xuất phát, chuyển vào `crates/crew/assets/` rồi kiểm chứng theo tiêu chí của từng task |

## Danh sách task

| Task | Tên | Phụ thuộc | NEED |
| ---- | --- | --------- | ---- |
| T5.1 | Định dạng và bộ nạp ROLE, playbook | M2 | 27, 55 |
| T5.2 | Danh mục ROLE theo scrum team | T5.1 | 55 |
| T5.3 | Playbook và workflow theo loại session | T5.1 | 18, 27 |
| T5.4 | Loại session và DoneEvaluator | T5.3, T4.5 | 18, 43, 45 |
| T5.5 | Đối soát chỉ đọc trước khi implement | T5.3, T3.11 | 22 |
| T5.6 | Đổi yêu cầu giữa chừng (H3b) | T5.3, S6 | 36, 42 |
| T5.7 | Nâng cấp không làm mất việc đang chạy | M1 | 28 |
| T5.8 | Kickoff | T5.3, T4.2, T3.10 | 04, 19, 23, 27, 39 |
| T5.9 | Prototype | T5.3 | 12 |
| T5.10 | Bản đồ quyết định, bản đồ tham chiếu, kế hoạch, truy vết | T5.3 | 11, 20, 21, 49 |

---

### T5.1 · Định dạng và bộ nạp ROLE, playbook

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | ROLE và playbook là file có phiên bản ở user-scope; mỗi lần dùng đều truy được nội dung |
| Căn cứ | NEED 27, 55; spec 5.4 |
| Được sửa | `crates/crew/src/roles.rs`, `crates/crew/src/playbooks.rs` (mở rộng bản tối thiểu của T2.4) |
| Bàn giao | Bộ nạp có kiểm schema; bộ cài seed; lưu nội dung và hash theo session |

**Thiết kế**
- Front matter ROLE theo spec 5.4: `id`, `version`, `task_kinds`, `allowed`, `tools_deny`, `handoff_schema`, `seed`.
- `allowed` chỉ được thu hẹp tập được phép ở user-scope; rộng hơn thì bị cắt về phần giao, kèm cảnh báo.
- Seed đóng gói trong crate. Lần đầu bật Crew thì chép vào `crew/roles/` và `crew/playbooks/` của user. Lần sau chỉ thêm file mới, không ghi đè file Human đã sửa, và ghi phiên bản seed.
- Mỗi lần dùng, Crew lưu nội dung và hash vào run store (NEED 28, 49).
- Nạp ROLE không tự cấp quyền cho tool trong profile (NEED 55).

**Tiêu chí nghiệm thu**
- Cho ROLE thiếu trường bắt buộc, khi nạp, thì lỗi chỉ ra trường thiếu.
- Cho ROLE có `allowed` rộng hơn user-scope, khi nạp, thì tập hiệu lực là phần giao, kèm cảnh báo.
- Cho Human đã sửa một ROLE seed, khi seed mới ra phiên bản khác, thì bản của Human giữ nguyên và Crew báo có bản seed mới.
- Cho một session đã dùng ROLE, khi đọc bản ghi spawn, thì có hash trùng nội dung đã dùng.

**Kiểm chứng:** `cargo test -p crew -- roles playbooks`

---

### T5.2 · Danh mục ROLE theo scrum team

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Có đủ ROLE cho các vai trò SDLC của brief NEED 55, khởi tạo từ bản đồ seed |
| Căn cứ | NEED 55; brief 5.3.1; rủi ro R8 của brief |
| Được sửa | `crates/crew/assets/roles/*.md` |
| Bàn giao | Danh mục ROLE |

**Danh mục và nguồn seed** (brief 5.3.1)

| ROLE | Nguồn |
| ---- | ----- |
| Business Analyst | Viết mới |
| Product Manager, Backlog Analyst (PO) | BMAD (PM, PO) |
| UX Researcher/Designer, UI Designer | BMAD (UX) |
| Software Engineer | BMAD (Dev); compound-engineering `implementation-worker` cho hợp đồng bàn giao |
| Software/Solution Architect | BMAD (Architect) |
| Tech Lead | GSD |
| Project/Delivery Manager | Viết mới |
| Release Manager | Viết mới |
| Platform/DevOps Engineer | Viết mới |
| SRE | wshobson `incident-response`, `postmortem-writing` |
| QA/Test Engineer | GSD (QA); compound-engineering `testing-reviewer` |
| Security Engineer | GSD (Security, `SECURITY.md`) |
| Evaluator (độc lập) | compound-engineering `ce-code-review`, `testing-reviewer` |
| Orchestrator | Bản của T2.4, bổ sung theo playbook |

**Quy tắc khi chuyển nội dung**
- Bỏ cơ chế trong danh sách “không dùng” của brief 5.3.1:
  - tự tiến hóa, tự duyệt, tự nâng model;
  - push thẳng `main`, bỏ qua PR;
  - duyệt bằng file cờ;
  - worker ACP tự duyệt mọi quyền.
- Mỗi ROLE ghi nguồn seed và phần đã bỏ trong front matter `seed`.
- Mỗi ROLE nêu chuẩn đầu ra và quy ước bàn giao theo `crew.handoff/1`.

**Tiêu chí nghiệm thu**
- Cho danh mục, khi nạp, thì mọi ROLE hợp lệ theo T5.1.
- Agent kiểm chứng độc lập đối chiếu từng ROLE với nguồn seed và danh sách “không dùng”. Không ROLE nào chứa chỉ dẫn tự duyệt, tự merge hay bỏ qua PR.

**Kiểm chứng:** `cargo test -p crew roles`; kiểm chứng độc lập theo checklist trên.

---

### T5.3 · Playbook và workflow theo loại session

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Mỗi loại session có workflow; Orchestrator dựng pipeline từ playbook và điều chỉnh có phiên bản |
| Căn cứ | NEED 18, 27; brief 4.3, 5.5; spec 9.1 |
| Được sửa | `crates/crew/assets/playbooks/*.md`, `crates/crew/src/playbooks.rs` |
| Bàn giao | Playbook cho Hỏi nhanh, Kickoff, Brainstorm, Research, Refine, Implement, Release / migration, Incident, Review / retro, Retirement |

**Thiết kế**
- Mỗi workflow gồm bước, ROLE cho từng bước, artifact đầu vào và đầu ra, sensor, điểm cần Human quyết.
- Workflow implement có bước đối soát chỉ đọc trước khi implement (T5.5).
- Release / migration, Retirement và Hỏi nhanh không có workflow nào trong 13 repo seed (brief 5.3.1), nên viết mới.
- Đo cỡ trước khi chạy (NEED 18): việc nhỏ đi đường nhanh với ít bước hơn, và vẫn có tiêu chí Done.

**Tiêu chí nghiệm thu**
- Cho mỗi loại session, khi nạp playbook, thì có workflow hợp lệ, mọi ROLE tham chiếu đều có trong danh mục.
- Cho việc nhỏ kiểu Hỏi nhanh, khi Orchestrator dựng pipeline, thì pipeline chỉ có các bước của đường nhanh.

**Kiểm chứng:** `cargo test -p crew playbooks`

---

### T5.4 · Loại session và DoneEvaluator

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Chỉ DoneEvaluator kết luận Done, theo điều kiện của từng loại session |
| Căn cứ | NEED 18, 43, 45; spec 11.2; brief 4.3 |
| Được sửa | `crates/crew/src/done.rs`; công cụ `crew_propose_done` |
| Bàn giao | DoneEvaluator với điều kiện của 11 dòng trong spec 11.2 |

**Thiết kế**
- `crew_propose_done` chỉ là đề nghị; DoneEvaluator kiểm từng điều kiện bằng dữ liệu trong run store, git và GitHub.
- Implement chỉ Done khi có sự kiện “PR merged” kèm `merge_commit_sha` (T4.5). PR mở hay CI xanh chưa đủ.
- Tiêu chí mặc định FAIL: tiêu chí `passed` phải có `evidence_ref` (NEED 43).
- Done thì khóa bản ghi kết quả (T1.1, NEED 41).

**Tiêu chí nghiệm thu** (mỗi loại session một test)
- Cho session Implement có PR đã mở và CI xanh nhưng chưa merged, khi đề nghị Done, thì bị từ chối kèm điều kiện còn thiếu.
- Cho session Research thiếu mức chắc chắn cho một nhận định quan trọng, khi đề nghị Done, thì bị từ chối.
- Cho session Review / retro còn một đề xuất chưa có quyết định của Human, khi đề nghị Done, thì bị từ chối.
- Cho agent viết “đã xong” trong tin nhắn, khi không gọi `crew_propose_done`, thì trạng thái không đổi.

**Kiểm chứng:** `cargo test -p crew done`

---

### T5.5 · Đối soát chỉ đọc trước khi implement

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Implement không phải bù cho đặc tả hay kế hoạch lệch nhau |
| Căn cứ | NEED 22; spec 9.1; seed spec-kit `analyze.md`, GSD `gsd-plan-checker` |
| Được sửa | `crates/crew/assets/playbooks/implement.md`; ROLE đối soát; bước kiểm trong `crates/crew/src/pipeline.rs` |
| Bàn giao | Bước đối soát chặn task implement |

**Thiết kế:** một worker chỉ đọc đối chiếu đặc tả ↔ kế hoạch ↔ nhiệm vụ và trả danh sách lệch có dẫn chứng. Còn lệch chưa xử lý thì task implement không chuyển `ready`.

**Tiêu chí nghiệm thu**
- Cho story có tiêu chí không có task nào phủ, khi đối soát, thì có lệch và task implement vẫn `pending`.
- Cho đối soát đạt, khi kiểm, thì task implement chuyển `ready`.
- Cho worker đối soát, khi chạy, thì gate từ chối mọi yêu cầu sửa file của nó, và H12 từ chối mọi `fs/write_text_file` của nó (T3.5).

**Kiểm chứng:** `cargo test -p crew pipeline`

---

### T5.6 · Đổi yêu cầu giữa chừng (H3b)

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Human đổi yêu cầu khi đang chạy; Crew biết phần nào hết hiệu lực và báo worker đúng cách theo provider |
| Căn cứ | NEED 36, 42; spec 3.2 (H3b), 9.7; báo cáo S6 |
| Được sửa | `crates/agent_servers/src/acp/v1.rs` (H3b, gọi `_session/steering`); `crates/crew/src/change_requests.rs`; công cụ `crew_steer_worker` |
| Bàn giao | Phiên bản INPUT mới; lan truyền `stale`; steering hoặc cancel rồi gửi lại |

**Thiết kế**
- Tin nhắn đổi yêu cầu tạo phiên bản INPUT mới (T2.6). Artifact và task dẫn xuất từ bản cũ thành `stale`.
- Báo worker đang chạy:
  - Claude, Codex: gửi `_session/steering`, theo API ở báo cáo S6. Cả hai adapter đều khai method này **[Đã kiểm]**.
  - Antigravity: không có steering; cancel lượt rồi gửi lại prompt có yêu cầu mới.
- Orchestrator trình Human phần bị ảnh hưởng trước khi chạy tiếp.

**Tiêu chí nghiệm thu**
- Cho task B dẫn xuất từ artifact A, khi INPUT của A đổi, thì B thành `stale`.
- Cho worker Claude đang chạy, khi Orchestrator gọi `crew_steer_worker`, thì agent giả nhận `_session/steering` với nội dung mới.
- Cho worker Antigravity đang chạy, khi steering, thì lượt bị cancel và có prompt mới.

**Kiểm chứng:** `cargo test -p crew change_requests`, `cargo test -p agent_servers`

---

### T5.7 · Nâng cấp không làm mất việc đang chạy

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Nâng cấp Crew hoặc adapter không làm hỏng session đang chạy |
| Căn cứ | NEED 28; spec 14.2 |
| Được sửa | `crates/crew/src/session.rs`, `crates/crew/src/upgrade.rs`; `docs/crew/upgrade-adapters.md` (mới) |
| Bàn giao | Ghim phiên bản theo session; quy trình đổi bản ghim adapter |

**Thiết kế**
- Session ghim: phiên bản Crew, phiên bản schema, hash cấu hình, hash playbook và ROLE, phiên bản adapter.
- Migration schema chỉ thêm mới (T1.1). Session không tương thích với bản Crew mới thì dừng, kèm lý do, không chạy tiếp với cấu hình đoán.
- Đổi bản ghim adapter:
  1. chạy toàn bộ contract test với bản mới;
  2. đo lại ngưỡng context của Antigravity và `$TMPDIR` của sandbox Claude;
  3. Human đồng ý;
  4. bản mới chỉ áp cho session mới.

**Tiêu chí nghiệm thu**
- Cho session đang chạy với adapter 0.95.0, khi bản ghim đổi sang bản mới, thì session đó vẫn dùng 0.95.0; session mới dùng bản mới.
- Cho session tạo bởi schema cũ không tương thích, khi mở bằng bản Crew mới, thì session dừng kèm lý do.

**Kiểm chứng:** `cargo test -p crew upgrade`

---

### T5.8 · Kickoff

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Session Kickoff đầu tiên dựng nền Outer Harness cho target repo |
| Căn cứ | NEED 04, 19, 23, 27, 39; brief 4.3 (Kickoff); spec 5.1, 5.2 |
| Được sửa | `crates/crew/assets/playbooks/kickoff.md`; `crates/crew/src/kickoff.rs` |
| Bàn giao | Workflow Kickoff chạy được |

**Thiết kế**
- Khảo sát repo: cách build, test, lint, chạy app; quy ước có sẵn (`AGENTS.md`, `CLAUDE.md`).
- Đề xuất khóa `crew` cho `.zed/settings.json`:
  - bước verification và baseline;
  - quyền môi trường;
  - repo GitHub và loại chủ sở hữu;
  - allowlist mạng trong trần user-scope;
  - `artifacts_path`.
- Human xác nhận từng giả định trong hội thoại (T2.6).
- File thêm vào repo đi qua PR đã merged (T4.5).
- Done theo brief 4.3: baseline check chạy lại từ worktree sạch cho cùng kết quả (T3.10).

**Tiêu chí nghiệm thu**
- Cho repo thử chưa có cấu hình Crew, khi chạy Kickoff, thì có PR thêm khóa `crew` vào `.zed/settings.json`, và mọi giả định đã có quyết định của Human.
- Cho PR Kickoff chưa merged, khi đề nghị Done, thì bị từ chối.
- Cho baseline cho hai kết quả khác nhau giữa hai lần chạy, khi đề nghị Done, thì bị từ chối kèm hai kết quả.

**Kiểm chứng:** `cargo test -p crew kickoff`; chạy thử trên repo thử của Human.

---

### T5.9 · Prototype

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Dùng prototype dễ chạy để chốt một câu hỏi thiết kế, không để nó thành code sản phẩm |
| Căn cứ | NEED 12; spec 9.5 (Prototype) |
| Được sửa | `crates/crew/src/prototype.rs`; luật gate trong `crates/crew/src/gate.rs` |
| Bàn giao | Workflow prototype; gate chặn PR từ nhánh prototype |

**Thiết kế:** prototype nằm trên nhánh `crew/prototype/*`. Gate chặn mọi yêu cầu mở PR từ nhánh này. Crew ghi lại câu hỏi, kết luận, giới hạn và nguồn thử trước khi áp vào sản phẩm.

**Tiêu chí nghiệm thu**
- Cho nhánh `crew/prototype/x`, khi có yêu cầu mở PR từ nhánh đó, thì bị từ chối.
- Cho prototype xong, khi đọc artifact, thì có câu hỏi, kết luận, giới hạn và nguồn thử.

**Kiểm chứng:** `cargo test -p crew prototype`

---

### T5.10 · Bản đồ quyết định, bản đồ tham chiếu, kế hoạch, truy vết

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Việc lớn được làm rõ bằng bản đồ quyết định; tham chiếu gắn với capability thật; kết quả truy được tới yêu cầu và bằng chứng |
| Căn cứ | NEED 11, 20, 21, 49; spec 11.3; seed mattpocock `grilling`, `wayfinder`; OpenSpec cho delta spec |
| Được sửa | `crates/crew/src/artifacts.rs`; công cụ `crew_write_artifact` |
| Bàn giao | Schema cho các artifact; kiểm `derived_from`; truy vết bằng chứng |

**Thiết kế**
- **Bản đồ quyết định:** câu hỏi, phương án, khuyến nghị, quyết định đã chốt (NEED 11).
- **Bản đồ tham chiếu ↔ capability:** mỗi tham chiếu sản phẩm gắn với một capability thật của hệ thống (NEED 20).
- **Kế hoạch có đầu ra kiểm được:** trên target repo, mỗi phần bàn giao là một lát dọc dùng được (NEED 21).
- **Truy vết (spec 11.3):** mỗi bằng chứng gắn phiên bản INPUT, commit, cặp Model–Harness và phần chưa chứng minh được.
- Mọi artifact phải có `derived_from`; thiếu thì `crew_write_artifact` từ chối.

**Tiêu chí nghiệm thu**
- Cho artifact không có `derived_from`, khi ghi, thì bị từ chối.
- Cho một bằng chứng, khi đọc, thì có phiên bản INPUT, commit, provider, model, effort, phiên bản adapter, hash ROLE và phần chưa chứng minh.
- Cho kế hoạch implement trên target repo, khi kiểm, thì mỗi phần bàn giao có đầu ra kiểm được và chạy được độc lập.

**Kiểm chứng:** `cargo test -p crew artifacts`

---

## Điều kiện xong M5

- [ ] T5.1–T5.10 đạt theo Định nghĩa Xong chung.
- [ ] Trên repo thử, chạy một session cho mỗi loại trong brief 4.3; mỗi session hoặc Done theo đúng điều kiện, hoặc bị từ chối Done với lý do đúng.
- [ ] Kickoff chạy được trên repo thử của Human.
