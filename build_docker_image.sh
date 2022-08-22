# Copyright (c) 2022 Evolving Software Corporation
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT
#!/bin/sh
# Builds the Docker image for the application.
# sudo apt autoremove -y
# sudo apt-get install lld clang -y
rm -r ./target/
# cargo install cargo-watch --force
# echo "printing for update"
cargo install --path .

docker build  -t evolvingsoftware/evolving_solutions_url_shortener_api:alpha .

# push to docker hub
docker push evolvingsoftware/evolving_solutions_url_shortener_api:alpha