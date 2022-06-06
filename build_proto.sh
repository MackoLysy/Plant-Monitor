#!/bin/bash

python3 third_party/nanopb/generator/nanopb_generator.py protobufs/plant_message.proto && \
mv protobufs/plant_message.pb.c plantNordicNode/src && \
mv protobufs/plant_message.pb.h plantNordicNode/include 