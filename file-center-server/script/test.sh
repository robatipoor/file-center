# user-bar
curl -X POST \
    --cookie "RUSESSION=IYnoxjKrTaOIri3sRtHUg83ioMU1L0itbylPtLDb6JChPL9eqtQ2ZS7zCnjkHPzBm5qWm6+4MZKs5+rm+AEE6USiYgCrfBkgH6C4bPvRRGz2zUydprCB1sI4jgM5Qk7JwN5FPENqE5u8x0s1VvHQKJ4gSLLBXtFLOROgIQzaQMb9ZG+NBL1N3YxGiR1TPztAVIqXNmyjBt98M3Zwebum3RHme4cXmHoEEaJs2sVndGGG" \
    -F file=@files/5.jpg \
localhost:8080/api/file/upload

curl -X GET \
    --cookie 'RUSESSION=IYnoxjKrTaOIri3sRtHUg83ioMU1L0itbylPtLDb6JChPL9eqtQ2ZS7zCnjkHPzBm5qWm6+4MZKs5+rm+AEE6USiYgCrfBkgH6C4bPvRRGz2zUydprCB1sI4jgM5Qk7JwN5FPENqE5u8x0s1VvHQKJ4gSLLBXtFLOROgIQzaQMb9ZG+NBL1N3YxGiR1TPztAVIqXNmyjBt98M3Zwebum3RHme4cXmHoEEaJs2sVndGGG' \
localhost:8080/api/file/list

curl -X GET \
    --cookie 'RUSESSION=IYnoxjKrTaOIri3sRtHUg83ioMU1L0itbylPtLDb6JChPL9eqtQ2ZS7zCnjkHPzBm5qWm6+4MZKs5+rm+AEE6USiYgCrfBkgH6C4bPvRRGz2zUydprCB1sI4jgM5Qk7JwN5FPENqE5u8x0s1VvHQKJ4gSLLBXtFLOROgIQzaQMb9ZG+NBL1N3YxGiR1TPztAVIqXNmyjBt98M3Zwebum3RHme4cXmHoEEaJs2sVndGGG' \
localhost:8080/api/file/download/ba20146cc76242638a15c662c610654f --output img-user-bar.jpg



curl -iv -H "Content-Type: application/json" -d '{"link":"ba20146cc76242638a15c662c610654f","username":"user-foo","access_type":"Read"}' \
--cookie 'RUSESSION=IYnoxjKrTaOIri3sRtHUg83ioMU1L0itbylPtLDb6JChPL9eqtQ2ZS7zCnjkHPzBm5qWm6+4MZKs5+rm+AEE6USiYgCrfBkgH6C4bPvRRGz2zUydprCB1sI4jgM5Qk7JwN5FPENqE5u8x0s1VvHQKJ4gSLLBXtFLOROgIQzaQMb9ZG+NBL1N3YxGiR1TPztAVIqXNmyjBt98M3Zwebum3RHme4cXmHoEEaJs2sVndGGG' \
-X POST http://localhost:8080/api/file/access


curl -X GET \
    --cookie 'RUSESSION=KNGPG+6uqCBk+p1scdLyTe3jOrY/QZAy5qoSKNK8AmamTq6bksdymGC9xvbADWoTTwbkIEwqfoMBQnsz2H4/S6/NJeC5SY9GmfJtzSE1L1erWRhis3m1flaxXgvLcwbaLqjDhq2S5MZ8P48YfDIviHqx1rQMO49S5g9Puxn1ZI5fKw8Oc07ubAUrRUOk2rVyQ0/66NMQfLl9011Xmh7vpsWKqeGbz3ZtM9B/azA8Vm0l' \
localhost:8080/api/file/download/ba20146cc76242638a15c662c610654f --output img-user-foo.jpg

