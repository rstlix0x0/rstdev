use rst_common::standard::chrono::Timelike;

/// `UID` is a value object used as unique identity that must have by each of identity
///
/// This trait using associated type of [`UID::Value`] to fill any possible types. Usually
/// it will either a string (`UUID`) or an integer (`AUTO_INCREMENT`), but it also be able
/// to used an id like `MongoDB ID Object Hash`
pub trait UID {
    type Value;

    fn uid(&self) -> Self::Value;
}

pub trait Timestamp {
    type CreatedAt: Timelike;
    type UpdatedAt: Timelike;

    fn created_at(&self) -> Self::CreatedAt;
    fn updated_at(&self) -> Self::UpdatedAt;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rst_common::standard::chrono::{DateTime, Utc};

    struct FakeTimestamp {
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>
    }

    impl Default for FakeTimestamp {
        fn default() -> Self {
            Self { created_at: Utc::now(), updated_at: Utc::now() }
        }
    }

    impl Timestamp for FakeTimestamp {
        type CreatedAt = DateTime<Utc>;
        type UpdatedAt = DateTime<Utc>;

        fn created_at(&self) -> Self::CreatedAt {
            self.created_at
        }

        fn updated_at(&self) -> Self::UpdatedAt {
            self.updated_at
        }
    }

    #[test]
    fn test_timestamp() {
        let fts = FakeTimestamp::default();
        assert!(!fts.created_at().to_string().is_empty());
        assert!(!fts.updated_at().to_string().is_empty())
    }
}