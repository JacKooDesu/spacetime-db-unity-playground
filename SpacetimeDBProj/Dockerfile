FROM clockworklabs/spacetime:latest

USER root

RUN mkdir -p /root/.local/share/spacetime/bin

RUN ln -s /home/spacetime/.local/share/spacetime/bin/current /root/.local/share/spacetime/bin/current

ENV PATH=/home/spacetime/.local/bin:/usr/share/dotnet:/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

ENTRYPOINT [ "bash" ]
