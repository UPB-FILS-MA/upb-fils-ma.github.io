---
layout: section
---
# USB 2.0
Universal Serial Bus

---
layout: two-cols
---
# Universal Serial Bus
2.0

- Used for communication between a host and several devices that each provide functions
- Two modes:
  - *host* - initiates the communication (usually a computer)
  - *device* - receives and transmits data when the *host* requests it
- each device has a 7 bit address assigned upon connect
  - maximum 127 devices connected to a USB host
- devices are interconnected using *hubs*
- USB devices tree

:: right ::

```mermaid
flowchart
	H(Host) --> Hub1(Hub)
	Hub1 --> Hub2(Hub)
	Hub1 --> D1(Device 1)
	Hub1 --> Hub3(Hub)
	Hub2 --> Hub4(Hub)
	Hub2 --> D3(Device 3)
	Hub4 --> D4(Device 4)
	Hub4 --> D5(Device 5)
	Hub3 --> D2(Device 2)

	classDef hub fill:#B0E3E6,stroke:#0E8088
	classDef host fill:#B1DDF0,stroke:#10739E
	classDef device fill:#FFE6CC,stroke:#D79B00
	classDef exception fill:#F8CECC,stroke:#B85450
	classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
	classDef start fill:#00ef00

	class H host
	class Hub1,Hub2,Hub3,Hub4 hub
	class D1,D2,D3,D4,D5 device
	class E exception
	class R3,F2 error
	class S3 start
```

---
---
# Bibliography
for this section

1. **Raspberry Pi Ltd**, *[RP2040 Datasheet](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf)*
   - Chapter 4 - *Peripherals*
     - Chapter 4.1 - *USB*

2. *[USB Made Simple](https://www.usbmadesimple.co.uk/)*

---
layout: two-cols
---
# USB Device

<style>
.two-columns {
    grid-template-columns: 3fr 6fr;
}
</style>

- can work as **host** or **device**, but not at the same time
- uses a differential line for transmission
- uses a 48 MHz clock
- maximum 16 endpoints (buffers)
  - *IN* - from **device** to **host**
  - *OUT* - from **host** to **device**
- endpoints 0 IN and OUT are used for control

:: right ::

<div align="center">
<img src="/usb/usb_device.svg" class="rounded">
</div>

---

# USB Packet
the smallest element of data transmission

<style>
img {
  background: #ffffff;
}
</style>

Token
<div align="center">
<img src="/usb/usb_packet_token.svg" class="rounded">
</div>

Data
<div align="center">
<img src="/usb/usb_packet_data.svg" class="rounded">
</div>

Handshake
<div align="center">
<img src="/usb/usb_packet_handshake.svg" class="rounded">
</div>


---

# Token Packet
usually asks for a data transmission

<style>
img {
  background: #ffffff;
}
</style>

| Type | PID | Description |
|-|-|-|
| *OUT* | `0001` | **host** wants to transmit data to the **device** |
| *IN* | `1001` | **host** wants to receive data from the **device** |
| *SETUP* | `1101` | **host** wants to setup the **device** |

Address: `ADDR`:`ENDP`

<div align="center">
<img src="/usb/usb_packet_token.svg" class="rounded">
</div>


---

# Data Packet
transmits data

<style>
img {
  background: #ffffff;
}
</style>

| Type | PID | Description |
|-|-|-|
| *DATA0* | `0011` | the data packet is the first one or follows after a `DATA1` packet |
| *DATA1* | `1011` | the data packet follows after a `DATA0` packet |

Data can be between 0 and 1024 bytes

<div align="center">
<img src="/usb/usb_packet_data.svg" class="rounded">
</div>


---

# Handshake Packet
acknowledges data

<style>
img {
  background: #ffffff;
}
</style>

| Type | PID | Description |
|-|-|-|
| *ACK* | `0010` | data has been **successfully received**  |
| *NACK* | `1010` | data has **not** been **successfully received** |
| *STALL* | `1110` | the device has an **error** |

<div align="center">
<img src="/usb/usb_packet_handshake.svg" class="rounded">
</div>


---
---
# Transmission Modes

- *Control* - used for configuration
- *Isochronous* - used for high bandwidth, best effort
- *Bulk* - used for low bandwidth, stream
- *Interrupt* - used for low bandwidth, guaranteed latency

---
---

# Control
used to control a device - ask for data

<div grid="~ cols-2 gap-5">

<div>

<v-click>

**Setup** - send a command (*GET_DESCRIPTOR*, ...)
```mermaid {scale: 0.8}
flowchart LR
	I(Idle) --> S
	S(Token
	SETUP) --> D(Data, 
	DATA0
	8 bytes)
	S -- Error --> I2(Idle)
	D --> A(Handshake
	ACK)
	D -- Error --> I2
	A --> I2

	classDef hub fill:#B0E3E6,stroke:#0E8088
	classDef host fill:#B1DDF0,stroke:#10739E
	classDef device fill:#FFE6CC,stroke:#D79B00
	classDef exception fill:#F8CECC,stroke:#B85450
	classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
	classDef start fill:#00ef00

	class S,D host
	class A device
	class I,I2 start
```

</v-click>

<v-click>

**Data** - *optional* several transfers, host transfers data
```mermaid {scale: 0.8}
flowchart LR
	I(Idle) --> S
	S(Token
	OUT) --> D1(Data
	DATA1)
	D1 -- Error --> I2(Idle)
	S -- Error --> I2
	D1 --> A(Handshake
	ACK)
	A --> I2

	classDef hub fill:#B0E3E6,stroke:#0E8088
	classDef host fill:#B1DDF0,stroke:#10739E
	classDef device fill:#FFE6CC,stroke:#D79B00
	classDef exception fill:#F8CECC,stroke:#B85450
	classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
	classDef start fill:#00ef00

	class S,D1 host
	class A device
	class I,I2 start
```

</v-click>

</div>

<div>

<v-after>

```mermaid {scale: 0.8}
flowchart LR
	I(Idle) --> S
	S(Token
	OUT) --> D0(Data
	DATA0)
	D0 -- Error --> I2(Idle)
	S -- Error --> I2
	D0 --> A(Handshake
	ACK)
	A --> I2

	classDef hub fill:#B0E3E6,stroke:#0E8088
	classDef host fill:#B1DDF0,stroke:#10739E
	classDef device fill:#FFE6CC,stroke:#D79B00
	classDef exception fill:#F8CECC,stroke:#B85450
	classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
	classDef start fill:#00ef00

	class S,D0 host
	class A device
	class I,I2 start
```

...

</v-after>

<v-click>

**Status** - report the status to the host

```mermaid {scale: 0.8}
flowchart LR
	I(Idle) --> S
	S(Token
	IN) --> D1(Data, 
	DATA1)
	S -- Error --> I2(Idle)
	D1 --> A(Handshake
	ACK)
	D1 -- Error --> I2
	A --> I2

	classDef hub fill:#B0E3E6,stroke:#0E8088
	classDef host fill:#B1DDF0,stroke:#10739E
	classDef device fill:#FFE6CC,stroke:#D79B00
	classDef exception fill:#F8CECC,stroke:#B85450
	classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
	classDef start fill:#00ef00

	class S,A host
	class D1 device
	class I,I2 start
```

</v-click>

</div>
</div>

---

# Control
used to control a device - send data

<div grid="~ cols-2 gap-5">

<div>

<v-click>

**Setup** - send a command (*SET_ADDRESS*, ...)
```mermaid {scale: 0.75}
flowchart LR
	I(Idle) --> S
	S(Token
	SETUP) --> D(Data, 
	DATA0
	8 bytes)
	S -- Error --> I2(Idle)
	D --> A(Handshake
	ACK)
	D -- Error --> I2
	A --> I2

	classDef hub fill:#B0E3E6,stroke:#0E8088
	classDef host fill:#B1DDF0,stroke:#10739E
	classDef device fill:#FFE6CC,stroke:#D79B00
	classDef exception fill:#F8CECC,stroke:#B85450
	classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
	classDef start fill:#00ef00

	class S,D host
	class A device
	class I,I2 start
```

</v-click>

<v-click>

**Data** - *optional* several transfers, device transfers the requested data
```mermaid {scale: 0.75}
flowchart LR
	I(Idle) --> S
	S(Token
	IN) --> D1(Data
	DATA1)
	D1 -- Error --> I2(Idle)
	S -- Error --> I2
	D1 --> A(Handshake
	ACK)
	A --> I2

	classDef hub fill:#B0E3E6,stroke:#0E8088
	classDef host fill:#B1DDF0,stroke:#10739E
	classDef device fill:#FFE6CC,stroke:#D79B00
	classDef exception fill:#F8CECC,stroke:#B85450
	classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
	classDef start fill:#00ef00

	class S,A host
	class D1 device
	class I,I2 start
```

</v-click>

</div>

<div>

<v-after>

```mermaid {scale: 0.8}
flowchart LR
	I(Idle) --> S
	S(Token
	IN) --> D0(Data
	DATA0)
	D0 -- Error --> I2(Idle)
	S -- Error --> I2
	D0 --> A(Handshake
	ACK)
	A --> I2

	classDef hub fill:#B0E3E6,stroke:#0E8088
	classDef host fill:#B1DDF0,stroke:#10739E
	classDef device fill:#FFE6CC,stroke:#D79B00
	classDef exception fill:#F8CECC,stroke:#B85450
	classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
	classDef start fill:#00ef00

	class S,A host
	class D0 device
	class I,I2 start
```

...

</v-after>

<v-click>

**Status** - report the status to the device

```mermaid {scale: 0.8}
flowchart LR
	I(Idle) --> S
	S(Token
	OUT) --> D1(Data, 
	DATA1)
	S -- Error --> I2(Idle)
	D1 --> A(Handshake
	ACK)
	D1 -- Error --> I2
	A --> I2

	classDef hub fill:#B0E3E6,stroke:#0E8088
	classDef host fill:#B1DDF0,stroke:#10739E
	classDef device fill:#FFE6CC,stroke:#D79B00
	classDef exception fill:#F8CECC,stroke:#B85450
	classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
	classDef start fill:#00ef00

	class S,D1 host
	class A device
	class I,I2 start
```

</v-click>

</div>
</div>


---
---
# Isochronous
fast but not reliable transfer

- has a guaranteed bandwidth
- allows data loss
- used for functions like streaming where loosing a packet has a minimal impact

<div grid="~ cols-2 gap-5">

<div>

<v-click>

**OUT** - transfer data from the host to the device
```mermaid {scale: 0.8}
flowchart LR
	I(Idle) --> S
	S(Token
	OUT) --> D0(Data, 
	DATAx)
	S -- Error --> I2(Idle)
	D0 --> I2

	classDef hub fill:#B0E3E6,stroke:#0E8088
	classDef host fill:#B1DDF0,stroke:#10739E
	classDef device fill:#FFE6CC,stroke:#D79B00
	classDef exception fill:#F8CECC,stroke:#B85450
	classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
	classDef start fill:#00ef00

	class S,D0 host
	class I,I2 start
```

</v-click>

</div>

<div>

<v-click>

**IN** - transfer data from the device to the host
```mermaid {scale: 0.8}
flowchart LR
	I(Idle) --> S
	S(Token
	IN) --> D0(Data, 
	DATAx)
	S -- Error --> I2(Idle)
	D0 --> I2(Idle)

	classDef hub fill:#B0E3E6,stroke:#0E8088
	classDef host fill:#B1DDF0,stroke:#10739E
	classDef device fill:#FFE6CC,stroke:#D79B00
	classDef exception fill:#F8CECC,stroke:#B85450
	classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
	classDef start fill:#00ef00

	class S host
	class D0 device
	class I,I2 start
```

</v-click>

</div>

</div>

---
---
# Bulk
slow, but reliable transfer

- does not have a guaranteed bandwidth
- secure transfer
- used for large data transfers where loosing packets is not permitted

<div grid="~ cols-2 gap-5">

<div>

<v-click>

**OUT** - transfer data from the host to the device
```mermaid {scale: 0.69}
flowchart LR
	I(Idle) --> S
	S(Token
	OUT) --> D0(Data, 
	DATAx)
	S -- Error --> I2(Idle)
	D0 -- Device Error --> SA(Handshake
	STALL)
	D0 -- Packet Error --> N(Handshake
	NACK)
	D0 --> A(Handshake
	ACK)
	A --> I2(Idle)
	N --> I2
	SA --> I2

	classDef hub fill:#B0E3E6,stroke:#0E8088
	classDef host fill:#B1DDF0,stroke:#10739E
	classDef device fill:#FFE6CC,stroke:#D79B00
	classDef exception fill:#F8CECC,stroke:#B85450
	classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
	classDef start fill:#00ef00

	class S,D0 host
	class A,N,SA device
	class I,I2 start
```

</v-click>

</div>

<div>

<v-click>

**IN** - transfer data from the device to the host
```mermaid {scale: 0.69}
flowchart LR
	I(Idle) --> S
	S(Token
	IN) --> D0(Data, 
	DATAx)
	S -- Error --> I2(Idle)
	D0 -- Device Error --> SA(Handshake
	STALL)
	D0 -- Packet Error --> N(Handshake
	NACK)
	D0 --> A(Handshake
	ACK)
	A --> I2
	N --> I2
	SA --> I2

	classDef hub fill:#B0E3E6,stroke:#0E8088
	classDef host fill:#B1DDF0,stroke:#10739E
	classDef device fill:#FFE6CC,stroke:#D79B00
	classDef exception fill:#F8CECC,stroke:#B85450
	classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
	classDef start fill:#00ef00

	class S,A,N,SA host
	class D0 device
	class I,I2 start
```

</v-click>

</div>

</div>

---
---
# Interrupt
transfer data at a minimum time interval

- the endpoint descriptor asks the host start an interrupt transfer at a time interval
- used for sending and receiving data at certain intervals

<div grid="~ cols-2 gap-5">

<div>

<v-click>

**OUT** - transfer data from the host to the device
```mermaid {scale: 0.69}
flowchart LR
	I(Idle) --> S
	S(Token
	OUT) --> D0(Data, 
	DATAx)
	S -- Error --> I2(Idle)
	D0 -- Device Error --> SA(Handshake
	STALL)
	D0 -- Packet Error --> N(Handshake
	NACK)
	D0 --> A(Handshake
	ACK)
	A --> I2(Idle)
	N --> I2
	SA --> I2

	classDef hub fill:#B0E3E6,stroke:#0E8088
	classDef host fill:#B1DDF0,stroke:#10739E
	classDef device fill:#FFE6CC,stroke:#D79B00
	classDef exception fill:#F8CECC,stroke:#B85450
	classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
	classDef start fill:#00ef00

	class S,D0 host
	class A,N,SA device
	class I,I2 start
```

</v-click>

</div>

<div>

<v-click>

**IN** - transfer data from the device to the host
```mermaid {scale: 0.69}
flowchart LR
	I(Idle) --> S
	S(Token
	IN) --> D0(Data, 
	DATAx)
	S -- Error --> I2(Idle)
	D0 -- Device Error --> SA(Handshake
	STALL)
	D0 -- Packet Error --> N(Handshake
	NACK)
	D0 --> A(Handshake
	ACK)
	A --> I2
	N --> I2
	SA --> I2

	classDef hub fill:#B0E3E6,stroke:#0E8088
	classDef host fill:#B1DDF0,stroke:#10739E
	classDef device fill:#FFE6CC,stroke:#D79B00
	classDef exception fill:#F8CECC,stroke:#B85450
	classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
	classDef start fill:#00ef00

	class S,A,N,SA host
	class D0 device
	class I,I2 start
```

</v-click>

</div>

</div>


---
layout: two-cols
---
# Device Organization
configuration, interfaces, endpoints

- a device can have multiple configurations
  - for instance different functionality based on power consumption
- a configuration has multiple interfaces
  - a device can perform multiple functions
  - Debugger
  - Serial Port
- each interface has multiple interfaces attached
  - endpoints are used for data transfer
  - maximum 16 endpoints, can be configured IN and OUT
- the device reports the descriptors in this order

:: right ::

<div align="center">
<img src="/usb/usb_device_descriptor.svg" class="rounded w-70">
</div>

---
layout: two-cols
---

# Connection

<style>
img {
  background: #ffffff;
}
</style>

```mermaid {scale: 0.45}
sequenceDiagram
	create participant Host
    activate Host
	create participant Device
    Host -->> Device: device plugged in, Reset
    Host ->> +Device: SETUP: GET_DEVICE_DESCRIPTOR (address 0, EP 0 IN)
    Device -->> -Host: DATA: Device Descriptor (EP 0 IN)
    Host -->> Device: Reset
    Host ->> +Device: SETUP: SET_ADDRESS (A) (address 0, EP 0 OUT)
    Device -->> -Host: ACK: (EP 0 OUT)
    Host ->> +Device: SETUP: GET_DEVICE_DESCRIPTOR (address A, EP 0 IN)
    Device -->> Host: DATA: Device Descriptor (EP 0 IN)
    Host -->> Device: ACK (EP 0 IN)
	deactivate Device
	Host ->> +Device: SETUP: GET_CONFIGURATION_DESCRIPTOR (1) (address A, EP 0 IN)
    Device -->> Host: DATA: Configuration Descriptor (EP 0 IN)
    Host -->> Device: ACK (EP 0 IN)
	deactivate Device
	Host ->> +Device: SETUP: SET_CONFIGURATION (1) (address A, EP 0 OUT)
    Device -->> Host: ACK (EP 0 OUT)
    deactivate Device
    deactivate Host
```

:: right ::

# Token SETUP Packet

The DATA packet of the SETUP Control Transfer

<div align="center">
<img src="/usb/usb_setup_request.svg" class="rounded">
</div>

*bmRequestType* field

<div align="center">
<img src="/usb/usb_bmrequest.svg" class="rounded">
</div>


---
---
# USB 1.0 and 2.0 Modes
| Mode | Speed | Version |
|-|-|-|
| Low Speed | 1.5 Mbit/s | 1.0 |
| Full Speed | 12 Mbit/s | 1.0 |
| High Speed | 480 Mbit/s | 2.0 |

---
---

# Facts

| | | |
|-|-|-|
| Transmission | *half duplex* | data must be sent in one direction at one time |
| Clock | *independent* | the **host** and the **device** must synchronize their clocks |
| Wires | *DP* / *DM* | data is sent in a differential way |
| Devices | *1 host* <br> *several devices* | a receiver and a transmitter |
| Speed | *480 MBbit/s* |  |

---
layout: two-cols
---
# Embassy API
for RP2040, setup the device

```rust {lines: false}
use embassy_rp::usb::{Driver, Instance, InterruptHandler};
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

let driver = Driver::new(p.USB, Irqs);

let mut config = Config::new(0xc0de, 0xcafe);
config.manufacturer = Some("Embassy");
config.product = Some("USB-serial example");
config.serial_number = Some("12345678");
config.max_power = 100;
config.max_packet_size_0 = 64;

// Required for windows compatibility.
config.device_class = 0xEF;
config.device_sub_class = 0x02;
config.device_protocol = 0x01;
config.composite_with_iads = true;
```

:: right ::

```rust {lines: false}
// It needs some buffers for building the descriptors.
let mut config_descriptor = [0; 256];
let mut bos_descriptor = [0; 256];
let mut control_buf = [0; 64];

let mut state = State::new();

let mut builder = Builder::new(
	driver,
	config,
	&mut config_descriptor,
	&mut bos_descriptor,
	&mut [], // no msos descriptors
	&mut control_buf,
);

// Create classes on the builder.
let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);

// Build the builder.
let mut usb = builder.build();

// Run the USB device.
let usb_driver = usb.run();
```

---
---
# Embassy API
for RP2040, use the USB device

```rust
let echo_loop = async {
	loop {
		class.wait_connection().await;
		info!("Connected");
		let _ = echo(&mut class).await;
		info!("Disconnected");
	}
};

// Run everything concurrently.
join(usb_driver, echo_loop).await;
```

```rust
async fn echo<'d, T: Instance + 'd>(class: &mut CdcAcmClass<'d, Driver<'d, T>>) -> Result<(), EndpointError> {
    let mut buf = [0; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];
        info!("data: {:x}", data);
        class.write_packet(data).await?;
    }
}

```
