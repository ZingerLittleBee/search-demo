use crate::model::search::full_text::FullTextSearchResult;
use crate::model::search::vector::VectorSearchResult;
use crate::model::search::ID;

pub struct Rank;

#[derive(Clone, Debug)]
pub struct RankResult {
    pub id: ID,
}

impl Rank {
    /// 按照分数之和排序
    /// 越大越靠前
    pub fn full_text_rank(data: Vec<FullTextSearchResult>) -> anyhow::Result<Vec<RankResult>> {
        let mut res = data;
        res.sort_by(|a, b| {
            b.score
                .iter()
                .map(|x| x.1)
                .sum::<f32>()
                .partial_cmp(&a.score.iter().map(|x| x.1).sum::<f32>())
                .ok_or(std::cmp::Ordering::Equal)
                .unwrap()
        });
        Ok(res
            .iter()
            .map(|x| RankResult { id: x.id.clone() })
            .collect())
    }
}

impl Rank {
    /// 按照 distance 值排序
    /// 越小越靠前
    pub fn vector_rank(data: Vec<VectorSearchResult>) -> anyhow::Result<Vec<RankResult>> {
        let mut res = data;
        res.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .ok_or(std::cmp::Ordering::Equal)
                .unwrap()
        });
        Ok(res
            .iter()
            .map(|x| RankResult { id: x.id.clone() })
            .collect())
    }
}

#[cfg(test)]
mod test {
    use crate::model::search::full_text::FullTextSearchResult;
    use crate::model::search::vector::VectorSearchResult;
    use crate::model::search::ID;
    use crate::rank::Rank;

    #[test]
    fn test_vector_rank() {
        let data = vec![
            VectorSearchResult {
                id: ID::new("1".to_string(), "text"),
                distance: 0.1,
            },
            VectorSearchResult {
                id: ID::new("2".to_string(), "text"),
                distance: 0.2,
            },
            VectorSearchResult {
                id: ID::new("3".to_string(), "text"),
                distance: 0.3,
            },
        ];
        let res = Rank::vector_rank(data).unwrap();
        assert_eq!(res.len(), 3);
        assert_eq!(res[0].id.id(), "1");
        assert_eq!(res[1].id.id(), "2");
        assert_eq!(res[2].id.id(), "3");
    }

    #[test]
    fn test_full_text_rank() {
        let data = vec![
            FullTextSearchResult {
                id: ID::new("1".to_string(), "text"),
                score: vec![("a".to_string(), 0.1), ("b".to_string(), 0.2)],
            },
            FullTextSearchResult {
                id: ID::new("2".to_string(), "text"),
                score: vec![("c".to_string(), 0.2), ("d".to_string(), 0.3)],
            },
            FullTextSearchResult {
                id: ID::new("3".to_string(), "text"),
                score: vec![("e".to_string(), 0.3), ("f".to_string(), 0.4)],
            },
        ];
        let res = Rank::full_text_rank(data);
        assert_eq!(res.is_ok(), true);
        let res = res.unwrap();
        assert_eq!(res.len(), 3);
        assert_eq!(res[0].id.id(), "3");
        assert_eq!(res[1].id.id(), "2");
        assert_eq!(res[2].id.id(), "1");
    }
}
