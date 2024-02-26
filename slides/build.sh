#!/bin/bash

for slides in $(find lectures/* -maxdepth 0 -type d -printf "%f\n"); do
    if [ $slides != "assets" ]; then
        echo "Building slides $slides"
        ln -s ../assets/global-bottom.vue lectures/$slides/global-bottom.vue
        npm run build -- lectures/$slides/slides.md --base /slides/$slides --out ../../../build/slides/$slides
        fail=$?
        rm lectures/$slides/global-bottom.vue
        if [ $fail != 0 ]; then
            echo "Failed to build slides $slides"
            exit $fail
        fi
    fi
done
