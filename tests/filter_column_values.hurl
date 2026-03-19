# Test filter column values endpoint - getting distinct values for filtering

# First, upload a CSV with test data
POST {{target}}/upload/csv
[MultipartFormData]
file: file,test_data.csv; text/csv

HTTP 200
[Asserts]
body contains "Success"
body contains "test_data"

# Get distinct values for the "name" column (no search filter)
GET {{target}}/database/tables/table/column/values?database=default&table=test_data&column=name

HTTP 200
[Asserts]
# Should contain checkboxes with all distinct names
body contains "filter_name"
body contains "Alice"
body contains "Bob"
body contains "Charlie"
# Each value should be a checkbox
body contains "type=\"checkbox\""

# Get distinct values for "city" column
GET {{target}}/database/tables/table/column/values?database=default&table=test_data&column=city

HTTP 200
[Asserts]
body contains "filter_city"
body contains "New York"
body contains "San Francisco"
body contains "Seattle"
body contains "type=\"checkbox\""

# Get distinct values for "age" column
GET {{target}}/database/tables/table/column/values?database=default&table=test_data&column=age

HTTP 200
[Asserts]
body contains "filter_age"
body contains "30"
body contains "25"
body contains "35"
body contains "type=\"checkbox\""

# Test search functionality - search for "San" in city column
GET {{target}}/database/tables/table/column/values?database=default&table=test_data&column=city&search=San

HTTP 200
[Asserts]
body contains "San Francisco"
body not contains "New York"
body not contains "Seattle"

# Test search functionality - search for "li" in name column (should match Alice)
GET {{target}}/database/tables/table/column/values?database=default&table=test_data&column=name&search=li

HTTP 200
[Asserts]
body contains "Alice"
body contains "Charlie"
body not contains "Bob"

# Test pagination - offset 0, should show first page (40 items or less)
GET {{target}}/database/tables/table/column/values?database=default&table=test_data&column=name&offset=0

HTTP 200
[Asserts]
body contains "filter_name"
# For small dataset, should not have "Load more"
# (our test data only has 3 names)

# Test with non-existent search term
GET {{target}}/database/tables/table/column/values?database=default&table=test_data&column=name&search=XYZ123

HTTP 200
[Asserts]
# Should return empty but valid response
body not contains "Alice"
body not contains "Bob"
body not contains "Charlie"
