#!/bin/bash
/usr/bin/tee output/nvimpam.stdin | ./target/debug/nvimpam | /usr/bin/tee output/nvimpam.stdout
