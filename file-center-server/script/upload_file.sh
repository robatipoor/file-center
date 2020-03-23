#!/usr/bin/env bash

curl -X POST \
    -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJuZXctdXNlciIsInJvbCI6MSwiaWF0IjoxNTg0ODk4MjQzLCJleHAiOjE1ODc0OTAyNDN9.sPN02ec7wRWj6mANwj5sT-GK3AKSBNNl2UKW8a_yt1E" \
    -d @file.txt
localhost:8080/api/file/upload
