#!/usr/bin/env bash

curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"username":"new-user","password":"password","email":"new-user@email.com"}' \
    localhost:8080/api/auth/register
