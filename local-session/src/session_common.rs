use muzzman_lib::prelude::*;

use crate::TLocalSession;

impl TSessionCommon for Box<dyn TLocalSession> {
    fn get_name(&self, uid: UID) -> SessionResult<String> {
        let inner = move || {
            Ok(match self.as_ref().get(uid)? {
                crate::Wraper::Element(element) => element.element.read().unwrap().name.clone(),
                crate::Wraper::Location(location) => location.location.read().unwrap().name.clone(),
                crate::Wraper::Module(module) => module.module.read().unwrap().name.clone(),
            })
        };
        inner().map_err(|e| SessionError::GetName(Box::new(e)))
    }

    fn set_name(&self, uid: UID, name: String) -> SessionResult<()> {
        match self
            .as_ref()
            .get(uid)
            .map_err(|e| SessionError::SetName(Box::new(e)))?
        {
            crate::Wraper::Element(e) => e.element.write().unwrap().name = name,
            crate::Wraper::Location(l) => l.location.write().unwrap().name = name,
            crate::Wraper::Module(m) => m.module.write().unwrap().name = name,
        }
        Ok(())
    }

    fn get_desc(&self, uid: UID) -> SessionResult<String> {
        let inner = move || {
            Ok(match self.as_ref().get(uid)? {
                crate::Wraper::Element(element) => element.element.read().unwrap().desc.clone(),
                crate::Wraper::Location(location) => location.location.read().unwrap().desc.clone(),
                crate::Wraper::Module(module) => module.module.read().unwrap().desc.clone(),
            })
        };
        inner().map_err(|e| SessionError::GetDesc(Box::new(e)))
    }

    fn set_desc(&self, uid: UID, desc: String) -> SessionResult<()> {
        match self
            .as_ref()
            .get(uid)
            .map_err(|e| SessionError::SetName(Box::new(e)))?
        {
            crate::Wraper::Element(e) => e.element.write().unwrap().desc = desc,
            crate::Wraper::Location(l) => l.location.write().unwrap().desc = desc,
            crate::Wraper::Module(m) => m.module.write().unwrap().desc = desc,
        }
        Ok(())
    }

    fn emit(&self, uid: UID, event: Event) -> SessionResult<()> {
        todo!()
    }

    fn notify(&self, uid: UID, to: UID, event: Event) -> SessionResult<()> {
        todo!()
    }

    fn subscribe(&self, uid: UID, to: UID) -> SessionResult<()> {
        todo!()
    }

    fn unsubscribe(&self, uid: UID, from: UID) -> SessionResult<()> {
        todo!()
    }

    fn events(&self, uid: UID, consume: bool) -> SessionResult<Vec<Event>> {
        todo!()
    }

    fn push_event(&self, uid: UID, event: Event) -> SessionResult<()> {
        todo!()
    }
}
