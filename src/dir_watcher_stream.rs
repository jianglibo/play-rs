use futures::{Async, Poll, Stream};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use futures::future::lazy;
use std::time::Duration;
use std::path::PathBuf;
use tokio::timer::Interval;


pub struct DirWatcher {
    // app_state: AppState,
    rx: std::sync::mpsc::Receiver<DebouncedEvent>,
    watcher: RecommendedWatcher,
    interval: Interval,
}

impl DirWatcher {
    pub fn new(watch_target: &str, debounce_duration: Duration, interval_duration: Duration, recursive_mode: RecursiveMode /*, app_state: AppState*/) -> DirWatcher {
        let watch_path = std::path::Path::new(watch_target);
        if !(watch_path.is_dir() && watch_path.exists()) {
            panic!("watch target {} does't exists.", watch_target);
        }
        let (tx, rx) = mpsc::channel();

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher: RecommendedWatcher = Watcher::new(tx, debounce_duration).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher
            .watch(watch_target, recursive_mode)
            .unwrap();
        info!("watch dir created.{:?}", rx);
        DirWatcher {
            // app_state,
            rx,
            watcher: watcher,
            interval: Interval::new_interval(interval_duration),
        }
    }
}

fn encode_msg(et: u8, pb: &PathBuf) -> Option<Vec<u8>> {
    let mut v = vec!(et, 0);
    v.extend(pb.to_str().unwrap().bytes());
    Some(v)
}

#[derive(Debug)]
struct FileChangeEvent<'a> {
    event_code: &'a u8,
    file_path: Option<&'a str>,
    after_path: Option<&'a str>,
}

fn decode_msg<'a>(vec_of_u8: &'a Vec<u8>) -> Option<FileChangeEvent<'a>> {
    if let Some((one, rest)) = vec_of_u8.split_first() {
        if let Some((zero, path_part)) = rest.split_first() {
            match one {
                7 => {
                    let mut iter = path_part.split(|c|c == &0);
                    if let (Some(a), Some(b)) = (iter.next(), iter.next()) {
                        Some(FileChangeEvent {
                            event_code: one,
                            file_path: Some(std::str::from_utf8(a).unwrap()),
                            after_path: Some(std::str::from_utf8(b).unwrap()),
                        })
                    } else {
                        None
                    }
                },
                _ =>  Some(FileChangeEvent {
                    event_code: one,
                    file_path: Some(std::str::from_utf8(path_part).unwrap()),
                    after_path: None,
                })
            }
        } else {
            None
        }
    } else {
        None
    }
}

impl Stream for DirWatcher {
    type Item = Vec<u8>;
    // The stream will never yield an error
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Vec<u8>>, ()> {
        try_ready!(
            self.interval.poll()
                // The interval can fail if the Tokio runtime is unavailable.
                // In this example, the error is ignored.
                .map_err(|_| ())
        );
        match self.rx.try_recv() {
            Ok(de) => match de {
                DebouncedEvent::NoticeWrite(pb) => Ok(Async::Ready(encode_msg(1, &pb))),
                DebouncedEvent::NoticeRemove(pb) => Ok(Async::Ready(encode_msg(2, &pb))),
                DebouncedEvent::Create(pb) => Ok(Async::Ready(encode_msg(3, &pb))), 
                DebouncedEvent::Write(pb) => Ok(Async::Ready(encode_msg(4, &pb))), 
                DebouncedEvent::Chmod(pb) => Ok(Async::Ready(encode_msg(5, &pb))), 
                DebouncedEvent::Remove(pb) => Ok(Async::Ready(encode_msg(6, &pb))),
                DebouncedEvent::Rename(src, dst) => {
                        let mut v = vec!(7, 0);
                        v.extend(src.to_str().unwrap().bytes());
                        v.push(0);
                        v.extend(dst.to_str().unwrap().bytes());
                        Ok(Async::Ready(Some(v)))
                },
                DebouncedEvent::Rescan => Ok(Async::Ready(Some(vec![8, 0]))),
                DebouncedEvent::Error(_notify_error, _option_pf) => Ok(Async::Ready(Some(vec![0]))),
            },
            Err(tre) => {
                match tre {
                    TryRecvError::Disconnected => {
                        error!("{:?}", tre);
                        // streams must not return Async::NotReady unless Async::NotReady was obtained by an inner stream or future
                        Ok(Async::Ready(None))
                    },
                    TryRecvError::Empty => {
                        Ok(Async::Ready(Some(Vec::new())))
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::fs::File;
    use std::io::Write;
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;
    use tempfile::tempdir;
    use crate::tests::tutil::init_log;
    use futures::{Future, Stream, Poll, Async};
    use std::fmt;


    pub struct Display10<T> {
    stream: T,
    curr: usize,
    count: usize,
}

impl<T> Display10<T> {
    fn new(stream: T) -> Display10<T> {
        Display10 {
            stream,
            curr: 0,
            count: 0,
        }
    }
}

impl<T> Future for Display10<T>
where
    T: Stream<Item = Vec<u8>>,
    T::Item: fmt::Debug,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> Poll<(), Self::Error> {
        while self.curr < 100 {
            // info!("start poll for {}th time.", self.curr);
            self.count += 1;
            let value = match try_ready!(self.stream.poll()) {
                Some(value) => value,
                // There were less than 10 values to display, terminate the
                // future.
                None => break,
            };
            if !value.is_empty() {
                let decoded = decode_msg(&value);
                println!("value #{} = {:?}", self.curr, decoded);
                self.curr += 1;
            }
        }
        info!("total poll times: {}", self.count);
        Ok(Async::Ready(()))
    }
}

    #[test]
    fn test_arbit() {
        init_log();
        let dir_watcher = DirWatcher::new("c:\\", Duration::from_secs(2), Duration::from_millis(10), RecursiveMode::Recursive);
        let display = Display10::new(dir_watcher);
        tokio::run(display);
    }
}
