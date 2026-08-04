# Zed 扩展：SAP VC（M2 — 高亮）

Zed 开发扩展目录：`C:\Users\10586006\.config\zed\extensions\dev\sapvc\`

## 结构

```
extension.toml                 # 声明 grammar（file:// 指向 tree-sitter-sapvc 仓库）+ rev
languages/sapvc/config.toml    # 语言名 / grammar / 注释符号
queries/sapvc/highlights.scm   # tree-sitter 高亮查询
```

## 启用步骤（M2 终端清单，需审批）

1. 生成 parser 并验证：
   ```bash
   cd C:\Users\10586006\01Project\05ProcessOptimize\sapvc-tools\tree-sitter-sapvc
   npm install            # 装 tree-sitter-cli
   npm run generate       # grammar.js → src/parser.c（可能有冲突要修）
   npm run parse "<某个 PRO/CONS 文件>"   # 验证 AST
   npm run highlight "<某个 PRO/CONS 文件>"  # 离线验证高亮查询
   ```
2. 让 Zed 能加载 grammar（file:// 需要 git 仓库 + SHA）：
   ```bash
   git init && git add -A && git commit -m "init: validated grammar"
   git rev-parse HEAD
   ```
3. 把 SHA 填入 `extension.toml` 的 `rev`
4. 重启 Zed → 打开 PRO/CONS 文件 → 底部状态栏确认语言为 "SAP VC"；
   若未自动识别，加用户设置：
   ```json
   "file_types": { "SAP VC": ["**/PRO_*.txt", "**/CONS_*.txt"] }
   ```

## 验证点

- `$SELF`/`$ROOT`/`$PARENT` 高亮为内建变量色
- `?=`、`IN`、`SPECIFIED`、`AND/OR/NOT`、`is invisible` 运算符色
- `$DEL_DEFAULT`/`TABLE`/`PFUNCTION` 函数色
- `OBJECTS:`/`CONDITION:`/`RESTRICTIONS:`/`INFERENCES:` 关键字色
- `*` 注释整行灰色（含中文注释）
- 字符串（单引号）与数字（日期 20250712）各自着色

## 已知待办

- `rev` 占位符（步骤 2/3 填写）
- tree-sitter generate 可能报冲突（`value` 的 `(...)` 与 `paren_group` 的 `(`）→ 修 grammar.js
- M3 才加 LSP（`[language_servers.sapvc-lsp]` + Rust 壳 `language_server_command`）
