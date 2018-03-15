# netdmx

[![Build Status](https://travis-ci.org/fooker/netdmx.svg?branch=master)](https://travis-ci.org/fooker/netdmx)

`netdmx` is a DMX controller driver with a very simple network interface.
It listens for UDP packes and sends the received data to a DMX universe.

`netdmx` is made to decouple the signal generating software, like a light controller or a simple animation generator, from the device specific driver implementation.
By just using the network interface, the signal generators must only be capable of sending UDP packets.


## Supported controllers

The following controllers are currently supported:
* Anyma uDMX
* Eurolite USB-DMX512 PRO Interface MK2 

More controllers will come over time. Pull-Requests are very welcome.


## Installation

To build and install `netdmx` the following dependencies are required:
* Rust >= 1.2
* libusb-1.0

After cloning the repository, it can be build with the following command:
``` bash
cargo build --release
```

The resulting binary can be found in `target/release/netdmx`.


## Usage

Starting `netdmx` requires the USB-DMX controller to be attached.
Then it can be started wit hthe following command:
``` bash
netdmx --type TYPE
```

Now, `netdmx` initializes the controller and is listening for UDP packets on `localhost:34254`.
To send data, you can simply send a UDP datagram containing the values for all 512 DMX channels.
The first byte in the datagram is then send to the first DMX channel, and so on. 


## Remarks

`netdmx` will not refresh the channels in any manner.
If the controller does not implement auto-refresh, the DMX universe is not updated at a regular interval.
Therefore the generating software should resend the current state even if there is no change.

Currently, there is no support for multiple devices of the same type on the same host system.
`netdmx` will pick the firs on it can find, but there is no guarantee of order in this.


## Generators

This is a list of known signal generators.
If you want to add your animation generator or light controller to the list, feel free to open a Issue or Pull-Request.

* [illuminad](https://git.maglab.space/mag.lab/illuminad) - MQTT controlled rotation through HCL color space 
