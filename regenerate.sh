#!/bin/sh
set -e
PS3='Please enter your choice: '
options=("regenerate screener" "regenerate backtest" "regenerate all" "Quit")
select opt in "${options[@]}"
do
    case $opt in
        "regenerate screener")
            echo "Regenerating: $opt";
            cargo run --bin rs_algo_scanner ;
            break
            ;;
        "regenerate backtest")
            echo "Deploying: $opt";
            cargo run --bin rs_algo_backtest ;
            break
            ;;
        "regenerate all")
            echo "Deploying: $opt";
            cargo run --bin rs_algo_scanner ; cargo run --bin rs_algo_backtest ;
            break
            ;;
        "Quit")
            break
            ;;
        *) echo "invalid option $REPLY";;
    esac
done
