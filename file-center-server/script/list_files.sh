#!/usr/bin/env bash

curl -X GET \
    -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyLWZvbyIsInJvbCI6MiwiaWF0IjoxNTg4MTc3MTE0LCJleHAiOjE1OTA3NjkxMTR9.8XbxL0yMKBR4igA_gIsAaUVpjnz2RWYFjgPkEEy-KZU" \
localhost:8080/api/file/list
