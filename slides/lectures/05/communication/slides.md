---
layout: section
---
# Communication
between tasks

---
layout: two-cols
---

# Synchronization
safely share data between tasks

- [`NoopMutex`](https://docs.embassy.dev/embassy-sync/git/default/blocking_mutex/type.NoopMutex.html) - used for data shared between tasks within the **same executor** 
- [`CriticalSectionMutex`](https://docs.embassy.dev/embassy-sync/git/default/blocking_mutex/type.CriticalSectionMutex.html) - used for data shared between multiple executors, ISRs and cores 
- [`ThreadModeMutex`](https://docs.embassy.dev/embassy-sync/git/default/blocking_mutex/struct.ThreadModeMutex.html) - used for data shared between tasks within **low priority executors** (**not** running in **ISRs** mode)

:: right ::

<div align="center">
<img src="/executor/isr_executor.svg" class="rounded">
</div>

- ISRs are executed in parallel with tasks
- embassy allows registering priority executors, that run tasks in ISRs
- some MCUs have multiple cores


---
---
# Channels
send data from a task to another

Embassy provides four types of channels 
| Type | Description |
|-|-|
| [`Channel`](https://docs.embassy.dev/embassy-sync/git/default/channel/struct.Channel.html) | A Multiple Producer Multiple Consumer (MPMC) channel. Each message is only received by a single consumer. |
| [`PriorityChannel`](https://docs.embassy.dev/embassy-sync/git/default/priority_channel/struct.PriorityChannel.html) | A Multiple Producer Multiple Consumer (MPMC) channel. Each message is only received by a single |consumer. Higher priority items are sifted to the front of the channel. |
| [`Signal`](https://docs.embassy.dev/embassy-sync/git/default/pubsub/struct.PubSubChannel.html) | Signalling latest value to a single consumer. |
| [`PubSubChannel`](https://docs.embassy.dev/embassy-sync/git/default/signal/struct.Signal.html) | A broadcast channel (publish-subscribe) channel. Each message is received by all consumers. |

---
---
# Channel
multiple produces multiple consumer

[`Channel`](https://docs.embassy.dev/embassy-sync/git/default/channel/struct.Channel.html) - A Multiple Producer Multiple Consumer (MPMC) channel. Each message is only received by a single consumer.

```mermaid
flowchart LR
    Task1 --> Channel
    Task2 --> Channel
    Channel --> D{Distributor}
    D --> Task3
    D --> Task4
    D --> Task5
```

---
---
# PriorityChannel

[`PriorityChannel`](https://docs.embassy.dev/embassy-sync/git/default/priority_channel/struct.PriorityChannel.html) - A Multiple Producer Multiple Consumer (MPMC) channel. Each message is only received by a single |consumer. Higher priority items are sifted to the front of the channel. 

```mermaid
flowchart LR
    Task1 --> P1(Priority 1)
    Task1 --> P2(Priority 2)
    Task2 --> P1
    Task2 --> P2
    P1 --> P2
    P2 --> D{Distributor}
    D --> Task3
    D --> Task4
    D --> Task5
```

---
---
# PubSubChannel
multiple produces multiple consumer

[`PubSubChannel`](https://docs.embassy.dev/embassy-sync/git/default/signal/struct.Signal.html) - A broadcast channel (publish-subscribe) channel. Each message is received by all consumers.

```mermaid
flowchart LR
    Task1 --> Channel
    Task2 --> Channel
    Channel --> Task3
    Channel --> Task4
    Channel --> Task5
```

---
---
# Channel Example

```rust{all|1|2|5,7,14,17-25|5,8-14}
enum LedState { On, Off }
static CHANNEL: Channel<ThreadModeRawMutex, LedState, 64> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // init led
    spawner.spawn(toggle_led(CHANNEL.sender(), Duration::from_millis(500))));
    loop {
        match CHANNEL.receive().await {
            LedState::On => led.on(),
            LedState::Off => led.off()
        }
    }
}

#[embassy_executor::task]
async fn execute_led(command: Sender<'static, ThreadModeRawMutex, LedState, 64>, delay: Duration) {
    let mut ticker = Ticker::every(delay);
    loop {
        control.send(LedState::On).await;
        ticker.next().await;
        control.send(LedState::Off).await;
        ticker.next().await;
    }
}
```
