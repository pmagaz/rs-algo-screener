#!/bin/sh
set -e
PS3='Please enter your choice: '
options=("build & deploy all" "build all" "deploy all" "build & deploy rs-algo-backend" "build & deploy rs-algo-scanner" "build & deploy rs-algo-client" "Quit")
select opt in "${options[@]}"
do
    case $opt in
        "build & deploy all")
            echo "Deploying: $opt";
            docker build -t cluster.loc:5000/rs-algo-backend:latest rs_algo_backend  ; docker build -t cluster.loc:5000/rs-algo-scanner:latest rs_algo_scanner ;  docker build -t cluster.loc:5000/rs-algo-client:latest rs_algo_client ; docker push cluster.loc:5000/rs-algo-backend:latest ; docker push cluster.loc:5000/rs-algo-scanner:latest ; docker push cluster.loc:5000/rs-algo-client:latest ; ansible-playbook playbook.yml  
            break
            ;;
        "build all")
            echo "Deploying: $opt";
            docker build -t cluster.loc:5000/rs-algo-backend:latest rs_algo_backend  ; docker build -t cluster.loc:5000/rs-algo-scanner:latest rs_algo_scanner ;  docker build -t cluster.loc:5000/rs-algo-client:latest rs_algo_client ; docker push cluster.loc:5000/rs-algo-backend:latest ; docker push cluster.loc:5000/rs-algo-scanner:latest ; docker push cluster.loc:5000/rs-algo-client:latest ;
            break
            ;;
        "deploy all")
            echo "Deploying: $opt";
            ansible-playbook playbook.yml
            break
            ;;
        "build & deploy rs-algo-backend")
            echo "Deploying: $opt";
            docker build -t cluster.loc:5000/rs-algo-backend:latest rs_algo_backend ; docker push cluster.loc:5000/rs-algo-backend:latest ; ansible-playbook playbook.yml 
            break
            ;;
        "build & deploy rs-algo-scanner")
            echo "Deploying: $opt";
            docker build -t cluster.loc:5000/rs-algo-scanner:latest rs_algo_scanner ; docker push cluster.loc:5000/rs-algo-scanner:latest ; ansible-playbook playbook.yml 
            break
            ;;
        "build & deploy rs-algo-client")
            echo "Deploying: $opt";
            docker build -t cluster.loc:5000/rs-algo-client:latest rs_algo_client ; docker push cluster.loc:5000/rs-algo-client:latest ; ansible-playbook playbook.yml 
            break
            ;;
        "Quit")
            break
            ;;
        *) echo "invalid option $REPLY";;
    esac
done
