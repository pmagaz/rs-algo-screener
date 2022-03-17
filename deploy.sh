#!/bin/sh
set -e
PS3='Please enter your choice: '
options=("deploy all" "build & deploy all" "build & deploy rs-algo-backend" "build & deploy rs-algo-screener" "Quit")
select opt in "${options[@]}"
do
    case $opt in
        "build & deploy all")
            echo "Deploying: $opt";
            docker login ; docker build -t pmagaz/rs-algo-backend:latest rs_algo_backend ; docker push pmagaz/rs-algo-backend:latest ; docker build -t pmagaz/rs-algo-screener:latest rs_algo_screener ; docker push pmagaz/rs-algo-screener:latest ; ansible-playbook playbook.yml  
            break
            ;;
        "deploy all")
            echo "Deploying: $opt";
            ansible-playbook playbook.yml
            break
            ;;
        "build & deploy rs-algo-backend")
            echo "Deploying: $opt";
            docker login ;  docker build -t pmagaz/rs-algo-backend:latest rs_algo_backend ; docker push pmagaz/rs-algo-backend:latest ; ansible-playbook playbook.yml 
            break
            ;;
        "build & deploy rs-algo-screener")
            echo "Deploying: $opt";
            docker login ; docker build -t pmagaz/rs-algo-screener:latest rs_algo_screener ; docker push pmagaz/rs-algo-screener:latest ; ansible-playbook playbook.yml 
            break
            ;;
        "Quit")
            break
            ;;
        *) echo "invalid option $REPLY";;
    esac
done
