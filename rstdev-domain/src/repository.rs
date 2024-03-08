use std::fmt::Debug;

use crate::BaseError;
use crate::entity::UID;

/// `Repository` is an trait abstraction used for `Repository Pattern` 
pub trait Repository {
    type Entity: Debug + Clone;
    type UIDType: UID;

    fn find_by_uid(&self, uid: Self::UIDType) -> Result<Self::Entity, BaseError>;
    fn save(&mut self, entity: Self::Entity) -> Result<(), BaseError>;
    fn remove(&mut self, uid: Self::UIDType) -> Result<(), BaseError>;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    use rst_common::standard::uuid;

    #[derive(Debug, Clone)]
    struct FakeUID {
        uid: String
    }

    impl FakeUID {
        fn new() -> Self {
            let uid = uuid::Uuid::new_v4();            
            Self { uid: uid.to_string() }
        }
    }

    impl UID for FakeUID {
        type Value = String;

        fn uid(&self) -> Self::Value {
            self.uid.clone()
        }
    }

    #[derive(Debug, Clone)]
    struct FakeEntity {
        uid: FakeUID
    }

    impl FakeEntity {
        fn new() -> Self {
            let uid = FakeUID::new();
            Self { uid }
        }
    }

    struct FakeRepo {
        db: HashMap<String, FakeEntity>
    }

    impl FakeRepo {
        fn new() -> Self {
            let db: HashMap<String, FakeEntity> = HashMap::new();
            Self { db }
        }
    }

    impl Repository for FakeRepo {
        type Entity = FakeEntity;
        type UIDType = FakeUID;

        fn find_by_uid(&self, uid: Self::UIDType) -> Result<Self::Entity, BaseError> {
            let get_entity = self.db.get(&uid.uid());
            match get_entity {
                Some(ent) => Ok(ent.clone()),
                None => Err(BaseError::RepositoryError("entity not found".to_string())) 
            }
        }

        fn remove(&mut self, uid: Self::UIDType) -> Result<(), BaseError> {
            self.db.remove(&uid.uid());
            Ok(())
        }

        fn save(&mut self, entity: Self::Entity) -> Result<(), BaseError> {
            self.db.insert(entity.uid.uid.to_string(), entity);
            Ok(())
        }
    }

    #[test]
    fn test_build_repo() {
        let mut repo = FakeRepo::new();
        let entity = FakeEntity::new();

        let _ = repo.save(entity.clone());
        let entity_loaded = repo.find_by_uid(entity.clone().uid);
        assert!(!entity_loaded.is_err());

        let entity2 = entity_loaded.unwrap();
        assert_eq!(entity.uid.uid().to_owned(), entity2.uid.uid().to_owned());

        let _ = repo.remove(entity.clone().uid);
        let find_entity = repo.find_by_uid(entity.clone().uid);
        assert!(find_entity.is_err());
        assert!(matches!(find_entity.unwrap_err(), BaseError::RepositoryError(_)))
    }
}