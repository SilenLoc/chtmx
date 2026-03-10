GET {{target}}/health/db

HTTP 200
[Asserts]
body contains "Database connection OK"
