parameters:  
    `--server-url` http://example.com:port  
    `--only-within` 1month 1day or something like that (see [parse_duration](https://docs.rs/parse_duration/latest/parse_duration/))  
    `--username` username  
    `--password` password  
environemnt variables available with form:  
`QBT_REANNOUNCE_{UPPERCASE}` like `QBT_REANNOUNCE_PASSWORD`

you can run this as a systemd timer