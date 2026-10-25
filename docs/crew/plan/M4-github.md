# M4 · GitHub

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Crew kết nối GitHub bằng GitHub App riêng: token theo đúng một repo, Issues làm nguồn sự thật, Crew là bên ghi tự động duy nhất, PR từ nhánh tích hợp, duyệt gắn với version đã xem, merge theo `sha` |
| Spec | Mục 18 bước 4; 12; 14.1 bước 2 |
| Cần có trước | M3 (nhánh tích hợp ở T3.14, commit ở T3.9) |
| Spike | — |
| Việc của Human | Tạo GitHub App trên github.com, cài vào repo thử, tải file `.pem` (T4.1). Crew không làm thay được |

## Danh sách task

| Task | Tên | Phụ thuộc | NEED |
| ---- | --- | --------- | ---- |
| T4.1 | Hướng dẫn tạo App, client REST, JWT và token installation | M3 | 05, 17 |
| T4.2 | Nhập private key và kiểm tra sẵn sàng | T4.1 | 05, 39 |
| T4.3 | Push qua askpass bằng token installation | T4.1 | 05 |
| T4.4 | Issues làm nguồn sự thật | T4.1 | 39, 40, 41, 42 |
| T4.5 | PR, duyệt theo version, merge theo `sha` | T4.3, T3.14 | 14, 35 |
| T4.6 | Đối soát effect GitHub | T4.4, T4.5, T1.3 | 44 |
| T4.7 | Artifact dạng file qua PR | T4.5 | 51 |

---

### T4.1 · Hướng dẫn tạo App, client REST, JWT và token installation

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Human tạo được App theo hướng dẫn; Crew lấy được token installation tối thiểu cho đúng một repo |
| Căn cứ | NEED 05, 17; spec 12 (JWT, Token installation) |
| Được sửa | `crates/crew_github/src/{client,auth}.rs`; `docs/crew/github-app.md` (mới) |
| Bàn giao | Hướng dẫn cho Human; client REST; token installation có cache |

**Thiết kế**
- **Hướng dẫn** theo mẫu bước của Human (T2.7): quyền tối thiểu (contents, issues, pull requests, checks; `workflows` chỉ khi cần), cách cài vào repo thử, cách tải `.pem`, và Crew sẽ kiểm gì sau đó (T4.2).
- **Client REST** tự viết trên `http_client` của Zed, không dùng `octocrab`. `octocrab` dựa vào tokio **[Nhớ]**; quyết định đã duyệt ở spec 12.
- **JWT RS256** bằng `jsonwebtoken`. Workspace khai `jsonwebtoken = { version = "10.0", default-features = false }` và hiện chưa crate nào dùng **[Đã kiểm]**. Bản 10 cần bật một backend crypto để ký RS256 **[Nhớ]**; chọn feature trong `crew_github/Cargo.toml` và ghi lý do.
- **Token installation** xin cho đúng một repo (`repositories: [<tên>]`) với quyền tối thiểu. Cache tới gần lúc hết hạn; token hết hạn sau khoảng 1 giờ **[Nhớ]**. Không bao giờ log token hay header `Authorization`.

**Tiêu chí nghiệm thu** (HTTP giả: `http_client::FakeHttpClient::create`, như `crates/context_server/src/oauth.rs:1960` **[Đã kiểm]**)
- Cho App id và private key giả, khi tạo JWT, thì header là RS256 và payload có `iss`, `iat`, `exp` hợp lệ.
- Cho yêu cầu token, khi gửi, thì body chỉ xin đúng một repo và đúng các quyền đã khai.
- Cho token còn hạn, khi gọi lại, thì dùng cache, không xin token mới.
- Cho log của client, khi chạy test, thì không chứa token hay header `Authorization`.

**Kiểm chứng:** `cargo test -p crew_github`

---

### T4.2 · Nhập private key và kiểm tra sẵn sàng

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Private key đi thẳng vào Keychain; Crew kiểm App đã sẵn sàng trước khi dùng |
| Căn cứ | NEED 05, 39; spec 12 (Private key của App, Kiểm tra onboarding); brief 4.3 (Onboarding) |
| Được sửa | `crates/crew_github/src/onboarding.rs`; action tối thiểu trong `crates/crew_ui`. Trang onboarding hoàn chỉnh ở T6.7 |
| Bàn giao | Nhập key; bộ kiểm sẵn sàng; danh tính bot của App cho commit (T3.9) |

**Thiết kế**
- Human chọn file `.pem` bằng hộp chọn file. Crew đọc file và ghi thẳng vào Keychain qua một `CredentialsProvider` (`crates/credentials_provider/src/credentials_provider.rs:11-34`) **[Đã kiểm]**. Key không đi qua hội thoại, log hay run store (NEED 05, 17).
- **Luôn là Keychain, bất kể kênh phát hành.** Không dùng `zed_credentials_provider::global(cx)`: trên kênh `dev` hàm này trả `DevelopmentCredentialsProvider`, lưu credential vào file JSON thường `development_credentials` trong thư mục config, trừ khi có `ZED_DEVELOPMENT_USE_KEYCHAIN` (`crates/zed_credentials_provider/src/zed_credentials_provider.rs:46-62, 116`), và `crates/zed/RELEASE_CHANNEL` của fork là `dev` **[Đã kiểm]**. Provider của Crew gọi API credentials của GPUI (`cx.write_credentials`), như `KeychainCredentialsProvider` làm (`:69`). Không ghi được vào Keychain thì từ chối lưu và báo Human.
- Kiểm sẵn sàng:
  - danh tính App;
  - App đã cài vào repo;
  - quyền đủ;
  - tạo thử được token;
  - loại chủ sở hữu repo (tổ chức hay cá nhân), dùng cho hierarchy ở T4.4;
  - ruleset có “dismiss stale approvals” và “require approval of most recent push”.
- Lấy danh tính bot của App để T3.9 dùng làm author của commit.

**Tiêu chí nghiệm thu**
- Cho file `.pem` hợp lệ, khi nhập, thì Keychain có key, còn run store và log không có.
- Cho kênh phát hành `dev` và không có `ZED_DEVELOPMENT_USE_KEYCHAIN`, khi nhập key, thì key nằm trong credential store của nền tảng (Keychain) và file `development_credentials` không chứa key.
- Cho Keychain trả lỗi khi ghi, khi nhập key, thì Crew từ chối lưu, báo Human, và key không nằm trong file nào.
- Cho App chưa cài vào repo, khi kiểm sẵn sàng, thì báo đúng bước thiếu, kèm hướng dẫn của Human.
- Cho ruleset thiếu “require approval of most recent push”, khi kiểm, thì báo thiếu và chưa coi là sẵn sàng.
- Cho repo của tổ chức hoặc cá nhân, khi kiểm, thì loại chủ sở hữu được ghi đúng.

**Kiểm chứng:** `cargo test -p crew_github onboarding` (Keychain giả); chạy app, chụp bước nhập key.

---

### T4.3 · Push qua askpass bằng token installation

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Crew push bằng danh tính App, không bao giờ bằng credential của Human |
| Căn cứ | NEED 05; spec 12 (Push) |
| Được sửa | `crates/crew_github/src/push.rs` |
| Bàn giao | Push nhánh tích hợp |

**Thiết kế**
- Dùng crate `git` và askpass của Zed. Git truyền socket askpass qua `ZED_ASKPASS_SOCKET` (`crates/git/src/repository.rs:4082`) **[Đã kiểm]**. Với lệnh push của Crew, delegate askpass trả `x-access-token` và token installation.
- Chạy git với `-c credential.helper=` để bỏ qua credential helper của user. Như vậy git không lấy credential của Human, và không lưu token vào Keychain **[Suy luận: kiểm bằng test]**.
- Push là một effect (T1.3): ghi `intent` trước khi push.

**Tiêu chí nghiệm thu** (repo bare cục bộ, askpass giả)
- Cho push của Crew, khi git hỏi credential, thì nhận đúng token installation.
- Cho user có credential helper lưu sẵn credential khác, khi Crew push, thì git không dùng credential đó.
- Cho log và output của git, khi push xong, thì không chứa token.

**Kiểm chứng:** `cargo test -p crew_github push`

---

### T4.4 · Issues làm nguồn sự thật

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Spec và ticket nằm trong Issues; Crew là bên ghi tự động duy nhất; sửa tay của Human được phát hiện và tôn trọng |
| Căn cứ | NEED 39, 40, 41, 42; spec 12 (Issues, Phát hiện lệch) |
| Được sửa | `crates/crew_github/src/issues.rs`, `crates/crew/src/artifacts.rs` |
| Bàn giao | Tạo, cập nhật issue và sub-issue; marker và hash; phát hiện lệch |

**Thiết kế**
- **Hierarchy theo loại chủ sở hữu (T4.2):** repo của tổ chức dùng issue types cùng sub-issues; repo cá nhân dùng label cùng sub-issues.
- **Marker** `<!-- crew:artifact:<id> v<n> -->` cùng hash nội dung trong mỗi issue Crew tạo.
- **Phát hiện lệch:** poll bằng conditional request (ETag) **[Nhớ]**. Hash khác bản Crew đã ghi nghĩa là Human đã sửa: báo, không ghi đè; artifact phụ thuộc thành `stale` (NEED 42).
- **Lịch sử:** không sửa phá hủy; mỗi thay đổi là một phiên bản mới, giữ liên kết tới bản cũ (NEED 41).
- Mỗi lần ghi là một effect có khóa idempotency (T1.3).

**Tiêu chí nghiệm thu** (GitHub giả)
- Cho repo cá nhân, khi tạo story có task con, thì dùng label và sub-issue; với repo của tổ chức thì dùng issue type và sub-issue.
- Cho issue Human đã sửa, khi Crew định cập nhật, thì Crew không ghi đè, có báo lệch, và artifact phụ thuộc thành `stale`.
- Cho ETag không đổi, khi poll, thì không tải lại nội dung.
- Cho hai lần ghi cùng khóa idempotency, khi thực hiện, thì chỉ có một issue.

**Kiểm chứng:** `cargo test -p crew_github issues`

---

### T4.5 · PR, duyệt theo version, merge theo `sha`

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Thay đổi implement đi qua PR từ nhánh tích hợp; chỉ merge đúng bản Human đã duyệt |
| Căn cứ | NEED 14, 35; spec 12 (PR và merge) |
| Được sửa | `crates/crew_github/src/pulls.rs`; công cụ `crew_github_*` trong `crates/crew/src/tools/` |
| Bàn giao | Tạo PR, đọc review, merge theo `sha`, xử lý PR bị đóng |

**Thiết kế**
- PR từ nhánh tích hợp (T3.14), bằng danh tính App.
- Review `APPROVED` của Human chỉ có hiệu lực khi `commit_id` của review trùng head hiện tại.
- Merge gửi kèm `sha` = head đã được duyệt **[Nhớ]**. Head đổi sau khi duyệt thì không merge và hỏi lại Human.
- PR bị đóng không merge thì báo, chờ Human.
- Sự kiện “PR merged” kèm `merge_commit_sha` được ghi lại cho DoneEvaluator (T5.4).

**Tiêu chí nghiệm thu** (GitHub giả)
- Cho review `APPROVED` trên commit cũ và head đã đổi, khi Orchestrator gọi merge, thì bị từ chối kèm lý do.
- Cho review `APPROVED` trên head hiện tại, khi merge, thì request merge có `sha` đúng head đó.
- Cho PR bị đóng, khi poll, thì có báo và task chuyển `waiting_human`.
- Cho PR đã merged, khi poll, thì có sự kiện kèm `merge_commit_sha`.

**Kiểm chứng:** `cargo test -p crew_github pulls`

---

### T4.6 · Đối soát effect GitHub

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | Sau khi Zed tắt giữa chừng, Crew biết thao tác GitHub nào đã xảy ra mà không làm lại |
| Căn cứ | NEED 44; spec 14.1 bước 2 |
| Được sửa | `crates/crew_github/src/reconcile.rs`; đăng ký vào khung reconciler của T1.3 |
| Bàn giao | Reconciler cho issue, PR, push, merge |

**Tiêu chí nghiệm thu**
- Cho effect tạo issue ở `intent` và trên GitHub đã có issue mang marker tương ứng, khi khởi động lại, thì effect thành `done` và không tạo issue mới.
- Cho effect merge ở `intent` và PR chưa merged, khi khởi động lại, thì effect thành `unknown` và có báo, không tự merge.

**Kiểm chứng:** `cargo test -p crew_github reconcile`

---

### T4.7 · Artifact dạng file qua PR

| Mục | Nội dung |
| --- | -------- |
| Mục tiêu | ADR, glossary và tài liệu quyết định là file trong repo, thay đổi qua PR |
| Căn cứ | NEED 51; spec 12 (ADR, glossary, tài liệu quyết định) |
| Được sửa | `crates/crew/src/artifacts.rs` |
| Bàn giao | Ghi artifact dạng file dưới `artifacts_path`; ADR cũ thành `superseded` |

`artifacts_path` được chốt ở Kickoff (T5.8). Trước đó, test dùng giá trị giả.

**Tiêu chí nghiệm thu**
- Cho một ADR mới thay ADR cũ, khi ghi, thì PR chứa ADR mới và ADR cũ chuyển `superseded`, có liên kết sang ADR mới.
- Cho `artifacts_path` chưa đặt, khi ghi artifact dạng file, thì bị từ chối kèm hướng dẫn chạy Kickoff.

**Kiểm chứng:** `cargo test -p crew artifacts`

---

## Điều kiện xong M4

- [ ] T4.1–T4.7 đạt theo Định nghĩa Xong chung.
- [ ] Trên repo thử thật (repo của Human, đã cài App): từ nhánh tích hợp ra PR. Human duyệt trên GitHub, rồi Crew merge theo `sha`. Lần thử thứ hai đẩy commit mới sau khi duyệt: Crew từ chối merge.
- [ ] Không có token hay private key trong log, run store hay body PR.
