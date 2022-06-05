# Plant Monitor

Simple plant monitor system base on `nRF51822` node and `NanoPi Neo`

It measure:
 - soil moisture
 - light intensity
 - air humidity 
 - air temperature

## nRF51822 Stack
 
Its written in `arduino framework` with `BLE stack` using `protobuffs` for message layer.

## NanoPi Neo

Backend:
- `rust` using `bazel` as build tool.
- `react.js` with `typescript` on front.