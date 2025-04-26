# weather-forecast
weather-forecast in Rust

`
http://localhost:8080/swagger-ui/
`
Run:
`
 RUST_LOG=debug cargo run
`
sccache:
`
 sccache --start-server
 sccache --stop-server
 SCCACHE_LOG=debug sccache --show-stats
`
Check Swagger:
`
curl -v http://localhost:8080/api-docs/openapi.json  # Should return 200
curl -v http://localhost:8080/swagger-ui/            # Should serve HTML
curl -v http://localhost:8080/swagger-ui/swagger-initializer.js  # Should serve JS
`