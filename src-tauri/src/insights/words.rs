use super::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::OnceLock,
};
static JIEBA: OnceLock<jieba_rs::Jieba> = OnceLock::new();
const STOP:&str="的 了 和 是 在 我 我们 你 你们 他 她 它 他们 与 也 就 都 而 及 但 并 一个 一些 这个 那个 这 那 有 没有 今天 明天 昨天 因为 所以 如果 已经 还是 进行 可以 需要 非常 the a an and or is are was were to of in on for with this that";
pub(super) fn tokens(body: &str) -> BTreeMap<String, i64> {
    let mut out = BTreeMap::new();
    for line in body.lines().filter(|l| !l.trim_start().starts_with("```")) {
        for w in JIEBA.get_or_init(jieba_rs::Jieba::new).cut(line, false) {
            let w = w.trim().to_lowercase();
            if w.chars().count() < 2
                || !w.chars().all(char::is_alphanumeric)
                || w.chars().all(char::is_numeric)
                || STOP.split_whitespace().any(|s| s == w)
            {
                continue;
            }
            *out.entry(w).or_default() += 1;
        }
    }
    out
}
pub(super) async fn read(
    db: &Database,
    from: &str,
    until: &str,
    prefs: &Preferences,
) -> Result<Vec<Word>> {
    let mut map: BTreeMap<String, (i64, Vec<Record>)> = BTreeMap::new();
    let stop: BTreeSet<_> = prefs.stop_words.iter().map(|s| s.to_lowercase()).collect();
    let mut cache = BTreeMap::new();
    for c in sqlx::query(
        "SELECT kind,id,fingerprint,tokens FROM insights_words WHERE date BETWEEN ? AND ?",
    )
    .bind(from)
    .bind(until)
    .fetch_all(&db.pool)
    .await?
    {
        cache.insert(
            (c.get::<String, _>("kind"), c.get::<String, _>("id")),
            (
                c.get::<String, _>("fingerprint"),
                c.get::<String, _>("tokens"),
            ),
        );
    }
    for (kind,sql) in [
 ("journal","SELECT id,entry_date date,title,body FROM journal_entries WHERE deleted_at IS NULL AND is_draft=0 AND entry_date BETWEEN ? AND ? AND (entry_date,id)>(?,?) ORDER BY entry_date,id LIMIT 250"),
 ("review","SELECT id,week_start_date date,'我的周回顾' title,body || ' ' || next_focus body FROM weekly_reviews WHERE deleted_at IS NULL AND week_start_date BETWEEN ? AND ? AND (week_start_date,id)>(?,?) ORDER BY week_start_date,id LIMIT 250")]{
 let mut cursor_day=from.to_string();let mut cursor_id=String::new();
 loop {let rows=sqlx::query(sql).bind(from).bind(until).bind(&cursor_day).bind(&cursor_id).fetch_all(&db.pool).await?;let n=rows.len();let mut tx=db.pool.begin().await?;
 for r in rows {let id:String=r.get("id");let body:String=r.get("body");let title:String=r.get("title");let day:String=r.get("date");
 cursor_day=day.clone();cursor_id=id.clone();
 let text=if kind=="journal"{format!("{title}\n{body}")}else{body};
 let fingerprint=checksum(text.as_bytes());
 let cached=cache.get(&(kind.to_string(),id.clone()));
 let words:BTreeMap<String,i64>=if let Some(c)=cached.filter(|c|c.0==fingerprint){serde_json::from_str(&c.1).map_err(|_|DbError::invalid())?}else{
 let words=tokens(&text);
 sqlx::query("INSERT INTO insights_words(kind,id,date,fingerprint,tokens) VALUES(?,?,?,?,?) ON CONFLICT(kind,id) DO UPDATE SET date=excluded.date,fingerprint=excluded.fingerprint,tokens=excluded.tokens").bind(kind).bind(&id).bind(&day).bind(fingerprint).bind(serde_json::to_string(&words).map_err(|_|DbError::invalid())?).execute(&mut *tx).await?;words};
 for (word,count) in words {if stop.contains(&word){continue}let row=map.entry(word).or_default();row.0+=count;if row.1.len()<20 {row.1.push(Record{kind:kind.into(),id:id.clone(),date:day.clone(),title:title.clone()})}}
 } tx.commit().await?;if n<250{break}
 }}
    let mut out: Vec<_> = map
        .into_iter()
        .map(|(word, (count, records))| Word {
            word,
            count,
            records,
        })
        .collect();
    out.sort_by(|a, b| b.count.cmp(&a.count).then(a.word.cmp(&b.word)));
    out.truncate(50);
    Ok(out)
}
