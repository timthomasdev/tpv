# tpv

tpv (total portfolio value) is a small command-line tool written in Rust to check the total value (in USD) of a cryptocurrency portfolio. It works by reading from a .portfolio file within the home directory. The first line of this file should be a Messario.io API key. All subsequent lines are portfolio entires. These should be formatted as lowercase ticker symbol followed by the total amount of coins. See below for example .portfolio file: 

```
this-is-a-fake-api-key-194859b-81a34f
btc 3.14
eth .99999
xtz 1234
```

After running tpv with this file, it should return:

```
Current price (USD) of Bitcoin: -----
worth: -----
Current price (USD) of Ethereum: -----
worth: -----
Current price (USD) of Tezos: -----
worth: -----
Total portfolio: -----
```

This is untested software. Please do not use it anywhere important.
