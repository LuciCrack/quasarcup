start-dev:
    cd backend && cargo watch -x run && cd .. &
    cd frontend && trunk serve
