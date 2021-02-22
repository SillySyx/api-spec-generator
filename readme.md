Generates an api specification from an json file.


# Features
* Simplifies maintaining api specifications
* Outputs markdown to either file or terminal
* Uses json files as input
* Includes a table of content


# Examples
To generate an api specification from a json file
```
api-specification-generator.exe -f ./path/to/file.json
```

To save the output into an file
```
api-specification-generator.exe -f ./path/to/file.json -s ./save/here.md
```


# Building this project
To build this project you need rust, this can be installed via [rustup.rs](http://rustup.rs)

To build simply run the command:
```
cargo build --release
```


# Json template
This is the json file structure used to generate the api specification.
```
[
    {
        "name": "Hello",
        "description": "This is a test",
        "permissions": [
            "Admin"
        ],
        "request_method": "GET",
        "request_url": "/api/me",
        "request_headers": {
            "Authorization": "1234"
        },
        "request_body": {
            "search": "1234"
        },
        "response": {
            "200 Ok": "Very nice"
        }
    }
]
```

And this is what the output will be
```
* [Hello](#hello)


# Hello
This is a test  

## Permissions
* Admin

## HTTP request
GET /api/me 

## Request headers
|Name|Value| 
|-|-| 
|Authorization|1234| 

## Request body
{
  "search": "1234"
}

## Response
`200 Ok` Very nice  
```