# Blackjack dealer

## Fast blackjack dealer

Build with cargo, starts up on port 1337 by default.

### Endpoints

- [x] /shuffle - returns a shuffled deck ()
- [x] /fouraces - returns a shuffled deck, but with the four aces on top of the deck
- [x] /bothblackjack - returns a deck with blackjack for both players. Dealer wins
- [x] /playerblackjack - returns a deck with blackjack for the player
- [x] /dealerblackjack - returns a deck with blackjack for the dealer
- [x] /dealerbust - returns a deck where dealer will bust
- [x] /playerbust - returns a deck where player will bust
- [x] /tie21 - Both players draw 21 - - This should cause dealer to lose, since they always draw until they have higher than player -
- [x] /custom?cards=[cards] - Use this with the shorthand expected from the candidate to put the cards in the `cards` argument on top, so /custom?cards=SA,SK,HA,HK would give player Ace of Spades (SA) and King of Spades (SK), and dealer Ace of Hearts (HA) and King of Hearts (HK)

### Backing endpoints

- /metrics - Prometheus metrics for requests and process
- /health - returns 200 OK

## Benchmarking

We're using [k6](https://k6.io) for benchmarking. This repo includes a k6benchmark.js file which can run on the server running this application or modified to access the URL where the application is deployed.

We've also included [Criterion](https://github.com/bheisler/criterion.rs) benchmarks, these can be run with

```sh
$ cargo bench
```

### Basic usage of k6

- For benchmarking the shuffle endpoint

```sh
$ k6 run --vus 50 --duration 30s k6benchmark.js
```

### k6 results (on a Ryzen 9 5900X (12C/24T)) built with --release for /shuffle

```
scenarios: (100.00%) 1 scenario, 50 max VUs, 40s max duration (incl. graceful stop):
* default: 50 looping VUs for 10s (gracefulStop: 30s)


running (10.0s), 00/50 VUs, 1974808 complete and 0 interrupted iterations
default ✓ [======================================] 50 VUs  10s

     data_received..................: 2.8 GB  279 MB/s
     data_sent......................: 172 MB  17 MB/s
     http_req_blocked...............: avg=1.18µs   min=330ns   med=850ns    max=4.71ms  p(90)=1.34µs   p(95)=1.74µs
     http_req_connecting............: avg=2ns      min=0s      med=0s       max=305.9µs p(90)=0s       p(95)=0s
     http_req_duration..............: avg=186.04µs min=28.63µs med=155.04µs max=7.07ms  p(90)=312.72µs p(95)=386.81µs
       { expected_response:true }...: avg=186.04µs min=28.63µs med=155.04µs max=7.07ms  p(90)=312.72µs p(95)=386.81µs
     http_req_failed................: 0.00%   ✓ 0             ✗ 1974808
     http_req_receiving.............: avg=15.78µs  min=3.86µs  med=10.99µs  max=6.72ms  p(90)=15.95µs  p(95)=18.47µs
     http_req_sending...............: avg=5.96µs   min=2.04µs  med=4.44µs   max=6.64ms  p(90)=6.53µs   p(95)=7.94µs
     http_req_tls_handshaking.......: avg=0s       min=0s      med=0s       max=0s      p(90)=0s       p(95)=0s
     http_req_waiting...............: avg=164.28µs min=16.71µs med=137.58µs max=5.36ms  p(90)=288.71µs p(95)=355.72µs
     http_reqs......................: 1974808 197459.999801/s
     iteration_duration.............: avg=245.73µs min=47.12µs med=180.94µs max=28.53ms p(90)=353.19µs p(95)=448.45µs
     iterations.....................: 1974808 197459.999801/s
     vus............................: 0       min=0           max=50
     vus_max........................: 50      min=50          max=50
```

### k6 results (on a hetzner CPX11 (2VCPU, 2GB RAM)) built with --release for /shuffle

scenarios: (100.00%) 1 scenario, 50 max VUs, 40s max duration (incl. graceful stop):

- default: 50 looping VUs for 10s (gracefulStop: 30s)

running (10.0s), 00/50 VUs, 107538 complete and 0 interrupted iterations
default ✗ [======================================] 50 VUs 10s

     data_received..................: 198 MB 20 MB/s
     data_sent......................: 9.4 MB 935 kB/s
     http_req_blocked...............: avg=3.4µs   min=552ns   med=1.39µs  max=5.31ms  p(90)=2.61µs  p(95)=3.63µs
     http_req_connecting............: avg=1.05µs  min=0s      med=0s      max=4.63ms  p(90)=0s      p(95)=0s
     http_req_duration..............: avg=4.57ms  min=55.96µs med=3.59ms  max=49.76ms p(90)=9.28ms  p(95)=12.14ms
       { expected_response:true }...: avg=4.57ms  min=55.96µs med=3.59ms  max=49.76ms p(90)=9.28ms  p(95)=12.14ms
     http_req_failed................: 0.00%  ✓ 0            ✗ 107538
     http_req_receiving.............: avg=56.17µs min=6.74µs  med=20.91µs max=34.14ms p(90)=80.49µs p(95)=101.03µs
     http_req_sending...............: avg=23.65µs min=2.92µs  med=7.91µs  max=22.52ms p(90)=15.11µs p(95)=51.58µs
     http_req_tls_handshaking.......: avg=0s      min=0s      med=0s      max=0s      p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=4.49ms  min=33.36µs med=3.52ms  max=49.65ms p(90)=9.17ms  p(95)=11.93ms
     http_reqs......................: 107538 10750.424771/s
     iteration_duration.............: avg=4.63ms  min=83.71µs med=3.64ms  max=49.8ms  p(90)=9.37ms  p(95)=12.25ms
     iterations.....................: 107538 10750.424771/s
     vus............................: 50     min=50         max=50
     vus_max........................: 50     min=50         max=50
