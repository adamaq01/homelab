# renovate: datasource=github-releases packageName=apple/pkl
ARG PKL_VERSION=0.32.1

FROM --platform=linux/amd64 alpine:3.23.3 AS renderer
ARG PKL_VERSION
RUN apk add --no-cache curl gcompat \
  && curl -L -o /usr/local/bin/pkl "https://github.com/apple/pkl/releases/download/${PKL_VERSION}/pkl-linux-amd64" \
  && chmod +x /usr/local/bin/pkl
WORKDIR /work
COPY plugin.pkl .
RUN pkl eval plugin.pkl -f yaml -o plugin.yaml

FROM --platform=linux/amd64 alpine:3.23.3
RUN apk add --no-cache gcompat
COPY --from=renderer /usr/local/bin/pkl /usr/local/bin/pkl
COPY --from=renderer /work/plugin.yaml /home/argocd/cmp-server/config/plugin.yaml
ENTRYPOINT ["/var/run/argocd/argocd-cmp-server"]
