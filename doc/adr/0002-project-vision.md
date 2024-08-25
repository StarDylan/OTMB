# 2. Project Vision

Date: 2024-08-24

## Status

Accepted

## Context

We want to work on a cool project.

## Decision

We are creating the One Time Pad Device.

The ideal operation follows these steps:

1. Physical contact key generation

    - By connecting 2 or more OTMBs, they will generate OTP keys.

2. Message Encryption and Sending

    - A terminal script is run upon plugin
    - You plug your OTMB into your computer and send a message to an ID that you have paired with

3. Receiving and Message Decryption

    - The receiver recieves a message then decrypts it with their own device.

4. Key Exhaustion

    - At some point, the OTP key will be used up and the device signals that there is not enough key left to send a message.

### Other Features

- When the pair button is pressed without any other device connected, the user will be able to see how
 much of the key they have left with LEDs.

- In an Emergency, pressing the pair button 10 times quickly wipes the device.

## Consequences

We will learn about security, embedded programming, and PCB design.
