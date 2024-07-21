# Useful commands
## to see what services are exposed on a grpc server
`grpcurl -plaintext -import-path ./proto -proto guestbook.proto '[::1]:10000' list`
    - `-plaintext` is to avoid the TLS handshake
    - reflection API must be supported in order for `list` to work
        - `tonic_reflection`?
        - TODO
            - https://docs.rs/tonic/latest/tonic/macro.include_file_descriptor_set.html
            - https://github.com/hyperium/tonic/blob/master/examples/src/reflection/server.rs

- grpcurl docker container I hardly know her
    - https://github.com/fullstorydev/grpcurl#docker
- command to run grpcurl under podman container (Arch aur package broken :( )
    - `podman run --network="host" -v /home/klaire/Dropbox/code/web-projects/guestbook/proto:/protos/ fullstorydev/grpcurl -import-path /protos -plaintext -proto /protos/guestbook.proto localhost:10000 guestbook.Guestbooks/get`

## SQLx
- use `cargo add sqlx --features runtime-async-std-native-tls,chrono,postgres,macros`
    - TLSsssssssssssssss

## Quickest PostgreSQL/Podman setup of all time:
https://medium.com/@pawanpg0963/run-postgresql-with-podman-as-docker-container-86ad392349d1
