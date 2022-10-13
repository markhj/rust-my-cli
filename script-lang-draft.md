
## echo
````
echo "Hello world"
````

## System exec
````
exec "\path\to\file --option"
````

## Arguments
````
echo "Hello %ARG_1%"
````

## match
````
match (%ARG_1%, %ARG_2)
    "a" "7.3"
        echo "a 7.3"
    "a" "8.1"
        echo "a 8.1"
````
