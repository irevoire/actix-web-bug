## Reproducer

1. Start a request listening on `http://localhost:8080/stream`
2. Kill the listener (ctrl+c on a curl request in my case)
3. See if you get a `dropped` message in the console.
