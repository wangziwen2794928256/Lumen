use super::*;
#[tokio::test]
#[ignore = "Explicit reproducible ten-year fixture benchmark; never uses production"]
async fn insights_ten_year_performance_baseline() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(dir.path()).await.unwrap();
    let begin = std::time::Instant::now();
    let series = "WITH RECURSIVE n(i) AS (SELECT 0 UNION ALL SELECT i+1 FROM n WHERE i<99999)";
    let stamp = "'2026-01-01T00:00:00Z'";
    sqlx::query(&format!("INSERT INTO music_tracks(id,title,artist,album,created_at,updated_at) VALUES('00000000-0000-4000-8000-000000000001','音乐模拟','艺术家模拟','专辑模拟',{stamp},{stamp})")).execute(&db.pool).await.unwrap();
    for sql in [
 format!("{series} INSERT INTO focus_sessions(id,started_at,ended_at,planned_seconds,actual_seconds,state,elapsed_ms,created_at,updated_at) SELECT printf('10000000-0000-4000-8000-%012d',i),date('2016-01-01','+'||(i%3650)||' days')||'T00:00:00Z',date('2016-01-01','+'||(i%3650)||' days')||'T00:01:00Z',60,60,'finished',60000,{stamp},{stamp} FROM n"),
 "INSERT INTO focus_intervals(session_id,segment_no,local_date,started_at,ended_at,elapsed_ms) SELECT id,1,substr(started_at,1,10),started_at,ended_at,60000 FROM focus_sessions".to_string(),
 format!("{series} INSERT INTO music_plays(id,track_id,played_at,duration_seconds,created_at,updated_at) SELECT printf('20000000-0000-4000-8000-%012d',i),'00000000-0000-4000-8000-000000000001',date('2016-01-01','+'||(i%3650)||' days')||'T12:00:00Z',60,{stamp},{stamp} FROM n"),
 format!("{series} INSERT INTO journal_entries(id,title,body,entry_date,created_at,updated_at) SELECT printf('30000000-0000-4000-8000-%012d',i),'十年模拟','数学数学',date('2016-01-01','+'||(i%3650)||' days'),{stamp},{stamp} FROM n WHERE i<50000"),
 format!("{series} INSERT INTO coding_problems(id,title,created_at,updated_at) SELECT printf('40000000-0000-4000-8000-%012d',i),'模拟题目',{stamp},{stamp} FROM n WHERE i<10000"),
 format!("{series} INSERT INTO coding_attempts(id,problem_id,outcome,attempted_on,manual_seconds,created_at,updated_at) SELECT printf('50000000-0000-4000-8000-%012d',i),printf('40000000-0000-4000-8000-%012d',i),'accepted',date('2016-01-01','+'||(i%3650)||' days'),60,{stamp},{stamp} FROM n WHERE i<10000")
 ]{sqlx::query(&sql).execute(&db.pool).await.unwrap();}
    let generated_ms = begin.elapsed().as_millis();
    let end = (date("2016-01-01").unwrap() + Duration::days(3649)).to_string();
    let f = Filter {
        from: "2016-01-01".into(),
        until: end.clone(),
        today: end.clone(),
    };
    let cold = std::time::Instant::now();
    let snapshot = db.insights_snapshot(f.clone()).await.unwrap();
    let cold_ms = cold.elapsed().as_millis();
    assert_eq!(snapshot.days.len(), 3650);
    assert_eq!(
        snapshot.days.iter().map(|d| d.deep_minutes).sum::<f64>(),
        100000.
    );
    assert_eq!(
        snapshot.days.iter().map(|d| d.coding_minutes).sum::<f64>(),
        10000.
    );
    assert_eq!(
        snapshot.days.iter().map(|d| d.music_count).sum::<i64>(),
        100000
    );
    assert_eq!(
        snapshot.days.iter().map(|d| d.journal_count).sum::<i64>(),
        50000
    );
    assert_eq!(
        snapshot
            .words
            .iter()
            .find(|w| w.word == "数学")
            .unwrap()
            .count,
        100000
    );
    let warm = std::time::Instant::now();
    let repeat = db.insights_snapshot(f).await.unwrap();
    let warm_ms = warm.elapsed().as_millis();
    assert_eq!(repeat.words[0].count, snapshot.words[0].count);
    let regular = std::time::Instant::now();
    db.insights_snapshot(Filter {
        from: (date(&end).unwrap() - Duration::days(6)).to_string(),
        until: end.clone(),
        today: end,
    })
    .await
    .unwrap();
    let regular_ms = regular.elapsed().as_millis();
    let output = json!({"fixture":"TempDir SQLite; generated from 2016-01-01; i modulo 3650","counts":{"days":3650,"focus_sessions":100000,"music_plays":100000,"journal_entries":50000,"problems":10000,"attempts":10000},"generated_ms":generated_ms,"ten_year_cold_ms":cold_ms,"ten_year_warm_ms":warm_ms,"seven_day_warm_ms":regular_ms,"correctness":"PASS: deep=100000 min,coding=10000 min,music=100000,journal=50000,数学=100000","build":"Windows debug root debug=0 jobs=1","body_profile":"short deterministic Chinese fixture; not worst-case megabyte journal bodies"});
    if let Ok(path) = std::env::var("LUMEN_PERF_REPORT") {
        std::fs::write(path, serde_json::to_string_pretty(&output).unwrap() + "\n").unwrap();
    }
    println!("{output}");
    assert!(
        regular_ms < 5000,
        "Regular query above 5s local acceptance ceiling"
    );
}
