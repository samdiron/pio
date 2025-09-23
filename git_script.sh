#!/bin/bash

cd ~
cd my_ip
git add ./*
git commit -a -m "updating public ip"
git push 
cd ~ 

