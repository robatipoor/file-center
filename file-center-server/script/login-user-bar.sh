#!/usr/bin/env bash

curl -iv -X POST \
    -H "Content-Type: application/json" \
    -d '{"username":"user-bar","password":"password"}' \
    localhost:8080/api/auth/login
