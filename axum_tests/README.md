

In Terminal 1: `cargo watch -q -c -w src/ -x run`   
In Terminal 2: `cargo watch -q -c -w tests/ -x 'test --test quick_dev -- --nocapture'`