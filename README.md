```shell
 ➜ jhdns -h
Usage: jhdns [OPTIONS] --name <DOMAIN_NAME>

Options:
  -n, --name <DOMAIN_NAME>      domain name
  -s, --server <DNS_SERVER_IP>  dns server ip [default: 8.8.8.8]
  -h, --help                    Print help
  -V, --version                 Print version
```

```shell
 ➜ jhdns -n google.com
response lenth: 44
response bytes:
 [215, 216, 129, 128, 0, 1, 0, 1, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1, 192, 12, 0, 1, 0, 1, 0, 0, 0, 55, 0, 4, 216, 58, 203, 78]
last 4 bytes(IPv4 on success)
[216, 58, 203, 78]
```

