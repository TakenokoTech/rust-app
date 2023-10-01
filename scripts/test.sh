#!/bin/bash

curl -X DELETE http://localhost:3030/data \
  -H "Content-Type: application/json" \
  -H "authorization: takenoko" \
  > /dev/null 2>&1

echo "delete done."

for i in {1..100}; do
  DATA=$(printf "{
    \"a%03d\": \"a%03d\",
    \"b%03d\": \"b%03d\",
    \"c%03d\": \"c%03d\"
  }" "$i" "$i" "$i" "$i" "$i" "$i")

  curl -X POST "http://localhost:3030/data" \
    -H "Content-Type: application/json" \
    -H "authorization: takenoko" \
    -d "$DATA" \
    > /dev/null 2>&1 &
done

echo "post done."

for i in {1..100}; do
  curl -X DELETE "http://localhost:3030/data?q=$(printf "c%03d" "$i")" \
    -H "Content-Type: application/json" \
    -H "authorization: takenoko" \
    > /dev/null 2>&1 &
done

echo "delete done."

for i in {1..100}; do
  curl -X GET "http://localhost:3030/data?q=$(printf "a%03d" "$i")" \
    -H "Content-Type: application/json" \
    -H "authorization: takenoko" \
    -w "\n"
done

echo "get done."
