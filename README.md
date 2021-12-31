## Diro

一个骰子表达式解析 lib 。

### To-Do List

表达式解析: 
- [x] 四则运算以及幂运算
- [x] 基本骰子表达式( xDy )
- [x] COC 规则(BPKQ)
- [ ] 无限规则
- [ ] 双重十字规则
- [ ] Fate 规则

解析结果解释运行:
- [x] 投掷并运算结果
- [x] 输出表达式字符串
- [x] S 表达式输出
- [ ] 格式化输出中间执行过程

Diro-py:
- [x] 解析表达式
- [x] 解释计算结果
- [ ] 格式化输出中间执行过程

### 表达式规则

最基本的骰子表达式为 xDy, x 表示骰子个数, y 为骰子面数, 二者的取值范围为 u16。

骰子表达式大小写不敏感。

COC 扩展规则可以在基础骰子表达式前后添加额外参数，可接受的参数有：
- b: 奖励骰
- p: 惩罚骰
- k: 取大骰
- q: 取小骰

### 相关项目

[OneDice](https://github.com/OlivOS-Team/onedice): Today, we stand as one. (with some distance 2333)