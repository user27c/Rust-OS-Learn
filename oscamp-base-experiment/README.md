# OS Camp - Rust & OS Advanced Experiments

## 2026春夏季开源操作系统训练营 - 基础阶段实验

本仓库是2026春夏季开源操作系统训练营的基础阶段实验，旨在通过实践练习帮助学员掌握Rust高级编程和操作系统核心概念。

### 致学员的 AI 使用指南

我们鼓励学员合理使用 AI 工具辅助学习，但请注意：
⚠️ 直接复制 AI 生成的答案如同「空中楼阁」—— 您将失去：

- 对 Rust 底层原理的深刻认知
- 排错调试的关键能力培养
- 从错误中成长的珍贵机会

✅ 正确打开方式：

- 先用手工实现理解基础流程
- 用 AI 优化时追问「为什么这样修改」
- 在本地反复验证每个命令的效果

### 操作流程

1. 在网络浏览器中用自己的账号登录 [cnb.cool](https://cnb.cool)。
2. Fork 本仓库, 解锁作业副本。
3. 在您 Fork 的仓库中点击 main 分支旁的「云原生开发」按钮，即可启动在线开发环境（WebIDE，也可以使用 SSH 进行远程连接），自动配置好 Rust 工具链。
4. 在 WebIDE 的终端中执行 `./target/debug/oscamp watch`，即可开始练习。
5. 完成练习后，将代码提交至远程仓库，并创建 PR，在 PR 页面会自动运行测评流水线。
6. 最后可以在 PR 页面来查看评分过程（可多次提交代码，每次提交都会触发评分，以最高分为准）
7. 最终成绩会显示在 opencamp.cn 的个人中心以及课程页面的晋级榜单处。

### 温馨提示: CNB平台成绩提交指南

- a. 注册opencamp和 cnb平台账号
- b. 登录opencamp，点击编辑个人信息
- c. 下拉页面，找到CNBName和GithubName (填入正确的账号用户名）
- d. 不记得CNB账户看这里 --> [点此查询CNB账号信息](https://cnb.cool/profile/account)
- 新建cnb，默认用户名是cnb.xxx（填入步骤c即可）
- 等待CI测评流水线运行，提交PR，就会更新成绩

### 注意事项

- 上述步骤有任何问题都可以找助教。

---

A Rust advanced and operating system introductory exercise repository in the style of [rustlings](https://github.com/rust-lang/rustlings).
Learn Rust concurrency programming, async programming, `no_std` development, and operating system core concepts through completing code and passing tests.

## Prerequisites

- Rust toolchain (stable, >= 1.75)
- Linux environment: most exercises target x86_64; **Module 4 (context switching) only supports riscv64** and requires a riscv64 environment or QEMU user-mode emulation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Exercise Structure

**6 modules, 23 exercises** in total, from easy to advanced:

### Module 1: Concurrency (Synchronous) — `01_concurrency_sync/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_thread_spawn` | `thread::spawn`, `move` closures, `join` |
| 2 | `02_mutex_counter` | `Arc<Mutex<T>>`, shared state concurrency |
| 3 | `03_channel` | `mpsc::channel`, multiple producer pattern |
| 4 | `04_process_pipe` | `Command`, `Stdio::piped()`, process pipes |

### Module 2: no_std Development — `02_no_std_dev/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_mem_primitives` | `no_std` memory primitives: memcpy, memset, memmove, strlen, strcmp |
| 2 | `02_bump_allocator` | `GlobalAlloc` trait, Bump allocator, CAS-based thread safety |
| 3 | `03_free_list_allocator` | Free-list allocator, intrusive linked list, first-fit strategy |
| 4 | `04_syscall_wrapper` | Cross-arch syscall ABI (x86_64/aarch64/riscv64), inline assembly |
| 5 | `05_fd_table` | File descriptor table, `Arc<dyn File>`, fd reuse strategy |

### Module 3: OS Concurrency Advanced — `03_os_concurrency/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_atomic_counter` | `AtomicU64`, `fetch_add`, CAS loop |
| 2 | `02_atomic_ordering` | Memory ordering, Release-Acquire, `OnceCell` |
| 3 | `03_spinlock` | Spinlock implementation, `compare_exchange`, `spin_loop` |
| 4 | `04_spinlock_guard` | RAII guard, `Deref`/`DerefMut`/`Drop` |
| 5 | `05_rwlock` | Writer-priority read-write lock from scratch (no `std::sync::RwLock`) |

### Module 4: Context Switching — `04_context_switch/` (riscv64 only)

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_stack_coroutine` | Callee-saved registers, stack frames, context switching |
| 2 | `02_green_threads` | Green thread scheduler, cooperative scheduling, yield |

Module 4 only runs on **riscv64**. Run `./check.sh` or use the `oscamp` CLI as with the rest of the repository — no separate scripts needed. See `exercises/04_context_switch/README.md` for details.

### Module 5: Async Programming — `05_async_programming/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_basic_future` | Manual implementation of `Future` trait, `Poll`, `Waker` |
| 2 | `02_tokio_tasks` | `tokio::spawn`, `JoinHandle`, concurrent tasks |
| 3 | `03_async_channel` | `tokio::sync::mpsc`, async producer-consumer |
| 4 | `04_select_timeout` | `tokio::select!`, timeout control, race execution |

### Module 6: Page Tables — `06_page_table/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_pte_flags` | SV39 PTE bit layout, bit operations to construct/parse page table entries |
| 2 | `02_page_table_walk` | Single-level page tables, VPN/offset splitting, address translation, page faults |
| 3 | `03_multi_level_pt` | SV39 three-level page tables, page table walk, huge pages (2MB) mapping |
| 4 | `04_tlb_sim` | TLB lookup/insert/FIFO replacement, flush (all/by page/by ASID), MMU simulation |

## Quick Start

```bash
# 1. Clone repository
git clone <repo-url> && cd oscamp-base-experiment

# 2. Build interactive CLI tool
cargo build -p oscamp-cli

# 3. Start interactive exercise mode (recommended)
./target/debug/oscamp watch
```

## Interactive CLI Tool (`oscamp`)

Built-in interactive terminal tool similar to rustlings, supporting real-time file watching and progress tracking:

```bash
oscamp              # Start interactive watch mode (default)
oscamp watch        # Same as above
oscamp list         # View completion status of all exercises
oscamp check        # Check all exercises in batch
oscamp run <pkg>    # Run tests for specified exercise
oscamp hint <pkg>   # View exercise hint
oscamp help         # Show help
```

### Watch Mode Features

- **Automatic file change detection**: Automatically re-run tests after saving files
- **Auto-jump**: Automatically jump to next unfinished exercise after current one passes
- **Real-time progress bar**: Show overall completion progress
- **Shortcuts**:
  - `h` — View hint for current exercise
  - `l` — View list of all exercises
  - `n` / `p` — Next / Previous exercise
  - `r` / `Enter` — Re-run tests
  - `q` / `Esc` — Quit

### Manual Execution

```bash
# Run tests for a specific exercise
cargo test -p thread_spawn

# View detailed output
cargo test -p thread_spawn -- --nocapture

# Check all exercises
cargo test --workspace
```

## Workflow

1. **Start**: Run `./target/debug/oscamp watch` to enter interactive mode
2. **Read**: Open current exercise file `src/lib.rs`, read documentation to understand concepts
3. **Code**: Find `todo!()` markers, complete code according to comment hints
4. **Save**: After saving file, CLI automatically re-runs tests
5. **Pass**: After passing tests, automatically jump to next exercise; press `h` to view hints anytime

## Submitting Scores

This repository uses a CI pipeline to automatically score your exercises and submit results to the leaderboard.

1. Fork this repository on [CNB](https://cnb.cool)
2. Complete exercises locally or in the cloud-native WebIDE
3. Push your changes to your fork
4. Create a **Pull Request** to the upstream repository — this triggers the scoring pipeline
5. The pipeline runs all tests, calculates your score (out of 100), and uploads it to the OpenCamp leaderboard

## Notes

- Some exercises (e.g., Module 2 syscall wrapper, Module 4 assembly) require a **Linux** environment; Module 4 only supports **riscv64**
- It is recommended to complete exercises in module order; within each module, exercises progress from easy to advanced

## License

MIT
