---
layout: section 
---
# Concurrency
Preemptive and Cooperative

---
---
# Bibliography
for this section

**Brad Solomon**, *[Async IO in Python: A Complete Walkthrough](https://realpython.com/async-io-python/)* 

---
layout: two-cols
---
# Preemptive Concurrency

<style>
.two-columns {
    grid-template-columns: 3fr 5fr;
}
</style>

- MCUs are usually *single core*[^rp2040]
- Tasks in parallel require an OS[^interrupts]
- Tasks can be suspended at any time
- **Switching** the task is **expensive**
- Tasks that do a lot of I/O which makes the **switching time longer than** the actual **processing time**

:: right ::

```mermaid
sequenceDiagram
    autonumber
    loop
        activate Task1
        SysTick --)OS: scheduler alarm
        deactivate Task1
        activate OS
        note over OS: save state for Task 1
        note over OS: restore state for Task 2
        OS-->> -Task2: schedule
        activate Task2
        SysTick --)OS: scheduler alarm
        deactivate Task2
        activate OS
        note over OS: save state for Task 2
        note over OS: restore state for Task 1
        OS--) -Task1: schedule
        activate Task1
        Task1 ->> Task1: wait for hardware
        deactivate Task1
    end
```

[^rp2040]: RP2040 is a dual core MCU, we use only one core
[^interrupts]: Running in an ISR is not considered a normal task

---
layout: two-cols
---
# Cooperative Concurrency

<style>
.two-columns {
    grid-template-columns: 3fr 5fr;
}
</style>

- tasks cannot be interrupted[^interrupts]
- **hardware** works in an **asynchronous** way
- tasks cooperate
  - give up the MCU for other tasks to use it while they wait for hardware
- there is **no need for an OS**, everything is done in **one single flow**
- **no penalty** for saving and restoring the state

[^interrupts]: except for ISR

:: right ::

```mermaid
sequenceDiagram
    autonumber
    loop
        activate Scheduler
        note over Scheduler: has next task ready?
        alt
            Scheduler->> -Task1: schedule
            activate Task1
            Task1 ->> +Hardware: request
            Hardware -->> Task1: in progress
            Task1 ->> -Scheduler: .await
        else
            activate Scheduler
            Scheduler->> -Task2: schedule
            activate Task2
            Task2 ->> Task2: process
            Task2 ->> -Scheduler: .await
        else
            note over Scheduler: wait event
            Hardware --) -Scheduler: event
        end
    end
```
