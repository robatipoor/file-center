#!/usr/bin/env bash

# curl -X GET \
# -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyLWZvbyIsInJvbCI6MiwiaWF0IjoxNTg0ODk0ODE1LCJleHAiOjE1ODc0ODY4MTV9.csbmggxQQ5ekD0KaYMKLEDHwFxTgMZvYkdIBbJkx1XI" \
# localhost:8080/api/health

curl --cookie 'RUSESSION=6Ik/FOitUeFeoCNo2EvltUTDycuyBzKEtBWgTeAURy5mB3R1tyoPHy7rA/hf46PL1Ivte/37JzTXq3lhkrPuXkEYlXUgcf36oY8U24KF1V2GQIXHb+GT/PWbg5rr8h1y08RqL4+FqVH2YfJp42KtKRlmHjjyhPSpcR+YZ7rINT0M3M5EL5me7R4E5VRIic0pWFRfGYrJHmrE1rxfJlexeB3solA/5Dc7VXEvJBZ3Okib' \
-X GET http://localhost:8080/api/health
