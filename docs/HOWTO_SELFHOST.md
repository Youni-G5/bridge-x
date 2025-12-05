# How to Self-Host BridgeX Relay Server

This guide explains how to deploy a BridgeX relay server on a VPS for remote access when devices are on different networks.

## When Do You Need a Relay?

By default, BridgeX works **locally** over LAN without any server. Use a relay server if:

- You want to access your desktop from outside your home network
- Devices are on different networks (e.g., mobile on 4G, desktop on home WiFi)
- Your network has restrictive NAT that blocks P2P connections

## Prerequisites

- VPS with Ubuntu 22.04+ (1GB RAM, 10GB disk minimum)
- Domain name (optional, but recommended)
- Basic Linux command line knowledge

## Option 1: Docker Deployment (Recommended)

### 1. Install Docker

```bash
sudo apt update
sudo apt install -y docker.io docker-compose
sudo systemctl enable docker
sudo systemctl start docker
```

### 2. Create Docker Compose File

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  bridgex:
    image: bridgex/backend:latest
    container_name: bridgex-relay
    restart: unless-stopped
    ports:
      - "8080:8080"
    environment:
      - BRIDGEX_HOST=0.0.0.0
      - BRIDGEX_PORT=8080
      - BRIDGEX_ENABLE_RELAY=true
      - BRIDGEX_LOG_LEVEL=info
    volumes:
      - ./data:/app/data
      - ./logs:/app/logs
    networks:
      - bridgex-net

networks:
  bridgex-net:
    driver: bridge

volumes:
  data:
  logs:
```

### 3. Start the Server

```bash
sudo docker-compose up -d
```

### 4. Check Logs

```bash
sudo docker-compose logs -f bridgex
```

## Option 2: Binary Deployment

### 1. Build the Backend

On your local machine:

```bash
cd backend
cargo build --release
```

### 2. Upload to VPS

```bash
scp target/release/bridgex-server user@your-vps:/usr/local/bin/
ssh user@your-vps "chmod +x /usr/local/bin/bridgex-server"
```

### 3. Create Systemd Service

On VPS, create `/etc/systemd/system/bridgex.service`:

```ini
[Unit]
Description=BridgeX Relay Server
After=network.target

[Service]
Type=simple
User=bridgex
WorkingDirectory=/opt/bridgex
ExecStart=/usr/local/bin/bridgex-server
Restart=on-failure
RestartSec=10

Environment="BRIDGEX_HOST=0.0.0.0"
Environment="BRIDGEX_PORT=8080"
Environment="BRIDGEX_ENABLE_RELAY=true"
Environment="BRIDGEX_DB_PATH=/opt/bridgex/data/bridge.db"

[Install]
WantedBy=multi-user.target
```

### 4. Create User and Directories

```bash
sudo useradd -r -s /bin/false bridgex
sudo mkdir -p /opt/bridgex/{data,logs}
sudo chown -R bridgex:bridgex /opt/bridgex
```

### 5. Start Service

```bash
sudo systemctl daemon-reload
sudo systemctl enable bridgex
sudo systemctl start bridgex
sudo systemctl status bridgex
```

## SSL/TLS Setup (Recommended)

### Using Nginx Reverse Proxy

#### 1. Install Nginx and Certbot

```bash
sudo apt install -y nginx certbot python3-certbot-nginx
```

#### 2. Configure Nginx

Create `/etc/nginx/sites-available/bridgex`:

```nginx
server {
    listen 80;
    server_name bridge.yourdomain.com;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

#### 3. Enable Site

```bash
sudo ln -s /etc/nginx/sites-available/bridgex /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

#### 4. Get SSL Certificate

```bash
sudo certbot --nginx -d bridge.yourdomain.com
```

## Firewall Configuration

### Open Required Ports

```bash
# HTTP/HTTPS
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp

# BridgeX backend (if not using reverse proxy)
sudo ufw allow 8080/tcp

# Enable firewall
sudo ufw enable
sudo ufw status
```

## Client Configuration

### Desktop App

No changes needed - the app will use the relay automatically if local connection fails.

### Mobile App

Edit API URL in settings:

```
https://bridge.yourdomain.com
```

Or if no SSL:

```
http://your-vps-ip:8080
```

## Monitoring

### Check Server Status

```bash
curl http://your-vps:8080/api/v1/health
```

Expected response:

```json
{
  "status": "ok",
  "service": "BridgeX Backend",
  "version": "0.1.0"
}
```

### View Logs

**Docker:**
```bash
sudo docker-compose logs -f bridgex
```

**Systemd:**
```bash
sudo journalctl -u bridgex -f
```

## Security Best Practices

### 1. Use Strong Firewall Rules

Only allow necessary ports:

```bash
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow ssh
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
```

### 2. Enable Fail2Ban

```bash
sudo apt install -y fail2ban
sudo systemctl enable fail2ban
sudo systemctl start fail2ban
```

### 3. Keep System Updated

```bash
sudo apt update && sudo apt upgrade -y
sudo apt autoremove -y
```

### 4. Use Strong Passwords

Disable password auth, use SSH keys only:

```bash
# /etc/ssh/sshd_config
PasswordAuthentication no
PubkeyAuthentication yes
```

### 5. Regular Backups

Backup BridgeX data:

```bash
sudo tar -czf bridgex-backup-$(date +%Y%m%d).tar.gz /opt/bridgex/data
```

## Troubleshooting

### Cannot Connect to Server

1. Check if service is running:
   ```bash
   sudo systemctl status bridgex
   ```

2. Check firewall:
   ```bash
   sudo ufw status
   ```

3. Test locally:
   ```bash
   curl http://127.0.0.1:8080/api/v1/health
   ```

### High CPU/Memory Usage

1. Check active connections:
   ```bash
   curl http://127.0.0.1:8080/api/v1/status
   ```

2. Monitor resources:
   ```bash
   htop
   ```

3. Restart service:
   ```bash
   sudo systemctl restart bridgex
   ```

### SSL Certificate Issues

1. Renew certificate:
   ```bash
   sudo certbot renew
   ```

2. Check certificate:
   ```bash
   sudo certbot certificates
   ```

## Cost Estimation

**Minimum VPS Specs:**
- 1 vCPU
- 1GB RAM
- 10GB SSD
- 1TB bandwidth

**Monthly Cost:** $5-10 USD

**Providers:**
- DigitalOcean: $6/month
- Vultr: $5/month
- Linode: $5/month
- Hetzner: €4/month

## Scaling for Multiple Users

### Increase Resources

For 10+ concurrent users:
- 2 vCPU
- 2GB RAM
- 20GB SSD

### Load Balancing (Advanced)

For 100+ users, use multiple servers with nginx load balancing:

```nginx
upstream bridgex_backend {
    server 10.0.0.1:8080;
    server 10.0.0.2:8080;
    server 10.0.0.3:8080;
}

server {
    location / {
        proxy_pass http://bridgex_backend;
    }
}
```

## Support

For issues or questions:
- GitHub Issues: https://github.com/Youni-G5/bridge-x/issues
- Documentation: https://github.com/Youni-G5/bridge-x/docs
