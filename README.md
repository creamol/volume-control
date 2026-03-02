# 🎧 蓝牙耳机重启音量自动修复方案

## 问题

Windows 蓝牙耳机在电脑重启后，系统音量可能自动跳回 **100%**，导致突然巨响。

## 解决方案

开机自动运行一个小程序，检测到音量为 100% 时自动改为 16%。

---

## 使用步骤

### 第一步：下载程序

前往 GitHub 下载编译好的 exe：

👉 https://github.com/creamol/volume-control/actions

点击最新绿色记录 → 页面底部 **Artifacts** → 下载 `volume-control.zip` → 解压得到 `volume-control.exe`

---

### 第二步：放入开机启动文件夹

按 `Win + R`，输入以下内容并回车：

```
shell:startup
```

将 `volume-control.exe` 复制到打开的文件夹中。

完整路径为：

```
C:\Users\你的用户名\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup
```

---

## 运行逻辑

```
电脑开机
  └─ 自动运行 volume-control.exe
       ├─ 当前音量 == 100%  →  改为 16%，取消静音
       └─ 当前音量 != 100%  →  什么都不做
```

---

## 源码

- 语言：Rust
- 仓库：https://github.com/creamol/volume-control
- 依赖：Windows API（`IAudioEndpointVolume`）
