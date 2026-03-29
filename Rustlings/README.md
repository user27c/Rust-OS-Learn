## 2026年春夏季操作系统训练营

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

### 练习内容

导学阶段将通过 Rustlings 进行测试，包含以下练习内容：

- 基础语法练习（变量、函数、条件语句等）
- 所有权与借用练习
- 结构体与枚举练习
- 错误处理练习
- 泛型与特征练习
- 并发编程练习

### 前置条件

- 您必须报名 2026年春夏季操作系统训练营
- 您必须在 opencamp.cn 的个人中心，填写您的 CNB 用户名来完成账号绑定

### 操作流程

1. 在网络浏览器中用自己的账号登录 [cnb.cool](https://cnb.cool)。
2. Fork 本仓库, 解锁作业副本。
3. 在您 Fork 的仓库中点击 main 分支旁的「云原生开发」按钮，即可启动在线开发环境（WebIDE，也可以使用 SSH 进行远程连接），自动配置好 Rust 工具链和 rustlings。
4. 在 WebIDE 的终端中执行 `rustlings watch`，即可开始练习。
5. 完成练习后，将代码提交至远程仓库，并创建 PR，在 PR 页面会自动运行测评流水线。
6. 最后可以在 PR 页面来查看评分过程（可多次提交代码，每次提交都会触发评分，以最高分为准）
7. 最终成绩会显示在 opencamp.cn 的个人中心以及课程页面的晋级榜单处。


### 温馨提示: CNB平台成绩提交指南

a.注册[opencamp](https://opencamp.cn)和 [cnb](https://cnb.cool)平台账号

b.登录[opencamp](https://opencamp.cn)，点击[编辑个人信息](https://opencamp.cn/my/edit?tab=userInfo)

c.下拉页面，找到CNBName和GithubName (填入正确的账号用户名）

d.不记得CNB账户看这里 -->[点此查询CNB账号信息](https://cnb.cool/profile/account)
新建cnb，默认用户名是cnb.xxx（填入步骤c即可）

等待CI测评流水线运行，提交PR，就会更新成绩

### 注意事项

- 上述步骤有任何问题都可以找助教。
- 下面是官方的 Rustlings 说明，可以参考，**请务必不要拉取下面的仓库！**

# rustlings 🦀❤️

</div>

Greetings and welcome to `rustlings`. This project contains small exercises to get you used to reading and writing Rust code. This includes reading and responding to compiler messages!

_...looking for the old, web-based version of Rustlings? Try [here](https://github.com/rust-lang/rustlings/tree/rustlings-1)_

Alternatively, for a first-time Rust learner, there are several other resources:

- [The Book](https://doc.rust-lang.org/book/index.html) - The most comprehensive resource for learning Rust, but a bit theoretical sometimes. You will be using this along with Rustlings!
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) - Learn Rust by solving little exercises! It's almost like `rustlings`, but online

## Getting Started

_Note: If you're on MacOS, make sure you've installed Xcode and its developer tools by typing `xcode-select --install`._
_Note: If you're on Linux, make sure you've installed gcc. Deb: `sudo apt install gcc`. Yum: `sudo yum -y install gcc`._

You will need to have Rust installed. You can get it by visiting https://rustup.rs. This'll also install Cargo, Rust's package/project manager.

## MacOS/Linux

Just run:

```bash
curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash
```
Or if you want it to be installed to a different path:

```bash
curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash -s mypath/
```

This will install Rustlings and give you access to the `rustlings` command. Run it to get started!

### Nix

Basically: Clone the repository at the latest tag, finally run `nix develop` or `nix-shell`.

```bash
# find out the latest version at https://github.com/rust-lang/rustlings/releases/latest (on edit 5.5.1)
git clone -b 5.5.1 --depth 1 https://github.com/rust-lang/rustlings
cd rustlings
# if nix version > 2.3
nix develop
# if nix version <= 2.3
nix-shell
```

## Windows

In PowerShell (Run as Administrator), set `ExecutionPolicy` to `RemoteSigned`:

```ps1
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

Then, you can run:

```ps1
Start-BitsTransfer -Source https://raw.githubusercontent.com/rust-lang/rustlings/main/install.ps1 -Destination $env:TMP/install_rustlings.ps1; Unblock-File $env:TMP/install_rustlings.ps1; Invoke-Expression $env:TMP/install_rustlings.ps1
```

To install Rustlings. Same as on MacOS/Linux, you will have access to the `rustlings` command after it. Keep in mind that this works best in PowerShell, and any other terminals may give you errors.

If you get a permission denied message, you might have to exclude the directory where you cloned Rustlings in your antivirus.

## Browser

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/rust-lang/rustlings)

[![Open Rustlings On Codespaces](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new/?repo=rust-lang%2Frustlings&ref=main)

## Manually

Basically: Clone the repository at the latest tag, run `cargo install --path .`.

```bash
# find out the latest version at https://github.com/rust-lang/rustlings/releases/latest (on edit 5.5.1)
git clone -b 5.5.1 --depth 1 https://github.com/rust-lang/rustlings
cd rustlings
cargo install --force --path .
```

If there are installation errors, ensure that your toolchain is up to date. For the latest, run:

```bash
rustup update
```

Then, same as above, run `rustlings` to get started.

## Doing exercises

The exercises are sorted by topic and can be found in the subdirectory `rustlings/exercises/<topic>`. For every topic there is an additional README file with some resources to get you started on the topic. We really recommend that you have a look at them before you start.

The task is simple. Most exercises contain an error that keeps them from compiling, and it's up to you to fix it! Some exercises are also run as tests, but rustlings handles them all the same. To run the exercises in the recommended order, execute:

```bash
rustlings watch
```

This will try to verify the completion of every exercise in a predetermined order (what we think is best for newcomers). It will also rerun automatically every time you change a file in the `exercises/` directory. If you want to only run it once, you can use:

```bash
rustlings verify
```

This will do the same as watch, but it'll quit after running.

In case you want to go by your own order, or want to only verify a single exercise, you can run:

```bash
rustlings run myExercise1
```

Or simply use the following command to run the next unsolved exercise in the course:

```bash
rustlings run next
```

In case you get stuck, you can run the following command to get a hint for your
exercise:

```bash
rustlings hint myExercise1
```

You can also get the hint for the next unsolved exercise with the following command:

```bash
rustlings hint next
```

To check your progress, you can run the following command:

```bash
rustlings list
```

## Testing yourself

After every couple of sections, there will be a quiz that'll test your knowledge on a bunch of sections at once. These quizzes are found in `exercises/quizN.rs`.

## Enabling `rust-analyzer`

Run the command `rustlings lsp` which will generate a `rust-project.json` at the root of the project, this allows [rust-analyzer](https://rust-analyzer.github.io/) to parse each exercise.

## Continuing On

Once you've completed Rustlings, put your new knowledge to good use! Continue practicing your Rust skills by building your own projects, contributing to Rustlings, or finding other open-source projects to contribute to.

## Uninstalling Rustlings

If you want to remove Rustlings from your system, there are two steps. First, you'll need to remove the exercises folder that the install script created
for you:

```bash
rm -rf rustlings # or your custom folder name, if you chose and or renamed it
```

Second, run `cargo uninstall` to remove the `rustlings` binary:

```bash
cargo uninstall rustlings
```

Now you should be done!

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

Development-focused discussion about Rustlings happens in the [**rustlings** stream](https://rust-lang.zulipchat.com/#narrow/stream/334454-rustlings)
on the [Rust Project Zulip](https://rust-lang.zulipchat.com). Feel free to start a new thread there
if you have ideas or suggestions!

## Contributors ✨

Thanks goes to the wonderful people listed in [AUTHORS.md](./AUTHORS.md) 🎉
