# Integration Tests

This folder contains integration tests which will be automatically run with `cargo test`.

In order for these to run you will need the following simulated hardware configured:

* `PXISlot2` - PXIe-6363 or similar X Series device.


## Running the Tests

By default, cargo runs tests in parallel.

You must run the integration tests with `cargo test -- --test-threads=1` to prevent conflicts.