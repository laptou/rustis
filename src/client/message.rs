use smallvec::SmallVec;

use crate::{resp::Command, PushSender, PubSubSender, ValueSender, RetryReason};

#[allow(clippy::large_enum_variant)] 
#[derive(Debug)]
pub(crate) enum Commands {
    None,
    Single(Command),
    Batch(Vec<Command>),
}

impl Commands {
    pub fn len(&self) -> usize {
        match &self {
            Commands::None => 0,
            Commands::Single(_) => 1,
            Commands::Batch(commands) => commands.len(),
        }
    }
}

impl IntoIterator for Commands {
    type Item = Command;
    type IntoIter = CommandsIterator;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Commands::None => CommandsIterator::Single(None),
            Commands::Single(command) => CommandsIterator::Single(Some(command)),
            Commands::Batch(commands) => CommandsIterator::Batch(commands.into_iter()),
        }
    }
}

impl<'a> IntoIterator for &'a Commands {
    type Item = &'a Command;
    type IntoIter = RefCommandsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Commands::None => RefCommandsIterator::Single(None),
            Commands::Single(command) => RefCommandsIterator::Single(Some(command)),
            Commands::Batch(commands) => RefCommandsIterator::Batch(commands.iter()),
        }
    }
}

#[allow(clippy::large_enum_variant)] 
pub enum CommandsIterator {
    Single(Option<Command>),
    Batch(std::vec::IntoIter<Command>),
}

impl Iterator for CommandsIterator {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Single(command) => command.take(),
            Self::Batch(iter) => iter.next(),
        }
    }
}

pub enum RefCommandsIterator<'a> {
    Single(Option<&'a Command>),
    Batch(std::slice::Iter<'a, Command>),
}

impl<'a> Iterator for RefCommandsIterator<'a> {
    type Item = &'a Command;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Single(command) => command.take(),
            Self::Batch(iter) => iter.next(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Message {
    pub commands: Commands,
    pub value_sender: Option<ValueSender>,
    pub pub_sub_senders: Option<Vec<(Vec<u8>, PubSubSender)>>,
    pub push_sender: Option<PushSender>,
    pub retry_reasons: Option<SmallVec<[RetryReason; 10]>>,
}

impl Message {
    pub fn single(command: Command, value_sender: ValueSender) -> Self {
        Message {
            commands: Commands::Single(command),
            value_sender: Some(value_sender),
            pub_sub_senders: None,
            push_sender: None,
            retry_reasons: None,
        }
    }

    pub fn single_forget(command: Command) -> Self {
        Message {
            commands: Commands::Single(command),
            value_sender: None,
            pub_sub_senders: None,
            push_sender: None,
            retry_reasons: None,
        }
    }

    pub fn batch(commands: Vec<Command>, value_sender: ValueSender) -> Self {
        Message {
            commands: Commands::Batch(commands),
            value_sender: Some(value_sender),
            pub_sub_senders: None,
            push_sender: None,
            retry_reasons: None,
        }
    }

    pub fn pub_sub(
        command: Command,
        value_sender: ValueSender,
        pub_sub_senders: Vec<(Vec<u8>, PubSubSender)>,
    ) -> Self {
        Message {
            commands: Commands::Single(command),
            value_sender: Some(value_sender),
            pub_sub_senders: Some(pub_sub_senders),
            push_sender: None,
            retry_reasons: None,
        }
    }

    pub fn monitor(
        command: Command,
        value_sender: ValueSender,
        push_sender: PushSender,
    ) -> Self {
        Message {
            commands: Commands::Single(command),
            value_sender: Some(value_sender),
            pub_sub_senders: None,
            push_sender: Some(push_sender),
            retry_reasons: None,
        }
    }

    pub fn client_tracking_invalidation(push_sender: PushSender) -> Self {
        Message {
            commands: Commands::None,
            value_sender: None,
            pub_sub_senders: None,
            push_sender: Some(push_sender),
            retry_reasons: None,
        }
    }
}
