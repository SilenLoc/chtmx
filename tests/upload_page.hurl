# Test upload page is accessible
GET {{target}}/upload

HTTP 200
[Asserts]
body contains "Upload CSV to ClickHouse"
body contains "Select CSV File"
