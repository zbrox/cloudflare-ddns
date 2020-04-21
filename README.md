# CloudFlare DDNS update tool

![](https://github.com/zbrox/cloudflare-ddns/workflows/Build/badge.svg)

This is a simple CLI you can use to continuously update an A DNS record for a domain using CloudFlare's free DDNS service.

## Options

```
    -t, --token <api-token>     The API token you need to generate in your Cloudflare profile
    -c, --cache <cache>      Cache file for previously reported IP address (if skipped the IP will be reported on every
                             execution)
    -f, --config <config>    Your TOML config file containing all the required options (email, auth_key, zone, domain)
                             which you can use instead of passing the arguments to the command line
    -d, --domain <domain>    The domain for which you want to report the current IP address
    -z, --zone <zone>        The zone in which your domain is (usually that is your base domain name)
```

### Config

You Can pass a path to a configuration file (`-f` or `--config`) instead of each option as a command line argument. The configuration should be a [TOML](https://github.com/toml-lang/toml) file and hold the same options. Here's a sample:

```TOML
api_token = "secretkey"
domain = "example.example.com"
zone = "example.com"
```

## Cloudflare Setup

You need to do some preparatory work in Cloudflare. Firstly this assumes you're using Cloudflare already to manage the DNS records for your domain.

### Initial DNS setup

You need to add a type `A` DNS record for your domain. The `Name` field you should fill in with the name of the subdomain. If you don't want to use a subdomain just type `@` then the base domain will be used. Then change the `Proxy status` field to be not `Proxied` but `DNS only`. This will allow you to input `0.0.0.0` in the `IPv4 Address` field. Then click the save button. You might need to wait sometime before the DNS record propagates.

### API token

We need to authenticate ourselves in front of the Cloudflare API. To do so we need to an API token to pass along with every request. You can generate an API token to use specifically with this application on Cloudflare in `My profile > API Tokens > Create Token`.