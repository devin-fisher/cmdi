use crate::opencli::v0_1::OptionElement;
use color_eyre::eyre::WrapErr;
use ratatui::crossterm::event::{self, Event as CrosstermEvent};
use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

/// The frequency at which tick events are emitted.
const TICK_FPS: f64 = 15.0;

/// Representation of all possible events.
#[derive(Clone, Debug)]
pub enum Event {
    /// An event that is emitted on a regular schedule.
    ///
    /// Use this event to run any code which has to run outside of being a direct response to a user
    /// event. e.g. polling external systems, updating animations, or rendering the UI based on a
    /// fixed frame rate.
    Tick,
    /// Crossterm events.
    ///
    /// These events are emitted by the terminal.
    Crossterm(CrosstermEvent),
    OptionSelection(OptionElement),
    Selection(Vec<usize>),
    NoSelection,
    /// Direct exit event
    ///
    ///
    Exit,
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    sender: mpsc::Sender<Event>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,

    pub read_events: Arc<AtomicBool>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`] and spawns a new thread to handle events.
    pub fn new() -> Self {
        let read_events = Arc::new(AtomicBool::new(true));
        let (sender, receiver) = mpsc::channel();
        let actor = EventThread::new(
            sender.clone(),
            read_events.clone(),
        );
        thread::spawn(|| actor.run());
        Self {
            sender,
            receiver,
            read_events,
        }
    }

    /// Receives an event from the sender.
    ///
    /// This function blocks until an event is received.
    ///
    /// # Errors
    ///
    /// This function returns an error if the sender channel is disconnected. This can happen if an
    /// error occurs in the event thread. In practice, this should not happen unless there is a
    /// problem with the underlying terminal.
    pub fn next(&self) -> color_eyre::Result<Event> {
        Ok(
            self.receiver
                .recv()?,
        )
    }

    /// Queue an event to be sent to the event receiver.
    ///
    /// This is useful for sending events to the event handler which will be processed by the next
    /// iteration of the application's event loop.
    pub fn send(&self, event: Event) {
        // Ignore the result as the reciever cannot be dropped while this struct still has a
        // reference to it
        let _ = self
            .sender
            .send(event);
    }

    pub fn enable_event_thread(&self) {
        // TODO make this clear that it just disables crossterm events
        self.read_events
            .store(
                true,
                Ordering::SeqCst,
            );
    }

    /// Disable reading crossterm events
    pub fn disable_events_thread(&self) {
        self.read_events
            .store(
                false,
                Ordering::SeqCst,
            );
        let tick_interval = Duration::from_secs_f64(1.0 / TICK_FPS);
        sleep(tick_interval * 2) // TODO this is hacky, I need to design a better loop (we need a way to know when the EventThread loop is disabled)
    }
}

/// A thread that handles reading crossterm events and emitting tick events on a regular schedule.
struct EventThread {
    /// Event sender channel.
    sender: mpsc::Sender<Event>,
    read_events: Arc<AtomicBool>,
}

impl EventThread {
    /// Constructs a new instance of [`EventThread`].
    fn new(sender: mpsc::Sender<Event>, read_events: Arc<AtomicBool>) -> Self {
        Self {
            sender,
            read_events,
        }
    }

    /// Runs the event thread.
    ///
    /// This function emits tick events at a fixed rate and polls for crossterm events in between.
    fn run(self) -> color_eyre::Result<()> {
        let tick_interval = Duration::from_secs_f64(1.0 / TICK_FPS);
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_interval.saturating_sub(last_tick.elapsed());

            // poll for crossterm events, ensuring that we don't block the tick interval
            if self
                .read_events
                .load(Ordering::SeqCst)
            {
                if event::poll(timeout).wrap_err("failed to poll for crossterm events")? {
                    last_tick = Instant::now();
                    match event::read().wrap_err("failed to read crossterm event")? {
                        CrosstermEvent::Key(key)
                            if key.modifiers == KeyModifiers::CONTROL
                                && key.code == KeyCode::Char('c') =>
                        {
                            self.send(Event::Exit)
                        }
                        e => self.send(Event::Crossterm(e)),
                    }
                    continue;
                }
            } else {
                sleep(tick_interval * 2); // TODO make this configurable
                continue;
            }

            // emit tick events at a fixed rate
            if last_tick.elapsed() >= tick_interval {
                last_tick = Instant::now();
                self.send(Event::Tick);
            }
        }
    }

    /// Sends an event to the receiver.
    fn send(&self, event: Event) {
        // Ignores the result because shutting down the app drops the receiver, which causes the send
        // operation to fail. This is expected behavior and should not panic.
        let _ = self
            .sender
            .send(event);
    }
}
