# fern-journal

Rust library for logging with Fern into the Systemd Journal

Example usage:

```
fern::Dispatch::new().chain(fern_journal::get_logger()).apply()?;
```
