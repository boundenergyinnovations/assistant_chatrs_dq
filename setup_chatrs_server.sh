#!/bin/bash
set -e

# Update the package list
sudo apt update

# Install Nginx
sudo apt install -y nginx

# Allow Nginx HTTP through the firewall
sudo ufw allow 'Nginx HTTP'

# Check the firewall status
sudo ufw status

# Check the status of Nginx service
sudo systemctl status nginx

# Create the Nginx configuration file for chatrs
sudo bash -c 'cat > /etc/nginx/sites-available/chatrs <<EOF
server {
    listen 80;
    server_name your_domain.com; # Replace with your domain or IP address

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
}
EOF'

# Enable the new site by creating a symbolic link
sudo ln -s /etc/nginx/sites-available/chatrs /etc/nginx/sites-enabled/

# Test Nginx configuration for syntax errors
sudo nginx -t

# Reload Nginx to apply changes
sudo systemctl reload nginx

mkdir chatrs
cd chatrs

wget -O Cargo.toml https://raw.githubusercontent.com/boundenergyinnovations/assistant_chatrs_dq/main/Cargo.toml
mkdir src
cd src
wget -O main.rs https://raw.githubusercontent.com/boundenergyinnovations/assistant_chatrs_dq/main/src/main.rs
wget -O routes.rs https://raw.githubusercontent.com/boundenergyinnovations/assistant_chatrs_dq/main/src/routes.rs
wget -O openai.rs https://raw.githubusercontent.com/boundenergyinnovations/assistant_chatrs_dq/main/src/openai.rs
cd ..
mkdir static
cd static
wget -O index.html https://raw.githubusercontent.com/boundenergyinnovations/assistant_chatrs_dq/main/static/index.html
wget -O index_code.html https://raw.githubusercontent.com/boundenergyinnovations/assistant_chatrs_dq/main/static/index_code.html
wget -O script.js https://raw.githubusercontent.com/boundenergyinnovations/assistant_chatrs_dq/main/static/script.js
wget -O styles.css https://raw.githubusercontent.com/boundenergyinnovations/assistant_chatrs_dq/main/static/styles.css
cd ..
wget -O chatrs_dq https://raw.githubusercontent.com/boundenergyinnovations/assistant_chatrs_dq/main/releases/chatrs_dq

