use super::{model::*, mutations, *};
pub(super) async fn create(
    tx: &mut Transaction<'_, Sqlite>,
    id: &str,
    timestamp: &str,
) -> Result<()> {
    let journey=EntityInput{title:"Fudan 2028".into(),journey_id:None,data:Details{description:"可编辑学习规划模板；日期是学习安排，非招生政策。400 是 planning target，不是分数预测。".into(),start_date:Some("2026-10-01".into()),target_date:Some("2028-04-30".into()),preferred_minutes:1500,maximum_minutes:2100,planning_target:Some(400),..Details::default()}};
    mutations::save(tx, id, "journey", None, &journey, timestamp).await?;
    for (position, (title, start, end)) in [
        ("论文优先 / 准备期", "2026-10-01", "2026-10-31"),
        ("基础 I", "2026-11-01", "2027-02-28"),
        ("基础 II", "2027-03-01", "2027-06-30"),
        ("暑期强化", "2027-07-01", "2027-08-31"),
        ("历年题", "2027-09-01", "2027-10-31"),
        ("最终冲刺", "2027-11-01", "2027-12-31"),
        ("笔试后 / 面试过渡", "2028-01-01", "2028-02-29"),
        ("面试强化", "2028-03-01", "2028-04-30"),
    ]
    .into_iter()
    .enumerate()
    {
        let input = EntityInput {
            title: title.into(),
            journey_id: Some(id.into()),
            data: Details {
                position: position as i64,
                start_date: Some(start.into()),
                target_date: Some(end.into()),
                ..Details::default()
            },
        };
        mutations::save(
            tx,
            &uuid::Uuid::new_v4().to_string(),
            "phase",
            None,
            &input,
            timestamp,
        )
        .await?;
    }
    let mut cs = None;
    for title in [
        "数学", "408", "英语", "政治", "编程", "研究", "竞赛", "其他",
    ] {
        let subject = uuid::Uuid::new_v4().to_string();
        mutations::save(
            tx,
            &subject,
            "subject",
            None,
            &EntityInput {
                title: title.into(),
                journey_id: Some(id.into()),
                data: Details::default(),
            },
            timestamp,
        )
        .await?;
        if title == "408" {
            cs = Some(subject)
        }
    }
    for title in ["数据结构", "计算机组成原理", "操作系统", "计算机网络"] {
        mutations::save(
            tx,
            &uuid::Uuid::new_v4().to_string(),
            "subject",
            None,
            &EntityInput {
                title: title.into(),
                journey_id: Some(id.into()),
                data: Details {
                    parent_id: cs.clone(),
                    ..Details::default()
                },
            },
            timestamp,
        )
        .await?;
    }
    Ok(())
}
