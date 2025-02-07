#!/bin/bash

if [ -z "$SLIDES_OUTPUT_FOLDER" ];
then
    SLIDES_OUTPUT_FOLDER=./dist
    mkdir -p $SLIDES_OUTPUT_FOLDER
fi

function build {
    series=$1
    slides=$2
    echo "Building slides $slides"
    ln -s ../assets/global-bottom.vue "lectures/$series/$slides/global-bottom.vue"
    npm run build -- "lectures/$series/$slides/slides.md" --base /slides/$series/$slides --out "../../../$SLIDES_OUTPUT_FOLDER/$series/$slides"
    fail=$?
    rm "lectures/$series/$slides/global-bottom.vue"
    return $fail
}

series=$1

if [ "$series" != "acs_cc" -a "$series" != "fils_en" ];
then
    echo "Please use on of the series acs_cc or fils_en"
    exit 1
fi

if [ "$2" != "" ];
then
    build $1 $2
else
    for slides in $(basename -a $(find lectures/$series/* -maxdepth 0 -type d)); do
        if [ $slides != "assets" -a $slides != "resources" ]; then
            build $series $slides
            fail=$?
            if [ $fail != 0 ]; then
                echo "Failed to build slides $series/$slides"
                exit $fail
            fi
        fi
    done
fi
