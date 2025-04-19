FROM ubuntu:latest

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y curl sudo wget git 
# Install essential packages

# Install IPM
COPY opt/ipm /opt/ipm
RUN chmod +x /opt/ipm/bin/ipm
RUN apt-get full-upgrade -y
RUN apt-get autoremove -y --purge
RUN apt-get purge -y apt --allow-remove-essential
RUN ln -s /opt/ipm/bin/ipm /usr/local/bin/ipm
RUN ipm www add apt https://archive.ubuntu.com/ubuntu --suites=plucky,plucky-updates,plucky-backports --components=main,restricted,universe,multiverse --architectures=amd64
RUN ipm www update
CMD ["/bin/bash"]