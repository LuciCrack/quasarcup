start-dev:
    cd backend && cargo watch -x 'run --verbose' && cd .. &
    cd frontend && trunk serve
