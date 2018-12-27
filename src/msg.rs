use std::process;
use std::str;
use std::vec;

use serde;
use serde_json;

use error::*;

/// Messages returned from a cargo sub-command.
pub struct MessageItr(vec::IntoIter<Message>);

impl MessageItr {
    pub(crate) fn from_command(mut cmd: process::Command) -> CargoResult<MessageItr> {
        let output = cmd
            .output()
            .map_err(|e| CargoError::new(ErrorKind::InvalidCommand).set_cause(e))?;
        if !output.status.success() {
            return Err(CargoError::new(ErrorKind::CommandFailed)
                .set_context(String::from_utf8_lossy(&output.stderr)));
        }

        let messages: Vec<Message> = str::from_utf8(&output.stdout)
            .expect("json to be UTF-8")
            .split('\n')
            .map(|s| Message {
                content: s.to_owned(),
            })
            .collect();

        Ok(Self {
            0: messages.into_iter(),
        })
    }
}

impl Iterator for MessageItr {
    type Item = Message;

    #[inline]
    fn next(&mut self) -> Option<Message> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.0.count()
    }
}

/// An individual message from a cargo sub-command.
pub struct Message {
    content: String,
}

impl Message {
    /// Deserialize the message.
    pub fn convert<'a, T>(&'a self) -> CargoResult<T>
    where
        T: serde::Deserialize<'a>,
    {
        let data = serde_json::from_str(self.content.as_str())
            .map_err(|e| CargoError::new(ErrorKind::InvalidOutput).set_cause(e))?;
        Ok(data)
    }
}
