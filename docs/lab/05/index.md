---
sidebar_position: 6
description: Asynchronous Programming with Embassy
slug: /lab/05
---

# 05 - Async/Await

This lab will teach you the principles of asynchronous programming, and its application in Embassy-rs.


## Resources

TBD

## Async functions

Up to now, during the labs, we've seen that, in order to be able to do multiple different actions "at once", we would use *tasks*. We would let the main function run, while also doing another action seemingly "in parallel" inside of another task. 
Let's take the following example: if we want to blink an LED every second while also waiting for a button press to do something else, we would need to spawn a new task in which we would wait for the button press, while blinking the LED in the main function. When thinking of how exactly this works, you would probably think that the task is running on a separate *thread* than the main function.
That would work in other cases, but not ours. Since only one thread can independently run per processor core, that means that, since we are using only one core of the RP2040 (which actually has only 2), we would only be able to run one thread at a time. And we don't have an OS to help us. So how exactly does the task wait for the button press in parallel with the LED blinking? 
Short answer is: it doesn't. In reality, both functions runs asynchronously. 

### Tasks

Tasks in Embassy are what we call "asynchronous functions". Asynchronous functions are different from normal functions, in the sense that they allow asynchronous code execution. Let's take an example from the previous lab:
```rust
#[embassy_executor::task]
async fn button_pressed(mut led: Output<'static, PIN_X>, mut button: Input<'static, PIN_X>) {
    loop {
	info!("waiting for button press");
        button.wait_for_falling_edge().await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let button = Input::new(peripherals.PIN_X, Pull::Up);
    let led2 = Output::new(peripherals.PIN_X, Level::Low);

    spawner.spawn(button_pressed(led2, button)).unwrap();

    let mut led = Output::new(peripherals.PIN_X, Level::Low);

    loop {
        led.toggle();

        Timer::after_millis(200).await;
    }
}
```
In this example, we notice that both the `button_pressed` and `main` functions are declared as `async`, telling the compiler to treat them as asynchronous functions. Inside the main function (which is also a task, actually), we blink the LED:

```rust
loop {
    led.toggle();

    Timer::after_millis(200).await;
}
```

## Await keyword

After setting the timer, our main function would need to wait until the alarm fires after 200 ms. Instead of just waiting and blocking the current and *only* thread of execution, it could do another action in the meantime. This is where the `await` keyword comes into play.
When using `await` inside of an asynchronous function, we are telling the MCU that this action might take more time to finish, so *do something else* until it's ready. Basically, the execution flow of the main function is halted exactly where `await` is used, and the MCU starts running another task. In our case, it would halt the main function while waiting for the alarm to go off and it could start running the code inside the `button_pressed` task.
```rust
loop {
    info!("waiting for button press");
    button.wait_for_falling_edge().await;
}
```
We can see that here, we also use the `wait_for_falling_edge` function asynchronously, meaning that until we get a signal that a button has been pressed, the MCU can decide to do other stuff. If it has nothing else to do, it goes to sleep, until it receives a signal that either the button has been pressed, or the timer has run out. Then, it will resume execution of the function where the action has completed. 
If, for example, the button is pressed, execution flow will resume inside of the `button_pressed` task, until it is interrupted by the next `await` in that function. If the timer also runs out by the time the `button_pressed` task execution reaches the next `await`, the continuation of the main function will be queued, until it gets to be run.
This method of development allows our programs to run seemingly "in parallel", without the need of multiple threads. Each task *voluntarily* pauses its execution and passes control over to whatever other task needs it. This means that it's the task's business to allow other tasks to run while it's idly waiting for something to happen on its end.
:::note
On an operating system, it would be the scheduler's job to decide when and for how long processes get to run.
:::

## Futures

Rust has a special datatype that represents an action that will complete sometime in the future. By using `await` on a `Future`, we are passing control to another task until the Future completes.
:::info
In the `button_pressed` task, the `wait_for_falling_edge` returns a `Future`, which is then `await`ed.
:::
This is a simplified version of what a `Future` in Rust really looks like:
```rust
enum Poll<T> {
    Pending,
    Ready(T),
}
trait Future {
   type Output;
   fn poll(&mut self) -> Poll<Self::Output>;
}
```
The `Future` has an `Output` type, that represents the type of the result that it will return once it completes. For `wait_for_falling_edge()`, the Output type is `()` (nothing).
The function `poll` returns a Poll type, which can either be `Pending`, or `Ready<T>` (T will be output in this case).
Let's break down what all of this means. A `Future` needs to be checked on, every now and then, to see what its status is. This is the job of the **Executor**. The executor must regularly ask the `Future` if it's completed, or if it needs more time before it can give a result. We can say that the `Future` is `poll`ed, and depending on whether it's ready to give a result or not, it gives its status as `Pending` or `Ready`. If it's still pending, it needs more time before it can return a result, so the executor moves on to poll another `Future`. Whenever the `Future` is completed, it returns `Ready` once polled, and the executor returns execution back to the function where the `Future` completed.
sequenceDiagram
    autonumber
    Executor->>+Future: poll()
    loop until the Future finishes all the requests to the Hardware
        Future->>+Hardware: execute_next_action()
        Hardware-->>Future: in_progress()
        note right of Hardware: performs the action in parallel
        Future-->>-Executor: Poll::Pending
        note over Executor: sleeps until an event arrives
        note right of Hardware: sends an event when job is done (interrupt)
        Hardware--)-Executor: event
        Executor->>+Future: poll()
    end
    Future->>Hardware: read_value()
    Hardware-->>Future: value
    Future-->>-Executor: Poll::Ready(value)

:::note
An efficient executor will not poll all tasks. Instead, tasks signal the executor that they are ready to make progress by using a `Waker`.
:::

TODO: real future in Rust

Under the hood, the Rust compiler is actually transforming our asynchronous function into a state-machine. That is why we can only use `await` inside of an `async` function, because it needs to be treated differently than an ordinary function in order to work asynchronously.

:::info
Asynchronous programming is widely used in web development. In JavaScript, the equivalent of a `Future` would be a `Promise`.
:::

Read more about how async/await works in Rust [here](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html).

## Channels

Up to this point, to be able to share peripherals across multiple tasks, we have been using global `Mutex`s or passing them directly as parameters to the tasks. But there are other, more convenient ways to send data to and from tasks. Instead of having to make global, static variables that are shared by tasks, we could choose to only send the information that we need from one task to another. To achieve this, we can use *channels*.

**Channels** allow a unidirectional flow of information between two endpoints: the *Sender* and the *Receiver*. The sender sends a value through the channel, and the receiver receives this value once it is ready to do so. Until it is ready, the data will be stored inside a queue. Channels in Embassy are *Multiple Producer, Multiple Consumer*, which means that we can have a channel associated with multiple senders and multiple receivers. 

:::info
Executors actually internally use channels when dealing with tasks. (how?)
:::

To use a channel in Embassy, we first need to declare a static instance of the channel. 

```rust
static CHANNEL: Channel<ThreadModeRawMutex, bool, 64> = Channel::new();
```

The second generic argument of `Channel` is the type of data that we will be sending through the channel. The third argument is the maximum number of values that can be stored in the queue. 

Let's say we spawn a task `task1` that runs a timer. Every second, we want to toggle an LED in the `main` function, based on the timer running in `task1`. For this, `task1` would need to 
send a signal to the `main` program every time the 1 second alarm has fired, meaning the task and the main program would share the channel. `task1` would *send* over the channel, and 
`main` would *receive*.

For this we need to pass a `Sender` endpoint to `task1` once we spawn it.

```rust
// ---- fn main() ----
spawner.spawn(task1(CHANNEL.sender())).unwrap();
```

The `sender()` function returns a new sender for the given channel.

Inside `task1`, we would just set a timer and wait until it fires. After it fires, we send a signal through the channel, to indicate that 1 second has elapsed.

```rust
#[embassy_executor::task]
async fn task1(channel_sender: Sender<'static, ThreadModeRawMutex, bool, 64>) {
    loop {
        Timer::after_secs(1).await;
        channel_sender.send(true).await;
    }
}
```

In the `main` function, we need to then wait for the signal, and once it's received, toggle the LED.

```rust
// ---- fn main() ----
loop {
    let value = CHANNEL.receive().await;
    match value {
        true => led.toggle().unwrap(),
        false => info!("We got something else")
    }
}
```

:::info
The reason we need all of this is because Rust doesn't allow us to mutably borrow more than once. To use a peripheral (say PWM) inside multiple tasks, 
we would need to either move it inside the task entirely, or use a mutable reference to it. If we have multiple tasks, though, once we move our peripheral variable *inside* the first task,
we can't pass it to another task, because the value was *moved* inside that task completely. And if we wanted to pass it as a mutable reference instead, we would quickly realize that Rust 
doesn't allow multiple mutable references at once, to avoid concurrent modifications. So this is why we need to either declare a global, static `Mutex` that any task can access, to ensure 
that the value cannot be modified concurrently by two different tasks, or use channels and keep the peripheral inside the `main` function.
:::

## Exercises

1. Connect an LED to GP0, an RGB LED to GP1, GP2, GP3 and a potentiometer to ADC0. Use Kicad to draw the schematic.
2. Change the monochromatic LED's intensity, using button A (SW_A) and button B(SW_B) on the Pico Explorer. Button A will increase the intensity, and button B will decrease it.

:::tip
- Use PWM to control the intensity.
- Create two tasks, one for button A, one for button B. Use a channel to send commands from each button task to the main task.
:::

3. Control the RGB LED's color with the button A and button B (red -> green -> blue). Button A will switch the color in one direction (red -> green -> blue) and button B in the other 
direction (red -> blue -> green).
4. In addition to the two buttons, control the RGB LED's intensity with the potentiometer.

5. Print to the screen of the Pico Explorer the color of the RGB LED and its intensity.