#!/usr/bin/env bash

cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
