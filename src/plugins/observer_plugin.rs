use uuid::Uuid;
use zmq::Socket;
use event_engine::{plugins::Plugin, events::Event};
use event_engine::errors::EngineError;
use event_engine::events::EventType;
use crate::{events, config::errors::Errors};
use crate::traps_utils;

use log::{info, error, debug};
use std::{thread, time};

struct ObserverPlugin {
    name: String,
    id: Uuid,
}
impl Plugin for ObserverPlugin {

    /// The entry point for the plugin. The engine will start the plugin in its own
    /// thread and execute this function.  The pub_socket is used by the plugin to 
    /// publish new events.  The sub_socket is used by the plugin to get events 
    /// published by other plugins.
    fn start(
        &self,
        pub_socket: Socket,
        sub_socket: Socket,
    ) -> Result<(), EngineError> {

        // Announce our arrival.
        info!("{}", format!("{}", Errors::PluginStarted(self.name.clone(), self.get_id().to_hyphenated().to_string())));
        thread::sleep(time::Duration::new(1, 0));

        // Send our alive event.
        let ev = events::PluginStartedEvent::new(self.get_id().clone(), self.name.clone());
        let bytes = match ev.to_bytes() {
            Ok(v) => v,
            Err(e) => {
                // Log the error and abort if we can't serialize our start up message.
                let msg = format!("{}", Errors::EventToBytesError(self.name.clone(), ev.get_name(), e.to_string()));
                error!("{}", msg);
                return Err(EngineError::PluginExecutionError(self.name.clone(), self.get_id().to_hyphenated().to_string(), msg));
            } 
        };

        // Send the event serialization succeeded.
        match pub_socket.send(bytes, 0) {
            Ok(_) => (),
            Err(e) => {
                // Log the error and abort if we can't send our start up message.
                let msg = format!("{}", Errors::SocketSendError(self.name.clone(), ev.get_name(), e.to_string()));
                error!("{}", msg);
                return Err(EngineError::PluginExecutionError(self.name.clone(), self.get_id().to_hyphenated().to_string(), msg));
            }
        };

        // Enter our infinite work loop.
        loop {
            // Wait on the subsciption socket.
            let bytes = match sub_socket.recv_bytes(0) {
                Ok(b) => b,
                Err(e) => {
                    // We log error and then move on. It would probably be a good idea to 
                    // pause before continuing if there are too many errors in a short period
                    // of time.  This would avoid filling up the log file and burning cycles
                    // when things go sideways for a while.
                    let msg = format!("{}", Errors::SocketRecvError(self.name.clone(), e.to_string()));
                    error!("{}", msg);
                    continue;
                }
            };

            // Get the generated event and its type.
            let gen_event = match traps_utils::bytes_to_gen_event(&bytes) {
                Ok(tuple)=> tuple,
                Err(e)=> {
                    error!("{}", e.to_string());
                    continue;
                }
            };

            // Process events we expect; log and disregard all others.
            let terminate = match gen_event.event_type().variant_name() {
                Some("NewImageEvent") => {
                    self.record_event("NewImageEvent");
                    false
                },
                Some("ImageReceivedEvent") => {
                    self.record_event("ImageReceivedEvent");
                    false
                },
                Some("ImageScoredEvent") => {
                    self.record_event("ImageScoredEvent");
                    false
                },
                Some("ImageStoredEvent") => {
                    self.record_event("ImageStoredEvent");
                    false
                },
                Some("ImageDeletedEvent") => {
                    self.record_event("ImageDeletedEvent");
                    false
                },
                Some("PluginStartedEvent") => {
                    self.record_event("PluginStartedEvent");
                    false
                },
                Some("PluginTerminatingEvent") => {
                    self.record_event("PluginTerminatingEvent");
                    false
                },
                Some("PluginTerminateEvent") => {
                    // Determine whether we are the target of this terminate event. The called method
                    // will return true if this plugin should shutdown.
                    self.record_event("PluginTerminateEvent");
                    traps_utils::process_plugin_terminate_event(gen_event, &self.id, &self.name)
                },
                None => {
                    let msg = format!("{}", Errors::EventNoneError(self.name.clone()));
                    error!("{}", msg);
                    false
                },
                Some(unexpected) => {
                    let msg = format!("{}", Errors::EventNotHandledError(self.name.clone(), unexpected.to_string()));
                    error!("{}", msg);
                    false
                }
            };
        
            // Determine if we should terminate our event read loop.
            if terminate {
                // Clean up and send the terminating event.
                traps_utils::send_terminating_event(&self.name, self.id.clone(), &pub_socket);
                break;
            }
        }

        // Shutting down.
        Ok(())
    }

    /// Return the event subscriptions, as a vector of strings, that this plugin is interested in.
    fn get_subscriptions(&self) -> Result<Vec<Box<dyn EventType>>, EngineError> {
        // This plugin subscribes to all events.  When events change so must this list.
        Ok(vec![
            Box::new(events::NewImageEvent::new(Uuid::new_v4(), String::from("fake"), vec![])),
            Box::new(events::ImageReceivedEvent::new(Uuid::new_v4())),
            Box::new(events::ImageScoredEvent::new(Uuid::new_v4(), vec![])),
            Box::new(events::ImageStoredEvent::new(Uuid::new_v4(), String::from("path"))),
            Box::new(events::ImageDeletedEvent::new(Uuid::new_v4())),
            Box::new(events::PluginTerminateEvent::new(Uuid::new_v4(), String::from("*"))),
            Box::new(events::PluginTerminatingEvent::new(Uuid::new_v4(), String::from("ObserverPlugin"))), 
            Box::new(events::PluginStartedEvent::new(Uuid::new_v4(), String::from("ObserverPlugin"))),
        ])
    }

    /// Returns the unique id for this plugin.
    fn get_id(&self) -> Uuid {self.id.clone()}
}

impl ObserverPlugin {
    pub fn new() -> Self {
        ObserverPlugin {
            name: "ObserverPlugin".to_string(),
            id: Uuid::new_v4(),
        }
    }

    // Just log receiving the event.
    fn record_event(&self, event_name: &str) {
        info!("\n  -> {} received event {}", self.name, String::from(event_name));
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn here_i_am() {
        println!("file test: observer.rs");
    }
}
