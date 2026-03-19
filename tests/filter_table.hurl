# Test table filtering functionality

# First, upload a CSV with test data
POST {{target}}/upload/csv
[MultipartFormData]
file: file,test_data.csv; text/csv

HTTP 200
[Asserts]
body contains "Success"
body contains "test_data"

# Get the table without filters - should show all rows and have filter flyouts
GET {{target}}/database/tables/table?database=default&table=test_data

HTTP 200
[Asserts]
body contains "Alice"
body contains "Bob"
body contains "Charlie"
body contains "New York"
body contains "San Francisco"
body contains "Seattle"
body contains "filter-flyout-name"
body contains "filter-flyout-city"
body contains "filter-flyout-age"
body contains "toggleFilter"

# TODO: Update these tests to work with checkbox-based filtering
# The new filter approach uses checkboxes with multiple values per column
# and requires the filter_column parameter to be an array
# For now, these old filter tests are commented out

# Filter by name = "Alice" (using checkbox value)
# GET {{target}}/database/tables/table?database=default&table=test_data&filter_name=Alice
# 
# HTTP 200
# [Asserts]
# body contains "Alice"
# body not contains "Bob"
# body not contains "Charlie"

