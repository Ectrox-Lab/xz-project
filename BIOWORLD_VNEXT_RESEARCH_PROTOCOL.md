# Bio-World vNext Research Protocol（研究级验证框架）

## 0. 协议定位
本协议用于约束 **Bio-World vNext** 的研发、运行、验证与审计流程，目标是构建可用于长期科学研究的数字生命演化平台。

本协议优先级高于“快速可跑通”目标；当“可跑通”与“可验证”冲突时，必须优先保证可验证。

---

## 1. 不可妥协的研究原则
1. **真实运行原则**：任何结论必须来自真实运行日志，不接受静态推演替代。
2. **局部信息原则**：个体决策禁止读取全局状态。
3. **最小先验原则**：禁止硬编码“应该协作成功”或“应该击败 Boss”。
4. **可复现实验原则**：同配置同种子应可复现统计结果范围。
5. **可审计原则**：关键机制必须可由审计脚本直接检测。
6. **失败诚实原则**：未达标即标注“未达标”，不得通过文字包装掩盖。

---

## 2. 系统能力边界（研究级，非 MVP）
平台必须支持：
- 三维大尺度网格（`50×50×16`）
- 长时间演化（建议最小 100,000 ticks 可运行）
- 多宇宙并行（目标 `128 universes`）
- 阿卡西跨宇宙经验迁移（受限概率 + 噪声继承）
- 协作涌现与灭绝级联分析
- CDI 与 Hazard Rate 关联建模

禁止将“研究级”退化为“单宇宙演示脚本”。

---

## 3. 架构与模块强约束
代码必须模块化；以下模块必须存在且有实质实现（非空壳）：

```
bio_world/
├── engine/
│   ├── world.rs
│   ├── cell.rs
│   ├── dna.rs
│   ├── energy.rs
│   └── physics.rs
├── evolution/
│   ├── mutation.rs
│   ├── reproduction.rs
│   ├── selection.rs
│   └── cooperation.rs
├── boss/
│   ├── boss_system.rs
│   └── difficulty_curve.rs
├── akashic/
│   ├── akashic_archive.rs
│   └── cross_universe_transfer.rs
├── metrics/
│   ├── cdi.rs
│   ├── stability.rs
│   └── extinction.rs
├── experiments/
│   ├── experiment_runner.rs
│   └── cross_seed.rs
├── output/
│   ├── csv_logger.rs
│   └── json_export.rs
└── main.rs
```

红线：
- 不允许单文件大一统（如 `main.rs > 2000` 行且承载核心逻辑）。
- 不允许仅定义结构体而无行为实现。

---

## 4. 六条元规则（唯一基础行为公理）
系统底层行为只能依赖以下六条：
1. 探索
2. 生存
3. 记忆
4. 复制
5. 传承
6. 进化

禁止新增“意识/人格/意图引擎”等高层伪机制。

---

## 5. Cell / DNA / Memory 研究约束

### 5.1 Cell 最小字段
必须包含：
- `id`
- `position(x,y,z)`
- `energy`
- `age`
- `alive`
- `dna`
- `memory`
- `cooperation_state`
- `signal_state`
- `lineage_id`

### 5.2 DNA 最小 10 维
必须至少包含：
- `move_speed`
- `sensing_radius`
- `attack_power`
- `defense`
- `cooperation_willingness`
- `signal_strength`
- `signal_frequency`
- `memory_capacity`
- `learning_rate`
- `mutation_rate`

必须记录：
- `mutation_history`
- 参数级变异统计分布

### 5.3 记忆层次
- 局部短期记忆（个体近期体验）
- 跨宇宙阿卡西记忆（概率继承）

阿卡西不能成为神谕，必须可失败。

---

## 6. 信息可见性与作弊红线
个体允许读取：
- 局部邻居
- 局部信号
- 局部能量
- 自身状态

个体禁止读取：
- global population
- global boss hp/status
- 全局最优路径
- 他体内部完整状态

任何“全局可见捷径”一票否决。

---

## 7. 能量经济与守恒检查
必须实现并记录：

### 来源
- `environment_energy`
- `food_nodes`
- `boss_reward`

### 消耗
- `movement_cost`
- `signal_cost`
- `memory_cost`
- `reproduction_cost`

必须提供 **能量账本审计**：
- 每 tick 总输入 / 总输出 / 系统存量
- 允许数值误差阈值（例如 `1e-6`）

---

## 8. 协作涌现判定协议
协作成功必须满足复合条件，至少包含：
- `attackers >= threshold`
- `synchrony > X`
- `signal_investment > Y`

禁止规则：
- `if neighbors > k then win`
- 任意“人数触发即自动胜利”逻辑

必须输出协作证据：
- 时序同步曲线
- 信号相位一致性
- 单体 vs 多体成功率对照

---

## 9. Boss 十级压力曲线协议
必须实现 10 级难度曲线（L1~L10），并记录参数曲线：
- L1 primitive cooperation
- L2 coordination
- L3 signal alignment
- L4 strategy
- L5 adaptive cooperation
- L6 swarm intelligence
- L7 strategic planning
- L8 meta coordination
- L9 innovation
- L10 human-level complexity (operationally defined)

说明：L10 必须给出工程化可计算定义，禁止空洞命名。

---

## 10. Akashic 跨宇宙迁移协议
必须输出 `akashic/akashic_archive.json`，至少包含：
- `elite_dna`
- `death_records`
- `strategy_patterns`
- `source_universe`
- `generation`

迁移规则：
- 概率采样
- 必须叠加变异
- 必须允许继承失败

禁止：
- 完美最终解直接回灌
- 即时全宇宙共享最优答案

---

## 11. 多宇宙并行协议
目标支持：`128 universes parallel`。

要求：
- 每宇宙独立随机种子
- 每宇宙独立参数集
- 每宇宙独立日志
- 汇总时保留 universe 级可追踪性

推荐并行：`rayon/crossbeam`。

---

## 12. 指标与方程（必须可复算）

### 12.1 CDI
```
CDI = signal_diversity
    × cooperation_density
    × memory_usage
    × exploration_rate
```

必须输出到 CSV，且提供每项子指标原值。

### 12.2 Hazard Rate
给出灭绝危险率模型：
```
P(extinction) = 1 - exp(-h(t) * Δt)
```
并验证：当 `CDI < I_crit` 时，`h(t)` 上升。

### 12.3 灭绝动力学
至少输出：
- population
- CDI
- death_rate
- extinction_events

用于分析 extinction cascade。

---

## 13. 研究实验矩阵（强制）

### A 生存闭环
- `births > 0`
- `deaths > 0`
- population 非常量

### B 进化闭环
- `mutation_count > 0`
- `dna_variance` 上升
- lineage 分化可观测

### C 压力闭环
- 高压死亡率 > 低压死亡率
- 至少一个 DNA 参数出现方向性偏移

### D 协作闭环
- 单体成功率接近 0
- 多体成功率显著高于单体
- 无自动胜利硬编码

### E 阿卡西闭环
- ON vs OFF 对照真实运行
- `adaptation_gain` 在 ON 场景提升
- 同时报告潜在代价（如死亡率变化）

### F 跨种子稳健性
- `seed = 1..10`
- 输出 `I_crit` 分布、hazard ratio、生存曲线

---

## 14. 输出与目录规范
必须生成：
- `BIOWORLD_ARCHITECTURE.md`
- `BIOWORLD_EXPERIMENT_PROTOCOL.md`
- `BIOWORLD_RESULTS.md`
- `runs/`（`population.csv`, `cdi.csv`, `mutation.csv`, `boss.csv`, `extinction.csv`）
- `akashic/`
- `audit/`

每份结果文件必须包含：
- 运行配置
- 随机种子
- 时间戳
- git commit hash

---

## 15. 审计框架（反作弊）
`audit/` 脚本至少检查：
1. boss auto-win 模式
2. 全局知识读取
3. 隐式捷径（神谕变量、后门奖励）
4. 结果文件与配置一致性
5. 指标可复算一致性

审计输出必须包含：`PASS/FAIL + 证据路径`。

---

## 16. 运行与复现实验协议
必须提供：
- `run_experiment.sh`

示例：
```bash
cargo run --release -- --seed 1 --ticks 10000
```

必须提供：
- 单次运行
- 批量 cross-seed
- 参数扫描（mutation / pressure / boss difficulty）

---

## 17. 统计与报告规范
`BIOWORLD_RESULTS.md` 必须逐实验给出：
- 验收条件
- 实测结果
- 是否通过
- 未通过原因
- 下一步修正方案

禁止“整体看起来良好”式含混表述。

---

## 18. 禁止行为（零容忍）
- 伪造 CSV/JSON
- 未运行先写结果
- 写死通过结论
- 以不可解释模型替代演化机制
- 擅自缩减实验矩阵并声称完成

触发任一项即判定交付无效。

---

## 19. 最终验收门槛
项目只有在以下同时成立时才算通过：
1. 代码结构符合模块化约束
2. A~F 实验有真实可复算数据
3. 审计脚本通过且可复现
4. 结论与原始日志一致

若任一条不成立：
- 状态必须标注为 **“研究平台未完成”**。

---

## 20. 执行口令（给 Codex）
你执行的是研究平台重建，不是 demo 生成。

你必须：
- 先实现系统，再运行实验，再写结果。
- 每一条结论都给出日志证据路径。
- 对未达标项明确写“未达标”。
- 不得编造任何数据。

你的成功标准不是“代码很多”，而是：
**可运行、可复现、可审计、可证伪。**
