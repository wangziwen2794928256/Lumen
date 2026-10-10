# Music data contract

009只增列/索引/表，保留旧track/play及album。新字段platform、identity_key（自动来源元数据hash）、source/manual或windows-media、source_app、journal_id、event_key和revision；preferences/receipts不注入初始行。默认manual、监听关闭、阈值30秒（范围5–600），精确来源ID必填才可启用。

手动Save支持参数SQL、请求id+payload hash幂等、乐观版本冲突；时长0–86400秒，时间必须RFC3339 UTC Z，标题/artist/album/platform≤300字符，日记关联必须存在且未删除。Trash/Restore软删除，自动事件不复活用户已删除记录。自动event唯一，时长单调增加而不是反复新增次数。

日期按本机localtime，查询最长366日间隔；记录最新100条，云最多500组/展示50项；count/seconds总计全量。Song按title+artist，Artist/Album单独归并；不混入Journal分词。Insights旧music聚合沿用同一表；已封存月不吸收后续补录。

音乐→日记为play.journal_id，界面跳转现有entry路由。手动编辑创建独立track，避免改写其他历史播放的元数据；未引用旧track保留，后续清理策略不得删除有历史的track。默认private，没有传输、OAuth凭证或公共导出。
