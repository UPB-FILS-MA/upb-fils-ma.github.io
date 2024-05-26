# Smart Door Lock System
Smart door lock: Keyless security for your space.

Author: Nistorescu Maria-Lorena
GitHub Project Link*: (https://github.com/LorenaNistorescu/SmartDoorLockSystem)

:::

## Description

The project is about a smart door lock system using Raspberry Pi Pico W. 

The door can be unlocked in 2 ways: RFID MODULE, SENSORS( knock knock_ knock knock ).
I intend to make a function that sends me back an e-mail which says how many times the door was unlocked in a day. Incredible, right?

## Motivation

Why did I choose this project?

Well, I went in a trip to Italy 2 months ago and I stayed to an airbnb that had a smart door lock system with password.

I knew about the project at school, and at that moment seemed very interesting to me the idea of a digital lock.
So I wanted to implement it and, maybe use it every day life.


## Architecture 

![SDOOR](https://github.com/UPB-FILS-MA/upb-fils-ma.github.io/assets/120091173/9e9a5397-384d-43d5-ad6c-95f6c4b8de39)

## Log

<!-- write every week your progress here -->

### Week 6 - 12 May
In this week, I tried to find more resources regarding to the mobile application I intend to make in order to unlock the door.

### Week 7 - 19 May
In this week, I finally finished the hardware part, including the kicad schematics.

### Week 20 - 26 May
In this week, I tried to do software part, my libraries seem to have multiple errors and I could not resolve all of them.

## Hardware

- *Raspberry Pi Pico W (microcontroller):* The main control unit for the smart door lock system, responsible for processing data, running the necessary functions, and communicating with other components.
  
- *RFID MODULE (for unlocking the door):* Reads RFID tags/cards to allow authorized access through the smart door lock system.
  
- *TTP223:* Capacitive touch sensor used for touch-based inputs, such as entering a code to unlock the door.
  
- *2N2222:* NPN transistor that may be used for switching purposes in the circuit, such as controlling the relay module or other components.
  
- *BUZZER 5V:* Emits an audible alert, like a beep or chime, to provide feedback when the door is unlocked or in case of an error.
  
- *RELAY MODULE:* Controls the electronic door lock mechanism, allowing the microcontroller to trigger the locking or unlocking of the door.
  
- *ELECTRONIC DOOR LOCK:* The physical locking mechanism that secures the door, controlled by the relay module based on input from the microcontroller.
  
- *Power Supply 12V:* Supplies the necessary power to the components in the system, ensuring they function properly.
  
- *Module DC-DC Step Down LM2596S  (connected to power supply):* Regulates the 12V power supply to provide lower voltage levels required by certain components in the system, such as the Raspberry Pi Pico W.


### Kicad Schematics

Shortly, I am going to admit that the KICAD schematics took me, with all the coffee breaks included, somewhere around 16-20 hours. :)
Also, I tried to make the schematics clearer, and maybe, you can help me to notice if you see any other mistakes.

Legend of the KICAD schematics: 
| Symbol | Component           |
|--------|---------------------|
| U1     | RFID                |
| U2     | SENSOR TOUCH        |
| U4     | LM5296S             |
| U6     | POWER SUPPLY        |
| U7     | LOCK                |
| U8     | RELAY MODULE        |
| Q1     | NPN transistor 2N2222 |

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 

| [Device](link://to/device) | This is used ... | [price](link://to/store) |



-->

| Device                                                                                                                 | Usage                  | Price                                                                                                                       |
|------------------------------------------------------------------------------------------------------------------------|------------------------|-----------------------------------------------------------------------------------------------------------------------------|
| [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html)                | The microcontroller    | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html)                               |
| [Electronic Door Lock 12V](https://cleste.ro/incuietoare-electrica-12v.html)                                           | Locking mechanism      | [30,74 RON]                                                                                                                 |
| [Module RFID RC522, similar with MFRC522](https://www.optimusdigital.ro/ro/cautare?controller=search&orderby=position&orderway=desc&search_query=Modul+RFID+RC522+%28similar+cu+MFRC522%29&submit_search= ) | RFID module            | [9.99 RON]                                                                                                                  |
| [Buzzer Passive 5V](https://www.optimusdigital.ro/ro/audio-buzzere/634-buzzer-pasiv-de-5-v.html?search_query=Buzzer+Pasiv+de+5+V%09+%09&results=15) | Sound indicator        | [1,40 RON]                                                                                                                  |
| [Coloured wires mom-mom (40p, 30 cm)](https://www.optimusdigital.ro/ro/fire-fire-mufate/881-set-fire-mama-mama-40p-15-cm.html?search_query=%09Fire+Colorate+Mama-Mama+%2840p%2C+30+cm%29+%09&results=10) | Connecting wires       | [6.98 RON]                                                                                                                  |
| [Coloured wires father-father (40p, 15 cm)](https://www.optimusdigital.ro/ro/fire-fire-mufate/884-set-fire-tata-tata-40p-10-cm.html?search_query=%09Fire+Colorate+Tata-Tata+%2840p%2C+15+cm%29&results=10) | Connecting wires       | [6.99 RON]                                                                                                                  |
| [Resistor 0.25W 22KΩ](https://www.optimusdigital.ro/ro/componente-electronice-rezistoare/858-rezistor-025w-18k.html?search_query=rezistor&results=120) | Resistor               | [0,10 RON]                                                                                                                  |
| [Transistor NPN 2n2222 TO-92](https://www.optimusdigital.ro/ro/componente-electronice-tranzistoare/935-tranzistor-s9013-npn-50-pcs-set.html?search_query=Tranzistor+NPN+2n2222+TO-92&results=9) | Transistor             | [0,34 RON]                                                                                                                  |
| [Module with 4 Relays, Blue](https://www.optimusdigital.ro/ro/electronica-de-putere-module-cu-releu/478-modul-releu-cu-4-canale-albastru.html?search_query=Modul+cu+4+Relee%2C+Albastru&results=3) | Relay module           | [13,88 RON]                                                                                                                 |
| [Breadboard HQ (830 Points)](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/8-breadboard-830-points.html?search_query=Breadboard+HQ+%28830+Puncte%29&results=12) | Prototyping board      | [9,98 RON]                                                                                                                  |
| [Cablaj de test 50 x 70 mm](https://www.optimusdigital.ro/ro/cautare?controller=search&orderby=position&orderway=desc&search_query=Cablaj+de+test+50+x+70+mm&submit_search=) | Test board             | [2,49 RON]                                                                                                                  |
| [Button with one touch, TTP223, with pins, 3V, Multicolor](https://www.emag.ro/buton-cu-o-singura-atingere-ttp223-cu-pini-3-v-multicolor-ttp223-mod-1ch/pd/DC6798MBM/) | Unlocking the door     | [18,33 RON]                                                                                                                 |
| [Module DC-DC Step Down LM2596S](https://www.optimusdigital.ro/en/adjustable-step-down-power-supplies/805-lm2596-dc-dc-module-with-voltage-display.html?gad_source=1&gclid=Cj0KCQjwu8uyBhC6ARIsAKwBGpSrrOEMIMOG86OegBIGfEvPyNZ_Anfs8OeiQJDm6vgONy293jpV3zcaAp7fEALw_wcB) | Voltage regulation     | [13 RON]                                                                                                                    |
| [Power Supply](https://www.emag.ro/adaptor-de-alimentare-12v-1-5a-jenuosr-cu-mufa-potrivit-pentru-alimentarea-cu-energie-a-receptorului-de-satelit-routerului-si-camerei-de-supraveghere-e05-w-12v-dyspq/pd/D96X4DYBM/?ref=sponsored_products_search_r_1_1&recid=recads_1_f55d29682d26527ec5e7afe3eb8a6371d1c13ff739b2934e1e29c5960ab91900_1716713164&aid=8734e059-b0db-11ee-a490-0229d980bfff&oid=152478516&aidr=a9bfebbf-1383-11ef-a490-0229d980bfff&scenario_ID=1) | Power supply unit      | [16 RON]                                                                                                                    |



## Software

| Library                         | Version     | Usage                                                                                                        |
|---------------------------------|-------------|-------------------------------------------------------------------------------------------------------------------|
| cortex-m                        | 0.7.7       | Provides low-level access to Cortex-M processor features such as registers and exceptions.                        |
| cortex-m-rt                     | 0.7.3       | Runtime support library for Cortex-M microcontrollers, providing startup code and vector table setup.             |
| embassy-usb-logger              | 0.2.0       | USB logging support for embedded applications using the Embassy framework.                                         |
| embedded-hal                   | 1.0.0       | Hardware Abstraction Layer (HAL) for embedded systems, providing standard interfaces for peripherals.              |
| log                             | 0.4.21      | Logging facade for Rust, allowing for logging messages at different levels (error, warn, info, debug, trace).      |
| defmt                           | 0.2.3       | Lightweight logging framework designed for resource-constrained embedded systems.                                   |
| defmt-rtt                       | 0.2.0       | RTT (Real-Time Transfer) support for `defmt`, enabling logging over RTT.                                           |
| defmt-macros                    | 0.2.3       | Macros for `defmt`, providing compile-time formatting and logging capabilities.                                     |
| panic-probe                     | 0.3.2       | Panic handler for embedded systems that allows inspecting the system state after a panic.                           |
| embassy-executor                | 0.5.0       | Executor for asynchronous tasks in the Embassy framework, with integrated timer support.                            |
| embassy-time                    | 0.3.0       | Time abstraction and utilities for the Embassy framework, with features for mocking and uptime-based timestamps.    |
| mfrc522                         | 0.2         | Driver for the MFRC522 RFID reader, used to interface with RFID tags.                                               |
| embassy-rp                      | 0.1.0       | Embassy framework support for the Raspberry Pi Pico (RP2040) microcontroller, including runtime support.            |





## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://www.youtube.com/watch?v=LfptYHFc6xU)| The main idea source|
2. [link](https://youtube.com/shorts/94idGywsd70?si=_esn9RnLSelw9FPq)| RFID DOOR LOCK|
3. [link](https://www.youtube.com/watch?v=VP0qLUOdvuU)| DOOR LOCK USING BLYNK APP TUTORIAL|
4. [link](https://youtu.be/RAbTdeLU2JQ?si=CxUgyN9DnoZCcLRv)| LM2596S tutorial building|

...
