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
        let inner = move || {
            let (event, subscribers) = match self.as_ref().get(uid)? {
                crate::Wraper::Element(e) => {
                    let event =
                        Event::From(e.element.read().unwrap().id.uid, Box::new(event.clone()));
                    let ids = e
                        .events
                        .read()
                        .unwrap()
                        .subscribers
                        .iter()
                        .cloned()
                        .collect::<Vec<UID>>();
                    (event, ids)
                }
                crate::Wraper::Location(l) => {
                    let event =
                        Event::From(l.location.read().unwrap().id.uid, Box::new(event.clone()));
                    let ids = l
                        .events
                        .read()
                        .unwrap()
                        .subscribers
                        .iter()
                        .cloned()
                        .collect::<Vec<UID>>();
                    (event, ids)
                }
                _ => return Err(SessionError::IsNotAnElementOrLocation),
            };

            let results = subscribers
                .into_iter()
                .map(|subscriber| {
                    match self.as_ref().get(subscriber)? {
                        crate::Wraper::Element(element) => {
                            element
                                .events
                                .write()
                                .unwrap()
                                .events
                                .push_back(event.clone());
                            if let Some(module) = element.element.read().unwrap().module.clone() {
                                let module = self.as_ref().module(module.uid)?;
                                let module = module.module.read().unwrap();
                                let module = &module.module;
                                let mut storage = element.storage.write().unwrap();
                                module.element_on_event(
                                    element.element.clone(),
                                    event.clone(),
                                    &mut storage,
                                )?;
                            }
                        }
                        crate::Wraper::Location(location) => {
                            location
                                .events
                                .write()
                                .unwrap()
                                .events
                                .push_back(event.clone());
                            if let Some(module) = location.location.read().unwrap().module.clone() {
                                let module = self.as_ref().module(module.uid)?;
                                let module = module.module.read().unwrap();
                                let module = &module.module;
                                let mut storage = location.storage.write().unwrap();
                                module.location_on_event(
                                    location.location.clone(),
                                    event.clone(),
                                    &mut storage,
                                )?;
                            }
                        }
                        _ => {}
                    }
                    SessionResult::Ok(())
                })
                .filter_map(|r| r.err())
                .collect::<Vec<SessionError>>();
            if results.is_empty() {
                Ok(())
            } else {
                Err(SessionError::Errors(results))
            }
        };
        inner().map_err(|e| SessionError::Emit(Box::new(e)))
    }

    fn notify(&self, uid: UID, to: UID, event: Event) -> SessionResult<()> {
        let inner = move || {
            let event = Event::From(uid, Box::new(event));
            match self.as_ref().get(to)? {
                crate::Wraper::Element(element) => {
                    element
                        .events
                        .write()
                        .unwrap()
                        .events
                        .push_back(event.clone());
                    if let Some(module) = element.element.read().unwrap().module.clone() {
                        let module = self.as_ref().module(module.uid)?;
                        let module = module.module.read().unwrap();
                        let module = &module.module;
                        let mut storage = element.storage.write().unwrap();
                        module.element_on_event(
                            element.element.clone(),
                            event.clone(),
                            &mut storage,
                        )?;
                    }
                }
                crate::Wraper::Location(location) => {
                    location
                        .events
                        .write()
                        .unwrap()
                        .events
                        .push_back(event.clone());
                    if let Some(module) = location.location.read().unwrap().module.clone() {
                        let module = self.as_ref().module(module.uid)?;
                        let module = module.module.read().unwrap();
                        let module = &module.module;
                        let mut storage = location.storage.write().unwrap();
                        module.location_on_event(
                            location.location.clone(),
                            event.clone(),
                            &mut storage,
                        )?;
                    }
                }
                _ => return Err(SessionError::IsNotAnElementOrLocation),
            }
            Ok(())
        };
        inner().map_err(|e| SessionError::Notify(Box::new(e)))
    }

    fn subscribe(&self, uid: UID, to: UID) -> SessionResult<()> {
        let inner = move || {
            match self.as_ref().get(to)? {
                crate::Wraper::Element(e) => {
                    e.events.write().unwrap().subscribers.insert(uid);
                }
                crate::Wraper::Location(l) => {
                    l.events.write().unwrap().subscribers.insert(uid);
                }
                _ => return Err(SessionError::IsNotAnElementOrLocation),
            }
            Ok(())
        };
        inner().map_err(|e| SessionError::Subscribe(Box::new(e)))
    }

    fn unsubscribe(&self, uid: UID, from: UID) -> SessionResult<()> {
        let inner = move || {
            match self.as_ref().get(from)? {
                crate::Wraper::Element(e) => {
                    e.events.write().unwrap().subscribers.remove(&uid);
                }
                crate::Wraper::Location(l) => {
                    l.events.write().unwrap().subscribers.remove(&uid);
                }
                _ => return Err(SessionError::IsNotAnElementOrLocation),
            }
            Ok(())
        };
        inner().map_err(|e| SessionError::Subscribe(Box::new(e)))
    }

    fn events(&self, uid: UID, consume: bool) -> SessionResult<Vec<Event>> {
        todo!()
    }

    fn push_event(&self, uid: UID, event: Event) -> SessionResult<()> {
        todo!()
    }

    fn get_buffer_size(&self, uid: UID) -> SessionResult<usize> {
        todo!()
    }

    fn set_buffer_size(&self, uid: UID, size: usize) -> SessionResult<()> {
        todo!()
    }

    fn remaining(&self, uid: UID) -> SessionResult<usize> {
        todo!()
    }

    fn read(&self, uid: UID, len: usize) -> SessionResult<Vec<u8>> {
        todo!()
    }

    fn write(&self, uid: UID, data: &[u8]) -> SessionResult<usize> {
        let inner = move || {
            let event = Event::NewData(data.to_vec());
            match self.as_ref().get(uid)? {
                crate::Wraper::Element(element) => {
                    element
                        .events
                        .write()
                        .unwrap()
                        .events
                        .push_back(event.clone());
                    if let Some(module) = element.element.read().unwrap().module.clone() {
                        let module = self.as_ref().module(module.uid)?;
                        let module = module.module.read().unwrap();
                        let module = &module.module;
                        let mut storage = element.storage.write().unwrap();
                        module.element_on_event(
                            element.element.clone(),
                            event.clone(),
                            &mut storage,
                        )?;
                    }
                }
                crate::Wraper::Location(location) => {
                    location
                        .events
                        .write()
                        .unwrap()
                        .events
                        .push_back(event.clone());
                    if let Some(module) = location.location.read().unwrap().module.clone() {
                        let module = self.as_ref().module(module.uid)?;
                        let module = module.module.read().unwrap();
                        let module = &module.module;
                        let mut storage = location.storage.write().unwrap();
                        module.location_on_event(
                            location.location.clone(),
                            event.clone(),
                            &mut storage,
                        )?;
                    }
                }
                _ => return Err(SessionError::IsNotAnElementOrLocation),
            };
            Ok(data.len())
        };
        inner().map_err(|e| SessionError::Write(Box::new(e)))
    }
}
