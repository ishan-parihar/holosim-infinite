# Deployment Guide

**Version**: 1.0.0
**Last Updated**: January 31, 2026
**Project**: Holographic Multi-Scale Cosmic Creation Simulator

---

## Table of Contents

1. [Pre-Deployment Checklist](#pre-deployment-checklist)
2. [Building for Production](#building-for-production)
3. [Deployment Strategies](#deployment-strategies)
4. [Monitoring](#monitoring)
5. [Backup Procedures](#backup-procedures)
6. [Scaling](#scaling)
7. [Maintenance](#maintenance)

---

## Pre-Deployment Checklist

### Code Review

- [ ] All tests pass
- [ ] Code reviewed and approved
- [ ] Documentation updated
- [ ] Performance benchmarks met

### Security

- [ ] No hardcoded credentials
- [ ] Dependencies audited
- [ ] Security scans passed
- [ ] Access controls configured

### Configuration

- [ ] Environment variables set
- [ ] Configuration files prepared
- [ ] Database connections verified
- [ ] API endpoints configured

### Infrastructure

- [ ] Server provisioned
- [ ] Network configured
- [ ] Storage allocated
- [ ] Monitoring set up

---

## Building for Production

### Optimized Build

```bash
# Build with optimizations
cargo build --release

# Enable LTO (Link Time Optimization)
cargo build --release --config "profile.release.lto = true"

# Strip debug symbols
strip target/release/holonic_realms
```

### Cross-Compilation

```bash
# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# macOS x86_64
cargo build --release --target x86_64-apple-darwin

# macOS ARM64
cargo build --release --target aarch64-apple-darwin

# Windows x86_64
cargo build --release --target x86_64-pc-windows-msvc
```

### Docker Build

Create `Dockerfile`:

```dockerfile
FROM rust:1.75.0 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/holonic_realms /usr/local/bin/

WORKDIR /data
CMD ["holonic_realms"]
```

Build and run:

```bash
docker build -t holonic_realms:1.0.0 .
docker run -d -p 8080:8080 holonic_realms:1.0.0
```

---

## Deployment Strategies

### 1. Simple Deployment

Copy the binary to the server:

```bash
# Build locally
cargo build --release

# Copy to server
scp target/release/holonic_realms user@server:/usr/local/bin/

# Run on server
ssh user@server
holonic_realms
```

### 2. Systemd Service

Create `/etc/systemd/system/holonic_realms.service`:

```ini
[Unit]
Description=Holographic Multi-Scale Cosmic Creation Simulator
After=network.target

[Service]
Type=simple
User=holonic
Group=holonic
WorkingDirectory=/opt/holonic_realms
ExecStart=/usr/local/bin/holonic_realms
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable holonic_realms
sudo systemctl start holonic_realms
sudo systemctl status holonic_realms
```

### 3. Kubernetes Deployment

Create `deployment.yaml`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: holonic-realms
spec:
  replicas: 3
  selector:
    matchLabels:
      app: holonic-realms
  template:
    metadata:
      labels:
        app: holonic-realms
    spec:
      containers:
      - name: holonic-realms
        image: holonic_realms:1.0.0
        ports:
        - containerPort: 8080
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "2000m"
```

Deploy:

```bash
kubectl apply -f deployment.yaml
```

---

## Monitoring

### Health Checks

```bash
# Check if service is running
curl http://localhost:8080/health

# Check metrics
curl http://localhost:8080/metrics
```

### Logging

Enable structured logging:

```bash
RUST_LOG=info,holonic_realms=debug holonic_realms
```

Log to file:

```bash
RUST_LOG=info holonic_realms 2>&1 | tee holonic_realms.log
```

### Metrics

Track:
- Request rate
- Response time
- Error rate
- Memory usage
- CPU usage
- Configuration discovery time
- Resonance calculation time

### Alerting

Set up alerts for:
- High error rate (> 5%)
- High response time (> 1s)
- High memory usage (> 90%)
- High CPU usage (> 80%)
- Service downtime

---

## Backup Procedures

### Database Backups

```bash
# Daily backup
0 2 * * * /usr/local/bin/backup_database.sh

# Backup script
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
pg_dump -U user database > /backup/database_$DATE.sql
```

### Configuration Backups

```bash
# Backup configuration files
tar -czf /backup/config_$(date +%Y%m%d).tar.gz /etc/holonic_realms/
```

### Data Backups

```bash
# Backup discovery database
cp -r /var/lib/holonic_realms/discoveries /backup/discoveries_$(date +%Y%m%d)/
```

### Retention Policy

- Daily backups: 7 days
- Weekly backups: 4 weeks
- Monthly backups: 12 months

---

## Scaling

### Horizontal Scaling

Add more instances:

```bash
# Kubernetes
kubectl scale deployment holonic-realms --replicas=5

# Docker Compose
docker-compose up -d --scale holonic_realms=5
```

### Vertical Scaling

Increase resources:

```yaml
resources:
  requests:
    memory: "2Gi"
    cpu: "1000m"
  limits:
    memory: "8Gi"
    cpu: "4000m"
```

### Load Balancing

Use a load balancer:
- Nginx
- HAProxy
- AWS ELB
- Kubernetes Service

---

## Maintenance

### Updates

```bash
# Pull latest code
git pull

# Build new version
cargo build --release

# Restart service
sudo systemctl restart holonic_realms
```

### Rollback

```bash
# Stop current version
sudo systemctl stop holonic_realms

# Restore previous version
cp /backup/holonic_realms.previous /usr/local/bin/holonic_realms

# Start service
sudo systemctl start holonic_realms
```

### Cleanup

```bash
# Clean old logs
find /var/log/holonic_realms -name "*.log" -mtime +30 -delete

# Clean old backups
find /backup -name "*.sql" -mtime +90 -delete
```

---

## Troubleshooting

### Service Won't Start

```bash
# Check logs
sudo journalctl -u holonic_realms -n 100

# Check configuration
sudo holonic_realms --check-config

# Check dependencies
ldd /usr/local/bin/holonic_realms
```

### High Memory Usage

```bash
# Check memory usage
ps aux | grep holonic_realms

# Profile memory
valgrind --leak-check=full /usr/local/bin/holonic_realms
```

### Slow Performance

```bash
# Profile CPU
perf record -g /usr/local/bin/holonic_realms
perf report

# Check system resources
top
htop
```

---

## Security

### Firewall

```bash
# Open necessary ports
sudo ufw allow 8080/tcp
sudo ufw enable
```

### SSL/TLS

```bash
# Generate SSL certificate
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes

# Configure SSL
# Add to configuration
ssl_key = /etc/ssl/private/key.pem
ssl_cert = /etc/ssl/certs/cert.pem
```

### Access Control

```bash
# Create dedicated user
sudo useradd -r -s /bin/false holonic

# Set permissions
sudo chown -R holonic:holonic /opt/holonic_realms
sudo chmod -R 750 /opt/holonic_realms
```

---

## Additional Resources

- [Getting Started Guide](GETTING_STARTED.md)
- [Troubleshooting Guide](TROUBLESHOOTING.md)
- [Performance Benchmarks](BENCHMARKS.md)
- [API Documentation](API.md)

---

**Deployment Guide Version**: 1.0.0
**Last Updated**: January 31, 2026
**Project**: Holographic Multi-Scale Cosmic Creation Simulator