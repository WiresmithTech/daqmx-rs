# Getting Started

The canonical repository is on github with a seperate repository for the -sys crate.

* [Crate Repo](https://github.com/WiresmithTech/daqmx-rs)
* [-sys Repo](https://github.com/WiresmithTech/ni-daqmx-sys)

At this early stage of the project there is a lot to still be worked out. 

I will consider pull requests on github but please first create an issue so we can discuss the changes you would like and how that fits into the development plans.

Minor issues like typos can be created immediately though.

# Testing

The unit tests do not require a hardware setup but are only basic since this code is mostly a wrapper for the -sys crate.

The integration tests require simulated hardware and are designed to exercise the API with the goal of identifying any soundness or interfacing issues. This does require some simulated hardware.

See the readme under the `test` folder for more information.