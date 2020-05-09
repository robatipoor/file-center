#!/usr/bin/env bash

curl -iv -X POST \
    -H "Content-Type: application/json" \
    -d '{"username":"new-user","password":"password"}' \
    localhost:8080/api/auth/login
