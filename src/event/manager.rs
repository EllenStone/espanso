/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::event::{KeyEventReceiver, ActionEventReceiver, Event, SystemEventReceiver};
use std::sync::mpsc::Receiver;

pub trait EventManager {
    fn eventloop(&self);
}

pub struct DefaultEventManager<'a> {
    receive_channel: Receiver<Event>,
    key_receivers: Vec<&'a dyn KeyEventReceiver>,
    action_receivers: Vec<&'a dyn ActionEventReceiver>,
    system_receivers: Vec<&'a dyn SystemEventReceiver>,
}

impl<'a> DefaultEventManager<'a> {
    pub fn new(receive_channel: Receiver<Event>, key_receivers: Vec<&'a dyn KeyEventReceiver>,
               action_receivers: Vec<&'a dyn ActionEventReceiver>,
               system_receivers: Vec<&'a dyn SystemEventReceiver>) -> DefaultEventManager<'a> {
        DefaultEventManager {
            receive_channel,
            key_receivers,
            action_receivers,
            system_receivers
        }
    }
}

impl <'a> EventManager for DefaultEventManager<'a> {
    fn eventloop(&self) {
        loop {
            match self.receive_channel.recv() {
                Ok(event) => {
                    match event {
                        Event::Key(key_event) => {
                            self.key_receivers.iter().for_each(move |&receiver| receiver.on_key_event(key_event.clone()));
                        },
                        Event::Action(action_event) => {
                            self.action_receivers.iter().for_each(|&receiver| receiver.on_action_event(action_event.clone()));
                        }
                        Event::System(system_event) => {
                            self.system_receivers.iter().for_each(move |&receiver| receiver.on_system_event(system_event.clone()));
                        }
                    }
                },
                Err(e) => panic!("Broken event channel {}", e),
            }
        }
    }
}