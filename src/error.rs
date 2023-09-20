use crate::prelude::RawLibraryError;

#[derive(Clone, Debug)]
pub enum SessionError {
    UIDWasDestroyed,
    InvalidUID,
    UIDIsNotAModule,
    UIDIsNotALocation,
    UIDIsNotAElement,

    IsNotAnElementOrLocation,

    RawModule(RawLibraryError),

    NoSession,
    NoModule,

    InvalidStatus,
    InvalidSettings(Vec<String>),

    ThereAreLessLocations,
    ThereAreLessElements,

    NoPermission,
    IsRoot,

    Errors(Vec<SessionError>),
    Custom(String),

    // Common
    GetName(Box<SessionError>),
    SetName(Box<SessionError>),

    GetDesc(Box<SessionError>),
    SetDesc(Box<SessionError>),

    Emit(Box<SessionError>),
    Notify(Box<SessionError>),

    Subscribe(Box<SessionError>),
    UnSubscribe(Box<SessionError>),

    Events(Box<SessionError>),
    PushEvent(Box<SessionError>),

    GetBufferSize(Box<SessionError>),
    SetBufferSize(Box<SessionError>),

    Remaining(Box<SessionError>),

    Read(Box<SessionError>),
    Write(Box<SessionError>),

    // Element
    CreateElement(Box<SessionError>),
    GetElement(Box<SessionError>),
    MoveElement(Box<SessionError>),
    ElementPath(Box<SessionError>),

    ElementGetParent(Box<SessionError>),

    ElementGetEnabled(Box<SessionError>),
    ElementSetEnabled(Box<SessionError>),

    ElementGetPath(Box<SessionError>),
    ElementSetPath(Box<SessionError>),

    ElementIsCompleted(Box<SessionError>),
    ElementIsError(Box<SessionError>),

    ElementGetStatuses(Box<SessionError>),
    ElementSetStatuses(Box<SessionError>),

    ElementGetStatus(Box<SessionError>),
    ElementSetStatus(Box<SessionError>),

    ElementGetStatusStr(Box<SessionError>),

    ElementGetUrl(Box<SessionError>),
    ElementSetUrl(Box<SessionError>),

    ElementGetProgress(Box<SessionError>),
    ElementGetDownloadSpeed(Box<SessionError>),
    ElementGetUploadSpeed(Box<SessionError>),
    ElementGetDownloadTotal(Box<SessionError>),
    ElementGetUploadTotal(Box<SessionError>),

    ElementGetData(Box<SessionError>),
    ElementSetData(Box<SessionError>),

    ElementGetSettings(Box<SessionError>),
    ElementSetSettings(Box<SessionError>),

    ElementGetModule(Box<SessionError>),
    ElementSetModule(Box<SessionError>),

    ElementWait(Box<SessionError>),

    DestroyElement(Box<SessionError>),

    // Location
    CreateLocation(Box<SessionError>),
    GetLocation(Box<SessionError>),
    GetDefaultLocation(Box<SessionError>),

    LocationGetParent(Box<SessionError>),

    LocationGetLocationsLen(Box<SessionError>),
    LocationGetLocations(Box<SessionError>),

    LocationGetElementsLen(Box<SessionError>),
    LocationGetElements(Box<SessionError>),

    LocationGetEnabled(Box<SessionError>),
    LocationSetEnabled(Box<SessionError>),

    LocationGetPath(Box<SessionError>),
    LocationSetPath(Box<SessionError>),

    LocationIsCompleted(Box<SessionError>),
    LocationIsError(Box<SessionError>),

    LocationGetStatuses(Box<SessionError>),
    LocationSetStatuses(Box<SessionError>),

    LocationGetStatus(Box<SessionError>),
    LocationSetStatus(Box<SessionError>),

    LocationGetStatusStr(Box<SessionError>),

    LocationGetProgress(Box<SessionError>),
    LocationGetDownloadSpeed(Box<SessionError>),
    LocationGetUploadSpeed(Box<SessionError>),
    LocationGetDownloadTotal(Box<SessionError>),
    LocationGetUploadTotal(Box<SessionError>),

    LocationGetData(Box<SessionError>),
    LocationSetData(Box<SessionError>),

    LocationGetSettings(Box<SessionError>),
    LocationSetSettings(Box<SessionError>),

    LocationGetModule(Box<SessionError>),
    LocationSetModule(Box<SessionError>),

    MoveLocation(Box<SessionError>),
    LocationPath(Box<SessionError>),

    DestroyLocation(Box<SessionError>),
}
