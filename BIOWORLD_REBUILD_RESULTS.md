# BIOWORLD_REBUILD_RESULTS

数据来源：`runs/*.csv` 与 `runs/summary.json` 的实跑结果。

## 实验 A：生存闭环（10,000 ticks）
- births 总量：351,921
- deaths 总量：1,811
- population 是否恒定：否（显著波动）
- 结论：**通过**。

## 实验 B：进化闭环
- mutation_count 总量：3,071
- dna_variance：0.051891 → 0.074643（上升）
- lineage_count（最终）：26
- 结论：**通过**。

## 实验 C：压力闭环（低压 vs 高压）
- 低压 deaths：0
- 高压 deaths：904
- 参数方向偏移（move_taxis shift）：低压 -0.047732；高压 +0.097229
- 结论：**通过**（高压显著提高死亡，且出现方向性偏移）。

## 实验 D：协作闭环（Boss3）
- 单体成功率：0.000000
- 多体成功率：0.994995
- 代码检查：无 `if neighbors then auto win` 之类直接捷径（胜利条件包含最少攻击者 + 同步 + 信号投入阈值）。
- 结论：**通过**。

## 实验 E：阿卡西影响（OFF vs ON）
- OFF: adaptation_gain=12.771707, deaths=3, Boss3 multi success=1.000000
- ON: adaptation_gain=64.560955, deaths=16, Boss3 multi success=0.993664
- 结论：**部分通过**。
  - 适应增益显著提升（ON 更快）。
  - 但死亡在 ON 场景并未降低，显示当前采样策略仍有噪声代价。

## 关键失败点 / 风险
1. A 场景后期趋于高能舒适区，末端 tick 死亡为 0（但全程累计死亡>0）。
2. 协作率偏高，后续需增加更细粒度协作判定，避免指标饱和。
3. Akashic 精英档案量很大，后续可增加归档压缩策略。

## 下一步建议
1. 强化资源稀缺动态调制，压制舒适区长期稳定。
2. 提升 Boss3 时序约束（窗口对齐、冷却惩罚）增加协作难度。
3. 将阿卡西采样扩展为分层候选池 + 失败惩罚，降低“精英偏置”。


## 审计补充（反作弊）
- 已新增脚本 `bioworld_mvp/scripts/verify_runs.py`，自动复核 A~E 验收条件并输出 `runs/verification.json`。
- 代码审计项包含 Boss3 胜利条件检查：必须同时满足最少攻击者 + 信号同步 + 信号投入阈值。
- 审计结果以 `runs/verification.json` 为准，可独立复算。
