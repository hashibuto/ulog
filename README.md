# ulog
micro structured logging for rust.  it's not meant to be feature packed, it's meant to be a simple, no-frills structured logger.

## Usage
There are two primary usage modes.  The first is to use the global logging functions:

```
use ulog;

# set the global logging level
ulog.set_log_level(ulog.DEBUG);

# log an info level message
ulog.info("this is some informative message")

# log a debug level message
ulog.debug("hey, something happened here")

# log with some additional structured data
ulog.info("something happened").data("name", name).data("id", id.to_string());
```

```
time="2025-01-13T20:33:34.640Z" level="INFO" message="this is some informative message"
time="2025-01-13T20:33:34.640Z" level="DEBUG" message="hey, something happened here"
time="2025-01-13T20:33:34.641Z" level="INFO" message="something happened" name="bobbo" id="032bf63e-0b83-4608-95b3-47f36ff68a4d"
```

The second mode of usage is to create a logger instance.  This might be useful for cases when certain data should be automatically included with each log message:

```
use ulog;

let mut logger = Ulogger::new();
logger.add_data("id", "032bf63e-0b83-4608-95b3-47f36ff68a4d");
logger.info("something happened").data("thing", "x");
logger.info("something happened again").data("thing", "y");
```

```
time="2025-01-13T20:33:34.641Z" level="INFO" message="something happened" id="032bf63e-0b83-4608-95b3-47f36ff68a4d" thing="x"
time="2025-01-13T20:33:34.641Z" level="INFO" message="something happened again" id="032bf63e-0b83-4608-95b3-47f36ff68a4d" thing="y"
```