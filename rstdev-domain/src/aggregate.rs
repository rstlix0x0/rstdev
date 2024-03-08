/// `domain_event` is a module from `aggregate` used to manage domain's events
/// by implementing `Observer Pattern`
pub mod domain_event {
    use crate::BaseError;
    
    use rst_common::standard::serde::Serialize;

    pub type EventName = String;

    pub trait Event {
        type TPayload: Serialize + Clone + Send + Sync;

        fn name(&self) -> EventName;
        fn payload(&self) -> Self::TPayload;
    }

    pub trait Observer<TPayloadType, TEvent>
    where
        TPayloadType: Serialize + Clone + Send + Sync,
        TEvent: Event<TPayload = TPayloadType>,
    {
        fn handle(&self, event: TEvent) -> Result<(), BaseError>;
    }

    pub trait Publisher {
        fn emit(&self, event: EventName) -> Result<(), BaseError>;
    }

}
