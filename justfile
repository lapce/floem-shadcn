test:
    cargo nextest run

wr:
    watchexec -w ./wr.sh --clear -r "sh ./wr.sh"
