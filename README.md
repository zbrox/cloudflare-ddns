# CloudFlare DDNS update tool

![](https://github.com/zbrox/cloudflare-ddns/workflows/Build/badge.svg)

This is a simple CLI you can use to continuously update an A DNS record for a domain using CloudFlare's free DDNS service.

## Options

```
    -k, --key <auth-key>     The auth key you need to generate in your Cloudflare profile
    -c, --cache <cache>      Cache file for previously reported IP address (if skipped the IP will be reported on every execution)
    -d, --domain <domain>    The domain for which you want to report the current IP address
    -e, --email <email>      Your Cloudflare login email
    -z, --zone <zone>        The zone in which your domain is (usually that is your base domain name)
```