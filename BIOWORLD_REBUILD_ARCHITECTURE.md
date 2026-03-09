# BIOWORLD_REBUILD_ARCHITECTURE

## 1. 公理层（唯一硬编码生命法则）
1. 探索
2. 生存
3. 记忆
4. 复制
5. 传承
6. 进化

> 除以上六条外，不引入“意识/人格/神谕”机制。

## 2. 模块架构
- **World Engine**: 25×25×8 网格、能量场、局部信号场、Boss 场、tick 调度。
- **Cell Engine**: Cell 状态、DNA(6 维)、局部记忆、局部动作决策、死亡/复制。
- **Evolution Layer**: 变异、继承、选择压力、种群控制（MAX_POP）。
- **Akashic Layer**: 死亡档案、精英档案、跨宇宙低概率 DNA 采样。
- **Metrics Layer**: 基础/协作/进化/CDI 指标输出。
- **Experiment Runner**: A~E 实验批处理，输出 CSV/JSON。

## 3. 数据流
1. 世界更新局部能量。
2. 细胞基于局部能量 + 局部信号 + 自身状态做动作。
3. 行动结算能量（代谢、移动、信号、Boss 交互）。
4. 触发死亡/复制；复制时发生变异与传承。
5. 记录 tick 指标、事件、阿卡西档案。

## 4. Tick 生命周期
每 tick 固定顺序：
1) environment update
2) cell perception
3) cell decision
4) action execute
5) energy accounting
6) reproduction
7) death check
8) metrics/logging

## 5. 空间与感知约束
- 空间固定：`25×25×8`。
- 个体仅可访问：局部能量、局部信号、自身状态。
- 禁止：全局地图、全局最优路径、Boss 全知信息、读他体内部状态。

## 6. Cell 最小状态
- `id`
- `position (x,y,z)`
- `energy`
- `alive/dead`（实现里通过是否存活于容器表示）
- `age`
- `generation`
- `dna`
- `memory`
- `last_action`
- `local_signal_state`
- `lineage_id`

## 7. DNA 结构（6 维）
- `move_randomness`
- `move_taxis`
- `energy_reserve_threshold`
- `signal_investment`
- `freq_preference`
- `freq_plasticity`

## 8. 能量与生死复制
- 死亡条件：`energy <= 0` 或超龄。
- 复制条件：`energy > dna.energy_reserve_threshold`。
- 复制结算：亲代分能量、子代继承 DNA + 可变异。

## 9. Boss 机制
- Boss1：单体可胜（低门槛）。
- Boss2：中压（需更多攻击者/同步）。
- Boss3：高压（需要多体 + 时机同步 + 信号同步 + 投入阈值）。
- 非“人数够自动赢”：还需同步系数与信号投入阈值。

## 10. 进化机制
- 变异：复制时按 mutation_rate 对 6 维参数独立扰动。
- 选择：由死亡/复制自然发生。
- 继承：子代继承谱系 lineage_id；阿卡西开启时低概率采样精英 DNA 并加噪。

## 11. 阿卡西机制
记录：死亡记录、精英 DNA、宇宙 id、代数等。
使用：仅低概率采样，不直接复制“完美解”，保证可失败。

## 12. 指标与 CDI
基础：generation/population/births/deaths/average_energy/dna_variance/lineage_count。
协作：cooperation_rate/mean_cluster_size/multi_cell_boss_success_rate/energy_transfer_count/signal_synchrony。
进化：mutation_count/nonzero_mutation_generations/elite_lineage_survival/adaptation_gain/extinction_events。

CDI 固定公式：
```
CDI = 0.25*memory_depth
    + 0.25*action_diversity
    + 0.25*cooperation_index
    + 0.25*survival_adaptation
```
