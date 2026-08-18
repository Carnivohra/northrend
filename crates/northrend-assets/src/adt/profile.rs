#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdtProfile {
    MonolithicMclq,
    MonolithicMfbo,
    MonolithicMh2o,
    Split,
    Modern,
}

impl AdtProfile {
    pub(super) const fn detect(has_mfbo: bool, has_mh2o: bool) -> Self {
        if has_mh2o {
            Self::MonolithicMh2o
        } else if has_mfbo {
            Self::MonolithicMfbo
        } else {
            Self::MonolithicMclq
        }
    }
}
