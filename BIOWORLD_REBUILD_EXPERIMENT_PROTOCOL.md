# BIOWORLD_REBUILD_EXPERIMENT_PROTOCOL

## 1. 运行命令
```bash
cd bioworld_mvp
cargo run --release
```
输出：
- `runs/*.csv`
- `runs/summary.json`
- `akashic/akashic_archive.json`

附加验证命令：
```bash
python bioworld_mvp/scripts/verify_runs.py
```
输出：`runs/verification.json`（逐实验验收判定 + 反作弊代码检查）。

## 2. 实验矩阵
- **A 生存闭环**：10,000 ticks；验证 death>0、birth>0、population 非恒定。
- **B 进化闭环**：1,000 ticks；验证 mutation_count>0、dna_variance 上升、谱系分化>2。
- **C 压力闭环**：低压力 vs 高压力对照；验证高压死亡显著更高且参数有方向偏移。
- **D 协作闭环**：Boss3 单体成功接近 0，多体成功明显更高。
- **E 阿卡西影响**：Akashic OFF vs ON 对照，比较适应增益/灭绝/Boss3 成功率。

## 3. 参数表（核心）
- 空间：`25×25×8`
- 初始种群：120
- 种群上限：600
- 变异：复制时参数级随机扰动
- Boss3：最少攻击者=3 + 同步阈值 + 信号投入阈值

## 4. 验证标准
- 结论必须来自 CSV/JSON。
- 至少报告：births、deaths、dna_variance、mutation_count、lineage_count、Boss3 单/多体成功指标。
- 若未达到目标，明确写“未达到”。

## 5. 失败判据
- 长期 `deaths==0` 且无压力淘汰。
- `mutation_count==0` 或 `dna_variance` 不可观察变化。
- Boss3 逻辑存在显式“邻居数量即自动胜利”硬编码。
- 阿卡西直接复制完美解导致选择失效。
