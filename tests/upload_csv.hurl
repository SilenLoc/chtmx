# Test CSV upload endpoint
POST {{target}}/upload/csv
[MultipartFormData]
file: file,test_data.csv; text/csv

HTTP 200
[Asserts]
body contains "Success"
body contains "test_data"
