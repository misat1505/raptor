#!/bin/bash

name="Elon Musk"

echo "Hi $name"

names=("Elon Musk" "Mark Zuckerberg" "Jeff Bezos")

for name in "${names[@]}"
do
  echo $name
done

i=1
j=2

echo "$((i + j))"

echo ${names[@]}
echo "${#names[@]}"
