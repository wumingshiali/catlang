# This document is only support Chinese now
# 版本控制逻辑
## 版本名组成
我们的版本控制使用[语义化版本](https://semver.org/lang/zh-CN/)的修改，在原本基础上添加LTS后缀，规则是这样：(based verison)-LTS(LTS verison)。
## 维护逻辑
普通版本没有额外维护。  
每个小版本的最后一个Bug修复版本就会从这个版本分支一个LTS版本，LTS版本额外维护到这个LTS版本之后的第3个LTS版本，之后会转成ESM只提供安全更新，在ESM的时候请你转到最新的LTS或者正常版本，ESM在1个季度之后会停止维护。
