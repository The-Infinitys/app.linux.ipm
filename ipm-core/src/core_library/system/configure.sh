#!/bin/bash

cd $IPM_WORK_DIR
if [! -d "package" ]; then
    mkdir package
fi
if [! -d "package/cache"]; then
    mkdir package/cache
fi
if [! -d "package/installed"]; then
    mkdir package/installed
fi
if [! -d "package/logs"]; then
    mkdir package/logs
fi
if [! -d "bin"]; then
    mkdir bin
fi

if [$IPM_EXEC_MODE == "debug"]; then
    cp ./target/debug/ipm ./bin/ipm
fi
