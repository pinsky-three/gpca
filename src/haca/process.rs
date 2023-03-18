type Process<N> = dyn Fn(&N, &[N]) -> N + Sync + Send;
