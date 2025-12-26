# Flowstate Development Roadmap

## Current State
- ✅ Basic HTTP monitoring (status codes + regex checks)
- ✅ YAML configuration loading
- ✅ Comprehensive test coverage
- ⚠️ **One-shot execution** (runs checks once and exits)

---

## Planned Architectural Refactor

### Goal
Separate configuration (immutable, from YAML) from runtime state (mutable, tracking monitoring status).

### Why This Matters
As we implement continuous monitoring, we need to track runtime state:
- Last check timestamp
- Current status (up/down)
- Consecutive failure count
- Alert cooldown timers
- Response time history
- State transition timestamps

**Current problem**: Mixing state with configuration creates issues:
- Hard to reload config without losing state
- State might accidentally serialize back to config
- Unclear what's "source of truth" vs runtime data
- Testing becomes messier

### Proposed Architecture

```
config::HostConfiguration  (immutable, from YAML)
    ↓
model::Host (runtime state + reference to config)
```

**config::HostConfiguration** - Pure configuration
- Deserializes from YAML
- Immutable after loading
- Can be reloaded without affecting runtime state
- Represents what the user configured

**model::Host** - Runtime state
- Created from HostConfiguration
- Tracks current status, last check time, etc.
- Mutable during monitoring
- Represents what's actually happening

### Example Structure

```rust
// config/host.rs - what users write in YAML
pub struct HostConfiguration {
    pub label: String,
    pub url: String,
    pub status: Option<u16>,
    pub regex: Option<String>,
    // ... config-only fields
}

// model/host.rs - what runs during monitoring
pub struct Host {
    config: Arc<HostConfiguration>,  // immutable config
    state: HostState,                // mutable runtime state
}

pub struct HostState {
    last_check: Option<Instant>,
    current_status: Status,
    consecutive_failures: u32,
    last_response_time: Option<Duration>,
    last_state_change: Option<Instant>,
    // ... runtime-only fields
}
```

### When to Do It
**Before implementing Phase 1** - Do it now while the codebase is small (~150 lines of monitoring code). Once continuous monitoring is added, refactoring becomes significantly harder.

---

## Development Phases

### Phase 1: Core Monitoring Loop (Foundation)
**Goal**: Make it actually monitor continuously

#### 1.1 Continuous monitoring scheduler
- [ ] Add configurable check intervals (e.g., every 30s, 1m, 5m)
- [ ] Implement concurrent monitoring of multiple hosts
- [ ] Add graceful shutdown on SIGTERM/SIGINT
- [ ] Handle panics in individual host monitors

#### 1.2 State management
- [ ] Track host status over time (up/down transitions)
- [ ] Detect state changes (healthy → unhealthy)
- [ ] Store recent check history (last N results)
- [ ] Implement state persistence (optional, for restarts)

---

### Phase 2: Alerts & Notifications (Make it useful)
**Goal**: Get notified when things break

#### 2.1 Alerting system
- [ ] Configurable notification channels (stdout, file, webhooks)
- [ ] Alert on state transitions only (avoid spam)
- [ ] Add alert throttling/cooldown periods
- [ ] Support multiple alert destinations per host

#### 2.2 Output formatting
- [ ] Pretty CLI output showing current status
- [ ] Optional JSON output for machine parsing
- [ ] TUI dashboard (optional, using `ratatui`)
- [ ] Status summary commands

---

### Phase 3: Enhanced Monitoring (More capabilities)
**Goal**: Monitor more than just HTTP

#### 3.1 Additional host types
- [ ] TCP port checks
- [ ] Ping/ICMP checks
- [ ] DNS resolution checks
- [ ] Custom script execution
- [ ] gRPC health checks

#### 3.2 Advanced HTTP features
- [ ] Custom headers support
- [ ] Authentication (Basic, Bearer tokens)
- [ ] TLS certificate expiry warnings
- [ ] Response time tracking (SLA monitoring)
- [ ] Custom timeout configuration
- [ ] Follow redirects configuration
- [ ] HTTP/2 and HTTP/3 support

---

### Phase 4: Operations & Deployment
**Goal**: Run it in production

#### 4.1 Configuration enhancements
- [ ] Hot-reload config without restart
- [ ] Config validation command (`flowstate check`)
- [ ] Environment variable interpolation
- [ ] Multiple config file support
- [ ] Config file includes/imports

#### 4.2 Deployment
- [ ] Dockerfile
- [ ] Systemd service file
- [ ] Kubernetes manifests (optional)
- [ ] Pre-built binaries via GitHub releases
- [ ] Homebrew formula (optional)
- [ ] Installation script

#### 4.3 Observability
- [ ] Prometheus metrics endpoint
- [ ] Better structured logging (JSON output)
- [ ] Self-monitoring (monitor the monitor)
- [ ] Health check endpoint
- [ ] OpenTelemetry tracing (optional)

---

### Phase 5: Polish & Distribution
**Goal**: Make it easy to use

#### 5.1 Documentation
- [ ] Comprehensive README with examples
- [ ] Configuration reference documentation
- [ ] Deployment guides (Docker, systemd, K8s)
- [ ] Troubleshooting guide
- [ ] Migration guides for breaking changes

#### 5.2 Developer experience
- [ ] GitHub Actions CI/CD
- [ ] Automated releases with changelog
- [ ] Benchmarking suite
- [ ] Performance regression tests
- [ ] Security audit
- [ ] Fuzzing tests

---

## Recommended Next Steps

1. **Immediate**: Implement the config/state architecture refactor (see above)
2. **Next**: Phase 1.1 - Continuous monitoring loop
3. **Then**: Phase 1.2 - State management and tracking

This transforms flowstate from a one-shot checker into a production-ready monitoring service.

---

## Future Considerations

### Potential Features (Post-v1.0)
- Dashboard web UI
- Historical data storage (SQLite/PostgreSQL)
- Distributed monitoring (multiple agents)
- Service dependency graphs
- Incident management integration
- Custom plugins/extensions API
- Multi-tenancy support

### Performance Targets
- Support 1000+ hosts on modest hardware
- Sub-second response to state changes
- Minimal memory footprint (<100MB for typical workloads)
- Efficient async I/O for concurrent checks

---

*Last updated: 2025-12-26*
