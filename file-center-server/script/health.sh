#!/usr/bin/env bash

# curl -X GET \
# -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyLWZvbyIsInJvbCI6MiwiaWF0IjoxNTg0ODk0ODE1LCJleHAiOjE1ODc0ODY4MTV9.csbmggxQQ5ekD0KaYMKLEDHwFxTgMZvYkdIBbJkx1XI" \
# localhost:8080/api/health

curl -X GET \
-H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJuZXctdXNlciIsInJvbCI6MSwiaWF0IjoxNTg0ODk4MjQzLCJleHAiOjE1ODc0OTAyNDN9.sPN02ec7wRWj6mANwj5sT-GK3AKSBNNl2UKW8a_yt1E" \
localhost:8080/api/health
