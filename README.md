# bei - assistant_chatrs_dq (Direct Query)  

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Usage](#usage)
4. [Contributing](#contributing)

## Introduction
The Concept:  
Imagine this: instead of navigating through a complex website trying to locate the information you need, you can simply watch a video that explains everything about the business. And if you have any questions? There's a chat feature ready to provide instant answers. No more digging through menus or struggling to find what you're looking for—just straightforward, easy access to the information you need.

Now, here's where it gets even more interesting. Whether you're on your laptop, tablet, or phone, you can use a simple curl command (or any code you're comfortable with) to interact directly with the site’s agent. This means you can pull information or perform actions on the site from any terminal, on any device, without the hassle of complex inputs or web scraping.

We believe that personal agents are the future. By calling a site's API directly, you can get the answers you need or interact with the site in the fastest, most cost-effective way possible. It's all about cutting out the unnecessary steps and giving you direct access to what you need, when you need it. Simple. Efficient. Powerful. 



We are using the server for our site: https://boundenergyinnovations.com


## Installation
NEED: AWS account or VM/VPS, OpenAI account, api key, assistant id. Tested and implemented on Ubuntu.

Start EC2/server with Ubuntu and settings for access to public HTTP/HTTPS, will need to set networking/VPC <-- video incomming.  


Copy and run setup script:
```sh
sh <(curl https://raw.githubusercontent.com/boundenergyinnovations/assistant_chatrs_dq/main/setup_chatrs_server.sh || wget -O - https://raw.githubusercontent.com/boundenergyinnovations/assistant_chatrs_dq/main/setup_chatrs_server.sh)
```

Edit Ngninx config:
```sh
cd /etc/nginx/sites-available/
sudo nano chatrs
```
Put your public IP or domain name in the file

Reload Nginx:
```sh
sudo systemctl reload nginx
```

Set keys/ids:
```sh
export OPENAI_API_KEY=sk-proj-xxxxxxxxxxxxx
export OPENAI_ASSISTANT_ID=asst_xxxxxxxx
```

OPTIONAL:
Install HTTPS SSL CERT with certbot:
```sh
sudo apt update
sudo apt install certbot python3-certbot-nginx
```

Get the cert *replace example.com with your url and don't forget to edit the Nginx config in /etc/nginx/sites-available/:
```sh
sudo certbot --nginx -d example.com
```

Check the cert:
```sh
sudo certbot certificates
```


## Usage
TO TEST:
```sh
chmod +x chatrs  
./chatrs
```

TO RUN:
```sh
nohup ./chatrs &
```
*this allows you to close the terminal keep it running

While logged in you can run:
```sh
jobs
```
this will show it currently running, type the number to select, generally '1' then CTRL-C to stop.

Or: 
```sh
ps -aux
````
to show what's running, then:
```sh
kill #
```
where # is the PID. eg. 'kill 66123'


## Contributing
The more the merrier. Any issues, give context. 
