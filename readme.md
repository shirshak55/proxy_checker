### Proxy Checker

Some proxies are too slow. Some proxies are too fast. We want to use fast
proxies so we need to sort them.

This program will sort all proxies according to the response time.

### Algorithm

-   Read all proxies from `proxies.txt`
-   Go to https://api.ipify.org?format=json for 10 times (so 10 request)
-   Using proxy measure the total time
-   Sort It
-   Write it to `good_proxies.txt` file

### How to use?

-   Put all the proxy in proxies.txt
-   Run the binary or use `cargo run --release`
-   It should create file sorted by proxy with least response time

### Proxy Format

At the moment the proxy format looks like this:

```
PROXY_URL:PORT:PROXY_USERNAME:PROXY_PASSWORD
```

If your proxy doesn't need PROXY_USERNAME Or PROXY_PASSWORD feel free to add
blank or dummy field like this:

```
zasdproxy.com:321:anything:anything_password
```

### Quirks

-   Median may be better than total time taken to process 10 request? If you
    need to use median then please raise issue on github tracker.
-   Currently only supports http proxy. If you need other proxy type raise issue
    on github I can fix that.

### Configuration

The source code is so simple I thought making configuration file seems overkill.
However if anyone feels it would be better to make some options like
--input-file=proxy.txt --output-file=good_proxy.txt I can do that. For that just
raise an issue in github issue tracker

### Author

Shirshak Bajgain
