#!/usr/bin/env bash

curl -X POST \
    -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyLWZvbyIsInJvbCI6MiwiaWF0IjoxNTg0ODk0ODE1LCJleHAiOjE1ODc0ODY4MTV9.csbmggxQQ5ekD0KaYMKLEDHwFxTgMZvYkdIBbJkx1XI" \
    -F file=@files/3.jpg \
localhost:8080/api/file/upload
